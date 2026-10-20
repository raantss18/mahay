//! Feedback and opt-in diagnostics (decision D28, `docs/FEEDBACK_SETUP.md`, `docs/PRIVACY.md`).
//!
//! Nothing is sent unless the user chooses: a feedback message is sent when they press Send, and
//! anonymous diagnostics only while "Help improve Mahay" is on (off by default). Both go to the
//! owner's Supabase project through its public key, which can only insert rows. What is sent:
//! a random per-install id, the app, the version, the event, and device facts (Android version,
//! manufacturer, model, screen class, language). Never documents, file names, text or location.

use std::path::PathBuf;
use std::sync::{Mutex, OnceLock, PoisonError};
use std::time::Duration;

use serde_json::{Value, json};

const URL: &str = env!("MAHAY_SUPABASE_URL");
const KEY: &str = env!("MAHAY_SUPABASE_KEY");
const VERSION: &str = env!("MAHAY_VERSION");

/// The kinds of message the form offers; the table's check constraint accepts exactly these.
pub const KINDS: [&str; 4] = ["bug", "idea", "question", "praise"];
/// Longest message the table accepts.
pub const MAX_MESSAGE: usize = 1000;

struct State {
    app: &'static str,
    dir: PathBuf,
    install_id: String,
    device: Mutex<Value>,
}

static STATE: OnceLock<State> = OnceLock::new();
static SENDING: Mutex<Status> = Mutex::new(Status::Idle);

/// Where a feedback message is.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Status {
    Idle,
    Sending,
    Sent,
    Failed(String),
}

/// Whether this build knows a Supabase project (a build without `accounts.env` does not).
pub fn configured() -> bool {
    !URL.is_empty() && !KEY.is_empty()
}

/// Start-up, once: the app's data folder, a panic hook that keeps the last panic for the next
/// start, and the events the previous run left (a crash, the daily "open") when diagnostics are on.
pub fn init(app: &'static str, dir: PathBuf) {
    let _ = std::fs::create_dir_all(&dir);
    let install_id = read_or_make_id(&dir);
    if STATE.set(State { app, dir: dir.clone(), install_id, device: Mutex::new(json!({})) }).is_err() {
        return;
    }
    let previous = std::panic::take_hook();
    let panic_file = dir.join("last_panic.txt");
    let hook_file = panic_file.clone();
    std::panic::set_hook(Box::new(move |info| {
        let place = info.location().map(|l| format!("{}:{}", l.file(), l.line())).unwrap_or_default();
        let text = format!("{place} {info}");
        let _ = std::fs::write(&hook_file, truncate(&text, 480));
        previous(info);
    }));
    if let Ok(text) = std::fs::read_to_string(&panic_file) {
        let _ = std::fs::remove_file(&panic_file);
        report("crash", Some(&text));
    }
    let today = days_since_epoch().to_string();
    let seen = dir.join("last_open");
    if std::fs::read_to_string(&seen).ok().as_deref() != Some(today.as_str()) {
        let _ = std::fs::write(&seen, &today);
        report("open", None);
    }
}

/// Device facts the platform layer knows (Android: version, manufacturer, model, ABI, screen class).
/// Keys are short strings; the whole object is limited to 2000 bytes by the table.
pub fn set_device(info: Value) {
    if let Some(s) = STATE.get() {
        *s.device.lock().unwrap_or_else(PoisonError::into_inner) = info;
    }
}

/// Whether anonymous diagnostics are on (off until the user turns them on).
pub fn diagnostics_enabled() -> bool {
    STATE.get().is_some_and(|s| s.dir.join("diagnostics.on").exists())
}

pub fn set_diagnostics(on: bool) {
    if let Some(s) = STATE.get() {
        let p = s.dir.join("diagnostics.on");
        if on {
            let _ = std::fs::write(p, b"1");
        } else {
            let _ = std::fs::remove_file(p);
        }
    }
}

/// Send one diagnostics event (`crash`, `native_crash`, `anr`, `low_memory`, `killed`,
/// `update_ok`, `update_failed`, `install_failed`, `open`), only when diagnostics are on.
pub fn report(event: &str, detail: Option<&str>) {
    let Some(s) = STATE.get() else { return };
    if !configured() || !diagnostics_enabled() {
        return;
    }
    let row = json!({
        "install_id": s.install_id, "app": s.app, "version": VERSION, "event": event,
        "detail": detail.map(|d| truncate(d, 480)), "device": device(s),
    });
    std::thread::spawn(move || {
        if let Err(e) = post("diagnostics", &row) {
            log::info!("diagnostics not sent: {e}");
        }
    });
}

/// A feedback message from the form.
pub struct Feedback {
    /// One of [`KINDS`].
    pub kind: String,
    pub message: String,
    /// An email or a handle, if the user wants an answer.
    pub contact: String,
    /// Whether to add the device facts.
    pub include_device: bool,
}

/// The device facts a message would carry, as the form shows them before sending.
pub fn device_preview() -> String {
    STATE.get().map(|s| serde_json::to_string_pretty(&device(s)).unwrap_or_default()).unwrap_or_default()
}

pub fn status() -> Status {
    SENDING.lock().unwrap_or_else(PoisonError::into_inner).clone()
}

pub fn reset_status() {
    let mut g = SENDING.lock().unwrap_or_else(PoisonError::into_inner);
    if !matches!(*g, Status::Sending) {
        *g = Status::Idle;
    }
}

/// Send `f` in the background; follow it with [`status`].
pub fn send_feedback(f: Feedback, repaint: impl Fn() + Send + 'static) {
    let Some(s) = STATE.get() else { return };
    let message = f.message.trim().to_string();
    if message.is_empty() || !KINDS.contains(&f.kind.as_str()) {
        return;
    }
    {
        let mut g = SENDING.lock().unwrap_or_else(PoisonError::into_inner);
        if matches!(*g, Status::Sending) {
            return;
        }
        *g = Status::Sending;
    }
    let contact = f.contact.trim();
    let row = json!({
        "install_id": s.install_id, "app": s.app, "version": VERSION, "kind": f.kind,
        "message": truncate(&message, MAX_MESSAGE),
        "contact": (!contact.is_empty()).then(|| truncate(contact, 190)),
        "device": f.include_device.then(|| device(s)),
    });
    std::thread::spawn(move || {
        let result = if configured() { post("feedback", &row) } else { Err("not available in this build".into()) };
        *SENDING.lock().unwrap_or_else(PoisonError::into_inner) = match result {
            Ok(()) => Status::Sent,
            Err(e) => Status::Failed(e),
        };
        repaint();
    });
}

fn device(s: &State) -> Value {
    let mut d = s.device.lock().unwrap_or_else(PoisonError::into_inner).clone();
    if let Some(o) = d.as_object_mut() {
        o.insert("os".into(), json!(std::env::consts::OS));
        o.insert("arch".into(), json!(std::env::consts::ARCH));
    }
    d
}

fn post(table: &str, row: &Value) -> Result<(), String> {
    let agent: ureq::Agent = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(20)))
        .user_agent(concat!("mahay-feedback/", env!("MAHAY_VERSION")))
        .build()
        .into();
    agent
        .post(format!("{}/rest/v1/{table}", URL.trim_end_matches('/')))
        .header("apikey", KEY)
        .header("Authorization", format!("Bearer {KEY}"))
        .header("Content-Type", "application/json")
        .header("Prefer", "return=minimal")
        .send(row.to_string())
        .map(|_| ())
        .map_err(|e| match e {
            // The tables are missing (docs/FEEDBACK_SETUP.md not run yet) or the key was refused.
            ureq::Error::StatusCode(404) => "the service is not ready yet (404)".into(),
            ureq::Error::StatusCode(c @ (401 | 403)) => format!("refused ({c})"),
            ureq::Error::StatusCode(429) => "too many messages, try later (429)".into(),
            other => other.to_string(),
        })
}

fn truncate(s: &str, max: usize) -> String {
    s.chars().take(max).collect()
}

fn days_since_epoch() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs() / 86_400).unwrap_or(0)
}

/// A random version-4 UUID kept in the app's folder: it names the install, nothing else.
fn read_or_make_id(dir: &std::path::Path) -> String {
    let path = dir.join("install_id");
    if let Ok(id) = std::fs::read_to_string(&path) {
        let id = id.trim();
        if id.len() == 36 {
            return id.to_string();
        }
    }
    let mut b = [0u8; 16];
    let ok = std::fs::File::open("/dev/urandom").and_then(|mut f| std::io::Read::read_exact(&mut f, &mut b)).is_ok();
    if !ok {
        let n = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(1);
        b.copy_from_slice(&(n ^ (u128::from(std::process::id()) << 64)).to_le_bytes());
    }
    b[6] = (b[6] & 0x0f) | 0x40;
    b[8] = (b[8] & 0x3f) | 0x80;
    let h: String = b.iter().map(|x| format!("{x:02x}")).collect();
    let id = format!("{}-{}-{}-{}-{}", &h[..8], &h[8..12], &h[12..16], &h[16..20], &h[20..]);
    let _ = std::fs::write(path, &id);
    id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_is_a_stable_uuid() {
        let dir = std::env::temp_dir().join(format!("mahay-fb-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let a = read_or_make_id(&dir);
        assert_eq!(a.len(), 36);
        assert_eq!(a.as_bytes()[14], b'4');
        assert_eq!(a, read_or_make_id(&dir));
        std::fs::remove_dir_all(dir).unwrap();
    }
}
