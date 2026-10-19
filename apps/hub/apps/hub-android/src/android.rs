//! Mahay HUB on Android, the only app users download by hand: it installs the other Mahay apps
//! straight from the release (downloaded, checked against the signed checksum list, handed to
//! Android's installer), keeps them up to date ("Update", "Update all"), opens them, and updates
//! itself through the shared update bar.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, Instant};

use eframe::egui;
use mahay_android::{AndroidApp, EguiBridge};
use mahay_update::{Release, Target, is_newer};
use hub_core::{MahayApp, SUITE};

use crate::text::{Lang, tagline, text};

/// The apps with an Android build, in the order shown.
const ON_ANDROID: [&str; 13] = ["sary", "taratasy", "soratra", "tabilao", "seho", "tsilo", "endrika", "pejy", "hazavana", "feo", "hetsika", "tselatra", "drafitra"];
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
        icon!("tsilo"),
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
    mahay_android::init(&app, "Mahay HUB");
    let options = eframe::NativeOptions { android_app: Some(app), renderer: eframe::Renderer::Wgpu, ..Default::default() };
    let result = eframe::run_native(
        "Mahay",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            let lang = Lang::from_tag(&mahay_android::locale());
            let bridge = EguiBridge::new(&cc.egui_ctx, mahay_about::AppInfo { id: "hub", name: "Mahay HUB" });
            let hub = Hub::new(lang, bridge, cc.egui_ctx.clone());
            hub.fetch_release();
            Ok(Box::new(hub))
        }),
    );
    if let Err(e) = result {
        log::error!("Mahay HUB stopped: {e}");
    }
}

/// One app's download or install.
#[derive(Clone, Debug)]
enum Job {
    Downloading(f32),
    /// Handed to Android's installer; the user confirms there.
    Installing,
    Failed(String),
}

type Shared<T> = Arc<Mutex<T>>;

fn lock<T>(m: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    m.lock().unwrap_or_else(PoisonError::into_inner)
}

struct Hub {
    lang: Lang,
    ctx: egui::Context,
    bridge: EguiBridge,
    /// The latest release (`None` while loading; `Err` offline).
    release: Shared<Option<Result<Release, String>>>,
    jobs: Shared<HashMap<&'static str, Job>>,
    /// Installed version of each app, refreshed every few seconds.
    installed: HashMap<&'static str, String>,
    checked: Option<Instant>,
    show_about: bool,
}

impl Hub {
    fn new(lang: Lang, bridge: EguiBridge, ctx: egui::Context) -> Self {
        Hub {
            lang,
            ctx,
            bridge,
            release: Arc::default(),
            jobs: Arc::default(),
            installed: HashMap::new(),
            checked: None,
            show_about: false,
        }
    }

    fn fetch_release(&self) {
        let (slot, ctx) = (self.release.clone(), self.ctx.clone());
        *lock(&slot) = None;
        let spawned = std::thread::Builder::new().name("hub-release".into()).spawn(move || {
            let r = mahay_update::latest_release().map_err(|e| e.to_string());
            *lock(&slot) = Some(r);
            ctx.request_repaint();
        });
        if let Err(e) = spawned {
            log::error!("release thread: {e}");
        }
    }

    fn refresh_installed(&mut self) {
        if self.checked.is_some_and(|t| t.elapsed() < Duration::from_secs(3)) {
            return;
        }
        self.checked = Some(Instant::now());
        self.installed = ON_ANDROID
            .into_iter()
            .filter_map(|id| mahay_android::installed_version(&format!("org.mahay.{id}")).map(|v| (id, v)))
            .collect();
        // An install that finished shows up here: its job is done.
        let latest = self.latest_version();
        let mut jobs = lock(&self.jobs);
        jobs.retain(|id, job| {
            !matches!(job, Job::Installing) || self.installed.get(id).is_none_or(|v| latest.as_deref().is_some_and(|l| is_newer(l, v)))
        });
        drop(jobs);
        self.ctx.request_repaint_after(Duration::from_secs(3));
    }

    fn latest_version(&self) -> Option<String> {
        lock(&self.release).as_ref().and_then(|r| r.as_ref().ok()).map(|r| r.version.clone())
    }

    fn outdated(&self, id: &str) -> bool {
        match (self.installed.get(id), self.latest_version()) {
            (Some(v), Some(latest)) => is_newer(&latest, v),
            _ => false,
        }
    }

    /// Download `id`'s APK from the release, verify it, and hand it to Android's installer.
    fn get(&self, id: &'static str) {
        let Some(Ok(release)) = lock(&self.release).clone() else { return };
        let Some(target) = Target::current() else { return };
        if matches!(lock(&self.jobs).get(id), Some(Job::Downloading(_) | Job::Installing)) {
            return;
        }
        lock(&self.jobs).insert(id, Job::Downloading(0.0));
        let installed = self.installed.get(id).cloned().unwrap_or_else(|| "0.0.0".into());
        let dir: PathBuf = mahay_android::data_dir().unwrap_or_else(|| PathBuf::from("/data/local/tmp")).join("apps").join(id);
        let (jobs, ctx) = (self.jobs.clone(), self.ctx.clone());
        let spawned = std::thread::Builder::new().name(format!("hub-get-{id}")).spawn(move || {
            let progress = {
                let (jobs, ctx) = (jobs.clone(), ctx.clone());
                move |p: f32| {
                    lock(&jobs).insert(id, Job::Downloading(p));
                    ctx.request_repaint();
                }
            };
            let job = match mahay_update::download(&release, id, &installed, target, &dir, &progress) {
                Ok(update) => match mahay_android::install_apk(&update.path) {
                    Ok(()) => Job::Installing,
                    Err(e) => Job::Failed(e),
                },
                Err(e) => Job::Failed(e.to_string()),
            };
            lock(&jobs).insert(id, job);
            ctx.request_repaint();
        });
        if let Err(e) = spawned {
            lock(&self.jobs).insert(id, Job::Failed(e.to_string()));
        }
    }
}

fn tile(ui: &mut egui::Ui, hub: &Hub, app: &MahayApp, width: f32, get: &mut Option<&'static str>) {
    let t = text(hub.lang);
    let available = ON_ANDROID.contains(&app.id);
    let installed = hub.installed.get(app.id);
    let job = lock(&hub.jobs).get(app.id).cloned();
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
                if let Some(src) = icon_of(app.id) {
                    let tint = if available { egui::Color32::WHITE } else { egui::Color32::from_white_alpha(110) };
                    ui.add(egui::Image::new(src).tint(tint).fit_to_exact_size(egui::vec2(52.0, 52.0)));
                }
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new(app.name).size(20.0).strong());
                    ui.label(egui::RichText::new(tagline(hub.lang, app.id, app.tagline)).size(14.0).weak());
                    if let Some(v) = installed {
                        ui.label(egui::RichText::new(format!("v{v}")).size(12.0).weak());
                    }
                });
            });
            ui.add_space(10.0);
            if !available {
                ui.label(egui::RichText::new(t.soon).size(14.0).weak());
                return;
            }
            let big = |label: &str, fill: Option<egui::Color32>| {
                let text = egui::RichText::new(label).size(16.0);
                let text = if fill.is_some() { text.color(egui::Color32::WHITE) } else { text };
                let mut b = egui::Button::new(text).min_size(egui::vec2(140.0, 44.0));
                if let Some(f) = fill {
                    b = b.fill(f);
                }
                b
            };
            match job {
                Some(Job::Downloading(p)) => {
                    ui.label(t.downloading);
                    ui.add(egui::ProgressBar::new(p).show_percentage());
                }
                Some(Job::Installing) => {
                    ui.label(t.installing);
                }
                failed => {
                    if let Some(Job::Failed(e)) = &failed {
                        ui.colored_label(egui::Color32::from_rgb(0xD7, 0x26, 0x3D), e);
                    }
                    ui.horizontal(|ui| {
                        if installed.is_some() {
                            if ui.add(big(t.open, Some(accent))).clicked() {
                                mahay_android::launch_app(&format!("org.mahay.{}", app.id));
                            }
                            if hub.outdated(app.id) && ui.add(big(t.update, None)).clicked() {
                                *get = Some(app.id);
                            }
                        } else if ui.add(big(t.get, Some(accent))).clicked() {
                            *get = Some(app.id);
                        }
                    });
                }
            }
        });
    });
}

impl eframe::App for Hub {
    fn raw_input_hook(&mut self, _ctx: &egui::Context, raw: &mut egui::RawInput) {
        self.bridge.raw_input_hook(raw);
    }

    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.refresh_installed();
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let t = text(self.lang);
        let mut apps: Vec<&MahayApp> = SUITE.iter().filter(|a| ON_ANDROID.contains(&a.id)).collect();
        apps.extend(SUITE.iter().filter(|a| !ON_ANDROID.contains(&a.id)));
        let mut get: Option<&'static str> = None;
        let mut get_all = false;
        let mut retry = false;
        let mut toggle_about = false;
        let mut bridge = std::mem::take(&mut self.bridge);
        let hub = &*self;
        bridge.inset_ui(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let margin = if ui.available_width() > 700.0 { 32.0 } else { 16.0 };
                egui::Frame::new().inner_margin(margin).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        if let Some(src) = icon_of("mahay") {
                            ui.add(egui::Image::new(src).fit_to_exact_size(egui::vec2(56.0, 56.0)));
                        }
                        ui.vertical(|ui| {
                            ui.label(egui::RichText::new("Mahay HUB").size(30.0).strong());
                            ui.label(egui::RichText::new(t.subtitle).size(15.0).weak());
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let label = if hub.show_about { t.back } else { t.about };
                            if ui.add(egui::Button::new(label).min_size(egui::vec2(0.0, 40.0))).clicked() {
                                toggle_about = true;
                            }
                        });
                    });
                    ui.add_space(8.0);
                    if hub.show_about {
                        mahay_about::page(ui, mahay_about::AppInfo { id: "hub", name: "Mahay HUB" });
                        return;
                    }
                    ui.horizontal_wrapped(|ui| {
                        let installed = hub.installed.len();
                        ui.label(egui::RichText::new(format!("{}: {installed}/{}", t.installed, ON_ANDROID.len())).size(14.0).weak());
                        match lock(&hub.release).as_ref() {
                            Some(Ok(r)) => {
                                ui.label(egui::RichText::new(format!("· {} {}", t.latest, r.version)).size(14.0).weak());
                            }
                            Some(Err(_)) => {
                                ui.label(egui::RichText::new(t.offline).size(14.0).weak());
                                if ui.button(t.retry).clicked() {
                                    retry = true;
                                }
                            }
                            None => {
                                ui.spinner();
                            }
                        }
                        if ON_ANDROID.iter().any(|id| hub.outdated(id))
                            && ui.add(egui::Button::new(egui::RichText::new(t.update_all).strong()).min_size(egui::vec2(0.0, 40.0))).clicked()
                        {
                            get_all = true;
                        }
                    });
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
                                tile(ui, hub, app, tile_w, &mut get);
                            }
                        });
                        ui.add_space(gap);
                    }
                    ui.add_space(8.0);
                    if ui.link(t.source).clicked() {
                        mahay_android::open_url(SOURCE);
                    }
                    ui.label(egui::RichText::new("MIT OR Apache-2.0").size(12.0).weak());
                });
            });
        });
        bridge.after_ui(ui.ctx());
        self.bridge = bridge;
        if toggle_about {
            self.show_about = !self.show_about;
        }
        if retry {
            self.fetch_release();
        }
        if let Some(id) = get {
            self.get(id);
        }
        if get_all {
            for id in ON_ANDROID.into_iter().filter(|id| self.outdated(id)) {
                self.get(id);
            }
        }
    }
}
