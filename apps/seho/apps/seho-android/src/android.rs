use eframe::egui;
use mahay_android::{AndroidApp, EguiBridge};
use seho_engine::Session;
use seho_ui_egui::{Inbox, InboxEvent, InboxFile, Services, SlideApp};

const PRESENTATION_MIMES: &[&str] = &[
    "application/vnd.openxmlformats-officedocument.presentationml.presentation",
    "application/vnd.openxmlformats-officedocument.presentationml.template",
    "application/vnd.openxmlformats-officedocument.presentationml.slideshow",
    "text/plain",
    "text/markdown",
    "application/octet-stream",
];

#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    mahay_android::init(&app, "Seho");
    log::info!("Seho {} starting", env!("CARGO_PKG_VERSION"));
    let options = mahay_android::native_options!(app);
    let result = eframe::run_native(
        "Seho",
        options,
        Box::new(|cc| {
            let inbox = Inbox::default();
            let mut app = SlideApp::new(Session::new(), services(inbox.clone(), cc.egui_ctx.clone()));
            if let Some(dir) = config_dir() {
                let (ui, prefs) = seho_ui_egui::settings::load(&dir);
                if let Some(ui) = ui {
                    app.ui = ui;
                }
                if let Some(prefs) = prefs {
                    app.session.prefs = prefs;
                }
            }
            app.session.recovery_dir = seho_engine::recovery::default_dir();
            if let Ok(v) = app.session.execute("file.recovery.open", &serde_json::json!({}))
                && let Some(n) = v.get("opened").and_then(|o| o.as_array()).map(Vec::len).filter(|n| *n > 0)
            {
                app.set_status(format!("Recovered {n} unsaved presentation{} from the last session", if n == 1 { "" } else { "s" }));
            }
            Ok(Box::new(Shell { app, bridge: EguiBridge::new(&cc.egui_ctx, mahay_about::AppInfo { id: "seho", name: "Seho" }) }))
        }),
    );
    if let Err(e) = result {
        log::error!("Seho stopped: {e}");
        mahay_android::show_error(&format!("{} could not start its display. {e}", "Seho"));
    }
}

struct Shell {
    app: SlideApp,
    bridge: EguiBridge,
}

impl eframe::App for Shell {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let (opened, notices) = mahay_android::poll();
        for doc in opened {
            let path = doc.path.to_string_lossy().into_owned();
            if let Err(e) = self.app.open_path(&path) {
                log::error!("{path}: {e}");
            }
        }
        if let Some(text) = notices.last() {
            self.app.set_status(text.clone());
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
        if let Some(dir) = config_dir() {
            seho_ui_egui::settings::save(&dir, &self.app.ui, &self.app.session.prefs);
        }
    }
}

fn config_dir() -> Option<std::path::PathBuf> {
    std::env::var_os("XDG_CONFIG_HOME").map(|c| std::path::PathBuf::from(c).join("seho"))
}

fn push(inbox: &Inbox, event: InboxEvent) {
    inbox.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push(event);
}

fn services(inbox: Inbox, ctx: egui::Context) -> Services {
    let open_inbox = inbox.clone();
    Services {
        open_async: Some(Box::new(move |purpose: &str, request: u64| {
            let inbox = &open_inbox;
            let mimes: &'static [&'static str] = match purpose {
                "picture" => &["image/*"],
                "audio" => &["audio/*"],
                "video" => &["video/*"],
                _ => {
                    // A presentation opens by path (see `Shell::logic`) so saves go back to it;
                    // the byte request is withdrawn.
                    push(inbox, InboxEvent::Cancelled { id: request });
                    mahay_android::open_document(PRESENTATION_MIMES);
                    return;
                }
            };
            let rx = mahay_android::pick_document(mimes);
            let (inbox, ctx) = (inbox.clone(), ctx.clone());
            let spawned = std::thread::Builder::new().name("seho-pick".into()).spawn(move || {
                let event = match rx.recv().ok().flatten().map(|p| (std::fs::read(&p.path), p.name)) {
                    Some((Ok(bytes), name)) => InboxEvent::File(InboxFile { id: request, name, bytes }),
                    Some((Err(e), name)) => {
                        log::warn!("{name}: {e}");
                        InboxEvent::Cancelled { id: request }
                    }
                    None => InboxEvent::Cancelled { id: request },
                };
                push(&inbox, event);
                ctx.request_repaint();
            });
            if let Err(e) = spawned {
                log::error!("picker thread: {e}");
            }
        })),
        pick_save: Some(Box::new(|name: &str| Some(mahay_android::save_target(name).to_string_lossy().into_owned()))),
        read: Some(Box::new(|p: &str| std::fs::read(p).map_err(|e| e.to_string()))),
        write: Some(Box::new(|p: &str, b: &[u8]| std::fs::write(p, b).map_err(|e| e.to_string()))),
        inbox: Some(inbox),
        clipboard_set_html: Some(Box::new(|_html: &str, text: &str| {
            mahay_android::set_clipboard_text(text);
            true
        })),
        audio_out: Some(Box::new(crate::audio::CpalOut::default()) as Box<dyn seho_media::AudioOut>),
        ..Default::default()
    }
}
