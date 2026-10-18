//! Join codes on the server: a short name for one share link, typed in place of the link.
//!
//! A code is 45 random bits ([`figcraft_collab::code`]). Like a link's token it is never stored:
//! a link keeps the code's hash, to find the link by, and its own token wrapped with the code, to
//! hand to whoever types the code (they then join as with the link). Both come from the code
//! stretched with SHA-256 ([`STRETCH`] rounds), so a copy of the data directory doesn't give the
//! codes away cheaply; guessing against a running server is held back by the limits on wrong codes
//! (`abuse.rs`).

use figcraft_collab::{code, same_secret};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Rounds of SHA-256 a code goes through before it is compared or used as a key.
pub const STRETCH: u32 = 10_000;

// A copy of the data directory mustn't give codes away cheaply.
const _: () = assert!(STRETCH >= 10_000);

/// Longest token a code can wrap (one SHA-256 output).
const MAX_TOKEN: usize = 32;

/// What a link keeps of its join code (`links.json`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Sealed {
    /// The stretched code's hash, hex: what a typed code is looked up by.
    pub hash: String,
    /// The link's token, wrapped with the stretched code (hex).
    pub token: String,
}

/// A typed code, stretched: ready to look links up with (what it is found by, and the pad its
/// link's token is wrapped with).
pub struct Key {
    hash: String,
    pad: [u8; 32],
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

impl Key {
    /// Stretch `code` (its symbols, as [`code::normalize`] gives them).
    pub fn new(code: &str) -> Key {
        let mut k: [u8; 32] = Sha256::new().chain_update(b"figcraft join code\0").chain_update(code.as_bytes()).finalize().into();
        for _ in 0..STRETCH {
            k = Sha256::digest(k).into();
        }
        let part = |what: &[u8]| -> [u8; 32] { Sha256::new().chain_update(what).chain_update(k).finalize().into() };
        Key { hash: hex(&part(b"find\0")), pad: part(b"wrap\0") }
    }

    /// What a link with this code keeps as [`Sealed::hash`].
    pub fn hash(&self) -> &str {
        &self.hash
    }

    /// Wrap or unwrap `bytes` (the same operation: XOR with a pad only this code gives).
    fn wrap(&self, bytes: &[u8]) -> Vec<u8> {
        bytes.iter().zip(self.pad).map(|(b, p)| b ^ p).collect()
    }

    /// Keep `token` under this code.
    pub fn seal(&self, token: &str) -> Result<Sealed, String> {
        if token.len() > MAX_TOKEN {
            return Err("this link's token is too long for a join code".into());
        }
        Ok(Sealed { hash: self.hash.clone(), token: hex(&self.wrap(token.as_bytes())) })
    }

    /// The token `sealed` keeps, when this is its code (compared in constant time).
    pub fn open(&self, sealed: &Sealed) -> Option<String> {
        if !same_secret(&self.hash, &sealed.hash) || sealed.token.len() > 2 * MAX_TOKEN {
            return None;
        }
        let wrapped: Option<Vec<u8>> =
            (0..sealed.token.len() / 2).map(|i| sealed.token.get(2 * i..2 * i + 2).and_then(|h| u8::from_str_radix(h, 16).ok())).collect();
        String::from_utf8(self.wrap(&wrapped?)).ok()
    }
}

/// A new code's symbols, from the operating system's random numbers.
pub fn new_code() -> Result<String, String> {
    figcraft_collab_net::token::random().map(code::online)
}
