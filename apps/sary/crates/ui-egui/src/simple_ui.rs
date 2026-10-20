//! Sary's Simple workspace (decision D27): the same engine and canvas as Pro, with a task-first
//! shell on top. A top bar (open, undo, redo, hold to compare, export, Pro), the picture, and at the
//! bottom one labelled strip of tasks with the controls of the chosen task above it. It is the
//! default on a phone; Pro (the full workspace) is one tap away and the choice is remembered.
//!
//! "Adjust" and "Filters" are non-destructive: they drive named adjustment layers
//! (`sary_engine::simple`), so Pro shows what Simple did and one undo takes it back.

use egui::{Align, Color32, CornerRadius, Layout, Margin, RichText, Sense, Stroke, StrokeKind, vec2};
use sary_engine::simple::{self, Look, RANGE};
use serde_json::json;

use crate::SaryApp;
use crate::icons;
use crate::state::Tool;
use crate::theme::Tokens;

/// The tasks of the bottom strip.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Panel {
    #[default]
    Adjust,
    Filters,
    Crop,
    Draw,
    Text,
    Retouch,
}

const PANELS: [(Panel, &str, &str); 6] = [
    (Panel::Adjust, "sliders-horizontal", "adjust"),
    (Panel::Filters, "wand-sparkles", "filters"),
    (Panel::Crop, "crop", "crop"),
    (Panel::Draw, "brush", "draw"),
    (Panel::Text, "type", "text"),
    (Panel::Retouch, "bandage-sparkle", "retouch"),
];

/// Looks: a name key and the slider values it stands for (light, contrast, colour, warmth).
const FILTERS: [(&str, [f32; 4]); 8] = [
    ("original", [0.0, 0.0, 0.0, 0.0]),
    ("vivid", [5.0, 15.0, 45.0, 0.0]),
    ("warm", [5.0, 5.0, 10.0, 45.0]),
    ("cool", [0.0, 5.0, 5.0, -45.0]),
    ("soft", [15.0, -15.0, 0.0, 10.0]),
    ("faded", [10.0, -30.0, -25.0, 5.0]),
    ("dramatic", [-10.0, 45.0, 10.0, -5.0]),
    ("mono", [0.0, 25.0, -100.0, 0.0]),
];

/// Crop shapes: key and width:height (0:0 keeps the picture's own shape).
const RATIOS: [(&str, u32, u32); 6] = [("original", 0, 0), ("square", 1, 1), ("r45", 4, 5), ("r43", 4, 3), ("r169", 16, 9), ("r916", 9, 16)];

/// Paint colours.
const COLORS: [[u8; 3]; 10] =
    [[0, 0, 0], [255, 255, 255], [229, 57, 53], [251, 140, 0], [253, 216, 53], [67, 160, 71], [30, 136, 229], [94, 53, 177], [233, 30, 99], [121, 85, 72]];

/// What the Simple workspace remembers while the app runs (nothing of it is saved).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct State {
    pub panel: Panel,
    look: Look,
    /// The document and revision `look` was last read from or written to.
    synced: Option<(sary_doc::DocId, u64)>,
    /// One key per slider drag: its edits share a history step.
    drag: u64,
    comparing: bool,
    erasing: bool,
    brush_px: f32,
    /// The size new text starts at, in points (0 = not chosen yet: a size that suits the picture).
    text_pt: f32,
    /// Which paint colour is chosen (index into [`COLORS`]).
    color: usize,
    /// A short message under the controls (a failed command), with the time it was set.
    note: Option<(String, f64)>,
}

/// Whether the Simple workspace is on: the user's choice, else Simple on a phone or tablet.
pub fn enabled(app: &SaryApp) -> bool {
    app.ui.simple_workspace.unwrap_or(cfg!(target_os = "android"))
}

pub fn set_enabled(app: &mut SaryApp, on: bool) {
    app.ui.simple_workspace = Some(on);
    if !on {
        // Back to the tools Pro starts with, and no half-finished compare.
        let _ = simple::set_compare(&mut app.session, false);
        app.ui.simple.comparing = false;
    }
}

// ---------- text (English, French, Malagasy) ----------

fn lang() -> mahay_about::Lang {
    match crate::i18n::current().code() {
        c if c.starts_with("fr") => mahay_about::Lang::Fr,
        _ => mahay_about::lang(),
    }
}

/// The text for `key` in the current language ("?" for a key without a translation; a test checks every key the screens use).
fn s(key: &str) -> &'static str {
    use mahay_about::Lang::{En, Fr, Mg};
    let row: (&'static str, &'static str, &'static str) = match key {
        "open" => ("Open", "Ouvrir", "Sokafy"),
        "undo" => ("Undo", "Annuler", "Foano"),
        "redo" => ("Redo", "Rétablir", "Averina"),
        "compare" => ("Hold to compare", "Maintenir pour comparer", "Tazomy hampitaha"),
        "compare_short" => ("Original", "Original", "Tany am-boalohany"),
        "export" => ("Export", "Exporter", "Avoaka"),
        "pro" => ("Pro", "Pro", "Pro"),
        "simple" => ("Simple", "Simple", "Tsotra"),
        "adjust" => ("Adjust", "Ajuster", "Fanitsiana"),
        "filters" => ("Filters", "Filtres", "Sivana"),
        "crop" => ("Crop", "Recadrer", "Fanapahana"),
        "draw" => ("Draw", "Dessiner", "Mamaky"),
        "text" => ("Text", "Texte", "Soratra"),
        "retouch" => ("Retouch", "Retoucher", "Fanamboarana"),
        "light" => ("Light", "Lumière", "Hazavana"),
        "contrast" => ("Contrast", "Contraste", "Fanalavirana"),
        "color" => ("Color", "Couleur", "Loko"),
        "warmth" => ("Warmth", "Chaleur", "Hafanana"),
        "enhance" => ("Enhance", "Améliorer", "Hatsarao"),
        "reset" => ("Reset", "Réinitialiser", "Averina amin'ny voalohany"),
        "original" => ("Original", "Original", "Tany am-boalohany"),
        "vivid" => ("Vivid", "Éclatant", "Mamirapiratra"),
        "warm" => ("Warm", "Chaud", "Mafana"),
        "cool" => ("Cool", "Froid", "Mangatsiaka"),
        "soft" => ("Soft", "Doux", "Malemy"),
        "faded" => ("Faded", "Délavé", "Nirevareva"),
        "dramatic" => ("Dramatic", "Dramatique", "Mahery vaika"),
        "mono" => ("Mono", "Noir et blanc", "Mainty sy fotsy"),
        "square" => ("Square", "Carré", "Efamira"),
        "r45" => ("4:5", "4:5", "4:5"),
        "r43" => ("4:3", "4:3", "4:3"),
        "r169" => ("16:9", "16:9", "16:9"),
        "r916" => ("9:16", "9:16", "9:16"),
        "rotate_left" => ("Rotate left", "Pivoter à gauche", "Alefa havia"),
        "rotate_right" => ("Rotate right", "Pivoter à droite", "Alefa havanana"),
        "flip" => ("Flip", "Miroir", "Hodinkaso"),
        "crop_help" => (
            "Pick a shape: the picture is cropped around its center.",
            "Choisissez une forme : l'image est recadrée autour de son centre.",
            "Mifidiana endrika: tapahina manodidina ny afovoany ny sary.",
        ),
        "size" => ("Size", "Taille", "Habe"),
        "eraser" => ("Eraser", "Gomme", "Fofoina"),
        "brush" => ("Brush", "Pinceau", "Borosy"),
        "text_help" => (
            "Tap the picture where the text goes, then type.",
            "Touchez l'image à l'endroit du texte, puis écrivez.",
            "Tsindrio ny toerana hapetrahana ny soratra, avy eo soraty.",
        ),
        "spot" => ("Spot heal", "Correcteur", "Fanasitranana"),
        "remove" => ("Remove", "Supprimer", "Esory"),
        "redeye" => ("Red eye", "Yeux rouges", "Mena ny maso"),
        "retouch_help" => (
            "Pick a tool, then paint over what you want gone.",
            "Choisissez un outil, puis passez sur ce que vous voulez effacer.",
            "Mifidiana fitaovana, avy eo alokoloho izay tianao hesorina.",
        ),
        "welcome" => ("Open a picture to start.", "Ouvrez une image pour commencer.", "Sokafy sary hanombohana."),
        "failed" => ("Could not do that:", "Impossible :", "Tsy nety:"),
        "dirty" => ("modified", "modifié", "novaina"),
        _ => ("?", "?", "?"),
    };
    match lang() {
        En => row.0,
        Fr => row.1,
        Mg => row.2,
    }
}

// ---------- chrome ----------

/// The Simple workspace's bars. Call where Pro would draw its panels; the canvas goes in the
/// central panel after it.
pub fn chrome(app: &mut SaryApp, ui: &mut egui::Ui) {
    sync_look(app);
    top_bar(app, ui);
    let has_doc = app.session.active().is_some();
    if !has_doc {
        empty_state(app, ui);
    } else if wide(ui) {
        side_layout(app, ui);
    } else {
        strip_layout(app, ui);
    }
}

/// A wide, low window (a tablet or a phone on its side): the tasks go in a rail on the left and
/// their controls in a panel on the right, so the picture keeps the room. Decided every frame
/// from the room left under the top bar, never from the device: a window resized, split or
/// rotated changes layout at once.
fn wide(ui: &egui::Ui) -> bool {
    let r = ui.available_rect_before_wrap();
    r.width() >= 640.0 && r.width() > r.height() * 1.15
}

/// A small "Simple" pill over Pro's bottom right corner, to come back to the Simple workspace.
pub fn pro_switch(app: &mut SaryApp, ctx: &egui::Context) {
    let t = Tokens::get(ctx);
    let mut go = false;
    egui::Area::new(egui::Id::new("simple-switch")).anchor(egui::Align2::RIGHT_BOTTOM, vec2(-12.0, -56.0)).order(egui::Order::Foreground).show(ctx, |ui| {
        let r =
            ui.add(egui::Button::new(RichText::new(s("simple")).color(t.accent_text).size(13.0)).fill(t.accent).corner_radius(14.0).min_size(vec2(64.0, 28.0)));
        go = r.clicked();
    });
    if go {
        set_enabled(app, true);
    }
}

/// Read the sliders back from the document when it, or what is in it, changed under us.
fn sync_look(app: &mut SaryApp) {
    let now = app.session.active().map(|st| (st.doc.id, st.revision, simple::look_of(&st.doc)));
    let Some((id, rev, look)) = now else {
        app.ui.simple.synced = None;
        return;
    };
    if app.ui.simple.synced != Some((id, rev)) {
        app.ui.simple.look = look;
        app.ui.simple.synced = Some((id, rev));
    }
}

fn apply(app: &mut SaryApp, look: Look, dragging: bool) {
    let key = format!("simple-{}", app.ui.simple.drag);
    match simple::apply_look(&mut app.session, look, dragging.then_some(key.as_str())) {
        Ok(()) => {
            app.ui.simple.look = look;
            app.ui.simple.synced = app.session.active().map(|st| (st.doc.id, st.revision));
        }
        Err(e) => app.ui.simple.note = Some((format!("{} {e}", s("failed")), 0.0)),
    }
}

fn top_bar(app: &mut SaryApp, ui: &mut egui::Ui) {
    let ctx = ui.ctx().clone();
    let t = Tokens::get(&ctx);
    let (can_undo, can_redo, name) = match app.session.active() {
        Some(st) => (st.history.can_undo(), st.history.can_redo(), format!("{}{}", st.doc.name, if st.is_dirty() { " •" } else { "" })),
        None => (false, false, String::new()),
    };
    let has_doc = app.session.active().is_some();
    let mut comparing = false;
    egui::Panel::top("simple_top").exact_size(52.0).frame(egui::Frame::NONE.fill(t.chrome).inner_margin(Margin::symmetric(8, 4))).show(ui, |ui| {
        ui.horizontal_centered(|ui| {
            if labelled(ui, "folder-open", s("open"), false, true).clicked() {
                let _ = crate::menus::invoke(app, &ctx, "file.open", json!({}));
            }
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if labelled(ui, "panels-top-left", s("pro"), false, true).clicked() {
                    set_enabled(app, false);
                }
                if has_doc && labelled(ui, "image", s("export"), false, true).clicked() {
                    let _ = crate::menus::invoke(app, &ctx, "file.export.exportAs", json!({}));
                }
                if has_doc {
                    let hold = icons::button(ui, "eye", 40.0, app.ui.simple.comparing, s("compare"));
                    comparing = hold.is_pointer_button_down_on();
                    if can_redo && icons::button(ui, "redo-2", 40.0, false, s("redo")).clicked() {
                        let _ = app.run("edit.redo", json!({}));
                    }
                    if can_undo && icons::button(ui, "undo-2", 40.0, false, s("undo")).clicked() {
                        let _ = app.run("edit.undo", json!({}));
                    }
                    // The name takes what room is left, shortened.
                    let room = ui.available_width();
                    if room > 90.0 {
                        ui.add(egui::Label::new(RichText::new(name).color(t.text_dim).size(13.0)).truncate());
                    }
                }
            });
        });
    });
    if has_doc {
        if comparing != app.ui.simple.comparing {
            app.ui.simple.comparing = comparing;
            ctx.request_repaint();
        }
        let _ = simple::set_compare(&mut app.session, comparing);
    }
}

/// An icon with a label under or beside it, as a flat button.
fn labelled(ui: &mut egui::Ui, icon: &str, text: &str, selected: bool, beside: bool) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let galley = ui.painter().layout_no_wrap(text.to_string(), egui::FontId::proportional(13.0), t.text);
    let size = if beside { vec2(galley.size().x + 44.0, 40.0) } else { vec2(0.0, 0.0) };
    let (rect, r) = ui.allocate_exact_size(size, Sense::click());
    let fill = if selected {
        t.accent_soft
    } else if r.hovered() {
        t.hover
    } else {
        Color32::TRANSPARENT
    };
    ui.painter().rect_filled(rect, CornerRadius::same(10), fill);
    let tint = if selected { t.accent } else { t.text };
    icons::paint(ui, egui::Rect::from_center_size(egui::pos2(rect.left() + 20.0, rect.center().y), vec2(24.0, 24.0)), icon, 20.0, tint);
    ui.painter().galley(egui::pos2(rect.left() + 36.0, rect.center().y - galley.size().y / 2.0), galley, tint);
    r.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, text));
    r
}

/// Phone shape: the controls above a strip of tasks along the bottom.
fn strip_layout(app: &mut SaryApp, ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    let mut chosen = None;
    egui::Panel::bottom("simple_tasks").exact_size(62.0).frame(egui::Frame::NONE.fill(t.chrome).inner_margin(Margin::symmetric(4, 4))).show(ui, |ui| {
        let w = ui.available_width() / PANELS.len() as f32;
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
            for (panel, icon, key) in PANELS {
                chosen = task_button(app, ui, panel, icon, key, vec2(w, 54.0)).or(chosen);
            }
        });
    });
    choose(app, chosen);
    egui::Panel::bottom("simple_options").frame(egui::Frame::NONE.fill(t.card).inner_margin(Margin::symmetric(14, 10))).show(ui, |ui| {
        // The same height for every task, so the picture does not jump when the task changes.
        ui.set_min_height(200.0);
        options_body(app, ui);
    });
}

/// Tablet or landscape shape: a rail of tasks on the left, their controls on the right.
fn side_layout(app: &mut SaryApp, ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    let mut chosen = None;
    egui::Panel::left("simple_rail").exact_size(88.0).resizable(false).frame(egui::Frame::NONE.fill(t.chrome).inner_margin(Margin::symmetric(4, 6))).show(
        ui,
        |ui| {
            egui::ScrollArea::vertical().id_salt("simple-rail").show(ui, |ui| {
                ui.spacing_mut().item_spacing = vec2(0.0, 4.0);
                for (panel, icon, key) in PANELS {
                    chosen = task_button(app, ui, panel, icon, key, vec2(ui.available_width(), 64.0)).or(chosen);
                }
            });
        },
    );
    choose(app, chosen);
    // A share of the width, within what the controls need.
    let width = (ui.available_width() * 0.34).clamp(300.0, 420.0);
    egui::Panel::right("simple_options_side")
        .exact_size(width)
        .resizable(false)
        .frame(egui::Frame::NONE.fill(t.card).inner_margin(Margin::symmetric(14, 10)))
        .show(ui, |ui| {
            options_body(app, ui);
        });
}

fn choose(app: &mut SaryApp, chosen: Option<Panel>) {
    if let Some(panel) = chosen {
        app.ui.simple.panel = panel;
        app.ui.simple.note = None;
        select_tool_for(app, panel);
    }
}

/// One task of the rail or strip: an icon over its label.
fn task_button(app: &SaryApp, ui: &mut egui::Ui, panel: Panel, icon: &str, key: &str, size: egui::Vec2) -> Option<Panel> {
    let t = Tokens::get(ui.ctx());
    let (rect, r) = ui.allocate_exact_size(size, Sense::click());
    let on = app.ui.simple.panel == panel;
    if on {
        ui.painter().rect_filled(rect.shrink2(vec2(3.0, 0.0)), CornerRadius::same(12), t.accent_soft);
    } else if r.hovered() {
        ui.painter().rect_filled(rect.shrink2(vec2(3.0, 0.0)), CornerRadius::same(12), t.hover);
    }
    let tint = if on { t.accent } else { t.text_dim };
    icons::paint(ui, egui::Rect::from_center_size(egui::pos2(rect.center().x, rect.top() + 18.0), vec2(26.0, 26.0)), icon, 22.0, tint);
    let label = s(key);
    ui.painter().text(egui::pos2(rect.center().x, rect.bottom() - 11.0), egui::Align2::CENTER_CENTER, label, egui::FontId::proportional(12.0), tint);
    r.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, label));
    r.clicked().then_some(panel)
}

/// The canvas tool a task uses: touch drags move the view unless the task paints or types.
fn select_tool_for(app: &mut SaryApp, panel: Panel) {
    app.ui.tool = match panel {
        Panel::Adjust | Panel::Filters | Panel::Crop => Tool::Hand,
        Panel::Draw => {
            if app.ui.simple.erasing {
                Tool::Eraser
            } else {
                Tool::Brush
            }
        }
        Panel::Text => Tool::Type,
        Panel::Retouch => match app.ui.tool {
            Tool::SpotHealing | Tool::Remove | Tool::RedEye => app.ui.tool,
            _ => Tool::SpotHealing,
        },
    };
    if app.ui.simple.brush_px < 1.0 {
        app.ui.simple.brush_px = app.session.tools.brush.size.max(8.0);
    }
}

/// The controls of the chosen task. Scrolls when the room is short (a small window, the keyboard up).
fn options_body(app: &mut SaryApp, ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    egui::ScrollArea::vertical().id_salt("simple-options").auto_shrink([false, true]).show(ui, |ui| {
        ui.spacing_mut().slider_width = (ui.available_width() - 150.0).max(100.0);
        ui.spacing_mut().interact_size.y = 32.0;
        ui.spacing_mut().item_spacing = vec2(8.0, 6.0);
        match app.ui.simple.panel {
            Panel::Adjust => adjust_panel(app, ui),
            Panel::Filters => filters_panel(app, ui),
            Panel::Crop => crop_panel(app, ui),
            Panel::Draw => draw_panel(app, ui),
            Panel::Text => {
                ui.label(RichText::new(s("text_help")).color(t.text_dim));
                text_size(app, ui);
                colors(app, ui);
            }
            Panel::Retouch => retouch_panel(app, ui),
        }
        if let Some((msg, _)) = &app.ui.simple.note {
            ui.colored_label(t.danger, msg);
        }
    });
}

fn adjust_panel(app: &mut SaryApp, ui: &mut egui::Ui) {
    let look = app.ui.simple.look;
    let mut v = [look.light, look.contrast, look.color, look.warmth];
    let mut changed = false;
    let mut stopped = false;
    for (i, key) in ["light", "contrast", "color", "warmth"].into_iter().enumerate() {
        ui.horizontal(|ui| {
            ui.add_sized([80.0, 30.0], egui::Label::new(s(key)));
            let r = ui.add(egui::Slider::new(&mut v[i], -RANGE..=RANGE).trailing_fill(true).step_by(1.0).smart_aim(true));
            changed |= r.changed();
            stopped |= r.drag_stopped() || r.lost_focus();
        });
    }
    if changed {
        apply(app, Look { light: v[0], contrast: v[1], color: v[2], warmth: v[3] }, true);
    }
    if stopped {
        app.ui.simple.drag += 1;
    }
    ui.horizontal(|ui| {
        if ui.button(RichText::new(s("enhance")).size(14.0)).clicked() {
            app.ui.simple.drag += 1;
            apply(app, Look { light: 8.0, contrast: 12.0, color: 18.0, warmth: 0.0 }, false);
        }
        if ui.button(RichText::new(s("reset")).size(14.0)).clicked() {
            app.ui.simple.drag += 1;
            apply(app, Look::default(), false);
        }
    });
}

fn filters_panel(app: &mut SaryApp, ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    let look = app.ui.simple.look;
    let mut pick = None;
    {
        ui.horizontal_wrapped(|ui| {
            for (key, vals) in FILTERS {
                let target = Look { light: vals[0], contrast: vals[1], color: vals[2], warmth: vals[3] };
                let on = [look.light, look.contrast, look.color, look.warmth].iter().zip(vals).all(|(a, b)| (a - b).abs() < 1.0);
                let (rect, r) = ui.allocate_exact_size(vec2(84.0, 48.0), Sense::click());
                let fill = if on {
                    t.accent_soft
                } else if r.hovered() {
                    t.hover
                } else {
                    t.field
                };
                ui.painter().rect(rect, CornerRadius::same(12), fill, Stroke::new(1.0, if on { t.accent } else { t.field_border }), StrokeKind::Inside);
                ui.painter().text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    s(key),
                    egui::FontId::proportional(13.0),
                    if on { t.accent_text } else { t.text },
                );
                r.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, s(key)));
                if r.clicked() {
                    pick = Some(target);
                }
            }
        });
    }
    if let Some(look) = pick {
        app.ui.simple.drag += 1;
        apply(app, look, false);
    }
}

fn crop_panel(app: &mut SaryApp, ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    ui.label(RichText::new(s("crop_help")).color(t.text_dim));
    let mut crop = None;
    let mut turn = None;
    ui.horizontal_wrapped(|ui| {
        for (key, w, h) in RATIOS {
            if ui.button(RichText::new(s(key)).size(14.0)).clicked() {
                crop = Some((w, h));
            }
        }
    });
    ui.horizontal_wrapped(|ui| {
        for (key, id) in
            [("rotate_left", "image.imageRotation.90ccw"), ("rotate_right", "image.imageRotation.90cw"), ("flip", "image.imageRotation.flipCanvasHorizontal")]
        {
            if ui.button(RichText::new(s(key)).size(14.0)).clicked() {
                turn = Some(id);
            }
        }
    });
    // The picture changes size or shape: refit it to the screen afterwards.
    let mut changed = Ok(Default::default());
    if let Some((w, h)) = crop
        && w != 0
        && let Some(p) = centered_crop(app, w, h)
    {
        changed = app.run("image.crop", p);
    }
    if let Some(id) = turn {
        changed = app.run(id, json!({}));
    }
    match changed {
        Ok(_) => {
            if crop.is_some_and(|(w, _)| w != 0) || turn.is_some() {
                let _ = app.run("view.fitOnScreen", json!({}));
            }
        }
        Err(e) => app.ui.simple.note = Some((format!("{} {e}", s("failed")), 0.0)),
    }
}

/// `image.crop` params for the largest `w:h` rectangle centred in the active picture.
fn centered_crop(app: &SaryApp, w: u32, h: u32) -> Option<serde_json::Value> {
    let st = app.session.active()?;
    let (iw, ih) = (st.doc.size.width, st.doc.size.height);
    let (x, y, cw, ch) = crop_rect(iw, ih, w, h)?;
    Some(json!({"x": x, "y": y, "width": cw, "height": ch}))
}

/// The largest `w:h` rectangle centred in an `iw × ih` picture, as `(x, y, width, height)`.
pub(crate) fn crop_rect(iw: u32, ih: u32, w: u32, h: u32) -> Option<(u32, u32, u32, u32)> {
    if iw == 0 || ih == 0 || w == 0 || h == 0 {
        return None;
    }
    // Compare iw/ih with w/h in integers: wider than the target crops the width.
    let (cw, ch) = if u64::from(iw) * u64::from(h) >= u64::from(ih) * u64::from(w) {
        ((u64::from(ih) * u64::from(w) / u64::from(h)) as u32, ih)
    } else {
        (iw, (u64::from(iw) * u64::from(h) / u64::from(w)) as u32)
    };
    let (cw, ch) = (cw.clamp(1, iw), ch.clamp(1, ih));
    Some(((iw - cw) / 2, (ih - ch) / 2, cw, ch))
}

fn draw_panel(app: &mut SaryApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        let t = Tokens::get(ui.ctx());
        for (on, key, tool) in [(!app.ui.simple.erasing, "brush", Tool::Brush), (app.ui.simple.erasing, "eraser", Tool::Eraser)] {
            let (rect, r) = ui.allocate_exact_size(vec2(96.0, 36.0), Sense::click());
            let fill = if on {
                t.accent_soft
            } else if r.hovered() {
                t.hover
            } else {
                t.field
            };
            ui.painter().rect(rect, CornerRadius::same(10), fill, Stroke::new(1.0, if on { t.accent } else { t.field_border }), StrokeKind::Inside);
            ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, s(key), egui::FontId::proportional(13.0), if on { t.accent_text } else { t.text });
            if r.clicked() {
                app.ui.simple.erasing = tool == Tool::Eraser;
                app.ui.tool = tool;
            }
        }
    });
    size_slider(app, ui);
    colors(app, ui);
}

fn retouch_panel(app: &mut SaryApp, ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    ui.label(RichText::new(s("retouch_help")).color(t.text_dim));
    ui.horizontal_wrapped(|ui| {
        for (key, tool) in [("spot", Tool::SpotHealing), ("remove", Tool::Remove), ("redeye", Tool::RedEye)] {
            let on = app.ui.tool == tool;
            let (rect, r) = ui.allocate_exact_size(vec2(110.0, 36.0), Sense::click());
            let fill = if on {
                t.accent_soft
            } else if r.hovered() {
                t.hover
            } else {
                t.field
            };
            ui.painter().rect(rect, CornerRadius::same(10), fill, Stroke::new(1.0, if on { t.accent } else { t.field_border }), StrokeKind::Inside);
            ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, s(key), egui::FontId::proportional(13.0), if on { t.accent_text } else { t.text });
            if r.clicked() {
                app.ui.tool = tool;
            }
        }
    });
    size_slider(app, ui);
}

/// The size new text starts at: a slider, first set to what suits the picture (a fourteenth of its height).
fn text_size(app: &mut SaryApp, ui: &mut egui::Ui) {
    if app.ui.simple.text_pt < 1.0 {
        let h = app.session.active().map_or(1000.0, |st| st.doc.size.height as f32);
        set_text_size(app, (h / 14.0).clamp(12.0, 160.0));
    }
    ui.horizontal(|ui| {
        ui.add_sized([80.0, 30.0], egui::Label::new(s("size")));
        let mut pt = app.ui.simple.text_pt;
        if ui.add(egui::Slider::new(&mut pt, 8.0..=400.0).logarithmic(true).step_by(1.0)).changed() {
            set_text_size(app, pt);
        }
    });
}

fn set_text_size(app: &mut SaryApp, pt: f32) {
    use sary_doc::text::{CharStyle, ParagraphStyle};
    app.ui.simple.text_pt = pt;
    let (mut style, para) = app
        .session
        .type_defaults
        .clone()
        .unwrap_or_else(|| (CharStyle { font_family: sary_text::fonts::DEFAULT_FAMILY.into(), ..Default::default() }, ParagraphStyle::default()));
    style.size_pt = pt;
    app.session.type_defaults = Some((style, para));
}

fn size_slider(app: &mut SaryApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.add_sized([80.0, 30.0], egui::Label::new(s("size")));
        let mut px = app.ui.simple.brush_px.max(1.0);
        if ui.add(egui::Slider::new(&mut px, 1.0..=300.0).logarithmic(true).step_by(1.0)).changed() {
            app.ui.simple.brush_px = px;
            app.session.tools.brush.size = px;
            app.session.tools.brush_changed();
        }
    });
}

fn colors(app: &mut SaryApp, ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal_wrapped(|ui| {
        // Ten dots fit one row of a narrow phone (411 dp).
        ui.spacing_mut().item_spacing.x = 4.0;
        for (i, c) in COLORS.iter().enumerate() {
            let (rect, r) = ui.allocate_exact_size(vec2(34.0, 34.0), Sense::click());
            let col = Color32::from_rgb(c[0], c[1], c[2]);
            ui.painter().circle_filled(rect.center(), 13.0, col);
            let on = app.ui.simple.color == i;
            ui.painter().circle_stroke(
                rect.center(),
                if on { 15.5 } else { 13.0 },
                Stroke::new(if on { 2.5 } else { 1.0 }, if on { t.accent } else { t.field_border }),
            );
            if r.clicked() {
                app.ui.simple.color = i;
                app.session.tools.foreground = [f32::from(c[0]) / 255.0, f32::from(c[1]) / 255.0, f32::from(c[2]) / 255.0, 1.0];
            }
        }
    });
}

fn empty_state(app: &mut SaryApp, ui: &mut egui::Ui) {
    let ctx = ui.ctx().clone();
    let t = Tokens::get(&ctx);
    egui::Panel::bottom("simple_empty").frame(egui::Frame::NONE.fill(t.card).inner_margin(Margin::symmetric(14, 14))).show(ui, |ui| {
        ui.vertical_centered(|ui| {
            ui.label(RichText::new(s("welcome")).size(16.0).color(t.text));
            if ui
                .add(egui::Button::new(RichText::new(s("open")).size(16.0).color(t.accent_text)).fill(t.accent).min_size(vec2(160.0, 44.0)).corner_radius(14.0))
                .clicked()
            {
                let _ = crate::menus::invoke(app, &ctx, "file.open", json!({}));
            }
        });
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crop_rect_is_centred_and_has_the_ratio() {
        // A 4000 × 3000 photo to a square, to 16:9 and to 9:16.
        assert_eq!(crop_rect(4000, 3000, 1, 1), Some((500, 0, 3000, 3000)));
        assert_eq!(crop_rect(4000, 3000, 16, 9), Some((0, 375, 4000, 2250)));
        assert_eq!(crop_rect(4000, 3000, 9, 16), Some((1156, 0, 1687, 3000)));
    }

    #[test]
    fn crop_rect_never_leaves_the_picture_or_divides_by_zero() {
        assert_eq!(crop_rect(0, 10, 1, 1), None);
        assert_eq!(crop_rect(10, 10, 0, 1), None);
        for (iw, ih) in [(1, 1), (1, 5000), (5000, 1), (7, 3)] {
            for (_, w, h) in RATIOS {
                if let Some((x, y, cw, ch)) = crop_rect(iw, ih, w, h) {
                    assert!(cw >= 1 && ch >= 1 && x + cw <= iw && y + ch <= ih, "{iw}x{ih} {w}:{h}");
                }
            }
        }
    }

    #[test]
    fn every_text_key_the_ui_uses_has_all_three_languages() {
        let keys = [
            "open",
            "undo",
            "redo",
            "compare",
            "export",
            "pro",
            "simple",
            "adjust",
            "filters",
            "crop",
            "draw",
            "text",
            "retouch",
            "light",
            "contrast",
            "color",
            "warmth",
            "enhance",
            "reset",
            "square",
            "rotate_left",
            "rotate_right",
            "flip",
            "crop_help",
            "size",
            "eraser",
            "brush",
            "text_help",
            "spot",
            "remove",
            "redeye",
            "retouch_help",
            "welcome",
            "failed",
        ];
        for k in keys.into_iter().chain(FILTERS.iter().map(|f| f.0)).chain(RATIOS.iter().map(|r| r.0)) {
            assert_ne!(s(k), "?", "missing text for {k}");
        }
    }
}
