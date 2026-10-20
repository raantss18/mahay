use std::sync::{Arc, Mutex, PoisonError};

use eframe::egui;
use endrika_engine::Session;
use endrika_ui_egui::FigApp;
use endrika_ui_egui::windows::with_app;
use mahay_android::{AndroidApp, EguiBridge};
use serde_json::json;

use crate::{recovery, services};

#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    mahay_android::init(&app, "Endrika");
    let ctx_slot: Arc<Mutex<Option<egui::Context>>> = Arc::new(Mutex::new(None));
    let fonts_slot = ctx_slot.clone();
    std::thread::spawn(move || {
        endrika_text::scan_system_fonts();
        if let Some(c) = fonts_slot.lock().unwrap_or_else(PoisonError::into_inner).as_ref() {
            c.request_repaint();
        }
    });
    let mut session = Session::empty();
    let wake_slot = ctx_slot.clone();
    session.live.install(Box::new(endrika_collab_net::NetLink::new(Arc::new(move || {
        if let Some(c) = wake_slot.lock().unwrap_or_else(PoisonError::into_inner).as_ref() {
            c.request_repaint();
        }
    }))));
    let recovery_root = services::config_dir().map(|d| d.join("Recovery"));
    let recovery = recovery::Shared::default();
    let svc = services::Desktop::load(recovery.clone());
    if let Some(prefs) = svc.prefs.clone() {
        session.prefs = prefs;
    }
    let recovered = recovery_root.as_deref().map(|r| recovery::recover_all(r, &mut session)).unwrap_or_default();
    let mut copies = recovery_root.as_deref().and_then(recovery::Recovery::start);
    if let Some(r) = copies.as_mut() {
        r.adopt(recovered.waiting);
    }
    *recovery::lock(&recovery) = copies;
    session.plugins.install_dir = services::config_dir().map(|d| d.join("Plugins"));
    let first_run = svc.prefs.is_none();
    if session.docs.is_empty() && !first_run {
        let _ = session.execute("file.new", &json!({}));
    }
    endrika_ui_egui::i18n::set(&session.prefs.language, Some(&mahay_android::locale()));
    let mut fig = FigApp::new(session, Box::new(svc));
    fig.ui.home = false;
    fig.note_recovered(recovered.copies);
    endrika_ui_egui::windows::install(fig);

    let options = mahay_android::native_options!(app);
    let result = eframe::run_native(
        "Endrika",
        options,
        Box::new(move |cc| {
            *ctx_slot.lock().unwrap_or_else(PoisonError::into_inner) = Some(cc.egui_ctx.clone());
            let bridge = EguiBridge::new(&cc.egui_ctx, mahay_about::AppInfo { id: "endrika", name: "Endrika" });
            Ok(Box::new(Shell { bridge, recovery }))
        }),
    );
    if let Err(e) = result {
        log::error!("Endrika stopped: {e}");
        mahay_android::show_error(&format!("{} could not start its display. {e}", "Endrika"));
    }
}

struct Shell {
    bridge: EguiBridge,
    recovery: recovery::Shared,
}

impl eframe::App for Shell {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let (opened, _notices) = mahay_android::poll();
        for doc in opened {
            let path = doc.path.to_string_lossy().into_owned();
            // `run` reports failures in the app itself (status line, dialogs).
            let _ = with_app(|fig| fig.run("file.open", json!({"path": path})));
        }
        let _ = with_app(|fig| endrika_ui_egui::windows::on_close_request(fig, ctx));
        let now = ctx.input(|i| i.time);
        if let Some(wait) = with_app(|fig| recovery::lock(&self.recovery).as_mut().and_then(|r| r.tick(&fig.session, now))).flatten() {
            ctx.request_repaint_after(std::time::Duration::from_secs_f64(wait));
        }
    }

    fn raw_input_hook(&mut self, _ctx: &egui::Context, raw: &mut egui::RawInput) {
        self.bridge.raw_input_hook(raw);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        self.bridge.inset_ui(ui, |ui| {
            let _ = with_app(|fig| endrika_ui_egui::windows::guard(fig, &ctx, |fig| fig.ui(ui)));
        });
        self.bridge.after_ui(&ctx);
    }

    fn on_exit(&mut self) {
        let copies = recovery::lock(&self.recovery).take();
        let _ = with_app(|fig| services::shutdown(fig, services::config_dir().as_deref(), copies));
    }
}
