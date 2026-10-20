use std::sync::Arc;

use eframe::egui;
use feo_engine::Engine;
use feo_ui_egui::{Services, SoundApp, dialogs};
use mahay_android::{AndroidApp, EguiBridge};

const AUDIO_MIMES: &[&str] = &["audio/*", "video/mp4", "video/webm"];
const MIDI_MIMES: &[&str] = &["audio/midi", "audio/x-midi", "audio/sp-midi"];

#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    mahay_android::init(&app, "Feo");
    let options = mahay_android::native_options!(app);
    let result = eframe::run_native(
        "Feo",
        options,
        Box::new(|cc| {
            let engine = Engine::default();
            let ui_state = feo_ui_egui::UiState::default();
            let player = Some(feo_playback::Player::with_config(Arc::new(engine.session().clone()), &ui_state.audio));
            let mut app = SoundApp::new(engine, player, services());
            app.autosave_dir = mahay_android::data_dir().map(|d| d.join("Autosave"));
            app.preset_dir = mahay_android::data_dir().map(|d| d.join("Presets"));
            app.ui = ui_state;
            let bridge = EguiBridge::new(&cc.egui_ctx, mahay_about::AppInfo { id: "feo", name: "Feo" });
            Ok(Box::new(Shell { app, bridge }))
        }),
    );
    if let Err(e) = result {
        log::error!("Feo stopped: {e}");
        mahay_android::show_error(&format!("{} could not start its display. {e}", "Feo"));
    }
}

fn services() -> Services {
    Services {
        pick_open: Some(Box::new(|purpose: &str, _exts: &[&str]| {
            let mimes: &[&str] = match purpose {
                "session.open" => &[],
                "file.import_midi" => MIDI_MIMES,
                _ => AUDIO_MIMES,
            };
            mahay_android::open_document(mimes);
            None
        })),
        pick_save: Some(Box::new(|_purpose: &str, default: &str| Some(mahay_android::save_target(default).to_string_lossy().into_owned()))),
        pick_folder: None,
        ..Default::default()
    }
}

/// What a picked file is: a session opens, MIDI and audio import (as on the desktop).
fn open_path(app: &mut SoundApp, p: &str) {
    let lower = p.to_ascii_lowercase();
    let r = if lower.ends_with(".scraft") {
        dialogs::run_or_ask(app, "session.open", serde_json::json!({"path": p}))
    } else if lower.ends_with(".mid") || lower.ends_with(".midi") || lower.ends_with(".smf") {
        app.run("file.import_midi", serde_json::json!({"path": p}))
    } else {
        let at = app.engine.session().edit.selection.start;
        app.run("file.import_audio", serde_json::json!({"path": p, "at": at}))
    };
    if let Err(e) = r {
        log::warn!("{p}: {e}");
    }
}

struct Shell {
    app: SoundApp,
    bridge: EguiBridge,
}

impl eframe::App for Shell {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let (opened, _notices) = mahay_android::poll();
        for doc in opened {
            open_path(&mut self.app, &doc.path.to_string_lossy());
        }
        self.app.guarded(|app| app.logic(ctx));
    }

    fn raw_input_hook(&mut self, _ctx: &egui::Context, raw: &mut egui::RawInput) {
        self.bridge.raw_input_hook(raw);
        self.app.raw_input_hook(raw);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let app = &mut self.app;
        self.bridge.inset_ui(ui, |ui| app.guarded(|app| app.ui(ui)));
        self.bridge.after_ui(ui.ctx());
    }
}
