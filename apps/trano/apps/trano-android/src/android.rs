use std::time::{Duration, Instant};

use eframe::egui;
use mahay_android::{AndroidApp, EguiBridge};
use trano_core::{MahayApp, SUITE};

use crate::text::{Lang, tagline, text};

/// The apps with an Android build in this release, in the order shown.
const ON_ANDROID: [&str; 5] = ["sary", "taratasy", "soratra", "tabilao", "seho"];
const RELEASES: &str = "https://github.com/raantss18/mahay/releases/latest";
const SOURCE: &str = "https://github.com/raantss18/mahay";

macro_rules! icon {
    ($id:literal) => {
        ($id, egui::include_image!(concat!("../../../../../brand/svg/", $id, ".svg")))
    };
}

fn icon_of(id: &str) -> Option<egui::ImageSource<'static>> {
    [
        icon!("mahay"),
        icon!("sary"),
        icon!("taratasy"),
        icon!("soratra"),
        icon!("tabilao"),
        icon!("seho"),
        icon!("tsipika"),
        icon!("endrika"),
        icon!("pejy"),
        icon!("hazavana"),
        icon!("feo"),
        icon!("hetsika"),
        icon!("tselatra"),
        icon!("drafitra"),
    ]
    .into_iter()
    .find(|(i, _)| *i == id)
    .map(|(_, src)| src)
}

#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    mahay_android::init(&app, "Trano");
    let options = eframe::NativeOptions { android_app: Some(app), renderer: eframe::Renderer::Wgpu, ..Default::default() };
    let result = eframe::run_native(
        "Mahay",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            let lang = Lang::from_tag(&mahay_android::locale());
            Ok(Box::new(Hub { lang, installed: Vec::new(), checked: None, bridge: EguiBridge::new(&cc.egui_ctx) }))
        }),
    );
    if let Err(e) = result {
        log::error!("Trano stopped: {e}");
    }
}

struct Hub {
    lang: Lang,
    /// Ids of the installed apps, refreshed every few seconds (apps come and go while we're away).
    installed: Vec<&'static str>,
    checked: Option<Instant>,
    bridge: EguiBridge,
}

impl Hub {
    fn refresh(&mut self, ctx: &egui::Context) {
        if self.checked.is_some_and(|t| t.elapsed() < Duration::from_secs(3)) {
            return;
        }
        self.checked = Some(Instant::now());
        self.installed = ON_ANDROID.into_iter().filter(|id| mahay_android::is_installed(&format!("org.mahay.{id}"))).collect();
        ctx.request_repaint_after(Duration::from_secs(3));
    }
}

fn tile(ui: &mut egui::Ui, lang: Lang, installed_ids: &[&str], app: &MahayApp, width: f32) {
    let t = text(lang);
    let available = ON_ANDROID.contains(&app.id);
    let installed = installed_ids.contains(&app.id);
    let [r, g, b] = app.color;
    let accent = egui::Color32::from_rgb(r, g, b);
    let frame = egui::Frame::new()
        .fill(ui.visuals().faint_bg_color)
        .corner_radius(16.0)
        .inner_margin(14.0)
        .stroke(ui.visuals().widgets.noninteractive.bg_stroke);
    frame.show(ui, |ui| {
        ui.set_width(width - 28.0);
        ui.set_min_height(150.0);
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                match icon_of(app.id) {
                    Some(src) => {
                        // Apps not on Android yet are shown faded.
                        let tint = if available { egui::Color32::WHITE } else { egui::Color32::from_white_alpha(110) };
                        ui.add(egui::Image::new(src).tint(tint).fit_to_exact_size(egui::vec2(52.0, 52.0)));
                    }
                    None => {
                        let (rect, _) = ui.allocate_exact_size(egui::vec2(52.0, 52.0), egui::Sense::hover());
                        ui.painter().rect_filled(rect, 12.0, accent.gamma_multiply(0.35));
                    }
                }
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new(app.name).size(20.0).strong());
                    ui.label(egui::RichText::new(tagline(lang, app.id, app.tagline)).size(14.0).weak());
                });
            });
            ui.add_space(10.0);
            if !available {
                ui.label(egui::RichText::new(t.soon).size(14.0).weak());
            } else if installed {
                let button = egui::Button::new(egui::RichText::new(t.open).size(16.0).color(egui::Color32::WHITE))
                    .fill(accent)
                    .min_size(egui::vec2(120.0, 44.0));
                if ui.add(button).clicked() {
                    mahay_android::launch_app(&format!("org.mahay.{}", app.id));
                }
            } else {
                let button = egui::Button::new(egui::RichText::new(t.get).size(16.0)).min_size(egui::vec2(120.0, 44.0));
                if ui.add(button).clicked() {
                    mahay_android::open_url(RELEASES);
                }
            }
        });
    });
}

impl eframe::App for Hub {
    fn raw_input_hook(&mut self, _ctx: &egui::Context, raw: &mut egui::RawInput) {
        self.bridge.raw_input_hook(raw);
    }

    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.refresh(ctx);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let t = text(self.lang);
        // Ported apps first, then the rest of the suite.
        let mut apps: Vec<&MahayApp> = SUITE.iter().filter(|a| ON_ANDROID.contains(&a.id)).collect();
        apps.extend(SUITE.iter().filter(|a| !ON_ANDROID.contains(&a.id)));
        let installed = self.installed.len();
        let (lang, installed_ids) = (self.lang, self.installed.clone());
        let bridge = &mut self.bridge;
        bridge.inset_ui(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let margin = if ui.available_width() > 700.0 { 32.0 } else { 16.0 };
                egui::Frame::new().inner_margin(margin).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        if let Some(src) = icon_of("mahay") {
                            ui.add(egui::Image::new(src).fit_to_exact_size(egui::vec2(56.0, 56.0)));
                        }
                        ui.vertical(|ui| {
                            ui.label(egui::RichText::new("Mahay").size(30.0).strong());
                            ui.label(egui::RichText::new(t.subtitle).size(15.0).weak());
                        });
                    });
                    ui.add_space(8.0);
                    ui.label(egui::RichText::new(format!("{}: {installed}/{}", t.installed, ON_ANDROID.len())).size(14.0).weak());
                    ui.add_space(12.0);
                    // Compact (phone): one column; medium: two; expanded (tablet landscape): three+.
                    let width = ui.available_width();
                    let columns = ((width / 300.0).floor() as usize).clamp(1, 4);
                    let gap = 12.0;
                    let tile_w = (width - gap * (columns as f32 - 1.0)) / columns as f32;
                    for row in apps.chunks(columns) {
                        ui.horizontal_top(|ui| {
                            ui.spacing_mut().item_spacing.x = gap;
                            for app in row {
                                tile(ui, lang, &installed_ids, app, tile_w);
                            }
                        });
                        ui.add_space(gap);
                    }
                    ui.add_space(8.0);
                    if ui.link(t.source).clicked() {
                        mahay_android::open_url(SOURCE);
                    }
                    ui.label(egui::RichText::new("Forked from ArtCraft · MIT OR Apache-2.0").size(12.0).weak());
                });
            });
        });
        bridge.after_ui(ui.ctx());
    }
}
