//! What eframe doesn't do on Android: the soft keyboard and the system clipboard.

/// Call [`EguiBridge::after_ui`] at the end of every frame.
#[derive(Debug, Default)]
pub struct EguiBridge {
    keyboard: bool,
    /// Last insets read, in physical pixels, and when.
    insets: [i32; 4],
    insets_read: Option<std::time::Instant>,
    /// The keyboard buffer as last turned into events, and the field it belongs to.
    ime_seen: String,
    ime_field: Option<egui::Id>,
    /// Whether the system bars were last set for a dark theme.
    dark_bars: Option<bool>,
    /// When and where (in points) the screen was last touched or clicked.
    last_tap: Option<(std::time::Instant, egui::Pos2)>,
}

impl EguiBridge {
    /// Set up the bridge for `app` and start its automatic updates (checked at start-up, then
    /// every 24 hours; the update bar appears when one is downloaded and verified).
    pub fn new(ctx: &egui::Context, app: mahay_about::AppInfo) -> Self {
        let waker_ctx = ctx.clone();
        crate::set_waker(move || waker_ctx.request_repaint());
        let lang = mahay_about::Lang::from_tag(&crate::locale());
        let dir = crate::data_dir().unwrap_or_else(|| std::path::PathBuf::from("/data/local/tmp"));
        let repaint = ctx.clone();
        let checker = mahay_update::Checker::new(app.id, dir, true, move || repaint.request_repaint());
        mahay_about::register(app, lang, checker, Box::new(|update| {
            if !mahay_update::still_valid(update) {
                return Err("The downloaded update was changed on disk; it will be downloaded again.".into());
            }
            crate::install_apk(&update.path)
        }));
        Self::default()
    }

    /// Run the app's `ui` inside the area the system bars, display cutout and soft keyboard
    /// leave free. Android 15+ draws apps edge to edge, under those bars.
    pub fn inset_ui(&mut self, ui: &mut egui::Ui, add: impl FnOnce(&mut egui::Ui)) {
        let stale = self.insets_read.is_none_or(|t| t.elapsed() >= std::time::Duration::from_millis(400));
        if stale {
            self.insets = crate::system_insets();
            self.insets_read = Some(std::time::Instant::now());
        }
        let ppp = ui.ctx().pixels_per_point().max(0.1);
        let [l, t, r, b] = self.insets.map(|v| v.max(0) as f32 / ppp);
        let full = ui.max_rect();
        // Behind the status and navigation bars: the app's own background, not black.
        ui.painter().rect_filled(full, 0.0, ui.visuals().panel_fill);
        let rect = egui::Rect::from_min_max(full.min + egui::vec2(l, t), full.max - egui::vec2(r, b));
        if rect.width() < 1.0 || rect.height() < 1.0 {
            add(ui);
            return;
        }
        ui.scope_builder(egui::UiBuilder::new().max_rect(rect), add);
    }

    /// Show the soft keyboard while a text field has focus, and copy what the app copied to the
    /// Android clipboard (egui-winit has no clipboard on Android).
    pub fn after_ui(&mut self, ctx: &egui::Context) {
        mahay_about::banner(ctx);
        let dark = ctx.global_style().visuals.dark_mode;
        if self.dark_bars != Some(dark) {
            self.dark_bars = Some(dark);
            crate::set_dark_bars(dark);
        }
        // Only a field the user tapped brings the keyboard up: a dialog that focuses its first
        // field by itself would otherwise cover half the screen with a keyboard nobody asked for.
        let ime_rect = ctx.output(|o| o.ime.map(|ime| ime.rect));
        let tapped_field = match (self.last_tap, ime_rect) {
            (Some((at, pos)), Some(rect)) => at.elapsed() < std::time::Duration::from_millis(700) && rect.expand(8.0).contains(pos),
            _ => false,
        };
        let wants = ime_rect.is_some() && (self.keyboard || tapped_field);
        let field = ctx.memory(|m| m.focused());
        if wants && self.ime_field.is_some() && field != self.ime_field {
            // Another text field: start from an empty keyboard buffer. (Not on the first one:
            // what was typed to start editing may be in the buffer already.)
            crate::reset_ime();
            self.ime_seen.clear();
        }
        self.ime_field = if wants { field } else { None };
        if wants {
            // Keyboard text arrives without waking eframe; look for it while the keyboard is up.
            ctx.request_repaint_after(std::time::Duration::from_millis(80));
        }
        if wants != self.keyboard {
            self.keyboard = wants;
            if !wants {
                crate::reset_ime();
                self.ime_seen.clear();
            }
            crate::set_keyboard(wants);
            // The keyboard slides in over a few hundred milliseconds: read the insets again then,
            // so the content moves above it. (Rotation and resizes repaint on their own.)
            self.insets_read = None;
            ctx.request_repaint_after(std::time::Duration::from_millis(450));
        }
        let copied: Vec<String> = ctx.output(|o| {
            o.commands
                .iter()
                .filter_map(|c| match c {
                    egui::OutputCommand::CopyText(t) => Some(t.clone()),
                    _ => None,
                })
                .collect()
        });
        if let Some(text) = copied.last() {
            crate::set_clipboard_text(text);
        }
    }

    /// A paste event with the Android clipboard's text, for a Ctrl+V from a hardware keyboard
    /// (egui-winit can't read the clipboard on Android, so it sends nothing).
    pub fn raw_input_hook(&mut self, raw: &mut egui::RawInput) {
        let tap = raw.events.iter().rev().find_map(|e| match e {
            egui::Event::PointerButton { pressed: true, pos, .. } => Some(*pos),
            egui::Event::Touch { phase: egui::TouchPhase::Start, pos, .. } => Some(*pos),
            _ => None,
        });
        if let Some(pos) = tap {
            self.last_tap = Some((std::time::Instant::now(), pos));
        }
        if self.keyboard {
            let now = crate::ime_text();
            if now != self.ime_seen {
                raw.events.extend(ime_events(&self.ime_seen, &now));
                self.ime_seen = now;
            }
        }
        let paste_key =
            raw.events.iter().any(|e| matches!(e, egui::Event::Key { key: egui::Key::V, pressed: true, modifiers, .. } if modifiers.command));
        if paste_key
            && !raw.events.iter().any(|e| matches!(e, egui::Event::Paste(_)))
            && let Some(text) = crate::clipboard_text()
        {
            raw.events.push(egui::Event::Paste(text));
        }
    }
}

/// The egui events that turn the keyboard buffer `before` into `after`: one Backspace per
/// character after their common prefix, then the new text. Word completion and autocorrect
/// rewrite the buffer this way too.
fn ime_events(before: &str, after: &str) -> Vec<egui::Event> {
    let common = before.chars().zip(after.chars()).take_while(|(a, b)| a == b).count();
    let removed = before.chars().count().saturating_sub(common);
    let added: String = after.chars().skip(common).collect();
    let mut events = Vec::new();
    for _ in 0..removed {
        for pressed in [true, false] {
            events.push(egui::Event::Key { key: egui::Key::Backspace, physical_key: None, pressed, repeat: false, modifiers: egui::Modifiers::NONE });
        }
    }
    // Enter comes through the buffer as a line break: send it as the key.
    for (i, line) in added.split('\n').enumerate() {
        if i > 0 {
            for pressed in [true, false] {
                events.push(egui::Event::Key { key: egui::Key::Enter, physical_key: None, pressed, repeat: false, modifiers: egui::Modifiers::NONE });
            }
        }
        if !line.is_empty() {
            events.push(egui::Event::Text(line.to_string()));
        }
    }
    events
}

#[cfg(test)]
mod tests {
    use super::ime_events;

    #[test]
    fn typing_appends_and_autocorrect_rewrites() {
        assert!(matches!(ime_events("Ma", "Mah").as_slice(), [egui::Event::Text(t)] if t == "h"));
        // "teh" corrected to "the": two backspaces, then "he".
        let ev = ime_events("teh", "the");
        assert_eq!(ev.len(), 5);
        assert!(matches!(ev.last(), Some(egui::Event::Text(t)) if t == "he"));
        assert!(ime_events("abc", "abc").is_empty());
        assert_eq!(ime_events("é", "").len(), 2);
        // "ok" then Enter: the text, then Enter pressed and released.
        let ev = ime_events("", "ok\n");
        assert_eq!(ev.len(), 3);
        assert!(matches!(ev[1], egui::Event::Key { key: egui::Key::Enter, pressed: true, .. }));
    }
}
