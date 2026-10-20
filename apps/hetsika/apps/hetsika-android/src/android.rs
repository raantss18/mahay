use eframe::egui;
use hetsika_engine::Session;
use hetsika_engine::autosave::AutosaveConfig;
use hetsika_ui_egui::HetsikaApp;
use hetsika_ui_egui::native_dialogs::FileDialog;
use mahay_android::{AndroidApp, EguiBridge};

use crate::{audio, audio_in};

#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    mahay_android::init(&app, "Hetsika");
    let options = mahay_android::native_options!(app);
    let result = eframe::run_native(
        "Hetsika",
        options,
        Box::new(|cc| {
            let mut session = Session::default();
            if let Some(dir) = mahay_android::data_dir().map(|d| d.join("Autosave"))
                && let Err(e) = session.start_autosave(AutosaveConfig::new(dir))
            {
                log::warn!("auto-save and crash recovery unavailable: {e}");
            }
            session.voiceover.input = Some(Box::new(audio_in::CpalIn::new(&session.prefs.audio_hardware.device_class)));
            let mut app = HetsikaApp::new(session);
            if let Some(rs) = cc.wgpu_render_state.clone() {
                app.set_wgpu(rs);
            }
            app.audio = Some(Box::new(audio::CpalOut::new()));
            app.hooks.run_dialog = Some(Box::new(run_dialog_off_thread));
            app.hooks.open_url = Some(Box::new(|url: &str| {
                mahay_android::open_url(url);
                Ok(())
            }));
            app.hooks.system_languages = Some(Box::new(|| vec![mahay_android::locale()]));
            let bridge = EguiBridge::new(&cc.egui_ctx, mahay_about::AppInfo { id: "hetsika", name: "Hetsika" });
            Ok(Box::new(Shell { app, bridge }))
        }),
    );
    if let Err(e) = result {
        log::error!("Hetsika stopped: {e}");
        mahay_android::show_error(&format!("{} could not start its display. {e}", "Hetsika"));
    }
}

/// Show `dialog` on its own thread (the Android picker answers there) and hand the paths back.
fn run_dialog_off_thread(dialog: FileDialog) -> std::sync::mpsc::Receiver<Vec<String>> {
    let (tx, rx) = std::sync::mpsc::channel();
    let spawned = std::thread::Builder::new().name("file-dialog".into()).spawn(move || {
        let path = |p: std::path::PathBuf| p.to_string_lossy().into_owned();
        let filtered = |exts: &[String]| rfd::FileDialog::new().add_filter("Media", exts);
        let paths: Vec<String> = match dialog {
            FileDialog::OpenFiles { exts } => filtered(&exts).pick_files().unwrap_or_default().into_iter().map(path).collect(),
            FileDialog::OpenFile { exts, .. } | FileDialog::Relink { exts, .. } => filtered(&exts).pick_file().map(path).into_iter().collect(),
            FileDialog::OpenProject => rfd::FileDialog::new().pick_file().map(path).into_iter().collect(),
            FileDialog::SaveProject { name } | FileDialog::SaveAs { name, .. } => {
                rfd::FileDialog::new().set_file_name(name).save_file().map(path).into_iter().collect()
            }
            FileDialog::Folder { .. } => Vec::new(),
        };
        let _ = tx.send(paths);
    });
    if let Err(e) = spawned {
        log::warn!("file dialog thread: {e}");
    }
    rx
}

struct Shell {
    app: HetsikaApp,
    bridge: EguiBridge,
}

impl eframe::App for Shell {
    fn logic(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // A project opens; video, audio and pictures from another app are imported.
        let (opened, _notices) = mahay_android::poll();
        for doc in opened {
            let path = doc.path.to_string_lossy().into_owned();
            let (cmd, params) = if path.to_ascii_lowercase().ends_with(".fcproj") {
                ("file.open", serde_json::json!({"path": path}))
            } else {
                ("file.import", serde_json::json!({"paths": [path]}))
            };
            if let Err(e) = self.app.session.execute(cmd, params) {
                log::warn!("{cmd}: {e}");
            }
        }
        eframe::App::logic(&mut self.app, ctx, frame);
    }

    fn raw_input_hook(&mut self, ctx: &egui::Context, raw: &mut egui::RawInput) {
        self.bridge.raw_input_hook(raw);
        eframe::App::raw_input_hook(&mut self.app, ctx, raw);
    }

    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let app = &mut self.app;
        self.bridge.inset_ui(ui, |ui| eframe::App::ui(app, ui, frame));
        self.bridge.after_ui(ui.ctx());
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::App::save(&mut self.app, storage);
    }
}
