//! The encrypted manifest (`manifest.mhy`): the only place where paths, sizes and dates appear.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::Error;
use crate::crypto::{Key, decrypt_file, encrypt_file};

/// The file id bound into the manifest's chunks.
const MANIFEST_ID: &str = "manifest";

/// One synced file, as the manifest records it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entry {
    /// The blob holding the content (`<id>.mhy`); `None` once a deleted file left the bin.
    pub id: Option<String>,
    pub size: u64,
    /// Modification time, Unix seconds.
    pub mtime: i64,
    /// SHA-256 of the plaintext, hex.
    pub sha256: String,
    /// Grows by one at every change, on any device.
    pub version: u64,
    /// Name of the device that made this version.
    pub device: String,
    /// Unix seconds when the file was deleted: it stays in the sync bin for 30 days.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<i64>,
}

impl Entry {
    pub fn is_live(&self) -> bool {
        self.deleted.is_none() && self.id.is_some()
    }
}

/// Every file of the sync folder, by relative path (`/` separators).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Manifest {
    pub v: u32,
    pub files: BTreeMap<String, Entry>,
}

impl Manifest {
    pub fn new() -> Self {
        Self { v: 1, files: BTreeMap::new() }
    }

    pub fn seal(&self, key: &Key) -> Result<Vec<u8>, Error> {
        let json = serde_json::to_vec(self).map_err(|_| Error::Format("manifest JSON"))?;
        encrypt_file(key, MANIFEST_ID, &json)
    }

    pub fn open(key: &Key, blob: &[u8]) -> Result<Self, Error> {
        let json = decrypt_file(key, MANIFEST_ID, blob)?;
        let m: Self = serde_json::from_slice(&json).map_err(|_| Error::Format("manifest JSON"))?;
        if m.v != 1 {
            return Err(Error::Format("unknown manifest version"));
        }
        Ok(m)
    }
}
