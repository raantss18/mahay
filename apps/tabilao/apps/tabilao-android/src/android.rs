use std::cell::RefCell;
use std::rc::Rc;

use eframe::egui;
use mahay_android::{AndroidApp, EguiBridge};
use tabilao_engine::Session;
use tabilao_ui_egui::{Inbox, Services, SheetApp};

/// Spreadsheet MIME types for the open picker.
const OPEN_MIMES: &[&str] = &[
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
    "application/vnd.ms-excel.sheet.macroEnabled.12",
    "application/vnd.ms-excel",
    "application/vnd.ms-excel.sheet.binary.macroEnabled.12",
    "application/vnd.oasis.opendocument.spreadsheet",
    "text/csv",
    "text/comma-separated-values",
    "text/tab-separated-values",
    "text/plain",
    "application/json",
    "application/octet-stream",
];
const PICTURE_MIMES: &[&str] = &["image/png", "image/jpeg"];

#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    mahay_android::init(&app, "Tabilao");
    log::info!("Tabilao {} starting", env!("CARGO_PKG_VERSION"));
    let options = eframe::NativeOptions { android_app: Some(app), renderer: eframe::Renderer::Wgpu, ..Default::default() };
    let result = eframe::run_native(
        "Tabilao",
        options,
        Box::new(|cc| {
            let picture: Rc<RefCell<Option<Inbox>>> = Rc::default();
            let mut session = Session::new();
            session.new_workbook();
            let mut app = SheetApp::new(session, services(picture.clone()));
            app.ui.language = tabilao_ui_egui::i18n::Language::system();
            app.ui.system_theme = true;
            SheetApp::setup_context_for_language(&cc.egui_ctx, app.ui.dark, app.ui.language);
            Ok(Box::new(Shell { app, picture, bridge: EguiBridge::new(&cc.egui_ctx, mahay_about::AppInfo { id: "tabilao", name: "Tabilao" }) }))
        }),
    );
    if let Err(e) = result {
        log::error!("Tabilao stopped: {e}");
    }
}

struct Shell {
    app: SheetApp,
    /// Set while Insert ▸ Picture waits for the picker, so the answer goes to the picture dialog
    /// and isn't opened as a workbook.
    picture: Rc<RefCell<Option<Inbox>>>,
    bridge: EguiBridge,
}

impl eframe::App for Shell {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let (opened, notices) = mahay_android::poll();
        for doc in opened {
            if let Some(inbox) = self.picture.borrow_mut().take() {
                match std::fs::read(&doc.path) {
                    Ok(bytes) => inbox.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push((doc.name, bytes)),
                    Err(e) => log::warn!("picture {}: {e}", doc.path.display()),
                }
            } else if let Some(path) = doc.path.to_str() {
                self.app.open_path(path);
            }
        }
        if let Some(text) = notices.last() {
            self.app.message = Some(("Tabilao".into(), text.clone()));
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

fn services(picture: Rc<RefCell<Option<Inbox>>>) -> Services {
    Services {
        open_async: Some(Box::new(|| mahay_android::open_document(OPEN_MIMES))),
        pick_save: Some(Box::new(|suggested: &str| mahay_android::save_target(suggested).to_str().map(str::to_string))),
        pick_picture_async: Some(Box::new(move |inbox| {
            *picture.borrow_mut() = Some(inbox);
            mahay_android::open_document(PICTURE_MIMES);
        })),
        paste_text: Some(Box::new(mahay_android::clipboard_text)),
        open_url: Some(Box::new(mahay_android::open_url)),
        ..Default::default()
    }
}
