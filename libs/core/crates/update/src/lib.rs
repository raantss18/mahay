//! Automatic updates for the Mahay suite, on the principles Tontolo uses:
//!
//! - one version for the whole suite ([`SUITE_VERSION`], the repository's `VERSION` file);
//! - stable file names on every release ([`asset_name`]), so the files can be found even when
//!   GitHub's API refuses (60 requests per hour per address): the `releases/latest` redirect
//!   gives the version and the names follow from it;
//! - `SHA256SUMS.txt` is signed with Ed25519 (`SHA256SUMS.txt.sig`); the public key is built in
//!   ([`key::PUBLIC_KEY`]). No signature, a bad signature, a file missing from the list or a
//!   hash that doesn't match: nothing is installed;
//! - downloads only over https from GitHub's own hosts;
//! - never a downgrade, and only one downloaded update kept;
//! - the release notes of every version since the installed one, from `CHANGELOG.md`.
//!
//! Installing is the platform's job (Android's package installer, replacing an AppImage); this
//! crate finds, downloads and verifies. [`Checker`] runs it on a worker thread.

mod checker;
pub mod key;
pub mod managed;

pub use checker::{Checker, State};

use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

use sha2::{Digest, Sha256};

/// The suite's version, from the repository's `VERSION` file at build time.
pub const SUITE_VERSION: &str = env!("MAHAY_VERSION");
/// Where releases are published.
pub const REPO: &str = "raantss18/mahay";
/// The checksum list every release carries, and its Ed25519 signature.
pub const SUMS: &str = "SHA256SUMS.txt";
pub const SUMS_SIG: &str = "SHA256SUMS.txt.sig";

/// Largest update file accepted (the biggest APK or AppImage is well under this).
const MAX_DOWNLOAD: u64 = 400 * 1024 * 1024;
/// Largest checksum list, notes or API answer read into memory.
const MAX_TEXT: u64 = 4 * 1024 * 1024;

/// Why an update couldn't go ahead. [`Error::key`] names it for the apps' translations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    /// No network, GitHub down, or no release at all.
    Unreachable,
    /// The release lacks the file for this app and platform, or the checksum list.
    Missing,
    /// The checksum list isn't signed by Mahay's key.
    BadSignature,
    /// The file's SHA-256 doesn't match the signed list.
    BadHash,
    /// A download link pointing somewhere other than GitHub over https.
    Origin,
    /// The offered version isn't newer than the installed one.
    NotNewer,
    /// Writing the file failed (disk full, permissions).
    Io(String),
}

impl Error {
    pub fn key(&self) -> &'static str {
        match self {
            Error::Unreachable => "upd_unreachable",
            Error::Missing => "upd_missing",
            Error::BadSignature => "upd_bad_signature",
            Error::BadHash => "upd_bad_hash",
            Error::Origin => "upd_origin",
            Error::NotNewer => "upd_downgrade",
            Error::Io(_) => "upd_io",
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Unreachable => f.write_str("the update server can't be reached"),
            Error::Missing => f.write_str("this release has no file for this device"),
            Error::BadSignature => f.write_str("the release isn't signed by Mahay"),
            Error::BadHash => f.write_str("the downloaded file is damaged or was changed"),
            Error::Origin => f.write_str("a download link points outside GitHub"),
            Error::NotNewer => f.write_str("the offered version isn't newer"),
            Error::Io(e) => write!(f, "couldn't write the update: {e}"),
        }
    }
}

impl std::error::Error for Error {}

/// What a build runs on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    /// Android, by ABI (`arm64-v8a`, `x86_64`).
    Android(&'static str),
    /// Linux AppImage, by architecture (`x86_64`, `aarch64`).
    Linux(&'static str),
}

impl Target {
    /// This build's target, if updates exist for it.
    pub fn current() -> Option<Target> {
        if cfg!(target_os = "android") {
            if cfg!(target_arch = "aarch64") {
                Some(Target::Android("arm64-v8a"))
            } else if cfg!(target_arch = "x86_64") {
                Some(Target::Android("x86_64"))
            } else {
                None
            }
        } else if cfg!(target_os = "linux") {
            if cfg!(target_arch = "x86_64") {
                Some(Target::Linux("x86_64"))
            } else if cfg!(target_arch = "aarch64") {
                Some(Target::Linux("aarch64"))
            } else {
                None
            }
        } else {
            None
        }
    }
}

/// The release file of `app` at `version` for `target`. These names never change (old builds
/// rely on them): `tabilao-0.2.0-android-arm64-v8a.apk`, `tabilao-0.2.0-linux-x86_64.AppImage`.
pub fn asset_name(app: &str, version: &str, target: Target) -> String {
    match target {
        Target::Android(abi) => format!("{app}-{version}-android-{abi}.apk"),
        Target::Linux(arch) => format!("{app}-{version}-linux-{arch}.AppImage"),
    }
}

/// A `major.minor.patch` version (a `-suffix` is ignored for ordering).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version(pub u32, pub u32, pub u32);

impl Version {
    pub fn parse(text: &str) -> Option<Version> {
        let core = text.trim().trim_start_matches('v').split(['-', '+', ' ']).next()?;
        let mut parts = core.split('.').map(|p| p.parse::<u32>().ok());
        let v = Version(parts.next()??, parts.next().flatten().unwrap_or(0), parts.next().flatten().unwrap_or(0));
        Some(v)
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.0, self.1, self.2)
    }
}

/// Whether `candidate` is strictly newer than `installed`. Unreadable versions are never newer.
pub fn is_newer(candidate: &str, installed: &str) -> bool {
    match (Version::parse(candidate), Version::parse(installed)) {
        (Some(c), Some(i)) => c > i,
        _ => false,
    }
}

/// One file of a release.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Asset {
    pub name: String,
    pub url: String,
}

/// The latest release: its version, page and files.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Release {
    pub version: String,
    pub page: String,
    pub assets: Vec<Asset>,
}

impl Release {
    pub fn asset(&self, name: &str) -> Option<&Asset> {
        self.assets.iter().find(|a| a.name == name)
    }

    /// The release described from its version alone: the stable names, at GitHub's download
    /// URLs. Used when the API refuses.
    pub fn from_version(version: &str, apps: &[&str]) -> Release {
        let dl = |n: &str| format!("https://github.com/{REPO}/releases/download/v{version}/{n}");
        let mut names: Vec<String> = Vec::new();
        for app in apps {
            for t in [Target::Android("arm64-v8a"), Target::Android("x86_64"), Target::Linux("x86_64"), Target::Linux("aarch64")] {
                names.push(asset_name(app, version, t));
            }
        }
        names.push(SUMS.into());
        names.push(SUMS_SIG.into());
        Release {
            version: version.to_string(),
            page: format!("https://github.com/{REPO}/releases/tag/v{version}"),
            assets: names.iter().map(|n| Asset { name: n.clone(), url: dl(n) }).collect(),
        }
    }
}

/// Every app the release carries, for [`Release::from_version`].
pub const APPS: [&str; 14] =
    ["hub", "sary", "taratasy", "soratra", "tabilao", "seho", "tsilo", "endrika", "pejy", "hazavana", "feo", "hetsika", "tselatra", "drafitra"];

fn agent() -> ureq::Agent {
    ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(60)))
        .user_agent(concat!("mahay-update/", env!("MAHAY_VERSION")))
        .build()
        .into()
}

/// The latest published release: GitHub's API, or else the `releases/latest` redirect.
pub fn latest_release() -> Result<Release, Error> {
    let agent = agent();
    let api = format!("https://api.github.com/repos/{REPO}/releases/latest");
    match agent.get(&api).header("Accept", "application/vnd.github+json").call() {
        Ok(mut resp) => {
            let text = resp.body_mut().with_config().limit(MAX_TEXT).read_to_string().map_err(|_| Error::Unreachable)?;
            if let Some(r) = parse_api_release(&text) {
                return Ok(r);
            }
        }
        Err(e) => log::info!("GitHub API: {e}; trying the releases/latest redirect"),
    }
    // Rate-limited or blocked: the redirect has no limit, and the file names are stable.
    let no_redirect: ureq::Agent = ureq::Agent::config_builder().max_redirects(0).timeout_global(Some(Duration::from_secs(30))).build().into();
    let resp = no_redirect.get(&format!("https://github.com/{REPO}/releases/latest")).call().map_err(|_| Error::Unreachable)?;
    let location = resp.headers().get("location").and_then(|v| v.to_str().ok()).unwrap_or_default().to_string();
    let version = tag_from_location(&location).ok_or(Error::Unreachable)?;
    Ok(Release::from_version(&version, &APPS))
}

/// A release from the API's JSON (`tag_name`, `html_url`, `assets[].name/browser_download_url`).
pub fn parse_api_release(json: &str) -> Option<Release> {
    let v: serde_json::Value = serde_json::from_str(json).ok()?;
    let version = v.get("tag_name")?.as_str()?.trim_start_matches('v').to_string();
    Version::parse(&version)?;
    let page = v.get("html_url").and_then(|p| p.as_str()).unwrap_or_default().to_string();
    let assets = v
        .get("assets")?
        .as_array()?
        .iter()
        .filter_map(|a| Some(Asset { name: a.get("name")?.as_str()?.to_string(), url: a.get("browser_download_url")?.as_str()?.to_string() }))
        .collect();
    Some(Release { version, page, assets })
}

/// `1.2.3` from a `.../releases/tag/v1.2.3` redirect target.
pub fn tag_from_location(location: &str) -> Option<String> {
    let tag = location.trim_end_matches('/').rsplit("/releases/tag/").next().filter(|t| *t != location)?;
    let version = tag.trim_start_matches('v');
    Version::parse(version)?;
    version.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '+')).then(|| version.to_string())
}

/// Whether a download URL is https on GitHub's hosts.
pub fn trusted_url(url: &str) -> bool {
    let Some(rest) = url.strip_prefix("https://") else { return false };
    let host = rest.split(['/', '?', '#']).next().unwrap_or_default();
    // No `user@host` or explicit port tricks.
    if host.contains(['@', ':']) {
        return false;
    }
    host == "github.com" || host.ends_with(".githubusercontent.com")
}

/// `name → sha256` from a `sha256sum` listing.
pub fn parse_sums(text: &str) -> Vec<(String, String)> {
    text.lines()
        .filter_map(|line| {
            let line = line.trim();
            let (hash, name) = line.split_once(char::is_whitespace)?;
            let name = name.trim().trim_start_matches('*');
            (hash.len() == 64 && hash.chars().all(|c| c.is_ascii_hexdigit()) && !name.is_empty())
                .then(|| (name.to_string(), hash.to_ascii_lowercase()))
        })
        .collect()
}

/// Whether `signature` (64 bytes, Ed25519) signs exactly `sums` with Mahay's key, or with
/// `public_key` when given (tests).
pub fn verify_sums(sums: &[u8], signature: &[u8], public_key: Option<&[u8; 32]>) -> bool {
    let key_bytes = public_key.copied().unwrap_or(key::PUBLIC_KEY);
    let Ok(key) = ed25519_dalek::VerifyingKey::from_bytes(&key_bytes) else { return false };
    let Ok(sig) = <[u8; 64]>::try_from(signature) else { return false };
    key.verify_strict(sums, &ed25519_dalek::Signature::from_bytes(&sig)).is_ok()
}

/// A verified update, ready to install.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Downloaded {
    pub version: String,
    pub path: PathBuf,
    pub sha256: String,
}

/// Download, check and keep `app`'s file of `release` in `dir` (one update kept: older files
/// there are removed). `installed` is the running version: an older or equal release is
/// refused. `progress` gets the fraction done.
pub fn download(
    release: &Release,
    app: &str,
    installed: &str,
    target: Target,
    dir: &Path,
    progress: &dyn Fn(f32),
) -> Result<Downloaded, Error> {
    if !is_newer(&release.version, installed) {
        return Err(Error::NotNewer);
    }
    let name = asset_name(app, &release.version, target);
    let asset = release.asset(&name).ok_or(Error::Missing)?;
    let sums = release.asset(SUMS).ok_or(Error::Missing)?;
    let sig = release.asset(SUMS_SIG).ok_or(Error::BadSignature)?;
    for a in [asset, sums, sig] {
        if !trusted_url(&a.url) {
            return Err(Error::Origin);
        }
    }
    let agent = agent();
    let sums_bytes = fetch(&agent, &sums.url)?;
    let sig_bytes = fetch(&agent, &sig.url)?;
    if !verify_sums(&sums_bytes, &sig_bytes, None) {
        return Err(Error::BadSignature);
    }
    let sums_text = String::from_utf8_lossy(&sums_bytes);
    let expected = parse_sums(&sums_text).into_iter().find(|(n, _)| *n == name).map(|(_, h)| h).ok_or(Error::Missing)?;

    std::fs::create_dir_all(dir).map_err(|e| Error::Io(e.to_string()))?;
    let dest = dir.join(&name);
    if file_sha256(&dest).as_deref() == Some(expected.as_str()) {
        return Ok(Downloaded { version: release.version.clone(), path: dest, sha256: expected });
    }
    let part = dir.join(format!("{name}.part"));
    let mut resp = agent.get(&asset.url).call().map_err(|_| Error::Unreachable)?;
    let total = resp.body().content_length().unwrap_or(0).min(MAX_DOWNLOAD);
    let mut reader = resp.body_mut().with_config().limit(MAX_DOWNLOAD).reader();
    let mut out = std::fs::File::create(&part).map_err(|e| Error::Io(e.to_string()))?;
    let mut hasher = Sha256::new();
    let mut buf = vec![0u8; 64 * 1024];
    let mut done: u64 = 0;
    loop {
        let n = reader.read(&mut buf).map_err(|_| Error::Unreachable)?;
        if n == 0 {
            break;
        }
        let chunk = buf.get(..n).unwrap_or_default();
        hasher.update(chunk);
        out.write_all(chunk).map_err(|e| Error::Io(e.to_string()))?;
        done = done.saturating_add(n as u64);
        if total > 0 {
            progress((done as f32 / total as f32).min(1.0));
        }
    }
    out.flush().map_err(|e| Error::Io(e.to_string()))?;
    drop(out);
    let got = hex(&hasher.finalize());
    if got != expected {
        let _ = std::fs::remove_file(&part);
        return Err(Error::BadHash);
    }
    std::fs::rename(&part, &dest).map_err(|e| Error::Io(e.to_string()))?;
    // One update kept.
    if let Ok(entries) = std::fs::read_dir(dir) {
        for e in entries.flatten() {
            let p = e.path();
            if p != dest && p.is_file() {
                let _ = std::fs::remove_file(p);
            }
        }
    }
    Ok(Downloaded { version: release.version.clone(), path: dest, sha256: expected })
}

/// Re-check a downloaded file just before installing it (it sat on disk meanwhile).
pub fn still_valid(update: &Downloaded) -> bool {
    file_sha256(&update.path).as_deref() == Some(update.sha256.as_str())
}

/// Linux AppImage: the running AppImage (`$APPIMAGE`), when the app runs from one.
pub fn running_appimage() -> Option<PathBuf> {
    if !cfg!(all(target_os = "linux", not(target_os = "android"))) {
        return None;
    }
    std::env::var_os("APPIMAGE").map(PathBuf::from).filter(|p| p.is_file())
}

/// Linux AppImage: replace the running AppImage with the verified update (copy next to it,
/// check again, make executable, rename over it: atomic on the same file system), start the
/// new one and exit. Returns only on failure.
pub fn install_appimage(update: &Downloaded) -> Result<(), String> {
    let target = running_appimage().ok_or("Mahay isn't running from an AppImage: update it with your package manager.")?;
    if !still_valid(update) {
        return Err(Error::BadHash.to_string());
    }
    let tmp = target.with_extension("new");
    std::fs::copy(&update.path, &tmp).map_err(|e| e.to_string())?;
    if file_sha256(&tmp).as_deref() != Some(update.sha256.as_str()) {
        let _ = std::fs::remove_file(&tmp);
        return Err(Error::BadHash.to_string());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&tmp, std::fs::Permissions::from_mode(0o755)).map_err(|e| e.to_string())?;
    }
    std::fs::rename(&tmp, &target).map_err(|e| e.to_string())?;
    std::process::Command::new(&target).spawn().map_err(|e| e.to_string())?;
    std::process::exit(0);
}

/// The notes of every version newer than `installed`, from the release's `CHANGELOG.md`.
pub fn notes_since(release: &Release, installed: &str) -> Option<String> {
    let url = format!("https://raw.githubusercontent.com/{REPO}/v{}/CHANGELOG.md", release.version);
    let text = String::from_utf8(fetch(&agent(), &url).ok()?).ok()?;
    let notes = changelog_since(&text, installed);
    (!notes.trim().is_empty()).then_some(notes)
}

/// The sections of `changelog` (newest first, `## [x.y.z]` headings) newer than `installed`.
pub fn changelog_since(changelog: &str, installed: &str) -> String {
    let mut out = String::new();
    let mut keep = false;
    for line in changelog.lines() {
        if let Some(rest) = line.strip_prefix("## ") {
            let version = rest.trim_start_matches('[').split([']', ' ']).next().unwrap_or_default();
            if Version::parse(version).is_some() {
                if !is_newer(version, installed) {
                    break;
                }
                keep = true;
                out.push_str(&format!("## {version}\n"));
                continue;
            }
            keep = false;
            continue;
        }
        if keep {
            out.push_str(line);
            out.push('\n');
        }
    }
    out
}

fn fetch(agent: &ureq::Agent, url: &str) -> Result<Vec<u8>, Error> {
    let mut resp = agent.get(url).call().map_err(|_| Error::Unreachable)?;
    resp.body_mut().with_config().limit(MAX_TEXT).read_to_vec().map_err(|_| Error::Unreachable)
}

fn file_sha256(path: &Path) -> Option<String> {
    let mut f = std::fs::File::open(path).ok()?;
    let mut hasher = Sha256::new();
    std::io::copy(&mut f, &mut hasher).ok()?;
    Some(hex(&hasher.finalize()))
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn versions_compare_numerically() {
        assert!(is_newer("0.10.0", "0.9.9"));
        assert!(is_newer("v1.0.0", "0.99.0"));
        assert!(!is_newer("0.2.0", "0.2.0"));
        assert!(!is_newer("0.1.9", "0.2.0"));
        assert!(!is_newer("garbage", "0.1.0"));
        assert_eq!(Version::parse("0.2.0-rc.1"), Some(Version(0, 2, 0)));
    }

    #[test]
    fn stable_names() {
        assert_eq!(asset_name("tabilao", "0.2.0", Target::Android("arm64-v8a")), "tabilao-0.2.0-android-arm64-v8a.apk");
        assert_eq!(asset_name("sary", "1.0.0", Target::Linux("x86_64")), "sary-1.0.0-linux-x86_64.AppImage");
    }

    #[test]
    fn only_github_over_https() {
        assert!(trusted_url("https://github.com/raantss18/mahay/releases/download/v0.2.0/a.apk"));
        assert!(trusted_url("https://objects.githubusercontent.com/x"));
        assert!(!trusted_url("http://github.com/x"));
        assert!(!trusted_url("https://github.com.evil.example/x"));
        assert!(!trusted_url("https://github.com@evil.example/x"));
        assert!(!trusted_url("https://evilgithubusercontent.com/x"));
    }

    #[test]
    fn redirect_gives_the_version() {
        assert_eq!(tag_from_location("https://github.com/raantss18/mahay/releases/tag/v0.2.0").as_deref(), Some("0.2.0"));
        assert_eq!(tag_from_location("https://github.com/raantss18/mahay/releases"), None);
        assert_eq!(tag_from_location("https://github.com/x/releases/tag/v1.0.0;rm -rf"), None);
    }

    #[test]
    fn fallback_release_lists_stable_names() {
        let r = Release::from_version("0.2.0", &APPS);
        assert!(r.asset("hub-0.2.0-android-arm64-v8a.apk").is_some());
        assert!(r.asset(SUMS_SIG).is_some());
        assert!(r.assets.iter().all(|a| trusted_url(&a.url)));
    }

    #[test]
    fn parses_sums_and_api() {
        let h = "a".repeat(64);
        let sums = parse_sums(&format!("{h}  tabilao-0.2.0-android-arm64-v8a.apk\n{h} *x.AppImage\nbad line\n"));
        assert_eq!(sums.len(), 2);
        assert_eq!(sums[1].0, "x.AppImage");
        let api = r#"{"tag_name":"v0.2.0","html_url":"https://github.com/raantss18/mahay/releases/tag/v0.2.0",
            "assets":[{"name":"SHA256SUMS.txt","browser_download_url":"https://github.com/raantss18/mahay/releases/download/v0.2.0/SHA256SUMS.txt"}]}"#;
        let r = parse_api_release(api).expect("release");
        assert_eq!(r.version, "0.2.0");
        assert_eq!(r.assets.len(), 1);
    }

    #[test]
    fn signature_must_match_the_key_and_the_text() {
        use ed25519_dalek::{Signer, SigningKey};
        let signing = SigningKey::from_bytes(&[7u8; 32]);
        let public = signing.verifying_key().to_bytes();
        let sums = b"abc  file\n";
        let sig = signing.sign(sums).to_bytes();
        assert!(verify_sums(sums, &sig, Some(&public)));
        assert!(!verify_sums(b"abd  file\n", &sig, Some(&public)));
        assert!(!verify_sums(sums, &sig[..63], Some(&public)));
        // Mahay's own key didn't sign this.
        assert!(!verify_sums(sums, &sig, None));
    }

    #[test]
    fn changelog_keeps_only_newer_sections() {
        let log = "# Changelog\n\n## [Unreleased]\n- wip\n\n## [0.3.0] — x\n### Added\n- c\n\n## [0.2.0] — y\n- b\n\n## [0.1.0]\n- a\n";
        let out = changelog_since(log, "0.1.0");
        assert!(out.contains("## 0.3.0") && out.contains("- c") && out.contains("- b"));
        assert!(!out.contains("- a") && !out.contains("wip"));
    }

    #[test]
    fn suite_version_is_set() {
        assert!(Version::parse(SUITE_VERSION).is_some());
    }
}

#[cfg(test)]
mod online {
    /// Talks to github.com: `cargo test -p mahay-update -- --ignored`. Needs a published
    /// non-prerelease release (`releases/latest` skips prereleases).
    #[test]
    #[ignore = "network"]
    fn finds_the_published_release() {
        let r = super::latest_release().expect("a release");
        assert!(super::Version::parse(&r.version).is_some());
        assert!(!r.assets.is_empty());
    }
}
