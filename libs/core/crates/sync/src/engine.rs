//! The sync engine: scan the local folder, compare it with the last synced state (the journal)
//! and with the remote manifest, then download, upload, keep conflict copies and fill the bin.
//!
//! Every step is saved to the journal as soon as it is done, so a network cut at any point
//! leaves a state the next run finishes from, with no lost and no duplicated change.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::Error;
use crate::backend::Backend;
use crate::crypto::{CheckBlock, KDF_ITERATIONS, Key, decrypt_file, encrypt_file, new_id, sha256_hex};
use crate::manifest::{Entry, Manifest};

/// Blob name of the encrypted manifest.
pub const MANIFEST: &str = "manifest.mhy";
/// Blob name of the passphrase check block (a copy of `meta/check`).
pub const CHECK: &str = "check.json";
/// How long a deleted file stays in the sync bin.
pub const BIN_DAYS: i64 = 30;
const JOURNAL: &str = "journal.json";

/// First device: creates the check block (and stores it on the backend). Other devices: checks
/// the passphrase against it. Returns the key and the check block (for `sync_items` once the
/// account exists).
pub fn open_key(backend: &dyn Backend, passphrase: &str) -> Result<(Key, CheckBlock), Error> {
    open_key_with(backend, passphrase, KDF_ITERATIONS)
}

/// [`open_key`] with a chosen iteration count for a new block (tests use fewer).
pub fn open_key_with(backend: &dyn Backend, passphrase: &str, iterations: u32) -> Result<(Key, CheckBlock), Error> {
    if let Some(raw) = backend.get(CHECK)? {
        let block = CheckBlock::from_json(&String::from_utf8_lossy(&raw))?;
        return Ok((block.unlock(passphrase)?, block));
    }
    let (block, key) = CheckBlock::create(passphrase, iterations)?;
    backend.put(CHECK, block.to_json()?.as_bytes())?;
    Ok((key, block))
}

/// Where and as whom to sync.
#[derive(Debug, Clone)]
pub struct Config {
    /// The local folder that is kept in sync.
    pub root: PathBuf,
    /// Private state (the journal); outside `root` or skipped by the scan.
    pub state_dir: PathBuf,
    /// This device's name, shown in conflict copies ("Tablette", "Laptop").
    pub device: String,
}

/// What one sync did, for the hub's status.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Report {
    pub uploaded: Vec<String>,
    pub downloaded: Vec<String>,
    /// Files deleted here because another device deleted them (still in the sync bin).
    pub deleted_here: Vec<String>,
    /// Files deleted here and moved to the sync bin on the remote.
    pub binned: Vec<String>,
    /// Conflict copies written here: `name (conflit <device> <date>).ext`.
    pub conflicts: Vec<String>,
    /// Bin entries older than 30 days whose blobs were removed.
    pub purged: Vec<String>,
    /// Files the manifest lists but whose blob is missing on the backend (tried again next run).
    pub missing: Vec<String>,
}

/// The last synced state of one path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Base {
    entry: Entry,
    /// Local modification time (ns) when it was synced: same size and time = same content.
    #[serde(default)]
    local_ns: Option<i64>,
}

/// A blob uploaded but not yet in the manifest (reused if the next run finds the same content).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Pending {
    id: String,
    sha256: String,
}

/// The offline journal, saved in `state_dir/journal.json`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct Journal {
    v: u32,
    base: BTreeMap<String, Base>,
    pending: BTreeMap<String, Pending>,
    /// Blobs we replaced or purged, deleted once the manifest no longer points to them.
    #[serde(default)]
    unused: BTreeSet<String>,
    last_sync: Option<i64>,
}

/// A manifest change, applied only if the remote version is still the one we planned on.
struct Change {
    path: String,
    expect: u64,
    entry: Entry,
    local_ns: Option<i64>,
}

struct Local {
    ns: i64,
    sha256: String,
}

pub struct Engine {
    cfg: Config,
    backend: Arc<dyn Backend>,
    key: Key,
    journal: Journal,
}

impl Engine {
    pub fn new(cfg: Config, backend: Arc<dyn Backend>, key: Key) -> Result<Self, Error> {
        let path = cfg.state_dir.join(JOURNAL);
        let journal = match fs::read(&path) {
            Ok(raw) => serde_json::from_slice(&raw).map_err(|_| Error::Format("journal JSON"))?,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Journal { v: 1, ..Journal::default() },
            Err(e) => return Err(Error::io(&path, e)),
        };
        Ok(Self { cfg, backend, key, journal })
    }

    /// Unix seconds of the last complete sync.
    pub fn last_sync(&self) -> Option<i64> {
        self.journal.last_sync
    }

    /// One full sync now.
    pub fn sync(&mut self) -> Result<Report, Error> {
        self.sync_at(now())
    }

    /// One full sync, `now` in Unix seconds (dates of deletes, bin expiry, conflict names).
    pub fn sync_at(&mut self, now: i64) -> Result<Report, Error> {
        let mut report = Report::default();
        let remote = self.load_manifest()?;

        // A manifest write whose answer was lost: our pending blob is already published.
        let adopted: Vec<String> = self
            .journal
            .pending
            .iter()
            .filter(|(p, pend)| remote.files.get(*p).and_then(|e| e.id.as_ref()) == Some(&pend.id))
            .map(|(p, _)| p.clone())
            .collect();
        for p in adopted {
            if let Some(e) = remote.files.get(&p) {
                // no local time: the file is hashed again in case it changed since
                self.journal.base.insert(p.clone(), Base { entry: e.clone(), local_ns: None });
            }
            self.journal.pending.remove(&p);
        }
        self.clean_unused(&remote);

        let local = self.scan()?;
        let mut paths: BTreeSet<String> = local.keys().cloned().collect();
        paths.extend(self.journal.base.keys().cloned());
        paths.extend(remote.files.keys().filter(|p| valid_path(p)).cloned());

        let mut uploads: Vec<(String, u64, u64)> = Vec::new(); // path, expected remote version, new version
        let mut changes: Vec<Change> = Vec::new();

        for path in paths {
            let l = local.get(&path).map(|l| l.sha256.clone());
            let b = self.journal.base.get(&path).cloned();
            let r = remote.files.get(&path).filter(|_| valid_path(&path)).cloned();
            let live_sha = |e: &Entry| e.is_live().then(|| e.sha256.clone());
            let b_sha = b.as_ref().and_then(|b| live_sha(&b.entry));
            let r_sha = r.as_ref().and_then(live_sha);
            let bv = b.as_ref().map_or(0, |b| b.entry.version);
            let rv = r.as_ref().map_or(0, |e| e.version);

            let local_changed = l != b_sha;
            let remote_changed = rv > bv;
            let next = rv.max(bv) + 1;

            match (local_changed, remote_changed) {
                (false, false) => {
                    // a file hashed again (recent or touched) whose content did not change
                    if let (Some(lc), Some(base)) = (local.get(&path), self.journal.base.get_mut(&path))
                        && base.local_ns != Some(lc.ns)
                    {
                        base.local_ns = trusted(lc.ns);
                    }
                    // the remote lost a change we published (two devices wrote the manifest at once,
                    // or the folder was emptied): publish it again, with a fresh blob
                    if rv < bv
                        && let Some(b) = b
                    {
                        if l.is_some() {
                            self.journal.unused.extend(b.entry.id.clone());
                            uploads.push((path, rv, bv + 1));
                        } else {
                            changes.push(Change { path, expect: rv, entry: b.entry, local_ns: None });
                        }
                    }
                }
                (true, false) => match (&l, &b) {
                    (Some(_), _) => uploads.push((path, rv, next)),
                    (None, Some(b)) if b.entry.is_live() => {
                        let entry = Entry { version: next, device: self.cfg.device.clone(), deleted: Some(now), ..b.entry.clone() };
                        changes.push(Change { path: path.clone(), expect: rv, entry, local_ns: None });
                        report.binned.push(path);
                    }
                    _ => {}
                },
                (false, true) | (true, true) => {
                    let Some(r) = r else { continue };
                    if l == r_sha {
                        // same content on both sides (or deleted on both)
                        let local_ns = local.get(&path).and_then(|l| trusted(l.ns));
                        self.set_base(&path, r, local_ns)?;
                    } else if !local_changed || l.is_none() {
                        // only the remote changed, or a local delete loses to a remote edit
                        if r.is_live() {
                            if !self.download(&path, &path, &r, &mut report)? {
                                continue;
                            }
                            let local_ns = self.local_ns(&path);
                            self.set_base(&path, r, local_ns)?;
                            report.downloaded.push(path);
                        } else {
                            self.delete_local(&path, b_sha.as_deref())?;
                            self.set_base(&path, r, None)?;
                            report.deleted_here.push(path);
                        }
                    } else if !r.is_live() {
                        // a local edit wins over a remote delete
                        self.set_base(&path, r, None)?;
                        uploads.push((path, rv, rv + 1));
                    } else {
                        // both edited: keep the remote copy beside ours, then publish ours
                        let copy = self.conflict_name(&path, &r);
                        if !self.download(&path, &copy, &r, &mut report)? {
                            continue;
                        }
                        self.set_base(&path, r, None)?;
                        uploads.push((copy.clone(), 0, 1));
                        report.conflicts.push(copy);
                        uploads.push((path, rv, rv + 1));
                    }
                }
            }
        }

        for (path, expect, version) in uploads {
            if let Some(c) = self.upload(&path, expect, version)? {
                changes.push(c);
                report.uploaded.push(path);
            }
        }

        self.commit(changes, now, &mut report)?;
        self.journal.last_sync = Some(now);
        self.save_journal()?;
        Ok(report)
    }

    /// Files in the sync bin (deleted less than 30 days ago), with the date they were deleted.
    pub fn bin(&self) -> Result<Vec<(String, i64)>, Error> {
        let m = self.load_manifest()?;
        Ok(m.files.into_iter().filter(|(_, e)| e.id.is_some()).filter_map(|(p, e)| Some((p, e.deleted?))).collect())
    }

    /// Takes a file out of the sync bin; the next sync brings it back on every device.
    pub fn restore(&mut self, path: &str) -> Result<bool, Error> {
        let mut m = self.load_manifest()?;
        let Some(e) = m.files.get_mut(path) else { return Ok(false) };
        if e.deleted.is_none() || e.id.is_none() {
            return Ok(false);
        }
        e.deleted = None;
        e.version += 1;
        e.device = self.cfg.device.clone();
        self.backend.put(MANIFEST, &m.seal(&self.key)?)?;
        Ok(true)
    }

    fn load_manifest(&self) -> Result<Manifest, Error> {
        match self.backend.get(MANIFEST)? {
            Some(blob) => Manifest::open(&self.key, &blob),
            None => Ok(Manifest::new()),
        }
    }

    fn set_base(&mut self, path: &str, entry: Entry, local_ns: Option<i64>) -> Result<(), Error> {
        self.journal.base.insert(path.to_owned(), Base { entry, local_ns });
        self.save_journal()
    }

    fn save_journal(&self) -> Result<(), Error> {
        let path = self.cfg.state_dir.join(JOURNAL);
        fs::create_dir_all(&self.cfg.state_dir).map_err(|e| Error::io(&self.cfg.state_dir, e))?;
        let json = serde_json::to_vec_pretty(&self.journal).map_err(|_| Error::Format("journal JSON"))?;
        mahay_atomic::atomic_write(&path, &json).map_err(|e| Error::io(&path, e))
    }

    fn abs(&self, rel: &str) -> PathBuf {
        rel.split('/').fold(self.cfg.root.clone(), |p, part| p.join(part))
    }

    fn local_ns(&self, rel: &str) -> Option<i64> {
        fs::metadata(self.abs(rel)).ok().and_then(|m| m.modified().ok()).map(unix_ns).and_then(trusted)
    }

    /// Every regular file under `root`, by relative path. Hidden files (and so Mahay's own
    /// temporary files) and the state folder are skipped.
    fn scan(&self) -> Result<BTreeMap<String, Local>, Error> {
        let mut out = BTreeMap::new();
        let mut dirs = vec![(self.cfg.root.clone(), String::new())];
        while let Some((dir, prefix)) = dirs.pop() {
            let rd = match fs::read_dir(&dir) {
                Ok(rd) => rd,
                Err(e) if e.kind() == std::io::ErrorKind::NotFound && prefix.is_empty() => continue,
                Err(e) => return Err(Error::io(&dir, e)),
            };
            for item in rd {
                let item = item.map_err(|e| Error::io(&dir, e))?;
                let Some(name) = item.file_name().to_str().map(str::to_owned) else {
                    log::warn!("sync: skipping a non-UTF-8 name in {}", dir.display());
                    continue;
                };
                if name.starts_with('.') || name.ends_with(".mahay_tmp") {
                    continue;
                }
                let full = item.path();
                if full == self.cfg.state_dir {
                    continue;
                }
                let rel = if prefix.is_empty() { name } else { format!("{prefix}/{name}") };
                let ft = item.file_type().map_err(|e| Error::io(&full, e))?;
                if ft.is_dir() {
                    dirs.push((full, rel));
                } else if ft.is_file()
                    && valid_path(&rel)
                    && let Some(l) = self.scan_one(&rel)?
                {
                    out.insert(rel, l);
                }
            }
        }
        Ok(out)
    }

    fn scan_one(&self, rel: &str) -> Result<Option<Local>, Error> {
        let full = self.abs(rel);
        let meta = match fs::metadata(&full) {
            Ok(m) => m,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(Error::io(&full, e)),
        };
        let ns = meta.modified().map(unix_ns).unwrap_or(0);
        let size = meta.len();
        if let Some(b) = self.journal.base.get(rel)
            && b.entry.is_live()
            && b.entry.size == size
            && b.local_ns == Some(ns)
        {
            return Ok(Some(Local { ns, sha256: b.entry.sha256.clone() }));
        }
        let data = fs::read(&full).map_err(|e| Error::io(&full, e))?;
        Ok(Some(Local { ns, sha256: sha256_hex(&data) }))
    }

    /// Fetches `entry`'s blob, checks it and writes it to `dest` (atomically). `false` when the
    /// blob is missing: that file is skipped, the others go on.
    fn download(&self, path: &str, dest: &str, entry: &Entry, report: &mut Report) -> Result<bool, Error> {
        let id = entry.id.as_deref().ok_or(Error::Format("entry without blob"))?;
        let Some(blob) = self.backend.get(&blob_name(id))? else {
            log::warn!("sync: the blob of {path} is missing");
            report.missing.push(path.to_owned());
            return Ok(false);
        };
        let plain = decrypt_file(&self.key, id, &blob)?;
        if sha256_hex(&plain) != entry.sha256 {
            return Err(Error::Integrity(format!("{path}: content does not match the manifest")));
        }
        let full = self.abs(dest);
        if let Some(parent) = full.parent() {
            fs::create_dir_all(parent).map_err(|e| Error::io(parent, e))?;
        }
        mahay_atomic::atomic_write(&full, &plain).map_err(|e| Error::io(&full, e))?;
        let mtime = UNIX_EPOCH + Duration::from_secs(entry.mtime.max(0).unsigned_abs());
        if let Err(e) = fs::File::options().write(true).open(&full).and_then(|f| f.set_modified(mtime)) {
            log::debug!("sync: could not set the date of {}: {e}", full.display());
        }
        Ok(true)
    }

    /// Deletes a local file another device deleted, unless it changed here since.
    fn delete_local(&self, path: &str, base_sha: Option<&str>) -> Result<(), Error> {
        let full = self.abs(path);
        match fs::read(&full) {
            Ok(data) if Some(sha256_hex(&data).as_str()) == base_sha => fs::remove_file(&full).map_err(|e| Error::io(&full, e)),
            Ok(_) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(Error::io(&full, e)),
        }
    }

    /// Encrypts and uploads the local file (or reuses the blob a cut run already sent).
    fn upload(&mut self, path: &str, expect: u64, version: u64) -> Result<Option<Change>, Error> {
        let full = self.abs(path);
        let (data, meta) = match fs::read(&full).and_then(|d| Ok((d, fs::metadata(&full)?))) {
            Ok(x) => x,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(Error::io(&full, e)),
        };
        let sha256 = sha256_hex(&data);
        let id = match self.journal.pending.get(path) {
            Some(p) if p.sha256 == sha256 => p.id.clone(),
            stale => {
                let stale = stale.map(|p| p.id.clone());
                let id = new_id()?;
                self.backend.put(&blob_name(&id), &encrypt_file(&self.key, &id, &data)?)?;
                self.journal.pending.insert(path.to_owned(), Pending { id: id.clone(), sha256: sha256.clone() });
                self.journal.unused.extend(stale);
                self.save_journal()?;
                id
            }
        };
        let modified = meta.modified().ok();
        let entry = Entry {
            id: Some(id),
            size: data.len() as u64,
            mtime: modified.map_or(0, |t| unix_ns(t) / 1_000_000_000),
            sha256,
            version,
            device: self.cfg.device.clone(),
            deleted: None,
        };
        Ok(Some(Change { path: path.to_owned(), expect, entry, local_ns: modified.map(unix_ns).and_then(trusted) }))
    }

    /// Writes the changes into a fresh copy of the manifest and empties the expired bin.
    fn commit(&mut self, changes: Vec<Change>, now: i64, report: &mut Report) -> Result<(), Error> {
        let mut m = self.load_manifest()?;
        let mut applied = Vec::new();
        let mut unused = Vec::new();
        for c in changes {
            let current = m.files.get(&c.path);
            if current.map_or(0, |e| e.version) != c.expect {
                // another device changed it meanwhile: the next run sees a conflict
                continue;
            }
            if let Some(old) = current.and_then(|e| e.id.clone())
                && Some(&old) != c.entry.id.as_ref()
                && c.entry.deleted.is_none()
            {
                unused.push(old);
            }
            m.files.insert(c.path.clone(), c.entry.clone());
            applied.push(c);
        }
        let expiry = now - BIN_DAYS * 86_400;
        for (path, e) in m.files.iter_mut() {
            if e.deleted.is_some_and(|d| d <= expiry)
                && let Some(id) = e.id.take()
            {
                unused.push(id);
                report.purged.push(path.clone());
            }
        }
        if applied.is_empty() && unused.is_empty() {
            return Ok(());
        }
        // remembered first: if the answer to the write below is lost, the next run cleans up
        self.journal.unused.extend(unused);
        self.save_journal()?;
        self.backend.put(MANIFEST, &m.seal(&self.key)?)?;
        for c in applied {
            if self.journal.pending.get(&c.path).map(|p| &p.id) == c.entry.id.as_ref() {
                self.journal.pending.remove(&c.path);
            }
            self.journal.base.insert(c.path, Base { entry: c.entry, local_ns: c.local_ns });
        }
        for p in &report.purged {
            if let Some(b) = self.journal.base.get_mut(p) {
                b.entry.id = None;
            }
        }
        self.clean_unused(&m);
        self.save_journal()
    }

    /// Deletes the unused blobs that `m` does not point to (best effort: the rest waits).
    fn clean_unused(&mut self, m: &Manifest) {
        let used: BTreeSet<&String> = m.files.values().filter_map(|e| e.id.as_ref()).chain(self.journal.pending.values().map(|p| &p.id)).collect();
        let mut done = Vec::new();
        for id in self.journal.unused.iter().filter(|id| !used.contains(id)) {
            match self.backend.delete(&blob_name(id)) {
                Ok(()) => done.push(id.clone()),
                Err(e) => {
                    log::info!("sync: unused blobs left for the next run: {e}");
                    break;
                }
            }
        }
        for id in done {
            self.journal.unused.remove(&id);
        }
    }

    /// `dir/name (conflit <device> <date>).ext`, made unique.
    fn conflict_name(&self, path: &str, remote: &Entry) -> String {
        let (dir, file) = path.rsplit_once('/').map_or(("", path), |(d, f)| (d, f));
        let (stem, ext) = match file.rfind('.') {
            Some(i) if i > 0 => (&file[..i], &file[i..]),
            _ => (file, ""),
        };
        let device: String = remote.device.chars().map(|c| if c.is_control() || "/\\:*?\"<>|".contains(c) { '-' } else { c }).collect();
        let device = if device.trim().is_empty() { "appareil".to_owned() } else { device.trim().to_owned() };
        let date = format_date(remote.mtime);
        let mut n = 1;
        loop {
            let suffix = if n == 1 { String::new() } else { format!(" {n}") };
            let name = format!("{stem} (conflit {device} {date}{suffix}){ext}");
            let rel = if dir.is_empty() { name } else { format!("{dir}/{name}") };
            if !self.abs(&rel).exists() {
                return rel;
            }
            n += 1;
        }
    }
}

fn blob_name(id: &str) -> String {
    format!("{id}.mhy")
}

/// A relative path from the manifest that is safe to write under the root.
fn valid_path(p: &str) -> bool {
    !p.is_empty()
        && p.len() <= 4096
        && !p.contains(['\\', '\0'])
        && p.split('/').all(|c| !c.is_empty() && c != "." && c != ".." && !c.starts_with('.') && !c.contains(':'))
}

/// A local modification time is only trusted to mean "unchanged" once it is two seconds old:
/// file systems keep coarse times, and a second write in the same tick would look the same.
fn trusted(ns: i64) -> Option<i64> {
    (ns < unix_ns(SystemTime::now()).saturating_sub(2_000_000_000)).then_some(ns)
}

fn now() -> i64 {
    unix_ns(SystemTime::now()) / 1_000_000_000
}

fn unix_ns(t: SystemTime) -> i64 {
    match t.duration_since(UNIX_EPOCH) {
        Ok(d) => i64::try_from(d.as_nanos()).unwrap_or(i64::MAX),
        Err(e) => -i64::try_from(e.duration().as_nanos()).unwrap_or(i64::MAX),
    }
}

/// `YYYY-MM-DD HH-MM` (UTC) from Unix seconds; no `:` so the name is valid everywhere.
fn format_date(secs: i64) -> String {
    let days = secs.div_euclid(86_400);
    let rem = secs.rem_euclid(86_400);
    // civil_from_days, Howard Hinnant
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    format!("{y:04}-{m:02}-{d:02} {:02}-{:02}", rem / 3600, rem % 3600 / 60)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dates() {
        assert_eq!(format_date(0), "1970-01-01 00-00");
        assert_eq!(format_date(1_792_412_700), "2026-10-19 12-25");
        assert_eq!(format_date(951_782_400), "2000-02-29 00-00");
    }

    #[test]
    fn paths() {
        for ok in ["a.txt", "dir/sub/a b.png", "fiainana/sary.ora"] {
            assert!(valid_path(ok), "{ok}");
        }
        for bad in ["", "/etc/passwd", "../x", "a/../b", "a//b", "a/./b", "c:\\x", "a\\b", ".hidden", "d/.git/config", "C:/x"] {
            assert!(!valid_path(bad), "{bad}");
        }
    }
}
