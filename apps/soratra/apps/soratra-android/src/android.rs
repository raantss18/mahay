use eframe::egui;
use mahay_android::{AndroidApp, EguiBridge};
use soratra_engine::Session;
use soratra_ui_egui::{Services, UiState, WordApp};

use crate::{clipboard, file_dialogs, region};

#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    mahay_android::init(&app, "Soratra");
    log::info!("Soratra {} starting", env!("CARGO_PKG_VERSION"));
    let options = eframe::NativeOptions { android_app: Some(app), renderer: eframe::Renderer::Wgpu, ..Default::default() };
    let result = eframe::run_native(
        "Soratra",
        options,
        Box::new(|cc| {
            let region = region::system();
            let (file_dialog, dialogs) = file_dialogs::hook(&cc.egui_ctx);
            let mut session = Session::new(soratra_engine::blank_document(&region));
            session.system_region = region;
            let mut app = WordApp::new(session, services(file_dialog));
            load_prefs(&mut app);
            if let Some(p) = settings_dir().map(|d| d.join(Session::USER_DICT_FILE)) {
                soratra_proof::load_user_dictionary(&p);
                app.session.user_dict_path = Some(p);
            }
            if let Some(dir) = settings_dir() {
                let ctx = cc.egui_ctx.clone();
                if let Err(e) = app.session.enable_recovery(dir.join("recovery"), std::sync::Arc::new(move || ctx.request_repaint())) {
                    log::warn!("local recovery unavailable: {e}");
                }
            }
            app.start_screen();
            Ok(Box::new(Shell { app, dialogs, bridge: EguiBridge::new(&cc.egui_ctx, mahay_about::AppInfo { id: "soratra", name: "Soratra" }) }))
        }),
    );
    if let Err(e) = result {
        log::error!("Soratra stopped: {e}");
    }
}

struct Shell {
    app: WordApp,
    dialogs: file_dialogs::Launcher,
    bridge: EguiBridge,
}

impl eframe::App for Shell {
    fn logic(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let (opened, notices) = mahay_android::poll();
        for doc in opened {
            let path = doc.path.to_string_lossy().into_owned();
            if let Err(e) = self.app.run("file.open", serde_json::json!({"path": path})) {
                log::warn!("{path}: {e}");
            }
        }
        for text in notices {
            log::info!("{text}");
        }
        self.app.logic(ctx);
        self.dialogs.show(frame);
    }

    fn raw_input_hook(&mut self, _ctx: &egui::Context, raw: &mut egui::RawInput) {
        self.bridge.raw_input_hook(raw);
        self.app.raw_input_hook(raw);
    }

    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let app = &mut self.app;
        self.bridge.inset_ui(ui, |ui| app.ui(ui));
        self.dialogs.show(frame);
        self.bridge.after_ui(ui.ctx());
    }

    fn on_exit(&mut self) {
        save_prefs(&self.app);
    }
}

fn settings_dir() -> Option<std::path::PathBuf> {
    soratra_control_key::settings_dir()
}

fn load_prefs(app: &mut WordApp) {
    if let Some(p) = settings_dir().map(|d| d.join("ui.json"))
        && let Ok(bytes) = std::fs::read(&p)
        && let Some(ui) = UiState::from_json(&bytes)
    {
        app.apply_prefs(ui);
    }
}

fn save_prefs(app: &WordApp) {
    let Some(dir) = settings_dir() else { return };
    if let Err(e) = std::fs::create_dir_all(&dir) {
        log::warn!("preferences: cannot create {}: {e}", dir.display());
        return;
    }
    match serde_json::to_vec_pretty(&app.prefs()) {
        Ok(bytes) => {
            if let Err(e) = std::fs::write(dir.join("ui.json"), bytes) {
                log::warn!("preferences: cannot save: {e}");
            }
        }
        Err(e) => log::warn!("preferences: cannot serialize: {e}"),
    }
}

fn services(file_dialog: file_dialogs::Hook) -> Services {
    Services {
        file_dialog: Some(file_dialog),
        clipboard_picture: Some(Box::new(clipboard::picture)),
        clipboard_text: Some(Box::new(clipboard::text)),
        clipboard_html: Some(Box::new(clipboard::html)),
        set_clipboard_html: Some(Box::new(clipboard::set_html)),
        clipboard_rtf: Some(Box::new(clipboard::rtf)),
        clipboard_formats: Some(Box::new(clipboard::formats)),
        ..Default::default()
    }
}
