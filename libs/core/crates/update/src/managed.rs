//! Mahay HUB managing the other apps (decision D21).
//!
//! Everything here runs without a window: the hub's start-up check, its daily job and the
//! requests from the other apps all call [`run`]. It keeps every invariant of [`crate::download`]
//! (signed `SHA256SUMS.txt`, SHA-256 per file, https on GitHub only, never a downgrade) because
//! it calls it; this module only decides *what* to update and *when*:
//!
//! - [`app_for_package`] is the only way a package name from outside becomes an app to update:
//!   an exact match against the suite's apps (never a path, a URI or a prefix);
//! - [`pending`] lists the installed apps older than the release, in the suite's order;
//! - [`decide`] applies the user's settings (automatic updates; unmetered networks unless
//!   "also on mobile data") to the network the device has now.

use std::path::{Path, PathBuf};

use crate::{Downloaded, Error, Release, Target, download, is_newer, latest_release};

/// The apps Mahay HUB manages, in the order it shows them (the hub itself updates through its
/// own update bar).
pub const SUB_APPS: [&str; 13] =
    ["sary", "taratasy", "soratra", "tabilao", "seho", "tsilo", "endrika", "pejy", "hazavana", "feo", "hetsika", "tselatra", "drafitra"];
/// The hub's package.
pub const HUB_PACKAGE: &str = "org.mahay.hub";
/// Every app's Android package is this prefix and its id.
const PREFIX: &str = "org.mahay.";

/// The Android package of a suite app.
pub fn package_of(id: &str) -> String {
    format!("{PREFIX}{id}")
}

/// The sub-app a package name stands for, or `None` for anything else (the hub itself, an
/// unknown app, a path, a URI, different case, extra characters).
pub fn app_for_package(package: &str) -> Option<&'static str> {
    let id = package.strip_prefix(PREFIX)?;
    SUB_APPS.iter().copied().find(|app| *app == id)
}

/// `id=version` lines (from Java) as known sub-apps with their installed version. Unknown ids
/// and unreadable lines are dropped.
pub fn parse_installed<S: AsRef<str>>(lines: &[S]) -> Vec<(&'static str, String)> {
    lines
        .iter()
        .filter_map(|line| {
            let (id, version) = line.as_ref().split_once('=')?;
            let id = SUB_APPS.iter().copied().find(|app| *app == id)?;
            crate::Version::parse(version)?;
            Some((id, version.trim().to_string()))
        })
        .collect()
}

/// An installed app the release is newer than.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pending {
    pub id: &'static str,
    pub installed: String,
    pub latest: String,
}

/// The installed apps that need an update to `latest`, in the suite's order. `only` restricts
/// the list to one app (a request from that app); an `only` that isn't a sub-app gives nothing.
pub fn pending(latest: &str, installed: &[(&'static str, String)], only: Option<&str>) -> Vec<Pending> {
    SUB_APPS
        .iter()
        .filter(|id| only.is_none_or(|o| o == **id))
        .filter_map(|id| {
            let (_, version) = installed.iter().find(|(i, _)| i == id)?;
            is_newer(latest, version).then(|| Pending { id, installed: version.clone(), latest: latest.to_string() })
        })
        .collect()
}

/// The apps still to install for "Install all", in the suite's order.
pub fn missing(installed: &[(&'static str, String)]) -> Vec<&'static str> {
    SUB_APPS.iter().copied().filter(|id| !installed.iter().any(|(i, _)| i == id)).collect()
}

/// What network the device has.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Network {
    None,
    /// Wi-Fi, ethernet: data costs nothing extra.
    Unmetered,
    /// Mobile data, a hotspot: the user pays by the megabyte.
    Metered,
}

impl Network {
    /// From the code Java reports (0 none, 1 unmetered, 2 metered).
    pub fn from_code(code: i32) -> Network {
        match code {
            1 => Network::Unmetered,
            2 => Network::Metered,
            _ => Network::None,
        }
    }
}

/// The user's choices, kept in `managed.json` in the hub's data folder.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    /// "Update automatically": at start, every day and in the background. On by default.
    pub auto: bool,
    /// "Also on mobile data". Off by default: data is expensive.
    pub mobile_data: bool,
    /// Seconds since the epoch of the last automatic run.
    pub last_run: u64,
}

impl Default for Settings {
    fn default() -> Self {
        Settings { auto: true, mobile_data: false, last_run: 0 }
    }
}

const FILE: &str = "managed.json";
/// Automatic runs are at least this far apart (seconds), so opening the hub often costs nothing.
pub const MIN_GAP: u64 = 3600;

impl Settings {
    pub fn load(dir: &Path) -> Settings {
        let Ok(text) = std::fs::read_to_string(dir.join(FILE)) else { return Settings::default() };
        let v: serde_json::Value = serde_json::from_str(&text).unwrap_or_default();
        let d = Settings::default();
        Settings {
            auto: v.get("auto").and_then(serde_json::Value::as_bool).unwrap_or(d.auto),
            mobile_data: v.get("mobileData").and_then(serde_json::Value::as_bool).unwrap_or(d.mobile_data),
            last_run: v.get("lastRun").and_then(serde_json::Value::as_u64).unwrap_or(d.last_run),
        }
    }

    pub fn save(&self, dir: &Path) {
        let _ = std::fs::create_dir_all(dir);
        let text = serde_json::json!({ "auto": self.auto, "mobileData": self.mobile_data, "lastRun": self.last_run }).to_string();
        if let Err(e) = std::fs::write(dir.join(FILE), text) {
            log::warn!("managed update settings: {e}");
        }
    }
}

/// Whether to go ahead, and if not why not.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Decision {
    Run,
    Disabled,
    NoNetwork,
    /// On mobile data without the user's "also on mobile data".
    Metered,
    TooSoon,
}

/// Apply the settings to this moment. `explicit` is a request from the user (the Install button
/// of an app, a button in the hub): it ignores the schedule and the "automatic" switch and may
/// use mobile data, because the user asked for this update now.
pub fn decide(settings: &Settings, network: Network, now: u64, explicit: bool) -> Decision {
    if network == Network::None {
        return Decision::NoNetwork;
    }
    if explicit {
        return Decision::Run;
    }
    if !settings.auto {
        return Decision::Disabled;
    }
    if network == Network::Metered && !settings.mobile_data {
        return Decision::Metered;
    }
    if now.saturating_sub(settings.last_run) < MIN_GAP {
        return Decision::TooSoon;
    }
    Decision::Run
}

/// One app's result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// Downloaded and verified: ready for the installer.
    Ready { id: &'static str, update: Downloaded },
    Failed { id: &'static str, error: Error },
}

/// Where an app's download is kept: `<root>/apps/<id>`.
pub fn dir_for(root: &Path, id: &str) -> PathBuf {
    root.join("apps").join(id)
}

/// Download and verify every pending update (one app after the other). `progress` gets the app
/// and the fraction done.
pub fn fetch(release: &Release, todo: &[Pending], target: Target, root: &Path, progress: &dyn Fn(&'static str, f32)) -> Vec<Outcome> {
    todo.iter()
        .map(|p| {
            let report = |f: f32| progress(p.id, f);
            match download(release, p.id, &p.installed, target, &dir_for(root, p.id), &report) {
                Ok(update) => Outcome::Ready { id: p.id, update },
                Err(error) => Outcome::Failed { id: p.id, error },
            }
        })
        .collect()
}

/// The whole headless update: read the latest release, pick the apps that are behind (only
/// `only` when given), download and verify them.
pub fn run(installed: &[(&'static str, String)], only: Option<&str>, root: &Path, progress: &dyn Fn(&'static str, f32)) -> Result<Vec<Outcome>, Error> {
    let target = Target::current().ok_or(Error::Missing)?;
    let release = latest_release()?;
    let todo = pending(&release.version, installed, only);
    if todo.is_empty() {
        return Ok(Vec::new());
    }
    Ok(fetch(&release, &todo, target, root, progress))
}

/// The verified downloads as `id\tversion\tsha256\tpath` lines, for the Java installer.
pub fn encode_ready(outcomes: &[Outcome]) -> Vec<String> {
    outcomes
        .iter()
        .filter_map(|o| match o {
            Outcome::Ready { id, update } => Some(format!("{id}\t{}\t{}\t{}", update.version, update.sha256, update.path.display())),
            Outcome::Failed { .. } => None,
        })
        .collect()
}

/// The background job's whole run, as lines for Java: the first is the status (`ok`,
/// `skip:<why>`, `refused`, `error:<key>`), then one `id\tversion\tsha256\tpath` per verified
/// download. `only_package` is an update request from an app: refused unless it names a
/// sub-app exactly. `installed` is `id=version` lines.
pub fn headless(root: &Path, installed: &[String], only_package: Option<&str>, network: Network, explicit: bool, now: u64) -> Vec<String> {
    let only = match only_package {
        Some(package) => match app_for_package(package) {
            Some(id) => Some(id),
            None => return vec!["refused".into()],
        },
        None => None,
    };
    let mut settings = Settings::load(root);
    match decide(&settings, network, now, explicit) {
        Decision::Run => {}
        why => return vec![format!("skip:{}", format!("{why:?}").to_lowercase())],
    }
    if !explicit {
        settings.last_run = now;
        settings.save(root);
    }
    match run(&parse_installed(installed), only, root, &|_, _| {}) {
        Ok(outcomes) => std::iter::once("ok".to_string()).chain(encode_ready(&outcomes)).collect(),
        Err(e) => vec![format!("error:{}", e.key())],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn installed(items: &[(&'static str, &str)]) -> Vec<(&'static str, String)> {
        items.iter().map(|(i, v)| (*i, v.to_string())).collect()
    }

    #[test]
    fn only_exact_sub_app_packages_are_accepted() {
        assert_eq!(app_for_package("org.mahay.sary"), Some("sary"));
        assert_eq!(app_for_package("org.mahay.drafitra"), Some("drafitra"));
        // The hub updates itself through its own bar; a request for it is refused.
        assert_eq!(app_for_package("org.mahay.hub"), None);
        for bad in [
            "",
            "sary",
            "org.mahay.",
            "org.mahay.sary ",
            " org.mahay.sary",
            "org.mahay.sary\n",
            "org.mahay.Sary",
            "ORG.MAHAY.SARY",
            "org.mahay.sary.evil",
            "org.mahay.sary/../x",
            "org.mahay.sary;rm",
            "com.example.sary",
            "file:///data/data/org.mahay.sary/x.apk",
            "/sdcard/x.apk",
            "org.mahay.unknown",
        ] {
            assert_eq!(app_for_package(bad), None, "{bad:?}");
        }
    }

    #[test]
    fn every_sub_app_is_a_suite_app_with_a_package() {
        for id in SUB_APPS {
            assert!(crate::APPS.contains(&id));
            assert_eq!(app_for_package(&package_of(id)), Some(id));
        }
        assert!(!SUB_APPS.contains(&"hub"));
        assert_eq!(SUB_APPS.len() + 1, crate::APPS.len());
    }

    #[test]
    fn installed_lines_drop_unknown_apps_and_bad_versions() {
        let got = parse_installed(&["sary=0.2.1", "hub=0.2.0", "evil=1.0.0", "feo=garbage", "tabilao", "seho=0.2.0"]);
        assert_eq!(got, installed(&[("sary", "0.2.1"), ("seho", "0.2.0")]));
    }

    #[test]
    fn pending_lists_older_installed_apps_in_suite_order() {
        let have = installed(&[("seho", "0.2.1"), ("sary", "0.2.1"), ("feo", "0.3.0"), ("tabilao", "0.1.0")]);
        let ids: Vec<_> = pending("0.3.0", &have, None).iter().map(|p| p.id).collect();
        // feo is already at 0.3.0, taratasy isn't installed: neither is listed.
        assert_eq!(ids, ["sary", "tabilao", "seho"]);
        // Never a downgrade, never an equal version.
        assert!(pending("0.2.0", &have, None).iter().all(|p| p.id == "tabilao"));
        assert!(pending("0.1.0", &have, None).is_empty());
        assert!(pending("garbage", &have, None).is_empty());
    }

    #[test]
    fn a_request_names_one_app_only() {
        let have = installed(&[("sary", "0.2.1"), ("seho", "0.2.1")]);
        let one = pending("0.3.0", &have, Some("seho"));
        assert_eq!(one.len(), 1);
        assert_eq!(one[0], Pending { id: "seho", installed: "0.2.1".into(), latest: "0.3.0".into() });
        assert!(pending("0.3.0", &have, Some("hub")).is_empty());
        assert!(pending("0.3.0", &have, Some("feo")).is_empty());
        assert!(pending("0.3.0", &have, Some("org.mahay.sary")).is_empty());
    }

    #[test]
    fn install_all_queue_follows_the_suite_order() {
        let have = installed(&[("taratasy", "0.2.1"), ("drafitra", "0.2.1")]);
        let queue = missing(&have);
        assert_eq!(queue.len(), 11);
        assert_eq!(queue[0], "sary");
        assert_eq!(queue[1], "soratra");
        assert!(!queue.contains(&"taratasy") && !queue.contains(&"drafitra"));
        assert!(missing(&installed(&SUB_APPS.map(|i| (i, "0.2.1")))).is_empty());
        assert_eq!(missing(&[]).len(), 13);
    }

    #[test]
    fn settings_default_to_unmetered_only() {
        let s = Settings::default();
        assert!(s.auto && !s.mobile_data);
        let now = 100_000;
        assert_eq!(decide(&s, Network::Unmetered, now, false), Decision::Run);
        assert_eq!(decide(&s, Network::Metered, now, false), Decision::Metered);
        assert_eq!(decide(&s, Network::None, now, false), Decision::NoNetwork);
        let mobile = Settings { mobile_data: true, ..s };
        assert_eq!(decide(&mobile, Network::Metered, now, false), Decision::Run);
        let off = Settings { auto: false, ..s };
        assert_eq!(decide(&off, Network::Unmetered, now, false), Decision::Disabled);
    }

    #[test]
    fn automatic_runs_are_spaced_but_a_request_is_not() {
        let now = 100_000;
        let ran = Settings { last_run: now - 60, ..Settings::default() };
        assert_eq!(decide(&ran, Network::Unmetered, now, false), Decision::TooSoon);
        let old = Settings { last_run: now - MIN_GAP, ..Settings::default() };
        assert_eq!(decide(&old, Network::Unmetered, now, false), Decision::Run);
        // The user asked: now, on any network, even with automatic updates off.
        let off = Settings { auto: false, last_run: now, ..Settings::default() };
        assert_eq!(decide(&off, Network::Metered, now, true), Decision::Run);
        assert_eq!(decide(&off, Network::None, now, true), Decision::NoNetwork);
    }

    #[test]
    fn settings_round_trip_and_survive_garbage() {
        let dir = std::env::temp_dir().join(format!("mahay-managed-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(Settings::load(&dir), Settings::default());
        let s = Settings { auto: false, mobile_data: true, last_run: 42 };
        s.save(&dir);
        assert_eq!(Settings::load(&dir), s);
        std::fs::write(dir.join("managed.json"), "not json").expect("write");
        assert_eq!(Settings::load(&dir), Settings::default());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_job_refuses_what_is_not_a_sub_app_before_anything_else() {
        let dir = std::env::temp_dir().join("mahay-managed-test-refuse");
        for bad in ["org.mahay.hub", "../../etc/passwd", "content://x/y", "org.mahay.sary/..", "", "evil"] {
            let out = headless(&dir, &["sary=0.0.1".into()], Some(bad), Network::Unmetered, true, 1_000_000);
            assert_eq!(out, ["refused"], "{bad:?}");
        }
    }

    #[test]
    fn the_job_honours_the_settings_without_touching_the_network() {
        let dir = std::env::temp_dir().join(format!("mahay-managed-test-skip-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let installed = ["sary=0.0.1".to_string()];
        // Mobile data, not allowed by default.
        assert_eq!(headless(&dir, &installed, None, Network::Metered, false, 1_000_000), ["skip:metered"]);
        assert_eq!(headless(&dir, &installed, None, Network::None, false, 1_000_000), ["skip:nonetwork"]);
        Settings { auto: false, ..Settings::default() }.save(&dir);
        assert_eq!(headless(&dir, &installed, None, Network::Unmetered, false, 1_000_000), ["skip:disabled"]);
        // A skipped run is not a run: the schedule is untouched.
        assert_eq!(Settings::load(&dir).last_run, 0);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn network_codes_from_java() {
        assert_eq!(Network::from_code(1), Network::Unmetered);
        assert_eq!(Network::from_code(2), Network::Metered);
        assert_eq!(Network::from_code(0), Network::None);
        assert_eq!(Network::from_code(99), Network::None);
    }

    #[test]
    fn ready_downloads_are_encoded_for_the_installer() {
        let ready = Outcome::Ready {
            id: "sary",
            update: Downloaded { version: "0.3.0".into(), path: PathBuf::from("/data/apps/sary/x.apk"), sha256: "ab".repeat(32) },
        };
        let failed = Outcome::Failed { id: "feo", error: Error::BadHash };
        let lines = encode_ready(&[ready, failed]);
        assert_eq!(lines, [format!("sary\t0.3.0\t{}\t/data/apps/sary/x.apk", "ab".repeat(32))]);
    }
}
