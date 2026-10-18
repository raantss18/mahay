use eframe::egui;
use hazavana_engine::Session;
use hazavana_ui_egui::pick::{PickKind, PickRequest};
use hazavana_ui_egui::{HazavanaApp, Services};
use mahay_android::{AndroidApp, EguiBridge};

/// How long background work may run when the app closes.
const EXIT_WAIT: std::time::Duration = std::time::Duration::from_secs(5);

#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    mahay_android::init(&app, "Hazavana");
    let options = eframe::NativeOptions { android_app: Some(app), renderer: eframe::Renderer::Wgpu, ..Default::default() };
    let result = eframe::run_native(
        "Hazavana",
        options,
        Box::new(|cc| {
            let session = open_session();
            let mut app = HazavanaApp::new(session, services(&cc.egui_ctx));
            hazavana_ui_egui::i18n::set_language(app.ui.language);
            // An empty library opens where Import is offered.
            hazavana_ui_egui::panels::empty_library::open_on_empty_library(&mut app);
            let bridge = EguiBridge::new(&cc.egui_ctx, mahay_about::AppInfo { id: "hazavana", name: "Hazavana" });
            Ok(Box::new(Shell { app, bridge }))
        }),
    );
    if let Err(e) = result {
        log::error!("Hazavana stopped: {e}");
    }
}

/// The library in the app's private storage (`files/Library`).
fn open_session() -> Session {
    let mut s = Session::new().with_fs().with_default_denoise_models().with_system_clock();
    if let Some(dir) = mahay_android::data_dir().map(|d| d.join("Library")) {
        if let Err(e) = s.open_library(&dir, false) {
            log::error!("library {}: {e}", dir.display());
        }
    }
    s.face_models_dir = hazavana_engine::config::default_face_models_dir();
    s
}

fn services(ctx: &egui::Context) -> Services {
    let ctx = ctx.clone();
    Services {
        // Dialogs run on their own thread (as on the desktop): the Android picker answers there.
        picker: Some(Box::new(move |req: PickRequest| {
            let (tx, rx) = std::sync::mpsc::channel();
            let repaint = ctx.clone();
            std::thread::Builder::new()
                .name("file-dialog".into())
                .spawn(move || {
                    let mut d = rfd::FileDialog::new();
                    if let Some((name, exts)) = &req.filter {
                        d = d.add_filter(name.clone(), exts);
                    }
                    if let Some(n) = &req.file_name {
                        d = d.set_file_name(n);
                    }
                    let s = |p: std::path::PathBuf| p.to_string_lossy().to_string();
                    let paths: Vec<String> = match req.kind {
                        PickKind::Files => d.pick_files().unwrap_or_default().into_iter().map(s).collect(),
                        PickKind::File => d.pick_file().map(s).into_iter().collect(),
                        PickKind::Folder => Vec::new(),
                        PickKind::Save => d.save_file().map(s).into_iter().collect(),
                    };
                    let _ = tx.send(paths);
                    repaint.request_repaint();
                })
                .map_err(|e| format!("could not start the file dialog: {e}"))?;
            Ok(rx)
        })),
        open_url: Some(Box::new(|url: &str| {
            mahay_android::open_url(url);
            Ok(())
        })),
        ..Default::default()
    }
}

struct Shell {
    app: HazavanaApp,
    bridge: EguiBridge,
}

impl eframe::App for Shell {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Photos shared or opened from another app are imported into the library.
        let (opened, _notices) = mahay_android::poll();
        if !opened.is_empty() {
            let paths: Vec<String> = opened.iter().map(|d| d.path.to_string_lossy().into_owned()).collect();
            if let Err(e) = hazavana_ui_egui::import::start_paths(&mut self.app, paths) {
                log::warn!("import: {e}");
            }
            self.app.ui.view = hazavana_ui_egui::state::ViewMode::PhotoGrid;
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

    fn on_exit(&mut self) {
        if let Err(e) = self.app.session.close_library() {
            log::error!("saving the library failed: {e}");
        }
        let _ = self.app.shutdown(EXIT_WAIT);
    }
}
