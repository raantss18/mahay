use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicI32, AtomicU8, Ordering};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant, SystemTime};

pub use android_activity::AndroidApp;
use jni::objects::{JClass, JObject, JObjectArray, JString, JValue};
use jni::{JNIEnv, JavaVM};

/// A document to open, copied into the app's private storage.
#[derive(Clone, Debug)]
pub struct Picked {
    /// The local copy, readable and writable with `std::fs`.
    pub path: PathBuf,
    /// The name Android shows for it (`OpenableColumns.DISPLAY_NAME`).
    pub name: String,
}

type Waker = Box<dyn Fn() + Send + Sync>;

/// What a local copy is tied to outside the app.
#[derive(Clone, Debug)]
enum Link {
    /// Saved with Save As, not yet placed: the first write asks where to put it.
    PendingExport { name: String },
    /// Waiting for the user to choose a place for it.
    Exporting,
    /// Mirrors a document the user picked or placed: writes are copied back to it.
    Uri { uri: String },
}

#[derive(Debug)]
struct Tracked {
    link: Link,
    /// Modification time last copied out (or read in).
    synced: Option<SystemTime>,
}

#[derive(Default)]
struct State {
    tracked: HashMap<PathBuf, Tracked>,
    /// Opens whose answer goes to a waiting caller instead of [`poll`].
    waiting: HashMap<i32, Sender<Option<Picked>>>,
    opened: Vec<Picked>,
    notices: Vec<String>,
    last_scan: Option<Instant>,
}

static APP: OnceLock<AndroidApp> = OnceLock::new();
static WAKER: Mutex<Option<Waker>> = Mutex::new(None);
static STATE: OnceLock<Mutex<State>> = OnceLock::new();
static NEXT_REQUEST: AtomicI32 = AtomicI32::new(1);

/// How often local copies are checked for new writes to copy out.
const SCAN_EVERY: Duration = Duration::from_millis(700);

/// Set up logging, a logging panic hook and the data folders. Call first in `android_main`,
/// before any thread starts (it sets environment variables).
pub fn init(app: &AndroidApp, tag: &'static str) {
    android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Info).with_tag(tag));
    std::panic::set_hook(Box::new(|info| log::error!("panic: {info}")));
    let _ = APP.set(app.clone());
    set_editor_info(app);
    let data = app.internal_data_path().unwrap_or_else(|| PathBuf::from("/data/local/tmp"));
    let cache = data.join("cache");
    let vars = [
        ("HOME", data.clone()),
        ("XDG_DATA_HOME", data.join("share")),
        ("XDG_CONFIG_HOME", data.join("config")),
        ("XDG_STATE_HOME", data.join("state")),
        ("XDG_CACHE_HOME", cache.clone()),
        ("TMPDIR", cache),
    ];
    for (key, dir) in vars {
        if let Err(e) = std::fs::create_dir_all(&dir) {
            log::warn!("couldn't create {}: {e}", dir.display());
        }
        // SAFETY: called at the top of `android_main`, before the app starts any other thread,
        // so nothing reads the environment concurrently.
        unsafe { std::env::set_var(key, &dir) };
    }
    if let Some(p) = launch_file() {
        with_state(|s| s.opened.push(p));
    }
    start_cleanup();
}

/// Delete old document copies on a worker thread, a little after start-up so the app's own
/// first reads come first. Copies in use (tracked, or open in this process) are kept.
fn start_cleanup() {
    let spawned = std::thread::Builder::new().name("mahay-docs-cleanup".into()).spawn(|| {
        std::thread::sleep(Duration::from_secs(20));
        let mut open: Vec<PathBuf> = with_state(|s| s.tracked.keys().cloned().collect()).unwrap_or_default();
        open.extend(crate::cleanup::open_files());
        let removed = crate::cleanup::clean_old_copies(&docs_dir(), crate::cleanup::KEEP_COPIES, &open);
        if removed > 0 {
            log::info!("removed {removed} document copies untouched for 30 days");
        }
    });
    if let Err(e) = spawned {
        log::warn!("couldn't start the document cleanup: {e}");
    }
}

/// Called when something arrives from Java (a picked document). Typically
/// `move || ctx.request_repaint()`.
pub fn set_waker(waker: impl Fn() + Send + Sync + 'static) {
    if let Ok(mut w) = WAKER.lock() {
        *w = Some(Box::new(waker));
    }
}

/// The app's private files folder.
pub fn data_dir() -> Option<PathBuf> {
    APP.get().and_then(|a| a.internal_data_path())
}

/// Where local copies of documents live: `files/docs/<n>/<name>`.
fn docs_dir() -> PathBuf {
    data_dir().unwrap_or_else(|| PathBuf::from("/data/local/tmp")).join("docs")
}

/// Ask the user for a document (`ACTION_OPEN_DOCUMENT`). `mimes` filters the picker; empty means
/// any file. The answer arrives later through [`poll`]; nothing arrives if the user cancels.
pub fn open_document(mimes: &[&str]) {
    start_open(0, mimes);
}

/// Like [`open_document`], but the answer (or `None` on cancel) goes to the returned receiver.
/// Wait on it from a worker thread, never from the thread running the app.
pub fn pick_document(mimes: &[&str]) -> Receiver<Option<Picked>> {
    let (tx, rx) = channel();
    let request = NEXT_REQUEST.fetch_add(1, Ordering::Relaxed).max(1);
    with_state(|s| s.waiting.insert(request, tx));
    if !start_open(request, mimes) {
        answer(request, None);
    }
    rx
}

fn start_open(request: i32, mimes: &[&str]) -> bool {
    let dir = fresh_dir().to_string_lossy().into_owned();
    with_activity(|env, act| {
        let array = string_array(env, mimes)?;
        let dir = env.new_string(&dir)?;
        env.call_method(
            act,
            "openDocument",
            "(I[Ljava/lang/String;Ljava/lang/String;)V",
            &[JValue::Int(request), JValue::Object(&array), JValue::Object(&dir)],
        )?;
        Ok(())
    })
    .is_some()
}

fn answer(request: i32, picked: Option<Picked>) {
    let tx = with_state(|s| s.waiting.remove(&request)).flatten();
    if let Some(tx) = tx {
        let _ = tx.send(picked);
    }
}

/// The path to write a Save As to. The app writes there right away; the next [`poll`] then asks
/// the user where to put the document (`ACTION_CREATE_DOCUMENT`), and later writes are copied
/// to that place.
pub fn save_target(suggested: &str) -> PathBuf {
    let name =
        Path::new(suggested).file_name().map(|n| n.to_string_lossy().into_owned()).filter(|n| !n.is_empty()).unwrap_or_else(|| "Document".into());
    let path = fresh_dir().join(&name);
    // Until the user chooses a place, this is the only copy: the cleanup must keep it.
    if let Some(marker) = crate::cleanup::unsaved_marker(&path)
        && let Err(e) = std::fs::write(&marker, b"")
    {
        log::warn!("couldn't mark {} as unsaved: {e}", path.display());
    }
    with_state(|s| {
        s.tracked.insert(path.clone(), Tracked { link: Link::PendingExport { name }, synced: None });
    });
    path
}

/// Documents to open now (picked, or the one the app was launched with), and short messages
/// for the user. Also copies new writes out to where each document came from. Call once a frame;
/// it only touches the disk every [`SCAN_EVERY`].
pub fn poll() -> (Vec<Picked>, Vec<String>) {
    let due = with_state(|s| {
        let due = s.last_scan.is_none_or(|t| t.elapsed() >= SCAN_EVERY);
        if due {
            s.last_scan = Some(Instant::now());
        }
        due
    })
    .unwrap_or(false);
    if due {
        sync_out();
    }
    with_state(|s| (std::mem::take(&mut s.opened), std::mem::take(&mut s.notices))).unwrap_or_default()
}

/// Copy local copies whose modification time moved to where they belong.
fn sync_out() {
    let work: Vec<(PathBuf, Link, SystemTime)> = with_state(|s| {
        s.tracked
            .iter()
            .filter_map(|(path, t)| {
                let modified = std::fs::metadata(path).and_then(|m| m.modified()).ok()?;
                (t.synced != Some(modified) && !matches!(t.link, Link::Exporting)).then(|| (path.clone(), t.link.clone(), modified))
            })
            .collect()
    })
    .unwrap_or_default();
    for (path, link, modified) in work {
        match link {
            Link::PendingExport { name } => {
                with_state(|s| {
                    if let Some(t) = s.tracked.get_mut(&path) {
                        t.link = Link::Exporting;
                    }
                });
                let mime = mime_for(&name);
                let src = path.to_string_lossy().into_owned();
                let started = with_activity(|env, act| {
                    let src = env.new_string(&src)?;
                    let name = env.new_string(&name)?;
                    let mime = env.new_string(mime)?;
                    env.call_method(
                        act,
                        "exportDocument",
                        "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
                        &[JValue::Object(&src), JValue::Object(&name), JValue::Object(&mime)],
                    )?;
                    Ok(())
                });
                if started.is_none() {
                    notice("Couldn't open the save dialog; the document is kept inside the app.");
                }
            }
            Link::Uri { uri } => {
                let src = path.to_string_lossy().into_owned();
                let ok = with_activity(|env, act| {
                    let src = env.new_string(&src)?;
                    let uri = env.new_string(&uri)?;
                    env.call_method(act, "writeBack", "(Ljava/lang/String;Ljava/lang/String;)Z", &[JValue::Object(&src), JValue::Object(&uri)])?.z()
                })
                .unwrap_or(false);
                if ok {
                    with_state(|s| {
                        if let Some(t) = s.tracked.get_mut(&path) {
                            t.synced = Some(modified);
                        }
                    });
                } else {
                    notice("Couldn't save back to the original document; it may have been moved or deleted. Use Save As.");
                    with_state(|s| {
                        if let Some(t) = s.tracked.get_mut(&path) {
                            t.synced = Some(modified);
                        }
                    });
                }
            }
            Link::Exporting => {}
        }
    }
}

/// Start another installed app by package name. `false` if it isn't installed.
pub fn launch_app(package: &str) -> bool {
    with_activity(|env, act| {
        let pkg = env.new_string(package)?;
        env.call_method(act, "launchApp", "(Ljava/lang/String;)Z", &[JValue::Object(&pkg)])?.z()
    })
    .unwrap_or(false)
}

/// Whether a package is installed.
pub fn is_installed(package: &str) -> bool {
    with_activity(|env, act| {
        let pkg = env.new_string(package)?;
        env.call_method(act, "isInstalled", "(Ljava/lang/String;)Z", &[JValue::Object(&pkg)])?.z()
    })
    .unwrap_or(false)
}

/// Open a link in the browser.
pub fn open_url(url: &str) {
    with_activity(|env, act| {
        let url = env.new_string(url)?;
        env.call_method(act, "openUrl", "(Ljava/lang/String;)V", &[JValue::Object(&url)])?;
        Ok(())
    });
}

/// The keyboard every Mahay app wants, set at start-up too: Android may open the keyboard by
/// itself (a field focused when a document opens), and it must behave the same way then.
/// Multi-line, so Enter arrives as "\n" in the buffer (the bridge turns it into an Enter key),
/// and never the full-screen editor that hides the app in landscape (IME_FLAG_NO_EXTRACT_UI |
/// IME_FLAG_NO_FULLSCREEN).
fn set_editor_info(app: &AndroidApp) {
    use android_activity::input::{ImeOptions, InputType, TextInputAction};
    app.set_ime_editor_info(
        InputType::TYPE_CLASS_TEXT | InputType::TYPE_TEXT_FLAG_MULTI_LINE,
        TextInputAction::None,
        ImeOptions::from_bits_retain(0x1000_0000 | 0x0200_0000),
    );
}

/// Show or hide the soft keyboard (GameActivity's GameTextInput).
pub fn set_keyboard(visible: bool) {
    if let Some(app) = APP.get() {
        if visible {
            set_editor_info(app);
            app.show_soft_input(true);
        } else {
            app.hide_soft_input(true);
        }
    }
}

/// System bars, cutout and soft keyboard insets in physical pixels: `[left, top, right, bottom]`.
pub fn system_insets() -> [i32; 4] {
    with_activity(|env, act| {
        let obj = env.call_method(act, "systemInsets", "()[I", &[])?.l()?;
        let array = jni::objects::JIntArray::from(obj);
        let mut out = [0i32; 4];
        env.get_int_array_region(&array, 0, &mut out)?;
        Ok(out)
    })
    .unwrap_or([0; 4])
}

/// The soft keyboard's text buffer (GameTextInput). winit 0.30 drops GameActivity's text
/// events, so [`crate::EguiBridge`] reads this buffer and turns its changes into egui events.
pub fn ime_text() -> String {
    APP.get().map(|a| a.text_input_state().text).unwrap_or_default()
}

/// Empty the soft keyboard's buffer, for a new text field.
pub fn reset_ime() {
    if let Some(app) = APP.get() {
        let empty = android_activity::input::TextSpan { start: 0, end: 0 };
        app.set_text_input_state(android_activity::input::TextInputState { text: String::new(), selection: empty, compose_region: None });
    }
}

/// Light icons on the status and navigation bars for a dark theme, dark icons for a light one.
pub fn set_dark_bars(dark: bool) {
    with_activity(|env, act| {
        env.call_method(act, "setDarkBars", "(Z)V", &[JValue::Bool(u8::from(dark))])?;
        Ok(())
    });
}

/// The user's language as a BCP 47 tag (`fr-FR`, `mg`, `en-US`).
pub fn locale() -> String {
    with_activity(|env, act| {
        let obj = env.call_method(act, "localeTag", "()Ljava/lang/String;", &[])?.l()?;
        Ok(String::from(env.get_string(&JString::from(obj))?))
    })
    .unwrap_or_else(|| "en".into())
}

/// Hand a verified APK to Android's installer (the user confirms). `Err` explains why not:
/// the user must first allow installs from this app (the settings page has been opened), or
/// the file isn't one this app downloaded.
pub fn install_apk(path: &Path) -> Result<(), String> {
    let p = path.to_string_lossy().into_owned();
    let code = with_activity(|env, act| {
        let p = env.new_string(&p)?;
        env.call_method(act, "installApk", "(Ljava/lang/String;)I", &[JValue::Object(&p)])?.i()
    });
    match code {
        Some(0) => Ok(()),
        Some(1) => Err("Allow Mahay to install apps in the settings page that just opened, then press Install again.".into()),
        _ => Err("This update couldn't be handed to Android's installer.".into()),
    }
}

/// Ask Mahay HUB to update this app (decision D21). `Ok` when the request was sent; `Err` says
/// why the hub can't take it, and the app then updates itself.
pub fn hand_off_to_hub() -> Result<(), String> {
    match with_activity(|env, act| env.call_method(act, "handOffToHub", "()I", &[])?.i()) {
        Some(0) => Ok(()),
        Some(1) => Err("Mahay HUB isn't installed".into()),
        Some(2) => Err("Mahay HUB isn't signed with this app's certificate".into()),
        Some(4) => Err("this app doesn't hold Mahay HUB's permission".into()),
        _ => Err("Mahay HUB didn't take the request".into()),
    }
}

/// Whether Mahay HUB is installed and signed with the same certificate as this app.
pub fn hub_usable() -> bool {
    with_activity(|env, act| env.call_method(act, "hubUsable", "()Z", &[])?.z()).unwrap_or(false)
}

/// Hub: schedule the daily background update (kept across reboots), or cancel it.
pub fn schedule_updates(auto: bool, mobile_data: bool) {
    with_activity(|env, act| {
        env.call_method(act, "scheduleUpdates", "(ZZ)V", &[JValue::Bool(u8::from(auto)), JValue::Bool(u8::from(mobile_data))])?;
        Ok(())
    });
}

/// Hub: 0 no network, 1 unmetered, 2 metered (`mahay_update::managed::Network::from_code`).
pub fn network_state() -> i32 {
    with_activity(|env, act| env.call_method(act, "networkState", "()I", &[])?.i()).unwrap_or(0)
}

/// Hub: hand a verified download to Android's installer without a dialog of ours; Android asks
/// the user only when it must. `sha256` is checked again while the file is copied.
pub fn install_background(id: &str, path: &Path, sha256: &str) -> bool {
    let p = path.to_string_lossy().into_owned();
    with_activity(|env, act| {
        let (id, p, sha) = (env.new_string(id)?, env.new_string(&p)?, env.new_string(sha256)?);
        env.call_method(
            act,
            "installBackground",
            "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Z",
            &[JValue::Object(&id), JValue::Object(&p), JValue::Object(&sha)],
        )?
        .z()
    })
    .unwrap_or(false)
}

/// Hub: whether an app says it is on screen.
pub fn is_app_foreground(package: &str) -> bool {
    with_activity(|env, act| {
        let pkg = env.new_string(package)?;
        env.call_method(act, "isAppForeground", "(Ljava/lang/String;)Z", &[JValue::Object(&pkg)])?.z()
    })
    .unwrap_or(false)
}

/// Hub: how many updates wait for the user's tap.
pub fn waiting_updates() -> i32 {
    with_activity(|env, act| env.call_method(act, "waitingUpdates", "()I", &[])?.i()).unwrap_or(0)
}

/// Hub: whether notifications can be shown.
pub fn notifications_allowed() -> bool {
    with_activity(|env, act| env.call_method(act, "notificationsAllowed", "()Z", &[])?.z()).unwrap_or(false)
}

/// Hub: ask for the notification permission (Android 13+). Call it only after the user chose to.
pub fn request_notifications() {
    with_activity(|env, act| {
        env.call_method(act, "requestNotifications", "()V", &[])?;
        Ok(())
    });
}

/// The installed version of another app (`org.mahay.sary`), or `None` when it isn't installed.
pub fn installed_version(package: &str) -> Option<String> {
    with_activity(|env, act| {
        let pkg = env.new_string(package)?;
        let obj = env.call_method(act, "installedVersion", "(Ljava/lang/String;)Ljava/lang/String;", &[JValue::Object(&pkg)])?.l()?;
        if obj.is_null() {
            return Ok(None);
        }
        Ok(Some(String::from(env.get_string(&JString::from(obj))?)))
    })
    .flatten()
}

/// `MahayActivity.nativeOnInstalled(String path, boolean ok, String message)`: an install ended.
#[unsafe(no_mangle)]
pub extern "system" fn Java_org_mahay_shell_MahayActivity_nativeOnInstalled(
    mut env: JNIEnv,
    _class: JClass,
    path: JString,
    ok: jni::sys::jboolean,
    message: JString,
) {
    let path = jstring(&mut env, &path).unwrap_or_default();
    let message = jstring(&mut env, &message).unwrap_or_default();
    if ok != 0 {
        log::info!("installed {path}");
    } else {
        log::warn!("install of {path} failed: {message}");
        with_state(|s| s.notices.push(format!("The installation didn't finish: {message}")));
    }
    wake();
}

/// What touches the screen, from `MotionEvent.getToolType`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolType {
    Unknown,
    Finger,
    Stylus,
    Mouse,
    Eraser,
    Palm,
}

/// Pointer ids tracked for [`tool_type`]; Android numbers pointers from 0 and reuses the lowest
/// free id, so higher ids don't occur in practice.
const POINTERS: usize = 32;
static TOOL_TYPES: [AtomicU8; POINTERS] = [const { AtomicU8::new(0) }; POINTERS];

/// The tool last seen for a touch pointer id (egui's `TouchId`, winit's `Touch::id`).
pub fn tool_type(pointer_id: u64) -> ToolType {
    let raw = usize::try_from(pointer_id).ok().and_then(|i| TOOL_TYPES.get(i)).map_or(0, |t| t.load(Ordering::Relaxed));
    match raw {
        1 => ToolType::Finger,
        2 => ToolType::Stylus,
        3 => ToolType::Mouse,
        4 => ToolType::Eraser,
        5 => ToolType::Palm,
        _ => ToolType::Unknown,
    }
}

/// `MahayActivity.nativeOnToolType(int pointerId, int toolType)`: called on the UI thread for
/// each pointer of a touch or hover event, before GameActivity queues the event for Rust.
#[unsafe(no_mangle)]
pub extern "system" fn Java_org_mahay_shell_MahayActivity_nativeOnToolType(
    _env: JNIEnv,
    _class: JClass,
    pointer_id: jni::sys::jint,
    tool_type: jni::sys::jint,
) {
    if let (Some(slot), Ok(tool)) = (usize::try_from(pointer_id).ok().and_then(|i| TOOL_TYPES.get(i)), u8::try_from(tool_type)) {
        slot.store(tool, Ordering::Relaxed);
    }
}

/// Text on the Android clipboard.
pub fn clipboard_text() -> Option<String> {
    with_activity(|env, act| {
        let obj = env.call_method(act, "clipboardText", "()Ljava/lang/String;", &[])?.l()?;
        if obj.is_null() {
            return Ok(None);
        }
        Ok(Some(String::from(env.get_string(&JString::from(obj))?)))
    })
    .flatten()
}

/// Put text on the Android clipboard.
pub fn set_clipboard_text(text: &str) {
    with_activity(|env, act| {
        let text = env.new_string(text)?;
        env.call_method(act, "setClipboardText", "(Ljava/lang/String;)V", &[JValue::Object(&text)])?;
        Ok(())
    });
}

/// `MahayActivity.nativeOnOpened(int request, String path, String name, String uri)`: a picked or
/// received document, already copied to `path` (null when the user cancelled). `uri` is where
/// writes go back to; null for read-only sources (a shared file). Request 0 goes to [`poll`].
#[unsafe(no_mangle)]
pub extern "system" fn Java_org_mahay_shell_MahayActivity_nativeOnOpened(
    mut env: JNIEnv,
    _class: JClass,
    request: jni::sys::jint,
    path: JString,
    name: JString,
    uri: JString,
) {
    let path = jstring(&mut env, &path).map(PathBuf::from);
    let name = jstring(&mut env, &name).unwrap_or_default();
    let uri = jstring(&mut env, &uri);
    let picked = path.map(|path| {
        let synced = std::fs::metadata(&path).and_then(|m| m.modified()).ok();
        if let Some(uri) = uri {
            with_state(|s| s.tracked.insert(path.clone(), Tracked { link: Link::Uri { uri }, synced }));
        }
        Picked { path, name }
    });
    if request == 0 {
        if let Some(p) = picked {
            with_state(|s| s.opened.push(p));
        }
    } else {
        answer(request, picked);
    }
    wake();
}

/// `MahayActivity.nativeOnExported(String path, String uri)`: where a Save As went, or null
/// `uri` if the user cancelled.
/// Keyboard changes not yet seen by the app: 1 = text changed, 2 = keyboard dismissed.
static IME_CHANGE: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);

/// The soft keyboard changed its text (or was dismissed): wake the app so the frame that shows
/// it runs now, instead of waiting for the next touch or timer.
#[unsafe(no_mangle)]
pub extern "system" fn Java_org_mahay_shell_MahayActivity_nativeOnImeChanged(_env: JNIEnv, _class: JClass, dismissed: jni::sys::jboolean) {
    IME_CHANGE.fetch_max(if dismissed != 0 { 2 } else { 1 }, std::sync::atomic::Ordering::AcqRel);
    wake();
}

/// What the soft keyboard did since the last call: `None` (nothing), `Some(false)` (its text
/// changed) or `Some(true)` (it was dismissed).
pub fn take_ime_change() -> Option<bool> {
    match IME_CHANGE.swap(0, std::sync::atomic::Ordering::AcqRel) {
        0 => None,
        n => Some(n == 2),
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_mahay_shell_MahayActivity_nativeOnExported(mut env: JNIEnv, _class: JClass, path: JString, uri: JString) {
    let Some(path) = jstring(&mut env, &path).map(PathBuf::from) else { return };
    let uri = jstring(&mut env, &uri);
    let synced = std::fs::metadata(&path).and_then(|m| m.modified()).ok();
    with_state(|s| match uri {
        Some(uri) => {
            // Saved outside the app: from now on the local file is a copy like any other.
            if let Some(marker) = crate::cleanup::unsaved_marker(&path) {
                let _ = std::fs::remove_file(marker);
            }
            s.tracked.insert(path, Tracked { link: Link::Uri { uri }, synced });
        }
        None => {
            s.tracked.remove(&path);
            s.notices.push("Not saved outside the app. Use Save As to choose a place.".into());
        }
    });
    wake();
}

/// The document the app was launched with (VIEW or SEND intent).
fn launch_file() -> Option<Picked> {
    let dir = fresh_dir().to_string_lossy().into_owned();
    with_activity(|env, act| {
        let dir = env.new_string(&dir)?;
        let value = env.call_method(act, "takeLaunchFile", "(Ljava/lang/String;)[Ljava/lang/String;", &[JValue::Object(&dir)])?.l()?;
        if value.is_null() {
            return Ok(None);
        }
        let array = JObjectArray::from(value);
        let path = array_string(env, &array, 0)?;
        let name = array_string(env, &array, 1)?;
        let uri = array_string(env, &array, 2)?;
        Ok(Some((path, name, uri)))
    })
    .flatten()
    .map(|(path, name, uri)| {
        let path = PathBuf::from(path);
        if !uri.is_empty() {
            let synced = std::fs::metadata(&path).and_then(|m| m.modified()).ok();
            with_state(|s| {
                s.tracked.insert(path.clone(), Tracked { link: Link::Uri { uri }, synced });
            });
        }
        Picked { path, name }
    })
}

/// A new, empty folder under [`docs_dir`], so two documents with the same name don't collide.
fn fresh_dir() -> PathBuf {
    let base = docs_dir();
    let stamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(0);
    let mut n: u32 = 0;
    loop {
        let dir = base.join(format!("{stamp:x}-{n}"));
        if !dir.exists() {
            if let Err(e) = std::fs::create_dir_all(&dir) {
                log::warn!("couldn't create {}: {e}", dir.display());
            }
            return dir;
        }
        n = n.saturating_add(1);
    }
}

/// The MIME type Android should label a document with, from its extension.
pub fn mime_for(name: &str) -> &'static str {
    let ext = Path::new(name).extension().and_then(|e| e.to_str()).map(str::to_ascii_lowercase).unwrap_or_default();
    match ext.as_str() {
        "pdf" => "application/pdf",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "webp" => "image/webp",
        "gif" => "image/gif",
        "tif" | "tiff" => "image/tiff",
        "svg" => "image/svg+xml",
        "psd" => "image/vnd.adobe.photoshop",
        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "doc" => "application/msword",
        "odt" => "application/vnd.oasis.opendocument.text",
        "rtf" => "application/rtf",
        "xlsx" | "xlsm" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "xls" => "application/vnd.ms-excel",
        "ods" => "application/vnd.oasis.opendocument.spreadsheet",
        "csv" => "text/csv",
        "pptx" => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        "ppt" => "application/vnd.ms-powerpoint",
        "odp" => "application/vnd.oasis.opendocument.presentation",
        "txt" | "md" => "text/plain",
        "html" | "htm" => "text/html",
        "json" => "application/json",
        _ => "application/octet-stream",
    }
}

fn notice(text: &str) {
    with_state(|s| s.notices.push(text.to_string()));
}

fn wake() {
    if let Ok(w) = WAKER.lock()
        && let Some(w) = w.as_ref()
    {
        w();
    }
}

fn with_state<T>(f: impl FnOnce(&mut State) -> T) -> Option<T> {
    let mut state = STATE.get_or_init(Default::default).lock().ok()?;
    Some(f(&mut state))
}

/// Run `f` with a JNI env attached to this thread and the activity object. Logs and returns
/// `None` on any JNI error (a pending Java exception is described and cleared).
fn with_activity<T>(f: impl FnOnce(&mut JNIEnv, &JObject) -> jni::errors::Result<T>) -> Option<T> {
    let app = APP.get()?;
    // SAFETY: android-activity keeps both pointers valid for the app's lifetime.
    let vm = unsafe { JavaVM::from_raw(app.vm_as_ptr().cast()) }.ok()?;
    let mut env = vm.attach_current_thread().ok()?;
    // SAFETY: as above; the activity is a global reference owned by android-activity.
    let activity = unsafe { JObject::from_raw(app.activity_as_ptr().cast()) };
    match f(&mut env, &activity) {
        Ok(v) => Some(v),
        Err(e) => {
            if env.exception_check().unwrap_or(false) {
                let _ = env.exception_describe();
                let _ = env.exception_clear();
            }
            log::warn!("MahayActivity call failed: {e}");
            None
        }
    }
}

pub(crate) fn jstring(env: &mut JNIEnv, s: &JString) -> Option<String> {
    if s.is_null() {
        return None;
    }
    env.get_string(s).ok().map(Into::into)
}

pub(crate) fn string_array<'a>(env: &mut JNIEnv<'a>, items: &[&str]) -> jni::errors::Result<JObjectArray<'a>> {
    let len = i32::try_from(items.len()).unwrap_or(i32::MAX);
    let array = env.new_object_array(len, "java/lang/String", JObject::null())?;
    for (i, item) in (0..len).zip(items) {
        let s = env.new_string(item)?;
        env.set_object_array_element(&array, i, s)?;
    }
    Ok(array)
}

fn array_string(env: &mut JNIEnv, array: &JObjectArray, index: i32) -> jni::errors::Result<String> {
    let item = env.get_object_array_element(array, index)?;
    if item.is_null() {
        return Ok(String::new());
    }
    Ok(env.get_string(&JString::from(item))?.into())
}
