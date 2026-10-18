use eframe::egui;
use mahay_android::{AndroidApp, EguiBridge};
use sary_engine::Session;
use sary_ui_egui::SaryApp;

use crate::services;

#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    mahay_android::init(&app, "Sary");
    log::info!("Sary {} starting", env!("CARGO_PKG_VERSION"));
    let persistence_path = mahay_android::data_dir().map(|d| d.join("sary.ron"));
    let options = eframe::NativeOptions { android_app: Some(app), renderer: eframe::Renderer::Wgpu, persistence_path, ..Default::default() };
    let presets = services::presets_dir().map(sary_engine::preset_store::open_dir_async);
    let result = eframe::run_native(
        "Sary",
        options,
        Box::new(move |cc| {
            let mut services = services::native(None);
            services.preset_store = presets;
            let mut app = SaryApp::new(Session::new(), services);
            app.background_jobs = true;
            if let Some(rs) = cc.wgpu_render_state.clone() {
                app.perf.gpu_info.set_adapter(&rs.adapter.get_info());
                if rs.adapter.get_info().device_type != eframe::wgpu::DeviceType::Cpu {
                    app.set_wgpu(rs);
                }
            }
            Ok(Box::new(Shell { app, bridge: EguiBridge::new(&cc.egui_ctx, mahay_about::AppInfo { id: "sary", name: "Sary" }) }))
        }),
    );
    if let Err(e) = result {
        log::error!("Sary stopped: {e}");
    }
}

struct Shell {
    app: SaryApp,
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
        let (opened, _notices) = mahay_android::poll();
        for doc in opened {
            self.app.open_path(&doc.path.to_string_lossy());
        }
        eframe::App::logic(&mut self.app, ctx, frame);
    }

    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let app = &mut self.app;
        self.bridge.inset_ui(ui, |ui| eframe::App::ui(app, ui, frame));
        self.bridge.after_ui(ui.ctx());
    }
}
