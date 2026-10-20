use drafitra_engine::Session;
use drafitra_ui_egui::{CadApp, Services};
use eframe::egui;
use mahay_android::{AndroidApp, EguiBridge};

/// DWG, DXF and SVG have no reliable MIME types on Android: show every file.
const OPEN_MIMES: &[&str] = &[];

#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    mahay_android::init(&app, "Drafitra");
    let persistence_path = mahay_android::data_dir().map(|d| d.join("drafitra.ron"));
    let options = mahay_android::native_options!(app, persistence_path);
    let result = eframe::run_native(
        "Drafitra",
        options,
        Box::new(|cc| {
            let mut app = CadApp::new(Session::empty(), services());
            app.system_languages = vec![mahay_android::locale()];
            if let Some(profile) = cc.storage.and_then(|s| s.get_string(drafitra_ui_egui::options::PROFILE_KEY)) {
                app.load_profile(&profile);
            }
            if let Some(prefs) = cc.storage.and_then(|s| s.get_string(drafitra_ui_egui::PREFS_KEY)) {
                app.load_prefs(&prefs);
            }
            let bridge = EguiBridge::new(&cc.egui_ctx, mahay_about::AppInfo { id: "drafitra", name: "Drafitra" });
            Ok(Box::new(Shell { app, bridge }))
        }),
    );
    if let Err(e) = result {
        log::error!("Drafitra stopped: {e}");
        mahay_android::show_error(&format!("{} could not start its display. {e}", "Drafitra"));
    }
}

fn services() -> Services {
    Services {
        pick_open: Some(Box::new(|| {
            mahay_android::open_document(OPEN_MIMES);
            None
        })),
        pick_save: Some(Box::new(|name: &str| Some(mahay_android::save_target(name).to_string_lossy().into_owned()))),
    }
}

struct Shell {
    app: CadApp,
    bridge: EguiBridge,
}

impl eframe::App for Shell {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let (opened, notices) = mahay_android::poll();
        for doc in opened {
            self.app.open_path(&doc.path.to_string_lossy());
        }
        if let Some(text) = notices.last() {
            self.app.set_status(text.clone());
        }
        self.app.logic(ctx);
    }

    fn raw_input_hook(&mut self, ctx: &egui::Context, raw: &mut egui::RawInput) {
        self.bridge.raw_input_hook(raw);
        self.app.raw_input_hook(raw);
        drafitra_ui_egui::cmdline::capture_tab(ctx, raw);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let app = &mut self.app;
        self.bridge.inset_ui(ui, |ui| app.ui(ui));
        self.bridge.after_ui(ui.ctx());
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        storage.set_string(drafitra_ui_egui::options::PROFILE_KEY, self.app.profile_json());
        storage.set_string(drafitra_ui_egui::PREFS_KEY, self.app.prefs_json());
    }
}
