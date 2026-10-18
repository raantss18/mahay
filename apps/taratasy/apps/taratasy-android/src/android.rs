use eframe::egui;
use mahay_android::{AndroidApp, EguiBridge};
use taratasy_ui_egui::{RecoveryStore, TaratasyApp};

#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    mahay_android::init(&app, "Taratasy");
    log::info!("Taratasy {} starting", env!("CARGO_PKG_VERSION"));
    let persistence_path = mahay_android::data_dir().map(|d| d.join("taratasy.ron"));
    let options = eframe::NativeOptions { android_app: Some(app), renderer: eframe::Renderer::Wgpu, persistence_path, ..Default::default() };
    let result = eframe::run_native(
        "Taratasy",
        options,
        Box::new(|cc| {
            let mut app = TaratasyApp::new();
            app.units = taratasy_ui_egui::units::Unit::system();
            if let Some(json) = cc.storage.and_then(|s| s.get_string("taratasy")) {
                app.restore(&json);
            }
            if let Some(dir) = RecoveryStore::default_dir() {
                app.enable_recovery(RecoveryStore::new(dir));
            }
            Ok(Box::new(Shell { app, bridge: EguiBridge::new(&cc.egui_ctx) }))
        }),
    );
    if let Err(e) = result {
        log::error!("Taratasy stopped: {e}");
    }
}

struct Shell {
    app: TaratasyApp,
    bridge: EguiBridge,
}

impl eframe::App for Shell {
    fn raw_input_hook(&mut self, ctx: &egui::Context, raw: &mut egui::RawInput) {
        self.bridge.raw_input_hook(raw);
        eframe::App::raw_input_hook(&mut self.app, ctx, raw);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::App::save(&mut self.app, storage);
    }

    fn logic(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let (opened, notices) = mahay_android::poll();
        for doc in opened {
            if let Some(path) = doc.path.to_str() {
                self.app.open_path(path);
            }
        }
        for text in notices {
            self.app.notify(text);
        }
        eframe::App::logic(&mut self.app, ctx, frame);
    }

    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let app = &mut self.app;
        self.bridge.inset_ui(ui, |ui| eframe::App::ui(app, ui, frame));
        self.bridge.after_ui(ui.ctx());
    }
}
