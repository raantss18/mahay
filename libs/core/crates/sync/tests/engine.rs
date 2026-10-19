//! Two (or three) devices syncing through one in-memory backend.

use std::fs;
use std::path::Path;
use std::sync::Arc;

use mahay_sync::{Backend, Config, Engine, Error, Key, MemoryBackend, Report, open_key_with};

const PASS: &str = "fanalahidy mahay";
const NOW: i64 = 1_792_412_700; // 2026-10-19 12:25 UTC
const DAY: i64 = 86_400;

struct Device {
    dir: tempfile::TempDir,
    engine: Engine,
    /// What secure storage would keep.
    key: [u8; 32],
}

impl Device {
    fn new(backend: &Arc<MemoryBackend>, name: &str) -> Self {
        let dir = tempfile::tempdir().unwrap();
        let (key, _) = open_key_with(backend.as_ref(), PASS, 1000).unwrap();
        let cfg = Config { root: dir.path().join("docs"), state_dir: dir.path().join("state"), device: name.into() };
        fs::create_dir_all(&cfg.root).unwrap();
        let raw = *key.to_bytes();
        let engine = Engine::new(cfg, backend.clone(), key).unwrap();
        Self { dir, engine, key: raw }
    }

    /// The same device after a restart (journal reloaded from disk).
    fn restart(self, backend: &Arc<MemoryBackend>, name: &str) -> Self {
        let key = Key::from_bytes(self.key).unwrap();
        let cfg = Config { root: self.root(), state_dir: self.dir.path().join("state"), device: name.into() };
        Self { engine: Engine::new(cfg, backend.clone(), key).unwrap(), dir: self.dir, key: self.key }
    }

    fn root(&self) -> std::path::PathBuf {
        self.dir.path().join("docs")
    }

    fn write(&self, rel: &str, data: &[u8]) {
        let p = self.root().join(rel);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(p, data).unwrap();
    }

    fn read(&self, rel: &str) -> Option<Vec<u8>> {
        fs::read(self.root().join(rel)).ok()
    }

    fn rm(&self, rel: &str) {
        fs::remove_file(self.root().join(rel)).unwrap();
    }

    fn sync(&mut self) -> Report {
        self.engine.sync_at(NOW).unwrap()
    }

    fn files(&self) -> Vec<(String, Vec<u8>)> {
        let mut out = Vec::new();
        walk(&self.root(), "", &mut out);
        out.sort();
        out
    }
}

fn walk(dir: &Path, prefix: &str, out: &mut Vec<(String, Vec<u8>)>) {
    for e in fs::read_dir(dir).unwrap() {
        let e = e.unwrap();
        let name = e.file_name().into_string().unwrap();
        let rel = if prefix.is_empty() { name.clone() } else { format!("{prefix}/{name}") };
        if e.file_type().unwrap().is_dir() {
            walk(&e.path(), &rel, out);
        } else {
            out.push((rel, fs::read(e.path()).unwrap()));
        }
    }
}

fn file_blobs(b: &MemoryBackend) -> usize {
    b.snapshot().keys().filter(|k| k.ends_with(".mhy") && *k != "manifest.mhy").count()
}

fn setup() -> (Arc<MemoryBackend>, Device, Device) {
    let backend = Arc::new(MemoryBackend::new());
    let a = Device::new(&backend, "Laptop");
    let b = Device::new(&backend, "Tablette");
    (backend, a, b)
}

#[test]
fn two_devices_and_provider_sees_nothing() {
    let (backend, mut a, mut b) = setup();
    a.write("tantara.txt", b"Indray andro ny hazavana...");
    a.write("sary/fiainana/vorona.ora", &vec![7u8; 3 << 20]);
    a.write("empty", b"");
    let r = a.sync();
    assert_eq!(r.uploaded.len(), 3);
    b.sync();
    assert_eq!(a.files(), b.files());

    // names, folders and contents are invisible to the provider
    for (name, blob) in backend.snapshot() {
        assert!(name == "check.json" || name == "manifest.mhy" || (name.len() == 36 && name.ends_with(".mhy")), "{name}");
        for needle in [&b"tantara"[..], b"sary", b"vorona", b"hazavana"] {
            assert!(!blob.windows(needle.len()).any(|w| w == needle), "{name} leaks");
        }
    }
    // nothing to do the second time
    assert_eq!(a.sync(), Report::default());
    assert_eq!(b.sync(), Report::default());
}

#[test]
fn edits_propagate_and_old_blobs_go() {
    let (backend, mut a, mut b) = setup();
    a.write("a.txt", b"v1");
    a.sync();
    b.sync();
    b.write("a.txt", b"version 2 from the tablet");
    assert_eq!(b.sync().uploaded, ["a.txt"]);
    assert_eq!(a.sync().downloaded, ["a.txt"]);
    assert_eq!(a.read("a.txt").unwrap(), b"version 2 from the tablet");
    assert_eq!(file_blobs(&backend), 1);
}

#[test]
fn wrong_passphrase_refused() {
    let (backend, _a, _b) = setup();
    assert!(matches!(open_key_with(backend.as_ref(), "not it", 1000), Err(Error::BadPassphrase)));
}

#[test]
fn conflict_keeps_both() {
    let (_backend, mut a, mut b) = setup();
    a.write("notes/plan.txt", b"base");
    a.sync();
    b.sync();
    a.write("notes/plan.txt", b"edited on the laptop");
    b.write("notes/plan.txt", b"edited on the tablet!");
    a.sync();
    let r = b.sync();
    let copy = "notes/plan (conflit Laptop ".to_owned();
    assert_eq!(r.conflicts.len(), 1);
    assert!(r.conflicts[0].starts_with(&copy) && r.conflicts[0].ends_with(").txt"), "{:?}", r.conflicts);
    assert_eq!(b.read("notes/plan.txt").unwrap(), b"edited on the tablet!");
    assert_eq!(b.read(&r.conflicts[0]).unwrap(), b"edited on the laptop");
    a.sync();
    assert_eq!(a.files(), b.files());
    assert_eq!(a.files().len(), 2);
    assert_eq!(a.sync(), Report::default());
    assert_eq!(b.sync(), Report::default());
}

#[test]
fn same_edit_on_both_sides_is_no_conflict() {
    let (_backend, mut a, mut b) = setup();
    a.write("x", b"1");
    a.sync();
    b.sync();
    a.write("x", b"same");
    b.write("x", b"same");
    a.sync();
    assert!(b.sync().conflicts.is_empty());
    assert_eq!(b.files().len(), 1);
}

#[test]
fn first_sync_with_different_files_of_the_same_name() {
    let (_backend, mut a, mut b) = setup();
    a.write("README", b"from a");
    b.write("README", b"from b");
    a.sync();
    let r = b.sync();
    assert_eq!(r.conflicts.len(), 1);
    a.sync();
    assert_eq!(a.files(), b.files());
}

#[test]
fn deletes_go_to_the_bin_for_30_days() {
    let (backend, mut a, mut b) = setup();
    a.write("old.txt", b"keep me a while");
    a.write("other.txt", b"x");
    a.sync();
    b.sync();
    a.rm("old.txt");
    assert_eq!(a.sync().binned, ["old.txt"]);
    assert_eq!(b.sync().deleted_here, ["old.txt"]);
    assert_eq!(b.read("old.txt"), None);
    assert_eq!(b.engine.bin().unwrap(), [("old.txt".to_owned(), NOW)]);
    assert_eq!(file_blobs(&backend), 2, "the blob stays while in the bin");

    // restored from the bin: back on both devices
    assert!(b.engine.restore("old.txt").unwrap());
    b.sync();
    a.sync();
    assert_eq!(a.read("old.txt").unwrap(), b"keep me a while");
    assert_eq!(b.read("old.txt").unwrap(), b"keep me a while");

    // deleted again, then 31 days later the blob is removed
    a.rm("old.txt");
    a.sync();
    b.sync();
    let r = b.engine.sync_at(NOW + 31 * DAY).unwrap();
    assert_eq!(r.purged, ["old.txt"]);
    assert_eq!(file_blobs(&backend), 1);
    assert!(b.engine.bin().unwrap().is_empty());
    assert!(!b.engine.restore("old.txt").unwrap());
    assert_eq!(a.engine.sync_at(NOW + 31 * DAY).unwrap(), Report::default());
}

#[test]
fn an_edit_wins_over_a_delete() {
    let (_backend, mut a, mut b) = setup();
    a.write("f.txt", b"1");
    a.sync();
    b.sync();
    a.rm("f.txt");
    b.write("f.txt", b"edited while deleted elsewhere");
    a.sync();
    b.sync();
    a.sync();
    assert_eq!(a.read("f.txt").unwrap(), b"edited while deleted elsewhere");

    // and the other way round: deleted here, edited there
    a.rm("f.txt");
    b.write("f.txt", b"edited again");
    b.sync();
    a.sync();
    assert_eq!(a.read("f.txt").unwrap(), b"edited again");
}

#[test]
fn a_changed_file_is_not_deleted_by_a_remote_delete() {
    let (_backend, mut a, mut b) = setup();
    a.write("f", b"1");
    a.sync();
    b.sync();
    a.rm("f");
    a.sync();
    b.write("f", b"22");
    b.sync();
    assert_eq!(b.read("f").unwrap(), b"22");
    a.sync();
    assert_eq!(a.read("f").unwrap(), b"22");
}

#[test]
fn offline_changes_wait_in_the_journal() {
    let (backend, mut a, mut b) = setup();
    a.write("a", b"1");
    a.sync();
    backend.set_offline(true);
    a.write("a", b"offline edit");
    a.write("new", b"made offline");
    let err = a.engine.sync_at(NOW).unwrap_err();
    assert!(err.is_transient(), "{err}");
    // the app restarts while still offline, then the network comes back
    let mut a = a.restart(&backend, "Laptop");
    backend.set_offline(false);
    let r = a.sync();
    assert_eq!(r.uploaded, ["a", "new"]);
    b.sync();
    assert_eq!(a.files(), b.files());
}

#[test]
fn network_cut_at_every_step_of_an_upload() {
    for cut in 0..30 {
        let (backend, mut a, mut b) = setup();
        a.write("keep.txt", b"already synced");
        a.sync();
        a.write("keep.txt", b"changed");
        for i in 0..4 {
            a.write(&format!("dir/f{i}.bin"), &vec![i as u8; 1 + i * 400_000]);
        }
        backend.fail_after(cut);
        let first = a.engine.sync_at(NOW);
        backend.set_offline(false);
        if let Err(e) = &first {
            assert!(e.is_transient(), "cut {cut}: {e}");
        }
        let mut a = a.restart(&backend, "Laptop");
        a.sync();
        assert_eq!(a.sync(), Report::default(), "cut {cut}");
        let r = b.sync();
        assert!(r.conflicts.is_empty(), "cut {cut}: {r:?}");
        assert_eq!(a.files(), b.files(), "cut {cut}");
        assert_eq!(file_blobs(&backend), 5, "cut {cut}: no orphan blob");
    }
}

#[test]
fn network_cut_at_every_step_of_a_download() {
    for cut in 0..20 {
        let (backend, mut a, mut b) = setup();
        for i in 0..5 {
            a.write(&format!("f{i}"), format!("content {i}").as_bytes());
        }
        a.sync();
        b.write("mine", b"made on the tablet");
        backend.fail_after(cut);
        let first = b.engine.sync_at(NOW);
        backend.set_offline(false);
        if let Err(e) = &first {
            assert!(e.is_transient(), "cut {cut}: {e}");
        }
        let mut b = b.restart(&backend, "Tablette");
        let r = b.sync();
        assert!(r.conflicts.is_empty(), "cut {cut}: {r:?}");
        a.sync();
        assert_eq!(a.files(), b.files(), "cut {cut}");
        assert_eq!(a.files().len(), 6);
        assert_eq!(file_blobs(&backend), 6, "cut {cut}");
    }
}

#[test]
fn lost_manifest_answer_is_not_a_conflict() {
    // the manifest write reached the server but its answer was lost
    let (backend, mut a, mut b) = setup();
    a.write("x", b"1");
    a.sync();
    a.write("x", b"two");
    backend.lose_answer_on("manifest.mhy");
    assert!(a.engine.sync_at(NOW).unwrap_err().is_transient());
    let r = a.sync();
    assert!(r.conflicts.is_empty() && r.uploaded.is_empty(), "{r:?}");
    b.sync();
    assert_eq!(b.read("x").unwrap(), b"two");
    assert_eq!(file_blobs(&backend), 1);
    // edited again after the lost answer: a plain upload, still no conflict
    a.write("x", b"three");
    let r = a.sync();
    assert_eq!(r.uploaded, ["x"]);
    assert!(b.sync().conflicts.is_empty());
    assert_eq!(b.read("x").unwrap(), b"three");
}

#[test]
fn a_lost_manifest_update_is_published_again() {
    // two devices wrote the manifest at once and A's change was overwritten
    let (backend, mut a, mut b) = setup();
    a.write("x", b"1");
    a.sync();
    let old = backend.snapshot()["manifest.mhy"].clone();
    a.write("x", b"newer");
    a.sync();
    backend.raw_put("manifest.mhy", old);
    a.sync();
    b.sync();
    assert_eq!(b.read("x").unwrap(), b"newer");
}

#[test]
fn tampered_blob_is_refused_and_not_written() {
    let (backend, mut a, mut b) = setup();
    a.write("secret.txt", b"do not alter");
    a.sync();
    let (name, mut blob) = backend.snapshot().into_iter().find(|(k, _)| k.len() == 36).unwrap();
    let n = blob.len();
    blob[n - 20] ^= 0x40;
    backend.raw_put(&name, blob);
    assert!(matches!(b.engine.sync_at(NOW), Err(Error::Decrypt)));
    assert_eq!(b.read("secret.txt"), None);
}

#[test]
fn a_manifest_from_another_key_is_refused() {
    let (backend, mut a, _b) = setup();
    a.write("x", b"1");
    a.sync();
    let other = Arc::new(MemoryBackend::new());
    let mut c = Device::new(&other, "Other");
    c.write("y", b"2");
    c.sync();
    backend.raw_put("manifest.mhy", other.get("manifest.mhy").unwrap().unwrap());
    assert!(matches!(a.engine.sync_at(NOW), Err(Error::Decrypt)));
}

#[test]
fn hidden_and_temporary_files_are_not_synced() {
    let (_backend, mut a, mut b) = setup();
    a.write(".git/config", b"x");
    a.write(".x.123-0.mahay_tmp", b"x");
    a.write("visible", b"x");
    assert_eq!(a.sync().uploaded, ["visible"]);
    b.sync();
    assert_eq!(b.files().len(), 1);
}

#[test]
fn modification_time_is_kept() {
    let (_backend, mut a, mut b) = setup();
    a.write("t", b"x");
    let when = std::time::UNIX_EPOCH + std::time::Duration::from_secs(1_700_000_000);
    fs::File::options().write(true).open(a.root().join("t")).unwrap().set_modified(when).unwrap();
    a.sync();
    b.sync();
    assert_eq!(fs::metadata(b.root().join("t")).unwrap().modified().unwrap(), when);
}

#[test]
fn a_missing_blob_does_not_block_the_rest() {
    let (backend, mut a, mut b) = setup();
    a.write("gone", b"its blob will vanish");
    a.sync();
    let name = backend.snapshot().into_keys().find(|k| k.len() == 36).unwrap();
    backend.delete(&name).unwrap();
    a.write("fine", b"still syncs");
    a.sync();
    let r = b.sync();
    assert_eq!(r.missing, ["gone"]);
    assert_eq!(b.read("fine").unwrap(), b"still syncs");
    assert_eq!(b.read("gone"), None);
}

#[test]
fn an_emptied_remote_folder_is_filled_again() {
    let (backend, mut a, _b) = setup();
    a.write("x", b"1");
    a.sync();
    let check = backend.get("check.json").unwrap().unwrap();
    for name in backend.list().unwrap() {
        backend.delete(&name).unwrap();
    }
    backend.put("check.json", &check).unwrap();
    assert_eq!(a.sync().uploaded, ["x"]);
    let mut c = Device::new(&backend, "Phone");
    c.sync();
    assert_eq!(c.read("x").unwrap(), b"1");
}
