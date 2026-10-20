//! Hetsika's Simple workspace (decision D27): a video *story* made in a few steps, on a phone as on a
//! tablet. A top bar (open, undo, redo, export, Pro), the picture, the story as a row of clips, and
//! six labelled tasks (Add, Edit, Text, Speed, Sound, Export) with their controls. The same project and
//! engine as Pro: every action is one engine command (`hetsika_engine::simple`), so Pro shows the very
//! same timeline and one undo takes each step back.

use egui::{Align, Color32, CornerRadius, Layout, Margin, Rect, RichText, Sense, Stroke, StrokeKind, pos2, vec2};
use hetsika_engine::simple::{self, StoryClip};
use hetsika_project::{ItemId, ItemKind};
use serde_json::json;

use crate::HetsikaApp;
use crate::icons::{self, Icon};
use crate::native_dialogs::FileDialog;
use crate::panels::monitor::{self, Which};
use crate::state::Mode;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Panel {
    #[default]
    Add,
    Edit,
    Text,
    Speed,
    Sound,
    Export,
}

const PANELS: [(Panel, Icon, &str); 6] = [
    (Panel::Add, Icon::Plus, "add"),
    (Panel::Edit, Icon::Razor, "edit"),
    (Panel::Text, Icon::Type, "text"),
    (Panel::Speed, Icon::Stopwatch, "speed"),
    (Panel::Sound, Icon::Speaker, "sound"),
    (Panel::Export, Icon::Export, "export"),
];

const SPEEDS: [f64; 6] = [25.0, 50.0, 100.0, 150.0, 200.0, 400.0];

/// What the Simple workspace remembers while the app runs.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct State {
    pub panel: Panel,
    /// The title being typed.
    title: String,
    /// A short message under the controls: what could not be done.
    note: Option<String>,
}

/// Whether the Simple workspace is on: the user's choice, else Simple on a phone or tablet.
pub fn enabled(app: &HetsikaApp) -> bool {
    app.ui.simple_workspace.unwrap_or(cfg!(target_os = "android"))
}

pub fn set_enabled(app: &mut HetsikaApp, on: bool) {
    app.ui.simple_workspace = Some(on);
    if on {
        // Simple works in the Edit mode; Import has no place in it.
        if app.ui.mode == Mode::Import {
            app.ui.mode = Mode::Edit;
        }
    }
}

// ---------- text (English, French, Malagasy) ----------

fn lang() -> mahay_about::Lang {
    match crate::i18n::current().code() {
        c if c.starts_with("fr") => mahay_about::Lang::Fr,
        _ => mahay_about::lang(),
    }
}

/// The text for `key` in the current language ("?" for a key without a translation; a test checks the keys).
fn s(key: &str) -> &'static str {
    use mahay_about::Lang::{En, Fr, Mg};
    let row: (&'static str, &'static str, &'static str) = match key {
        "open" => ("Open", "Ouvrir", "Sokafy"),
        "undo" => ("Undo", "Annuler", "Foano"),
        "redo" => ("Redo", "Rétablir", "Averina"),
        "export" => ("Export", "Exporter", "Avoaka"),
        "pro" => ("Pro", "Pro", "Pro"),
        "simple" => ("Simple", "Simple", "Tsotra"),
        "back" => ("Back", "Retour", "Hiverina"),
        "add" => ("Add", "Ajouter", "Ampio"),
        "edit" => ("Edit", "Montage", "Fanapahana"),
        "text" => ("Text", "Texte", "Soratra"),
        "speed" => ("Speed", "Vitesse", "Hafainganana"),
        "sound" => ("Sound", "Son", "Feo"),
        "welcome" => ("Make a video story: add clips, cut them, add a title, export.", "Faites une vidéo : ajoutez des clips, coupez-les, ajoutez un titre, exportez.", "Manaova horonantsary: ampio ny sary mihetsika, tapaho, ampio lohateny, avoaka."),
        "start" => ("Start a story", "Commencer une vidéo", "Atombohy ny horonantsary"),
        "add_media" => ("Add video or photo", "Ajouter une vidéo ou une photo", "Ampio horonantsary na sary"),
        "add_music" => ("Add music", "Ajouter de la musique", "Ampio mozika"),
        "add_demo" => ("Try with a demo clip", "Essayer avec un clip de démonstration", "Andramo amin'ny clip fanandramana"),
        "story_empty" => ("The story is empty: add a clip.", "La vidéo est vide : ajoutez un clip.", "Foana ny horonantsary: ampio clip."),
        "cut_here" => ("Cut here", "Couper ici", "Tapaho eto"),
        "keep_from" => ("Keep from here", "Garder à partir d'ici", "Tehirizo manomboka eto"),
        "keep_until" => ("Keep until here", "Garder jusqu'ici", "Tehirizo hatramin'ny eto"),
        "remove" => ("Remove clip", "Supprimer le clip", "Esory ny clip"),
        "earlier" => ("Earlier", "Avant", "Aloha"),
        "later" => ("Later", "Après", "Aoriana"),
        "pick_clip" => ("Tap a clip in the story first.", "Touchez d'abord un clip de la vidéo.", "Tsindrio aloha clip ao amin'ny horonantsary."),
        "title_hint" => ("Title text", "Texte du titre", "Lahatsoratry ny lohateny"),
        "add_title" => ("Add title at the playhead", "Ajouter le titre à la position de lecture", "Ampio lohateny eo amin'ny toerana alefa"),
        "speed_help" => ("Speed of the chosen clip: the clips after it follow.", "Vitesse du clip choisi : les clips suivants suivent.", "Hafainganan'ny clip voafidy: manaraka izy ireo."),
        "volume" => ("Volume (dB)", "Volume (dB)", "Hafanky (dB)"),
        "sound_help" => ("Volume of the chosen clip, or music under the whole story.", "Volume du clip choisi, ou musique sous toute la vidéo.", "Hafanky ny clip voafidy, na mozika ho an'ny horonantsary manontolo."),
        "export_help" => ("Choose the size and where to save it; Export makes the video file.", "Choisissez la taille et l'endroit ; Exporter crée le fichier vidéo.", "Fidio ny habe sy ny toerana; ny Avoaka no manao ny rakitra horonantsary."),
        "export_go" => ("Export the video…", "Exporter la vidéo…", "Avoaka ny horonantsary…"),
        "failed" => ("Could not do that:", "Impossible :", "Tsy nety:"),
        "seconds" => ("s", "s", "s"),
        "no_clip" => ("(no clip)", "(aucun clip)", "(tsy misy clip)"),
        _ => ("?", "?", "?"),
    };
    match lang() {
        En => row.0,
        Fr => row.1,
        Mg => row.2,
    }
}

// ---------- the screen ----------

/// Draw the Simple workspace over `full` (replaces the header, the panels and the status bar).
pub fn show(app: &mut HetsikaApp, ui: &mut egui::Ui, full: Rect) {
    let t = app.tokens;
    let mut child = ui.new_child(egui::UiBuilder::new().max_rect(full).layout(Layout::top_down(Align::Min)));
    let ui = &mut child;
    top_bar(app, ui);
    if app.ui.mode == Mode::Export {
        // The full export screen, with the bar above to get back.
        let rest = ui.available_rect_before_wrap();
        crate::panels::export_mode::show(app, ui, rest);
        return;
    }
    if app.ui.mode == Mode::Import {
        app.ui.mode = Mode::Edit;
    }
    if app.session.active_sequence().is_none() {
        empty_state(app, ui);
        return;
    }
    let rest = ui.available_rect_before_wrap();
    if wide(rest) {
        side_layout(app, ui, rest);
    } else {
        strip_layout(app, ui, rest);
    }
    let _ = t;
}

/// A wide, low window: the tasks go in a rail and their controls at the right, the picture keeps the room.
fn wide(r: Rect) -> bool {
    r.width() >= 700.0 && r.width() > r.height() * 1.15
}

fn top_bar(app: &mut HetsikaApp, ui: &mut egui::Ui) {
    let ctx = ui.ctx().clone();
    let t = app.tokens;
    let exporting = app.ui.mode == Mode::Export;
    let (can_undo, can_redo) = (app.session.history.can_undo(), app.session.history.can_redo());
    egui::Panel::top("het_simple_top").exact_size(52.0).frame(egui::Frame::NONE.fill(t.header_bg).inner_margin(Margin::symmetric(8, 4))).show(ui, |ui| {
        ui.horizontal_centered(|ui| {
            if exporting {
                if labelled(ui, Icon::ChevronRight, s("back"), t).clicked() {
                    app.ui.mode = Mode::Edit;
                }
            } else if labelled(ui, Icon::Folder, s("open"), t).clicked() {
                let _ = crate::menus::invoke(app, &ctx, "file.open", json!({}));
            }
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if labelled(ui, Icon::Workspaces, s("pro"), t).clicked() {
                    set_enabled(app, false);
                }
                if !exporting {
                    if can_redo && icons::button(ui, Icon::Redo, 36.0, false, &t, s("redo")).clicked() {
                        let _ = crate::menus::invoke(app, &ctx, "edit.redo", json!({}));
                    }
                    if can_undo && icons::button(ui, Icon::Undo, 36.0, false, &t, s("undo")).clicked() {
                        let _ = crate::menus::invoke(app, &ctx, "edit.undo", json!({}));
                    }
                }
            });
        });
    });
}

/// An icon with a label beside it, as a flat button.
fn labelled(ui: &mut egui::Ui, icon: Icon, text: &str, t: crate::theme::Tokens) -> egui::Response {
    let galley = ui.painter().layout_no_wrap(text.to_string(), egui::FontId::proportional(13.0), t.text);
    let (rect, r) = ui.allocate_exact_size(vec2(galley.size().x + 46.0, 40.0), Sense::click());
    let fill = if r.hovered() { t.hover } else { Color32::TRANSPARENT };
    ui.painter().rect_filled(rect, CornerRadius::same(10), fill);
    icons::paint(ui.painter(), Rect::from_center_size(pos2(rect.left() + 22.0, rect.center().y), vec2(22.0, 22.0)), icon, t.text);
    ui.painter().galley(pos2(rect.left() + 38.0, rect.center().y - galley.size().y / 2.0), galley, t.text);
    r.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, text));
    r
}

fn empty_state(app: &mut HetsikaApp, ui: &mut egui::Ui) {
    let ctx = ui.ctx().clone();
    let t = app.tokens;
    ui.add_space(40.0);
    ui.vertical_centered(|ui| {
        ui.add_space(24.0);
        ui.label(RichText::new(s("welcome")).size(17.0).color(t.text));
        ui.add_space(16.0);
        if ui.add(egui::Button::new(RichText::new(s("start")).size(17.0).color(t.text)).fill(t.accent).min_size(vec2(220.0, 48.0)).corner_radius(14.0)).clicked() {
            match new_story(app) {
                Ok(()) => add_media(app, &ctx),
                Err(e) => app.ui.simple.note = Some(format!("{} {e}", s("failed"))),
            }
        }
        if let Some(n) = &app.ui.simple.note {
            ui.add_space(8.0);
            ui.colored_label(Color32::from_rgb(0xE5, 0x39, 0x35), n);
        }
    });
}

/// A new sequence (the story) in the open project.
fn new_story(app: &mut HetsikaApp) -> Result<(), String> {
    app.session.execute("file.newSequence", json!({"name": "Story"})).map(|_| ()).map_err(|e| e.to_string())
}

// ---------- layouts ----------

/// Phone shape: the picture, the story, the controls and the row of tasks, top to bottom.
fn strip_layout(app: &mut HetsikaApp, ui: &mut egui::Ui, rest: Rect) {
    let t = app.tokens;
    let tasks_h = 62.0;
    let options_h = 170.0_f32.min(rest.height() * 0.30);
    let strip_h = 92.0;
    let mut chosen = None;
    egui::Panel::bottom("het_simple_tasks").exact_size(tasks_h).frame(egui::Frame::NONE.fill(t.header_bg).inner_margin(Margin::symmetric(4, 4))).show(ui, |ui| {
        let w = ui.available_width() / PANELS.len() as f32;
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
            for (panel, icon, key) in PANELS {
                chosen = task_button(app, ui, panel, icon, key, vec2(w, 54.0)).or(chosen);
            }
        });
    });
    choose(app, chosen);
    egui::Panel::bottom("het_simple_options").exact_size(options_h).frame(egui::Frame::NONE.fill(t.panel_bg).inner_margin(Margin::symmetric(12, 8))).show(ui, |ui| {
        options_body(app, ui);
    });
    egui::Panel::bottom("het_simple_story").exact_size(strip_h).frame(egui::Frame::NONE.fill(t.app_bg).inner_margin(Margin::symmetric(8, 6))).show(ui, |ui| {
        story_strip(app, ui);
    });
    let monitor_rect = ui.available_rect_before_wrap();
    monitor::show(app, ui, monitor_rect, Which::Program);
}

/// Tablet or landscape shape: a rail of tasks at the left, the picture and the story in the middle,
/// the controls at the right.
fn side_layout(app: &mut HetsikaApp, ui: &mut egui::Ui, rest: Rect) {
    let t = app.tokens;
    let mut chosen = None;
    egui::Panel::left("het_simple_rail").exact_size(88.0).resizable(false).frame(egui::Frame::NONE.fill(t.header_bg).inner_margin(Margin::symmetric(4, 6))).show(ui, |ui| {
        egui::ScrollArea::vertical().id_salt("het-simple-rail").show(ui, |ui| {
            ui.spacing_mut().item_spacing = vec2(0.0, 4.0);
            for (panel, icon, key) in PANELS {
                chosen = task_button(app, ui, panel, icon, key, vec2(ui.available_width(), 64.0)).or(chosen);
            }
        });
    });
    choose(app, chosen);
    let width = (rest.width() * 0.34).clamp(300.0, 420.0);
    egui::Panel::right("het_simple_options_side").exact_size(width).resizable(false).frame(egui::Frame::NONE.fill(t.panel_bg).inner_margin(Margin::symmetric(12, 8))).show(ui, |ui| {
        options_body(app, ui);
    });
    egui::Panel::bottom("het_simple_story_side").exact_size(92.0).frame(egui::Frame::NONE.fill(t.app_bg).inner_margin(Margin::symmetric(8, 6))).show(ui, |ui| {
        story_strip(app, ui);
    });
    let monitor_rect = ui.available_rect_before_wrap();
    monitor::show(app, ui, monitor_rect, Which::Program);
}

fn choose(app: &mut HetsikaApp, chosen: Option<Panel>) {
    if let Some(panel) = chosen {
        app.ui.simple.panel = panel;
        app.ui.simple.note = None;
    }
}

/// One task of the rail or strip: an icon over its label.
fn task_button(app: &HetsikaApp, ui: &mut egui::Ui, panel: Panel, icon: Icon, key: &str, size: egui::Vec2) -> Option<Panel> {
    let t = app.tokens;
    let (rect, r) = ui.allocate_exact_size(size, Sense::click());
    let on = app.ui.simple.panel == panel;
    if on {
        ui.painter().rect_filled(rect.shrink2(vec2(3.0, 0.0)), CornerRadius::same(12), t.pill_active_bg);
    } else if r.hovered() {
        ui.painter().rect_filled(rect.shrink2(vec2(3.0, 0.0)), CornerRadius::same(12), t.hover);
    }
    let tint = if on { t.accent } else { t.text_dim };
    icons::paint(ui.painter(), Rect::from_center_size(pos2(rect.center().x, rect.top() + 18.0), vec2(26.0, 26.0)), icon, tint);
    let label = s(key);
    ui.painter().text(pos2(rect.center().x, rect.bottom() - 11.0), egui::Align2::CENTER_CENTER, label, egui::FontId::proportional(12.0), tint);
    r.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, label));
    r.clicked().then_some(panel)
}

// ---------- the story ----------

fn story_strip(app: &mut HetsikaApp, ui: &mut egui::Ui) {
    let t = app.tokens;
    let clips: Vec<StoryClip> = simple::story(&app.session);
    if clips.is_empty() {
        ui.label(RichText::new(s("story_empty")).color(t.text_dim));
        return;
    }
    let selected: Vec<_> = app.session.state.selection.clone();
    let playhead = app.session.playhead();
    let mut pick = None;
    egui::ScrollArea::horizontal().id_salt("het-simple-story").auto_shrink([false, false]).show(ui, |ui| {
        ui.horizontal(|ui| {
            for clip in &clips {
                let (rect, r) = ui.allocate_exact_size(vec2(132.0, 76.0), Sense::click());
                let on = selected.contains(&clip.id);
                let here = clip.start <= playhead && playhead < clip.end();
                let fill = if on { t.pill_active_bg } else { t.clip_bar_bg };
                ui.painter().rect(rect, CornerRadius::same(10), fill, Stroke::new(if on { 2.0 } else { 1.0 }, if on { t.accent } else { t.field_border }), StrokeKind::Inside);
                if here {
                    ui.painter().rect_filled(Rect::from_min_size(rect.min + vec2(6.0, rect.height() - 8.0), vec2(rect.width() - 12.0, 3.0)), CornerRadius::same(2), t.accent);
                }
                let name = ui.painter().layout(clip.name.clone(), egui::FontId::proportional(13.0), t.text, rect.width() - 16.0);
                ui.painter().galley(rect.min + vec2(8.0, 8.0), name, t.text);
                let info = format!("{:.1} {}{}", clip.seconds(), s("seconds"), if (clip.speed - 1.0).abs() > 0.01 { format!(" · ×{:.2}", clip.speed) } else { String::new() });
                ui.painter().text(rect.left_bottom() + vec2(8.0, -14.0), egui::Align2::LEFT_CENTER, info, egui::FontId::proportional(12.0), t.text_dim);
                if r.clicked() {
                    pick = Some(clip.clone());
                }
            }
        });
    });
    if let Some(clip) = pick {
        app.stop();
        if let Err(e) = simple::select(&mut app.session, &clip) {
            app.ui.simple.note = Some(format!("{} {e}", s("failed")));
        }
    }
}

// ---------- the controls of the chosen task ----------

fn options_body(app: &mut HetsikaApp, ui: &mut egui::Ui) {
    let t = app.tokens;
    egui::ScrollArea::vertical().id_salt("het-simple-options").auto_shrink([false, true]).show(ui, |ui| {
        ui.spacing_mut().item_spacing = vec2(8.0, 6.0);
        ui.spacing_mut().interact_size.y = 32.0;
        match app.ui.simple.panel {
            Panel::Add => add_panel(app, ui),
            Panel::Edit => edit_panel(app, ui),
            Panel::Text => text_panel(app, ui),
            Panel::Speed => speed_panel(app, ui),
            Panel::Sound => sound_panel(app, ui),
            Panel::Export => export_panel(app, ui),
        }
        if let Some(msg) = &app.ui.simple.note {
            ui.colored_label(Color32::from_rgb(0xE5, 0x39, 0x35), msg);
        }
        let _ = t;
    });
}

fn button(ui: &mut egui::Ui, text: &str, enabled: bool) -> bool {
    ui.add_enabled(enabled, egui::Button::new(RichText::new(text).size(14.0)).min_size(vec2(0.0, 36.0))).clicked()
}

fn run(app: &mut HetsikaApp, f: impl FnOnce(&mut hetsika_engine::Session) -> hetsika_engine::Result<()>) {
    match f(&mut app.session) {
        Ok(()) => app.ui.simple.note = None,
        Err(e) => app.ui.simple.note = Some(format!("{} {e}", s("failed"))),
    }
}

fn add_panel(app: &mut HetsikaApp, ui: &mut egui::Ui) {
    let ctx = ui.ctx().clone();
    ui.horizontal_wrapped(|ui| {
        if button(ui, s("add_media"), true) {
            add_media(app, &ctx);
        }
        if button(ui, s("add_music"), true) {
            add_music(app, &ctx);
        }
        if button(ui, s("add_demo"), true) {
            add_demo(app);
        }
    });
}

fn edit_panel(app: &mut HetsikaApp, ui: &mut egui::Ui) {
    let has = simple::selected(&app.session).is_some();
    if !has {
        ui.label(RichText::new(s("pick_clip")).color(app.tokens.text_dim));
    }
    ui.horizontal_wrapped(|ui| {
        if button(ui, s("cut_here"), true) {
            app.stop();
            run(app, simple::split_at_playhead);
        }
        if button(ui, s("keep_from"), has) {
            app.stop();
            run(app, simple::trim_start_to_playhead);
        }
        if button(ui, s("keep_until"), has) {
            app.stop();
            run(app, simple::trim_end_to_playhead);
        }
        if button(ui, s("remove"), has) {
            app.stop();
            run(app, simple::remove_selected);
        }
        if button(ui, s("earlier"), has) {
            run(app, |sn| simple::move_selected(sn, -1));
        }
        if button(ui, s("later"), has) {
            run(app, |sn| simple::move_selected(sn, 1));
        }
    });
}

fn text_panel(app: &mut HetsikaApp, ui: &mut egui::Ui) {
    let mut title = std::mem::take(&mut app.ui.simple.title);
    ui.add(egui::TextEdit::singleline(&mut title).hint_text(s("title_hint")).desired_width(f32::INFINITY));
    if button(ui, s("add_title"), !title.trim().is_empty()) {
        let text = title.clone();
        run(app, |sn| simple::add_title(sn, &text));
        title.clear();
    }
    app.ui.simple.title = title;
}

fn speed_panel(app: &mut HetsikaApp, ui: &mut egui::Ui) {
    let t = app.tokens;
    ui.label(RichText::new(s("speed_help")).color(t.text_dim));
    let selected = simple::selected(&app.session);
    ui.horizontal_wrapped(|ui| {
        for pct in SPEEDS {
            let on = selected.as_ref().is_some_and(|c| (c.speed * 100.0 - pct).abs() < 1.0);
            let (rect, r) = ui.allocate_exact_size(vec2(72.0, 38.0), Sense::click());
            let fill = if on { t.pill_active_bg } else { t.field_bg };
            ui.painter().rect(rect, CornerRadius::same(10), fill, Stroke::new(1.0, if on { t.accent } else { t.field_border }), StrokeKind::Inside);
            ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, format!("{}%", pct as i32), egui::FontId::proportional(14.0), if on { t.accent } else { t.text });
            if r.clicked() && selected.is_some() {
                app.stop();
                run(app, |sn| simple::set_speed(sn, pct));
            }
        }
    });
    if selected.is_none() {
        ui.label(RichText::new(s("pick_clip")).color(t.text_dim));
    }
}

fn sound_panel(app: &mut HetsikaApp, ui: &mut egui::Ui) {
    let ctx = ui.ctx().clone();
    let t = app.tokens;
    ui.label(RichText::new(s("sound_help")).color(t.text_dim));
    if button(ui, s("add_music"), true) {
        add_music(app, &ctx);
    }
    if let Some(clip) = simple::selected(&app.session) {
        let gain = app.session.active_sequence().and_then(|q| q.find_item(clip.id).map(|(_, it)| it.gain_db)).unwrap_or(0.0) as f32;
        let mut db = gain;
        ui.horizontal(|ui| {
            ui.label(s("volume"));
            ui.spacing_mut().slider_width = (ui.available_width() - 90.0).max(100.0);
            if ui.add(egui::Slider::new(&mut db, -30.0..=12.0).step_by(1.0)).changed() {
                let key = format!("simple-gain-{}", clip.id.0);
                run(app, |sn| sn.execute("clip.audioGain", json!({"clips": [clip.id.0], "mode": "set", "db": db, "coalesce": key})).map(|_| ()));
            }
        });
    }
}

fn export_panel(app: &mut HetsikaApp, ui: &mut egui::Ui) {
    ui.label(RichText::new(s("export_help")).color(app.tokens.text_dim));
    if button(ui, s("export_go"), true) {
        app.stop();
        app.ui.mode = Mode::Export;
    }
}

// ---------- importing ----------

fn media_ids(app: &HetsikaApp) -> std::collections::BTreeSet<ItemId> {
    app.session.project.items.iter().filter(|(_, it)| matches!(it.kind, ItemKind::Media(_))).map(|(id, _)| *id).collect()
}

/// Ask for files, import them, and put what came in at the end of the story.
fn add_media(app: &mut HetsikaApp, _ctx: &egui::Context) {
    let exts: Vec<&str> = hetsika_media::VIDEO_EXTENSIONS.iter().chain(hetsika_media::STILL_EXTENSIONS).copied().collect();
    app.pick_ui(FileDialog::open_files(&exts), |app, paths| import_and_place(app, paths, false));
}

/// Ask for sound files and put the first under the whole story.
fn add_music(app: &mut HetsikaApp, _ctx: &egui::Context) {
    let exts: Vec<&str> = hetsika_media::AUDIO_EXTENSIONS.to_vec();
    app.pick_ui(FileDialog::open_files(&exts), |app, paths| import_and_place(app, paths, true));
}

fn import_and_place(app: &mut HetsikaApp, paths: Vec<String>, music: bool) {
    if paths.is_empty() {
        return;
    }
    if app.session.active_sequence().is_none()
        && let Err(e) = new_story(app)
    {
        app.ui.simple.note = Some(format!("{} {e}", s("failed")));
        return;
    }
    let before = media_ids(app);
    let bin = app.import_bin().0;
    if let Err(e) = app.session.execute("file.import", json!({"paths": paths, "bin": bin})) {
        app.ui.simple.note = Some(format!("{} {e}", s("failed")));
        return;
    }
    let new: Vec<ItemId> = media_ids(app).into_iter().filter(|i| !before.contains(i)).collect();
    place_items(app, &new, music);
}

fn place_items(app: &mut HetsikaApp, items: &[ItemId], music: bool) {
    for item in items {
        let r = if music {
            // Under the story, from its start, on the second sound track.
            app.session.execute("timeline.place", json!({"item": item.0, "track": "A2", "time": 0, "video": false})).map(|_| ())
        } else {
            simple::append(&mut app.session, *item)
        };
        if let Err(e) = r {
            app.ui.simple.note = Some(format!("{} {e}", s("failed")));
            return;
        }
        if music {
            break;
        }
    }
    app.ui.simple.note = None;
}

/// One demo scene as a first clip, for a try without any file.
fn add_demo(app: &mut HetsikaApp) {
    let before = media_ids(app);
    let scenes = ["OceanSunset", "Aurora", "CityNight", "Dunes", "Plasma", "Forest"];
    let n = simple::story(&app.session).len() % scenes.len();
    if let Err(e) = app.session.execute("file.importDemoFootage", json!({"scene": scenes[n]})) {
        app.ui.simple.note = Some(format!("{} {e}", s("failed")));
        return;
    }
    let new: Vec<ItemId> = media_ids(app).into_iter().filter(|i| !before.contains(i)).collect();
    place_items(app, &new, false);
}

/// A small "Simple" pill over Pro's bottom right corner, to come back to the Simple workspace.
pub fn pro_switch(app: &mut HetsikaApp, ctx: &egui::Context) {
    let t = app.tokens;
    let mut go = false;
    egui::Area::new(egui::Id::new("het-simple-switch")).anchor(egui::Align2::RIGHT_BOTTOM, vec2(-12.0, -34.0)).order(egui::Order::Foreground).show(ctx, |ui| {
        let r = ui.add(egui::Button::new(RichText::new(s("simple")).color(t.text).size(13.0)).fill(t.accent).corner_radius(14.0).min_size(vec2(64.0, 28.0)));
        go = r.clicked();
    });
    if go {
        set_enabled(app, true);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_text_key_the_screens_use_has_all_three_languages() {
        let keys = [
            "open", "undo", "redo", "export", "pro", "simple", "back", "add", "edit", "text", "speed", "sound", "welcome", "start", "add_media", "add_music", "add_demo", "story_empty", "cut_here",
            "keep_from", "keep_until", "remove", "earlier", "later", "pick_clip", "title_hint", "add_title", "speed_help", "volume", "sound_help", "export_help", "export_go", "failed", "seconds",
            "no_clip",
        ];
        for k in keys {
            assert_ne!(s(k), "?", "missing text for {k}");
        }
    }

    #[test]
    fn wide_means_a_wide_low_window() {
        let r = |w: f32, h: f32| Rect::from_min_size(pos2(0.0, 0.0), vec2(w, h));
        assert!(wide(r(1280.0, 600.0)));
        assert!(!wide(r(411.0, 800.0)));
        assert!(!wide(r(800.0, 900.0)), "a tall tablet in portrait stays in the phone shape");
    }
}
