//! The background updater each app runs: a check at start-up (at most once an hour), then every
//! 24 hours while the app is open; the update is downloaded and verified in the background and
//! the app shows it as ready. Installing always waits for the user (Tontolo's rule).

use std::path::PathBuf;
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::{Downloaded, Error, Target, download, is_newer, latest_release, notes_since, SUITE_VERSION};

/// Where the updater is.
#[derive(Clone, Debug, PartialEq)]
pub enum State {
    Idle,
    Checking,
    UpToDate,
    /// A newer version exists but can't be installed from here (a .deb, a distribution
    /// package): say so and point to Mahay HUB or the package manager.
    Available { version: String },
    Downloading { version: String, progress: f32 },
    /// Downloaded and verified: offer to install.
    Ready { update: Downloaded, notes: Option<String> },
    /// A newer version exists and Mahay HUB, which manages this app, will download and install
    /// it: this app downloads nothing.
    Managed { version: String },
    Failed(Error),
}

type Waker = Arc<dyn Fn() + Send + Sync>;

/// One app's updater.
#[derive(Clone)]
pub struct Checker {
    app: &'static str,
    dir: PathBuf,
    can_install: bool,
    /// Whether Mahay HUB manages this app right now (installed and signed with our certificate).
    /// Asked at every check, so installing or removing the hub takes effect at once.
    managed: Option<Arc<dyn Fn() -> bool + Send + Sync>>,
    state: Arc<Mutex<State>>,
    waker: Waker,
}

impl std::fmt::Debug for Checker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Checker").field("app", &self.app).field("state", &self.state()).finish_non_exhaustive()
    }
}

const AUTO_EVERY: Duration = Duration::from_secs(24 * 3600);
const MIN_GAP: u64 = 3600;

impl Checker {
    /// `dir` keeps the downloaded update and the settings (`update.json`). `can_install` is false
    /// where the app can't replace itself (a .deb install): then a newer version is only reported.
    pub fn new(app: &'static str, dir: PathBuf, can_install: bool, waker: impl Fn() + Send + Sync + 'static) -> Self {
        Checker { app, dir, can_install, managed: None, state: Arc::new(Mutex::new(State::Idle)), waker: Arc::new(waker) }
    }

    /// Let Mahay HUB take over updates while `managed()` is true: a newer version is then only
    /// reported ([`State::Managed`]) and nothing is downloaded here, so the update is not
    /// fetched twice. When it is false the app updates itself as before.
    #[must_use]
    pub fn managed_by(mut self, managed: impl Fn() -> bool + Send + Sync + 'static) -> Self {
        self.managed = Some(Arc::new(managed));
        self
    }

    pub fn state(&self) -> State {
        self.state.lock().unwrap_or_else(PoisonError::into_inner).clone()
    }

    /// Automatic updates on (the default) or off.
    pub fn auto(&self) -> bool {
        self.settings().0
    }

    pub fn set_auto(&self, on: bool) {
        let (_, last) = self.settings();
        self.save_settings(on, last);
    }

    /// Check now unless one ran in the last hour, then every 24 hours, while auto is on.
    pub fn start_auto(&self) {
        let this = self.clone();
        let spawned = std::thread::Builder::new().name("mahay-update".into()).spawn(move || {
            loop {
                let (auto, last) = this.settings();
                if auto && now().saturating_sub(last) >= MIN_GAP {
                    this.run();
                }
                std::thread::sleep(AUTO_EVERY);
            }
        });
        if let Err(e) = spawned {
            log::warn!("updater thread: {e}");
        }
    }

    /// Check (and download) now, on a worker thread: the "Check for updates" button.
    pub fn check_now(&self) {
        let this = self.clone();
        if let Err(e) = std::thread::Builder::new().name("mahay-update-now".into()).spawn(move || this.run()) {
            log::warn!("updater thread: {e}");
        }
    }

    /// Forget a ready update the user declined (it stays on disk until the next one).
    pub fn dismiss(&self) {
        self.set(State::Idle);
    }

    fn run(&self) {
        if matches!(self.state(), State::Checking | State::Downloading { .. }) {
            return;
        }
        let Some(target) = Target::current() else { return };
        self.set(State::Checking);
        let (auto, _) = self.settings();
        self.save_settings(auto, now());
        let release = match latest_release() {
            Ok(r) => r,
            Err(e) => return self.set(State::Failed(e)),
        };
        if !is_newer(&release.version, SUITE_VERSION) {
            return self.set(State::UpToDate);
        }
        if !self.can_install {
            return self.set(State::Available { version: release.version.clone() });
        }
        if self.managed.as_ref().is_some_and(|managed| managed()) {
            return self.set(State::Managed { version: release.version.clone() });
        }
        let version = release.version.clone();
        self.set(State::Downloading { version: version.clone(), progress: 0.0 });
        let progress = {
            let state = self.state.clone();
            let waker = self.waker.clone();
            let version = version.clone();
            move |p: f32| {
                *state.lock().unwrap_or_else(PoisonError::into_inner) = State::Downloading { version: version.clone(), progress: p };
                waker();
            }
        };
        match download(&release, self.app, SUITE_VERSION, target, &self.dir.join("updates"), &progress) {
            Ok(update) => {
                let notes = notes_since(&release, SUITE_VERSION);
                self.set(State::Ready { update, notes });
            }
            Err(e) => self.set(State::Failed(e)),
        }
    }

    fn set(&self, s: State) {
        *self.state.lock().unwrap_or_else(PoisonError::into_inner) = s;
        (self.waker)();
    }

    fn settings_path(&self) -> PathBuf {
        self.dir.join("update.json")
    }

    fn settings(&self) -> (bool, u64) {
        let Ok(text) = std::fs::read_to_string(self.settings_path()) else { return (true, 0) };
        let v: serde_json::Value = serde_json::from_str(&text).unwrap_or_default();
        (v.get("auto").and_then(serde_json::Value::as_bool).unwrap_or(true), v.get("lastCheck").and_then(serde_json::Value::as_u64).unwrap_or(0))
    }

    fn save_settings(&self, auto: bool, last: u64) {
        let _ = std::fs::create_dir_all(&self.dir);
        let text = serde_json::json!({ "auto": auto, "lastCheck": last }).to_string();
        if let Err(e) = std::fs::write(self.settings_path(), text) {
            log::warn!("update settings: {e}");
        }
    }
}

fn now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}
