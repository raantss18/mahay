//! What eframe doesn't do on Android: the soft keyboard and the system clipboard.

/// Call [`EguiBridge::after_ui`] at the end of every frame.
#[derive(Debug, Default)]
pub struct EguiBridge {
    keyboard: bool,
    /// Last insets read, in physical pixels, and when.
    insets: [i32; 4],
    insets_read: Option<std::time::Instant>,
}

impl EguiBridge {
    pub fn new(ctx: &egui::Context) -> Self {
        let ctx = ctx.clone();
        crate::set_waker(move || ctx.request_repaint());
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
        let wants = ctx.output(|o| o.ime.is_some());
        if wants != self.keyboard {
            self.keyboard = wants;
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
        let paste_key = raw.events.iter().any(|e| {
            matches!(e, egui::Event::Key { key: egui::Key::V, pressed: true, modifiers, .. } if modifiers.command)
        });
        if paste_key
            && !raw.events.iter().any(|e| matches!(e, egui::Event::Paste(_)))
            && let Some(text) = crate::clipboard_text()
        {
            raw.events.push(egui::Event::Paste(text));
        }
    }
}
