use eframe::egui;
use mahay_android::{AndroidApp, EguiBridge};
use pejy_engine::Session;
use pejy_ui_egui::{DesignApp, Services};

#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    mahay_android::init(&app, "Pejy");
    let options = mahay_android::native_options!(app);
    let result = eframe::run_native(
        "Pejy",
        options,
        Box::new(|cc| {
            let mut session = Session::new();
            session.recovery_dir = mahay_android::data_dir().map(|d| d.join("Recovery"));
            let _ = session.execute("file.recovery.open", &serde_json::json!({}));
            let app = DesignApp::new(session, services());
            let bridge = EguiBridge::new(&cc.egui_ctx, mahay_about::AppInfo { id: "pejy", name: "Pejy" });
            Ok(Box::new(Shell { app, bridge }))
        }),
    );
    if let Err(e) = result {
        log::error!("Pejy stopped: {e}");
        mahay_android::show_error(&format!("{} could not start its display. {e}", "Pejy"));
    }
}

fn services() -> Services {
    Services {
        // The picker answers on a later frame (`Shell::logic`); `None` now means "nothing yet".
        pick_open: Some(Box::new(|_purpose: &str| {
            mahay_android::open_document(&[]);
            None
        })),
        pick_save: Some(Box::new(|name: &str| Some(mahay_android::save_target(name).to_string_lossy().into_owned()))),
        read: Some(Box::new(|p: &str| std::fs::read(p).map_err(|e| e.to_string()))),
        write: Some(Box::new(|p: &str, b: &[u8]| pejy_engine::atomic::write_atomic(std::path::Path::new(p), b).map_err(|e| e.to_string()))),
        clipboard_text: Some(Box::new(mahay_android::clipboard_text)),
        ..Default::default()
    }
}

struct Shell {
    app: DesignApp,
    bridge: EguiBridge,
}

impl eframe::App for Shell {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let (opened, notices) = mahay_android::poll();
        if !opened.is_empty() {
            self.app.open_documents_later(opened.into_iter().map(|d| d.path.to_string_lossy().into_owned()));
        }
        if let Some(text) = notices.last() {
            self.app.status(text.clone());
        }
        self.app.logic(ctx);
    }

    fn raw_input_hook(&mut self, _ctx: &egui::Context, raw: &mut egui::RawInput) {
        self.bridge.raw_input_hook(raw);
        self.app.raw_input_hook(raw);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let app = &mut self.app;
        self.bridge.inset_ui(ui, |ui| app.ui(ui));
        self.bridge.after_ui(ui.ctx());
    }
}
