use eframe::egui;
use mahay_android::{AndroidApp, EguiBridge};
use serde_json::json;
use tselatra_ui_egui::{FileDialog, TselatraApp};

use crate::audio_out;

#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    mahay_android::init(&app, "Tselatra");
    tselatra_engine::text::fonts::scan_system_in_background();
    let options = eframe::NativeOptions { android_app: Some(app), renderer: eframe::Renderer::Wgpu, ..Default::default() };
    let result = eframe::run_native(
        "Tselatra",
        options,
        Box::new(|cc| {
            let gpu_failures = tselatra_ui_egui::gpu_failure::GpuFailureBridge::new(&cc.egui_ctx);
            if let Some(rs) = &cc.wgpu_render_state {
                let lost = gpu_failures.clone();
                rs.device.set_device_lost_callback(move |reason, message| {
                    lost.report(&format!("GPU device lost ({reason:?}): {message}"), true);
                });
            }
            let mut session = tselatra_host::session();
            session.check_footage_on_open = true;
            session.autosave.background = true;
            if let Some(dir) = tselatra_host::config_dir() {
                session.config = Some(std::sync::Arc::new(tselatra_engine::config::DirConfig::new(dir)));
            }
            session.load_settings();
            let recovery = session.begin_recovery();
            let show_home = session.prefs.startup.show_home_on_launch;
            let _ = session.execute("window.restoreScriptPanels", json!({}));
            let mut app = TselatraApp::new(session);
            app.set_gpu_failure_bridge(gpu_failures);
            app.ui.start_screen = show_home;
            if let Some(r) = recovery {
                app.offer_recovery(r);
            }
            app.hooks.audio_device = Some(Box::new(audio_out::open));
            app.hooks.audio_devices = Some(Box::new(audio_out::devices));
            app.hooks.clipboard_text = Some(Box::new(mahay_android::clipboard_text));
            let ctx = cc.egui_ctx.clone();
            app.hooks.file_dialog_async = Some(Box::new(move |dialog: FileDialog, done| {
                let ctx = ctx.clone();
                let spawned = std::thread::Builder::new().name("file-dialog".into()).spawn(move || {
                    let path = |p: std::path::PathBuf| p.to_string_lossy().into_owned();
                    let paths: Vec<String> = match dialog {
                        FileDialog::Open { exts, many: true } => rfd::FileDialog::new().add_filter("Media", &exts).pick_files().unwrap_or_default().into_iter().map(path).collect(),
                        FileDialog::Open { exts, many: false } => rfd::FileDialog::new().add_filter("Media", &exts).pick_file().map(path).into_iter().collect(),
                        FileDialog::OpenProject => rfd::FileDialog::new().pick_file().map(path).into_iter().collect(),
                        FileDialog::Folder => Vec::new(),
                        FileDialog::Save { default, .. } => {
                            let name = std::path::Path::new(&default).file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or(default);
                            rfd::FileDialog::new().set_file_name(name).save_file().map(path).into_iter().collect()
                        }
                    };
                    done(paths);
                    ctx.request_repaint();
                });
                if let Err(e) = spawned {
                    log::warn!("file dialog thread: {e}");
                }
            }));
            let bridge = EguiBridge::new(&cc.egui_ctx, mahay_about::AppInfo { id: "tselatra", name: "Tselatra" });
            Ok(Box::new(Shell { app, bridge }))
        }),
    );
    if let Err(e) = result {
        log::error!("Tselatra stopped: {e}");
    }
}

struct Shell {
    app: TselatraApp,
    bridge: EguiBridge,
}

impl eframe::App for Shell {
    fn logic(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let _ = mahay_android::poll();
        self.app.logic(ctx, frame);
    }

    fn raw_input_hook(&mut self, ctx: &egui::Context, raw: &mut egui::RawInput) {
        self.bridge.raw_input_hook(raw);
        self.app.raw_input_hook(ctx, raw);
    }

    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let app = &mut self.app;
        self.bridge.inset_ui(ui, |ui| app.ui(ui, frame));
        self.bridge.after_ui(ui.ctx());
    }

    fn on_exit(&mut self) {
        self.app.on_exit();
    }
}
