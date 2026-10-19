//! What every Mahay app shows the same way, on desktop and Android (Tontolo's model):
//!
//! - [`about_ui`]: the About page. Icon, name and version, what the app does, where its source
//!   is, automatic updates (status, on/off, check now, install), a QR code to install Mahay on
//!   another device, the author, the licences.
//! - [`UpdateBanner`]: a small bar at the bottom of the window when an update has been
//!   downloaded and verified: "Mahay 0.3.0 is ready · What's new · Install · Later".
//!
//! Installing is the platform's: the app registers an `install` function (Android's package
//! installer, or replacing the AppImage and restarting) with [`register`], once at start-up.
//! Then any screen calls [`page`] and the frame loop calls [`banner`]; no updater has to be
//! passed through the app's own structures.

mod text;

pub use text::Lang;

use egui::{Color32, RichText, Ui, vec2};
#[cfg(not(target_arch = "wasm32"))]
use mahay_update::{Checker, Downloaded, State};
use text::t;

/// The suite's version (the repository's `VERSION` file).
pub const SUITE_VERSION: &str = env!("MAHAY_VERSION");

/// Installs a verified update. `Err` carries a message for the user.
#[cfg(not(target_arch = "wasm32"))]
pub type InstallFn = Box<dyn Fn(&Downloaded) -> Result<(), String> + Send + Sync>;

#[cfg(not(target_arch = "wasm32"))]
struct Registered {
    app: AppInfo,
    lang: Lang,
    checker: Checker,
    install: InstallFn,
}

#[cfg(not(target_arch = "wasm32"))]
static REGISTERED: std::sync::OnceLock<Registered> = std::sync::OnceLock::new();
/// Asks Mahay HUB to update this app. `Err` says why it can't (the hub is missing or isn't
/// signed with the same certificate); the app then updates itself as before.
#[cfg(not(target_arch = "wasm32"))]
pub type HandOffFn = Box<dyn Fn() -> Result<(), String> + Send + Sync>;

#[cfg(not(target_arch = "wasm32"))]
static HANDOFF: std::sync::OnceLock<HandOffFn> = std::sync::OnceLock::new();
/// When the update was last handed to the hub; the bar says so for [`HANDED_FOR`].
#[cfg(not(target_arch = "wasm32"))]
static HANDED: std::sync::Mutex<Option<std::time::Instant>> = std::sync::Mutex::new(None);
#[cfg(not(target_arch = "wasm32"))]
const HANDED_FOR: std::time::Duration = std::time::Duration::from_secs(120);
#[cfg(not(target_arch = "wasm32"))]
static BANNER: std::sync::Mutex<Option<UpdateBanner>> = std::sync::Mutex::new(None);

/// Register this app's updater (once, at start-up) and start the automatic checks.
#[cfg(not(target_arch = "wasm32"))]
pub fn register(app: AppInfo, lang: Lang, checker: Checker, install: InstallFn) {
    checker.start_auto();
    let _ = REGISTERED.set(Registered { app, lang, checker, install });
}

/// Android apps: register how to hand an update to Mahay HUB (once, at start-up, after
/// [`register`]). The "Install" of a [`State::Managed`] update calls it.
#[cfg(not(target_arch = "wasm32"))]
pub fn register_handoff(handoff: HandOffFn) {
    let _ = HANDOFF.set(handoff);
}

/// The "Install" of an update Mahay HUB will install. On success the bar says "updating through
/// Mahay HUB"; when the hub can't take it, this app checks again and, the hub being unusable
/// now, downloads the update itself (the usual "Install" then follows).
#[cfg(not(target_arch = "wasm32"))]
fn hand_off(checker: &Checker) -> Result<(), String> {
    let result = match HANDOFF.get() {
        Some(f) => f(),
        None => Err("no hub".into()),
    };
    match result {
        Ok(()) => {
            *HANDED.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(std::time::Instant::now());
            Ok(())
        }
        Err(_) => {
            checker.check_now();
            Ok(())
        }
    }
}

/// Whether an update was handed to the hub a moment ago.
#[cfg(not(target_arch = "wasm32"))]
fn handed_off() -> bool {
    HANDED.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some_and(|t| t.elapsed() < HANDED_FOR)
}

/// Desktop start-up: register `app`'s updater. From an AppImage, updates install themselves
/// (the AppImage is replaced, then the app restarts); from a distribution package they are only
/// reported. Keeps its files in `$XDG_DATA_HOME/mahay/<app>` (or `~/.local/share/...`).
#[cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))]
pub fn register_desktop(app: AppInfo, ctx: &egui::Context) {
    let base = std::env::var_os("XDG_DATA_HOME")
        .map(std::path::PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| std::path::PathBuf::from(h).join(".local/share")))
        .unwrap_or_else(std::env::temp_dir);
    let dir = base.join("mahay").join(app.id);
    let ctx = ctx.clone();
    let can_install = mahay_update::running_appimage().is_some();
    let checker = Checker::new(app.id, dir, can_install, move || ctx.request_repaint());
    register(app, Lang::from_env(), checker, Box::new(mahay_update::install_appimage));
}

/// The registered updater, for screens that show its state (Mahay HUB).
#[cfg(not(target_arch = "wasm32"))]
pub fn updater() -> Option<&'static Checker> {
    REGISTERED.get().map(|r| &r.checker)
}

/// The language the app registered (else the environment's).
pub fn lang() -> Lang {
    #[cfg(not(target_arch = "wasm32"))]
    if let Some(r) = REGISTERED.get() {
        return r.lang;
    }
    Lang::from_env()
}

/// The About page for `app`, with the registered updater if there is one.
pub fn page(ui: &mut Ui, app: AppInfo) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let r = REGISTERED.get();
        let install = r.map(|r| &r.install as &dyn Fn(&Downloaded) -> Result<(), String>);
        about_ui(ui, app, lang(), r.map(|r| &r.checker), install);
    }
    #[cfg(target_arch = "wasm32")]
    about_ui(ui, app, lang());
}

/// The update bar; call at the end of every frame. Does nothing until an update is ready.
#[cfg(not(target_arch = "wasm32"))]
pub fn banner(ctx: &egui::Context) {
    let Some(r) = REGISTERED.get() else { return };
    let mut slot = BANNER.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    slot.get_or_insert_with(UpdateBanner::default).show(ctx, r.app, r.lang, &r.checker, &r.install);
}

#[cfg(target_arch = "wasm32")]
pub fn banner(_ctx: &egui::Context) {}

/// The repository, its release page and the licences, as shown to users.
pub const SOURCE_URL: &str = "https://github.com/raantss18/mahay";
pub const LATEST_URL: &str = "https://github.com/raantss18/mahay/releases/latest";
pub const LICENSES_URL: &str = "https://github.com/raantss18/mahay/blob/main/THIRD_PARTY_LICENSES.md";
pub const DISCUSSIONS_URL: &str = "https://github.com/raantss18/mahay/discussions";

/// The author, as in Tontolo's About page.
pub const AUTHOR_NAME: &str = "RANIRIAMANJAKA Antsa";
const AUTHOR_LINKS: [(&str, &str); 4] = [
    ("github.com/raantss18", "https://github.com/raantss18"),
    ("antsa.raniriamanjaka@univ-antananarivo.mg", "mailto:antsa.raniriamanjaka@univ-antananarivo.mg"),
    ("+261 33 43 838 82", "tel:+261334383882"),
    ("WhatsApp +261 33 43 838 82", "https://wa.me/261334383882"),
];

/// Open-source libraries the apps are built on, credited on the About page.
const BUILT_WITH: [(&str, &str); 6] = [
    ("egui / eframe", "https://github.com/emilk/egui"),
    ("wgpu", "https://wgpu.rs"),
    ("winit", "https://github.com/rust-windowing/winit"),
    ("rustls", "https://github.com/rustls/rustls"),
    ("android-activity", "https://github.com/rust-mobile/android-activity"),
    ("RustCrypto, dalek-cryptography", "https://github.com/dalek-cryptography/curve25519-dalek"),
];

/// Which app the page is for.
#[derive(Clone, Copy, Debug)]
pub struct AppInfo {
    /// Machine id: `tabilao`.
    pub id: &'static str,
    /// Display name: `Tabilao`.
    pub name: &'static str,
}

/// How the app installs a verified update. `Err` carries a message for the user.
#[cfg(not(target_arch = "wasm32"))]
pub type Install<'a> = &'a dyn Fn(&Downloaded) -> Result<(), String>;

/// The About page. `updater` is `None` where the app can't update itself.
#[cfg(not(target_arch = "wasm32"))]
pub fn about_ui(ui: &mut Ui, app: AppInfo, lang: Lang, updater: Option<&Checker>, install: Option<Install<'_>>) {
    about_body(ui, app, lang, |ui| {
        if let Some(checker) = updater {
            ui.label(RichText::new(t(lang, "updates")).strong());
            updates_section(ui, lang, checker, install);
            ui.separator();
        }
    });
}

/// The About page (web: no updater).
#[cfg(target_arch = "wasm32")]
pub fn about_ui(ui: &mut Ui, app: AppInfo, lang: Lang) {
    about_body(ui, app, lang, |_| {});
}

fn about_body(ui: &mut Ui, app: AppInfo, lang: Lang, updates: impl FnOnce(&mut Ui)) {
    egui::ScrollArea::vertical().id_salt("mahay-about").show(ui, |ui| {
        ui.vertical_centered(|ui| {
            if let Some(tex) = icon(ui.ctx(), app.id) {
                ui.add(egui::Image::new(&tex).fit_to_exact_size(vec2(72.0, 72.0)));
            }
            ui.label(RichText::new(format!("{} {}", app.name, SUITE_VERSION)).size(22.0).strong());
            ui.label(t(lang, tagline_key(app.id)));
            ui.label(RichText::new(t(lang, "suite")).weak());
            ui.hyperlink_to("github.com/raantss18/mahay", SOURCE_URL);
        });
        ui.separator();
        updates(ui);

        ui.vertical_centered(|ui| {
            qr_code(ui, 4.0);
            ui.label(RichText::new(t(lang, "qr_title")).strong());
            ui.label(RichText::new(t(lang, "qr_help")).weak());
            ui.hyperlink_to(t(lang, "qr_link"), LATEST_URL);
        });
        ui.separator();

        ui.label(RichText::new(t(lang, "author")).strong());
        ui.label(RichText::new(AUTHOR_NAME).strong());
        ui.label(RichText::new(t(lang, "author_role")).weak());
        for (label, url) in AUTHOR_LINKS {
            ui.hyperlink_to(label, url);
        }
        ui.hyperlink_to(t(lang, "discussions"), DISCUSSIONS_URL);
        ui.separator();

        ui.label(RichText::new(t(lang, "built_with")).strong());
        ui.horizontal_wrapped(|ui| {
            for (name, url) in BUILT_WITH {
                ui.hyperlink_to(name, url);
            }
        });
        ui.add_space(4.0);
        ui.label(t(lang, "license"));
        ui.hyperlink_to(t(lang, "third_party"), LICENSES_URL);
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn updates_section(ui: &mut Ui, lang: Lang, checker: &Checker, install: Option<Install<'_>>) {
    let mut auto = checker.auto();
    if ui.checkbox(&mut auto, t(lang, "auto")).changed() {
        checker.set_auto(auto);
    }
    match checker.state() {
        State::Idle => {
            ui.label(RichText::new(format!("{} {SUITE_VERSION}", t(lang, "installed"))).weak());
        }
        State::Checking => {
            ui.horizontal(|ui| {
                ui.spinner();
                ui.label(t(lang, "checking"));
            });
        }
        State::UpToDate => {
            ui.label(format!("✓ {}", t(lang, "up_to_date")));
        }
        State::Available { version } => {
            ui.label(format!("{} {version}", t(lang, "available")));
            ui.hyperlink_to(t(lang, "qr_link"), LATEST_URL);
        }
        State::Downloading { version, progress } => {
            ui.label(format!("{} {version}…", t(lang, "downloading")));
            ui.add(egui::ProgressBar::new(progress).show_percentage());
        }
        State::Ready { update, notes } => {
            ui.label(RichText::new(format!("{} {}", t(lang, "ready"), update.version)).strong());
            if let Some(notes) = &notes {
                egui::CollapsingHeader::new(t(lang, "whats_new")).id_salt("mahay-about-notes").show(ui, |ui| {
                    notes_ui(ui, notes);
                });
            }
            if let Some(install) = install
                && ui.button(t(lang, "install")).clicked()
                && let Err(e) = install(&update)
            {
                ui.colored_label(Color32::from_rgb(0xD7, 0x26, 0x3D), e);
            }
        }
        State::Managed { version } => {
            ui.label(RichText::new(format!("{} {version}", t(lang, "ready"))).strong());
            if handed_off() {
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label(t(lang, "via_hub"));
                });
                ui.ctx().request_repaint_after(std::time::Duration::from_secs(1));
            } else {
                ui.label(RichText::new(t(lang, "managed_by_hub")).weak());
                if ui.button(t(lang, "install")).clicked() {
                    let _ = hand_off(checker);
                }
            }
        }
        State::Failed(e) => {
            ui.colored_label(Color32::from_rgb(0xD7, 0x26, 0x3D), format!("{}: {e}", t(lang, "failed")));
        }
    }
    if !matches!(checker.state(), State::Checking | State::Downloading { .. }) && ui.button(t(lang, "check_now")).clicked() {
        checker.check_now();
    }
}

/// The bar shown over the app when an update is ready. Call after the app's `ui`.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Default)]
pub struct UpdateBanner {
    /// The version the user put off ("Later"): not shown again this session.
    later: Option<String>,
    notes_open: bool,
    error: Option<String>,
}

#[cfg(not(target_arch = "wasm32"))]
impl UpdateBanner {
    pub fn show(&mut self, ctx: &egui::Context, app: AppInfo, lang: Lang, checker: &Checker, install: Install<'_>) {
        let (version, notes, ready) = match checker.state() {
            State::Ready { update, notes } => (update.version.clone(), notes, Some(update)),
            State::Managed { version } => (version, None, None),
            _ => return,
        };
        if self.later.as_deref() == Some(version.as_str()) {
            return;
        }
        egui::Area::new(egui::Id::new("mahay-update-banner"))
            .anchor(egui::Align2::CENTER_BOTTOM, vec2(0.0, -16.0))
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                egui::Frame::popup(ui.style()).inner_margin(12.0).show(ui, |ui| {
                    ui.set_max_width(520.0);
                    ui.horizontal(|ui| {
                        if let Some(tex) = icon(ui.ctx(), app.id) {
                            ui.add(egui::Image::new(&tex).fit_to_exact_size(vec2(28.0, 28.0)));
                        }
                        ui.label(RichText::new(format!("{} {version} — {}", app.name, t(lang, "ready_short"))).strong());
                    });
                    if self.notes_open
                        && let Some(notes) = &notes
                    {
                        egui::ScrollArea::vertical().max_height(220.0).show(ui, |ui| notes_ui(ui, notes));
                    }
                    if let Some(e) = &self.error {
                        ui.colored_label(Color32::from_rgb(0xD7, 0x26, 0x3D), e);
                    }
                    ui.horizontal(|ui| {
                        let install_button = egui::Button::new(RichText::new(t(lang, "install")).strong()).min_size(vec2(110.0, 40.0));
                        if handed_off() {
                            ui.spinner();
                            ui.label(t(lang, "via_hub"));
                            ui.ctx().request_repaint_after(std::time::Duration::from_secs(1));
                        } else if ui.add(install_button).clicked() {
                            self.error = match &ready {
                                Some(update) => install(update).err(),
                                None => hand_off(checker).err(),
                            };
                        }
                        if notes.is_some() && ui.add(egui::Button::new(t(lang, "whats_new")).min_size(vec2(0.0, 40.0))).clicked() {
                            self.notes_open = !self.notes_open;
                        }
                        if ui.add(egui::Button::new(t(lang, "later")).min_size(vec2(0.0, 40.0))).clicked() {
                            self.later = Some(version.clone());
                            checker.dismiss();
                        }
                    });
                });
            });
    }
}

/// Release notes (Markdown from the CHANGELOG), shown plainly.
#[cfg(not(target_arch = "wasm32"))]
fn notes_ui(ui: &mut Ui, notes: &str) {
    for line in notes.lines() {
        let line = line.trim_end();
        if let Some(h) = line.strip_prefix("## ") {
            ui.label(RichText::new(h).strong());
        } else if let Some(h) = line.strip_prefix("### ") {
            ui.label(RichText::new(h).italics());
        } else if let Some(item) = line.strip_prefix("- ") {
            ui.label(format!("• {}", item.replace("**", "").replace('`', "")));
        } else if !line.trim().is_empty() {
            ui.label(line.trim().replace("**", "").replace('`', ""));
        }
    }
}

fn tagline_key(id: &str) -> &'static str {
    match id {
        "sary" => "tag_sary",
        "taratasy" => "tag_taratasy",
        "soratra" => "tag_soratra",
        "tabilao" => "tag_tabilao",
        "seho" => "tag_seho",
        "tsilo" => "tag_tsilo",
        "endrika" => "tag_endrika",
        "pejy" => "tag_pejy",
        "hazavana" => "tag_hazavana",
        "feo" => "tag_feo",
        "hetsika" => "tag_hetsika",
        "tselatra" => "tag_tselatra",
        "drafitra" => "tag_drafitra",
        _ => "tag_hub",
    }
}

/// The app's icon (96×96 RGBA, generated by `brand/apply.py`), loaded once per context.
pub fn icon(ctx: &egui::Context, id: &str) -> Option<egui::TextureHandle> {
    let bytes: &[u8] = match id {
        "sary" => include_bytes!("../assets/sary.rgba"),
        "taratasy" => include_bytes!("../assets/taratasy.rgba"),
        "soratra" => include_bytes!("../assets/soratra.rgba"),
        "tabilao" => include_bytes!("../assets/tabilao.rgba"),
        "seho" => include_bytes!("../assets/seho.rgba"),
        "hub" => include_bytes!("../assets/hub.rgba"),
        "tsilo" => include_bytes!("../assets/tsilo.rgba"),
        "endrika" => include_bytes!("../assets/endrika.rgba"),
        "pejy" => include_bytes!("../assets/pejy.rgba"),
        "hazavana" => include_bytes!("../assets/hazavana.rgba"),
        "feo" => include_bytes!("../assets/feo.rgba"),
        "hetsika" => include_bytes!("../assets/hetsika.rgba"),
        "tselatra" => include_bytes!("../assets/tselatra.rgba"),
        "drafitra" => include_bytes!("../assets/drafitra.rgba"),
        _ => include_bytes!("../assets/mahay.rgba"),
    };
    const SIDE: usize = 96;
    if bytes.len() != SIDE * SIDE * 4 {
        return None;
    }
    let key = egui::Id::new(("mahay-icon", id));
    if let Some(tex) = ctx.data(|d| d.get_temp::<egui::TextureHandle>(key)) {
        return Some(tex);
    }
    let image = egui::ColorImage::from_rgba_unmultiplied([SIDE, SIDE], bytes);
    let tex = ctx.load_texture(format!("mahay-icon-{id}"), image, egui::TextureOptions::LINEAR);
    ctx.data_mut(|d| d.insert_temp(key, tex.clone()));
    Some(tex)
}

/// The QR code to the latest release, drawn as squares (`module` points each), on white.
pub fn qr_code(ui: &mut Ui, module: f32) {
    let grid: Vec<&[u8]> = include_str!("../assets/qr-latest.txt").lines().map(str::as_bytes).collect();
    let n = grid.len();
    let quiet = 2.0 * module;
    let side = n as f32 * module + 2.0 * quiet;
    let (rect, _) = ui.allocate_exact_size(vec2(side, side), egui::Sense::hover());
    let p = ui.painter();
    p.rect_filled(rect, 6.0, Color32::WHITE);
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == b'1' {
                let min = rect.min + vec2(quiet + x as f32 * module, quiet + y as f32 * module);
                p.rect_filled(egui::Rect::from_min_size(min, vec2(module, module)), 0.0, Color32::BLACK);
            }
        }
    }
}
