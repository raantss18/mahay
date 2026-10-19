#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
//! End-to-end encrypted sync for the Mahay suite (protocol: `docs/SYNC.md`, decision D20).
//!
//! The provider (Google Drive, a Nextcloud/WebDAV server, the in-memory test store) only ever
//! sees a flat `Mahay/` folder of opaque blobs:
//!
//! - `check.json`: the passphrase check block ([`CheckBlock`], the same as `meta/check`);
//! - `manifest.mhy`: the encrypted [`Manifest`] (paths, sizes, dates, SHA-256, versions);
//! - `<random id>.mhy`: one encrypted file each, in 1 MiB AES-256-GCM chunks.
//!
//! [`Engine`] keeps a local folder in sync with it: offline journal, conflict copies
//! `name (conflit <device> <date>).ext`, deletes kept 30 days in the sync bin.

pub mod backend;
pub mod crypto;
pub mod engine;
pub mod manifest;
pub mod webdav;

use std::path::{Path, PathBuf};

pub use backend::{Backend, BackendError, MemoryBackend};
pub use crypto::{CheckBlock, KDF_ITERATIONS, Key};
pub use engine::{Config, Engine, Report, open_key, open_key_with};
pub use manifest::{Entry, Manifest};
pub use webdav::WebDav;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Backend(#[from] BackendError),
    #[error("{}: {source}", path.display())]
    Io { path: PathBuf, source: std::io::Error },
    /// Wrong key, or a blob that was altered, reordered or cut.
    #[error("decryption failed: wrong key or damaged data")]
    Decrypt,
    #[error("wrong sync passphrase")]
    BadPassphrase,
    #[error("invalid data: {0}")]
    Format(&'static str),
    #[error("integrity: {0}")]
    Integrity(String),
    #[error("the system random generator failed")]
    Random,
}

impl Error {
    pub(crate) fn io(path: &Path, source: std::io::Error) -> Self {
        Self::Io { path: path.to_owned(), source }
    }

    /// Worth trying again later (no network, server busy).
    pub fn is_transient(&self) -> bool {
        matches!(self, Self::Backend(BackendError::Network(_)))
    }
}
