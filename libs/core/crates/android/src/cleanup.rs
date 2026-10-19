//! Old document copies. Every document the app opens is copied into `files/docs/<n>/`; the
//! original stays where the user keeps it, so a copy nobody has touched for a month is only
//! taking space. A copy that is the only one (a Save As whose place the user never chose) is
//! marked with a `<n>.unsaved` file next to its folder and always kept.

use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

/// How long an untouched copy is kept.
pub(crate) const KEEP_COPIES: Duration = Duration::from_secs(30 * 24 * 60 * 60);

/// The marker that keeps the folder holding `path` (a document under `files/docs/<n>/`).
pub(crate) fn unsaved_marker(path: &Path) -> Option<PathBuf> {
    let dir = path.parent()?;
    let name = dir.file_name()?.to_string_lossy();
    Some(dir.with_file_name(format!("{name}.unsaved")))
}

/// Delete the folders under `docs` whose newest file is older than `keep`, unless marked
/// unsaved or holding one of `open` (paths in use). Returns how many folders went. Anything
/// that can't be read is kept.
pub(crate) fn clean_old_copies(docs: &Path, keep: Duration, open: &[PathBuf]) -> usize {
    let Ok(entries) = std::fs::read_dir(docs) else { return 0 };
    let now = SystemTime::now();
    let mut removed = 0;
    for entry in entries.flatten() {
        let path = entry.path();
        let Ok(meta) = std::fs::symlink_metadata(&path) else { continue };
        if !meta.is_dir() {
            // A marker whose folder is gone.
            if path.extension().is_some_and(|e| e == "unsaved") && !path.with_extension("").exists() {
                let _ = std::fs::remove_file(&path);
            }
            continue;
        }
        let marked = path.file_name().map(|n| path.with_file_name(format!("{}.unsaved", n.to_string_lossy()))).is_some_and(|m| m.exists());
        let real = std::fs::canonicalize(&path).unwrap_or_else(|_| path.clone());
        let in_use = open.iter().any(|p| p.starts_with(&path) || p.starts_with(&real));
        let old = newest_change(&path).is_some_and(|t| now.duration_since(t).is_ok_and(|age| age >= keep));
        if marked || in_use || !old {
            continue;
        }
        match std::fs::remove_dir_all(&path) {
            Ok(()) => removed += 1,
            Err(e) => log::warn!("couldn't remove the old copy {}: {e}", path.display()),
        }
    }
    removed
}

/// The latest modification time of `path` and everything under it; `None` if any of it can't
/// be read.
fn newest_change(path: &Path) -> Option<SystemTime> {
    let meta = std::fs::symlink_metadata(path).ok()?;
    let mut newest = meta.modified().ok()?;
    if meta.is_dir() {
        for entry in std::fs::read_dir(path).ok()? {
            newest = newest.max(newest_change(&entry.ok()?.path())?);
        }
    }
    Some(newest)
}

/// The files this process has open, from `/proc/self/fd`.
pub(crate) fn open_files() -> Vec<PathBuf> {
    let Ok(fds) = std::fs::read_dir("/proc/self/fd") else { return Vec::new() };
    fds.flatten().filter_map(|fd| std::fs::read_link(fd.path()).ok()).filter(|p| p.is_absolute()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    fn age(path: &Path, by: Duration) -> std::io::Result<()> {
        let when = SystemTime::now().checked_sub(by).unwrap_or(SystemTime::UNIX_EPOCH);
        File::open(path)?.set_modified(when)
    }

    #[test]
    fn removes_only_old_unmarked_unused_copies() -> std::io::Result<()> {
        let stamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(0);
        let docs = std::env::temp_dir().join(format!("mahay-cleanup-{stamp}"));
        let month = KEEP_COPIES + Duration::from_secs(3600);
        let make = |name: &str, old: bool| -> std::io::Result<PathBuf> {
            let dir = docs.join(name);
            std::fs::create_dir_all(&dir)?;
            let file = dir.join("doc.png");
            std::fs::write(&file, b"x")?;
            if old {
                age(&file, month)?;
                age(&dir, month)?;
            }
            Ok(file)
        };
        make("old", true)?;
        make("fresh", false)?;
        let unsaved = make("unsaved", true)?;
        let open = make("open", true)?;
        // A folder that is old but whose file was just written.
        let edited = make("edited", true)?;
        std::fs::write(&edited, b"y")?;
        let marker = unsaved_marker(&unsaved).unwrap_or_default();
        File::create(&marker)?;
        File::create(docs.join("gone.unsaved"))?;

        let removed = clean_old_copies(&docs, KEEP_COPIES, std::slice::from_ref(&open));

        let left: Vec<bool> = ["old", "fresh", "unsaved", "open", "edited"].iter().map(|n| docs.join(n).exists()).collect();
        let marker_kept = marker.exists();
        let orphan_kept = docs.join("gone.unsaved").exists();
        std::fs::remove_dir_all(&docs)?;
        assert_eq!(removed, 1);
        assert_eq!(left, [false, true, true, true, true]);
        assert!(marker_kept);
        assert!(!orphan_kept);
        Ok(())
    }

    #[test]
    fn sees_its_own_open_files() -> std::io::Result<()> {
        let path = std::env::temp_dir().join(format!("mahay-open-{}", std::process::id()));
        let file = File::create(&path)?;
        let seen = open_files().contains(&std::fs::canonicalize(&path)?);
        drop(file);
        std::fs::remove_file(&path)?;
        assert!(seen);
        Ok(())
    }

    #[test]
    fn marker_sits_next_to_the_folder() {
        let m = unsaved_marker(Path::new("/data/files/docs/18a-0/Picture.png"));
        assert_eq!(m, Some(PathBuf::from("/data/files/docs/18a-0.unsaved")));
    }
}
