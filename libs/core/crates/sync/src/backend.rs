//! Where blobs go. A backend only ever sees opaque names and encrypted bytes.

use std::collections::BTreeMap;
use std::sync::Mutex;

/// What a backend can fail with.
#[derive(Debug, thiserror::Error)]
pub enum BackendError {
    /// No network, a timeout, a server error: try again later.
    #[error("network: {0}")]
    Network(String),
    /// Wrong credentials or a revoked token: ask the user.
    #[error("access refused: {0}")]
    Auth(String),
    /// The server answered something we do not understand.
    #[error("backend: {0}")]
    Other(String),
}

/// A flat store of blobs under the user's `Mahay/` folder.
///
/// Names are made of `[A-Za-z0-9._-]` only (see [`valid_name`]): `<id>.mhy`, `manifest.mhy`,
/// `check.json`.
pub trait Backend: Send + Sync {
    /// Writes (or replaces) a blob.
    fn put(&self, name: &str, data: &[u8]) -> Result<(), BackendError>;
    /// Reads a blob; `Ok(None)` when it does not exist.
    fn get(&self, name: &str) -> Result<Option<Vec<u8>>, BackendError>;
    /// Names of every blob.
    fn list(&self) -> Result<Vec<String>, BackendError>;
    /// Deletes a blob; deleting a missing one is not an error.
    fn delete(&self, name: &str) -> Result<(), BackendError>;
}

/// Whether `name` is a safe blob name.
pub fn valid_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 128
        && !name.starts_with('.')
        && name.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'-'))
}

fn check_name(name: &str) -> Result<(), BackendError> {
    if valid_name(name) { Ok(()) } else { Err(BackendError::Other(format!("bad blob name {name:?}"))) }
}

/// An in-memory backend for tests, which can be cut off on demand.
#[derive(Default)]
pub struct MemoryBackend {
    inner: Mutex<MemoryInner>,
}

#[derive(Default)]
struct MemoryInner {
    blobs: BTreeMap<String, Vec<u8>>,
    offline: bool,
    /// Operations allowed before the network drops; `None` = no limit.
    ops_left: Option<u32>,
    ops: u64,
    /// The next `put` of this name is stored but answered as a network failure.
    lose_answer: Option<String>,
}

impl MemoryBackend {
    pub fn new() -> Self {
        Self::default()
    }

    /// Cuts or restores the network.
    pub fn set_offline(&self, offline: bool) {
        let mut g = self.lock();
        g.offline = offline;
        if !offline {
            g.ops_left = None;
        }
    }

    /// Lets `n` more operations through, then behaves as offline.
    pub fn fail_after(&self, n: u32) {
        self.lock().ops_left = Some(n);
    }

    /// The next `put` of `name` reaches the store but its answer is lost.
    pub fn lose_answer_on(&self, name: &str) {
        self.lock().lose_answer = Some(name.to_owned());
    }

    /// Operations served so far.
    pub fn ops(&self) -> u64 {
        self.lock().ops
    }

    /// A copy of every blob (for inspection in tests).
    pub fn snapshot(&self) -> BTreeMap<String, Vec<u8>> {
        self.lock().blobs.clone()
    }

    /// Replaces a blob directly, bypassing failures (to simulate tampering).
    pub fn raw_put(&self, name: &str, data: Vec<u8>) {
        self.lock().blobs.insert(name.to_owned(), data);
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, MemoryInner> {
        self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn op<T>(&self, f: impl FnOnce(&mut BTreeMap<String, Vec<u8>>) -> T) -> Result<T, BackendError> {
        let mut g = self.lock();
        if let Some(n) = g.ops_left {
            if n == 0 {
                g.offline = true;
            } else {
                g.ops_left = Some(n - 1);
            }
        }
        if g.offline {
            return Err(BackendError::Network("offline".into()));
        }
        g.ops += 1;
        Ok(f(&mut g.blobs))
    }
}

impl Backend for MemoryBackend {
    fn put(&self, name: &str, data: &[u8]) -> Result<(), BackendError> {
        check_name(name)?;
        self.op(|b| {
            b.insert(name.to_owned(), data.to_vec());
        })?;
        let mut g = self.lock();
        if g.lose_answer.as_deref() == Some(name) {
            g.lose_answer = None;
            return Err(BackendError::Network("answer lost".into()));
        }
        Ok(())
    }

    fn get(&self, name: &str) -> Result<Option<Vec<u8>>, BackendError> {
        check_name(name)?;
        self.op(|b| b.get(name).cloned())
    }

    fn list(&self) -> Result<Vec<String>, BackendError> {
        self.op(|b| b.keys().cloned().collect())
    }

    fn delete(&self, name: &str) -> Result<(), BackendError> {
        check_name(name)?;
        self.op(|b| {
            b.remove(name);
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names() {
        for ok in ["manifest.mhy", "check.json", "0123abcd.mhy"] {
            assert!(valid_name(ok), "{ok}");
        }
        for bad in ["", "..", ".hidden", "a/b", "a b", "é.mhy", "a\\b"] {
            assert!(!valid_name(bad), "{bad}");
        }
    }

    #[test]
    fn memory_fails_on_demand() {
        let m = MemoryBackend::new();
        m.put("a", b"1").unwrap();
        m.fail_after(1);
        assert_eq!(m.get("a").unwrap().as_deref(), Some(&b"1"[..]));
        assert!(matches!(m.get("a"), Err(BackendError::Network(_))));
        m.set_offline(false);
        assert_eq!(m.list().unwrap(), ["a"]);
        m.delete("a").unwrap();
        m.delete("a").unwrap();
        assert_eq!(m.get("a").unwrap(), None);
    }
}
