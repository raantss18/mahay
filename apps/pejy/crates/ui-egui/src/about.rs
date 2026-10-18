//! Help › About Pejy: tabs About · Contributors · Models. About is the splash with the
//! Mahay mark, version, and community links (Discord first and largest), plus the other Mahay
//! apps; Contributors and Models are the compiled-in credits (`crate::credits`).

use pejy_engine::links;
use egui::{Color32, Rect, Sense, Stroke, pos2, vec2};
use serde_json::json;

use crate::DesignApp;
use crate::theme::{Tokens, semibold};

/// Mahay brand blue (from `docs/brand/mahay-mark.svg`).
pub const BRAND: Color32 = Color32::from_rgb(0x4e, 0x7b, 0xfb);
/// Discord's brand colour, used for the "Join the discussions" buttons (colour only; no Discord artwork).
pub const DISCORD: Color32 = Color32::from_rgb(0x58, 0x65, 0xf2);

/// The Mahay mark outline (viewBox -10 -19.42 136.34 136.34), from `docs/brand/mahay-mark.svg`.
const MARK_PATH: &str = "M104.28,49.49L81.55,0h-31.23l-3.17,4.76L14.75,53.63,0,75.85l21.55,21.55,63.79-36.94,16.99,37.04,14.01-21.74-12.06-26.27ZM32.89,65.66l32.42-48.87,10.91,23.77-43.32,25.09Z";

/// The other Mahay apps (app-page slug, name, what it is) — same list as the README.
pub const SIBLINGS: &[(&str, &str, &str)] = &[
    ("sary", "Sary", "image editing"),
    ("tsilo", "Tsilo", "vector illustration"),
    ("hetsika", "Hetsika", "video editing, color and sound"),
    ("hazavana", "Hazavana", "photo library and raw development"),
    ("taratasy", "Taratasy", "reading, organizing and protecting PDFs"),
    ("tselatra", "Tselatra", "motion graphics and visual effects"),
];

/// The About window's tabs, in `UiState::about_tab` order (`help.about {tab}` names them in lowercase).
pub const ABOUT_TABS: [&str; 3] = ["About", "Contributors", "Models"];

/// Paint the Mahay mark into `rect` (square), rasterised once per size and cached.
pub fn paint_mark(ui: &egui::Ui, rect: Rect, color: Color32) {
    use pejy_render::vello_cpu::{self, kurbo, peniko};
    let ctx = ui.ctx();
    let ppp = ctx.pixels_per_point();
    let px = ((rect.width().min(rect.height()) * ppp).round() as u32).clamp(8, 1024);
    let key = egui::Id::new(("mahay_mark", px, color.to_array()));
    let tex: Option<egui::TextureHandle> = ctx.data(|d| d.get_temp(key));
    let tex = tex.unwrap_or_else(|| {
        let mut rc = vello_cpu::RenderContext::new_with(px as u16, px as u16, vello_cpu::RenderSettings { num_threads: 0, ..Default::default() });
        let k = px as f64 / 136.34;
        rc.set_transform(kurbo::Affine::scale(k) * kurbo::Affine::translate((10.0, 19.42)));
        let [r, g, b, a] = color.to_srgba_unmultiplied();
        rc.set_paint(peniko::Color::from_rgba8(r, g, b, a));
        if let Ok(p) = kurbo::BezPath::from_svg(MARK_PATH) {
            rc.fill_path(&p);
        }
        rc.flush();
        let mut pm = vello_cpu::Pixmap::new(px as u16, px as u16);
        let mut res = vello_cpu::Resources::new();
        rc.render(&mut pm, &mut res);
        let ci = egui::ColorImage::from_rgba_premultiplied([px as usize, px as usize], pm.data_as_u8_slice());
        let t = crate::widgets::load_texture(ctx, "mahay_mark", ci, egui::TextureOptions::LINEAR);
        ctx.data_mut(|d| d.insert_temp(key, t.clone()));
        t
    });
    let side = rect.width().min(rect.height());
    let r = Rect::from_center_size(rect.center(), vec2(side, side));
    ui.painter().image(tex.id(), r, Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE);
}

/// A speech-bubble glyph (drawn in code) for Discord buttons.
pub fn paint_chat_icon(p: &egui::Painter, c: egui::Pos2, s: f32, color: Color32) {
    let body = Rect::from_center_size(c - vec2(0.0, s * 0.06), vec2(s, s * 0.72));
    p.rect_filled(body, s * 0.24, color);
    let tail = vec![
        pos2(body.left() + s * 0.22, body.bottom() - 1.0),
        pos2(body.left() + s * 0.12, body.bottom() + s * 0.24),
        pos2(body.left() + s * 0.44, body.bottom() - 1.0),
    ];
    p.add(egui::Shape::convex_polygon(tail, color, Stroke::NONE));
    let eye = s * 0.09;
    let bg = if color.r() as u32 + color.g() as u32 + color.b() as u32 > 380 { DISCORD } else { Color32::WHITE };
    p.circle_filled(body.center() - vec2(s * 0.18, 0.0), eye, bg);
    p.circle_filled(body.center() + vec2(s * 0.18, 0.0), eye, bg);
}

/// The big "Join the discussions" button. Returns true when clicked.
pub fn discord_button(ui: &mut egui::Ui, label: &str, size: egui::Vec2) -> bool {
    let (r, resp) = ui.allocate_exact_size(size, Sense::click());
    let resp = resp.on_hover_text(links::DISCORD).on_hover_cursor(egui::CursorIcon::PointingHand);
    let fill = if resp.hovered() { Color32::from_rgb(0x47, 0x52, 0xc4) } else { DISCORD };
    ui.painter().rect_filled(r, size.y / 2.0, fill);
    let icon = size.y * 0.5;
    let font = semibold((size.y * 0.42).max(11.0));
    let galley = crate::rtl::plain(ui.ctx(), label, font, Color32::WHITE);
    let total = icon + 8.0 + galley.size().x;
    let x0 = r.center().x - total / 2.0;
    paint_chat_icon(ui.painter(), pos2(x0 + icon / 2.0, r.center().y), icon, Color32::WHITE);
    ui.painter().galley(pos2(x0 + icon + 8.0, r.center().y - galley.size().y / 2.0), galley, Color32::WHITE);
    resp.clicked()
}

/// A text link that opens `url`. Returns true when clicked.
fn link(ui: &mut egui::Ui, text: &str, url: &str, color: Color32) -> bool {
    let resp = ui.add(egui::Label::new(egui::RichText::new(text).size(13.0).color(color).underline()).sense(Sense::click()));
    let resp = resp.on_hover_text(url).on_hover_cursor(egui::CursorIcon::PointingHand);
    resp.clicked()
}

pub fn show(app: &mut DesignApp, ctx: &egui::Context) {
    if !app.ui.about {
        return;
    }
    let t = Tokens::get(ctx);
    let mut open: Option<&'static str> = None;
    let area = egui::Id::new("about");
    let instance = area.with("placement");
    let resp = crate::modal_drag::modal(ctx, area, instance).show(ctx, |ui| {
        ui.set_width(520.0);
        mahay_about::page(ui, mahay_about::AppInfo { id: "pejy", name: "Pejy" });
        ui.vertical_centered(|ui| {
            if ui.button(crate::i18n::tr(&app.ui.language, "  Close  ")).clicked() {
                app.ui.about = false;
            }
        });
    });
    if resp.should_close() {
        app.ui.about = false;
    }
    if let Some(k) = open {
        let _ = app.run(&format!("help.{k}"), json!({}));
    }
}
