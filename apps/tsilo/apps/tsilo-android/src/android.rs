use eframe::egui;
use mahay_android::{AndroidApp, EguiBridge};
use tsilo_engine::Session;
use tsilo_engine::cmd::fileio;
use tsilo_ui_egui::picks::PickRequest;
use tsilo_ui_egui::{Services, TsiloApp};

use crate::clipboard;

#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    mahay_android::init(&app, "Tsilo");
    let options = eframe::NativeOptions { android_app: Some(app), renderer: eframe::Renderer::Wgpu, ..Default::default() };
    let result = eframe::run_native(
        "Tsilo",
        options,
        Box::new(|cc| {
            let mut app = TsiloApp::new(Session::new(), services(cc.egui_ctx.clone()));
            let data = mahay_android::data_dir();
            let sub = |name: &str| data.as_ref().map(|d| d.join(name).to_string_lossy().to_string());
            app.session.swatch_libraries.set_user_dir(sub("Swatches"));
            app.session.style_libraries.set_user_dir(sub("Graphic Styles"));
            app.session.libraries.set_dir(sub("Libraries"));
            app.session.recovery.set_default_folder(sub("Data Recovery"));
            let bridge = EguiBridge::new(&cc.egui_ctx, mahay_about::AppInfo { id: "tsilo", name: "Tsilo" });
            Ok(Box::new(Shell { app, bridge }))
        }),
    );
    if let Err(e) = result {
        log::error!("Tsilo stopped: {e}");
    }
}

fn write_file(path: &str, bytes: &[u8]) -> Result<(), String> {
    fileio::write_atomic(std::path::Path::new(path), bytes).map_err(|e| e.to_string())
}

/// Show `request`'s dialog (blocking: called on a dialog thread, never the app's).
fn show(request: &PickRequest) -> Vec<String> {
    let s = |p: std::path::PathBuf| p.to_string_lossy().to_string();
    match request {
        PickRequest::Open(pick) => {
            let d = pick.filters.iter().fold(rfd::FileDialog::new(), |d, (name, exts)| d.add_filter(*name, exts));
            d.pick_file().map(s).into_iter().collect()
        }
        PickRequest::OpenMany => rfd::FileDialog::new().pick_files().unwrap_or_default().into_iter().map(s).collect(),
        PickRequest::Save(pick) => rfd::FileDialog::new().set_file_name(&pick.name).save_file().map(s).into_iter().collect(),
        // No folder picker on Android yet.
        PickRequest::Folder => Vec::new(),
    }
}

fn services(ui: egui::Context) -> Services {
    Services {
        start_pick: Some(Box::new(move |request: PickRequest| {
            let (tx, rx) = std::sync::mpsc::channel();
            let ui = ui.clone();
            std::thread::Builder::new()
                .name("file-dialog".into())
                .spawn(move || {
                    let _ = tx.send(show(&request));
                    ui.request_repaint();
                })
                .ok()?;
            Some(rx)
        })),
        read: Some(Box::new(|p: &str| std::fs::read(p).map_err(|e| e.to_string()))),
        write: Some(Box::new(write_file)),
        exists: Some(Box::new(|p: &str| std::path::Path::new(p).exists())),
        write_shared: Some(std::sync::Arc::new(write_file)),
        stream_shared: Some(std::sync::Arc::new(|p: &str, fill: &mut dyn FnMut(&mut dyn std::io::Write) -> std::io::Result<()>| {
            fileio::write_file_with(p, fill).map_err(|e| e.to_string())
        })),
        system_clipboard: Some(clipboard::system_clipboard()),
        open_url: Some(Box::new(|url: &str| mahay_android::open_url(url))),
        ..Default::default()
    }
}

struct Shell {
    app: TsiloApp,
    bridge: EguiBridge,
}

impl eframe::App for Shell {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let _ = mahay_android::poll();
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
