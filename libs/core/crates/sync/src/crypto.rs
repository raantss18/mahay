//! Keys, the passphrase check block and the chunked file format.
//!
//! - Key: PBKDF2-HMAC-SHA256 of the sync passphrase, 310,000 iterations, 16-byte random salt.
//! - Check block (`meta/check`, as in Tontolo): JSON `{"v":1,"salt","iter","check"}` where
//!   `check` is base64 of `nonce ‖ AES-256-GCM("mahay-sync-check-v1", aad "meta|check")`.
//! - Blob: `MHY1` then chunks of `nonce(12) ‖ ciphertext ‖ tag(16)`, 1 MiB of plaintext each,
//!   with the associated data `mahay-v1|<file id>|<chunk index>|<last flag>`. A reordered,
//!   swapped, cut or altered chunk fails to open.

use std::num::NonZeroU32;

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as B64;
use ring::aead::{AES_256_GCM, Aad, LessSafeKey, NONCE_LEN, Nonce, UnboundKey};
use ring::rand::{SecureRandom, SystemRandom};
use serde::{Deserialize, Serialize};
use zeroize::Zeroizing;

use crate::Error;

/// PBKDF2 iterations for a new passphrase (OWASP 2023 figure for HMAC-SHA256, as Tontolo).
pub const KDF_ITERATIONS: u32 = 310_000;
/// Plaintext bytes per encrypted chunk.
pub const CHUNK_SIZE: usize = 1 << 20;
const TAG_LEN: usize = 16;
const SALT_LEN: usize = 16;
const MAGIC: &[u8; 4] = b"MHY1";
const CHECK_PLAIN: &[u8] = b"mahay-sync-check-v1";
const CHECK_AAD: &[u8] = b"meta|check";

/// The 256-bit sync key. Its bytes are wiped when it is dropped.
pub struct Key {
    raw: Zeroizing<[u8; 32]>,
    aead: LessSafeKey,
}

impl Key {
    /// Derives the key from a passphrase (slow on purpose: run it off the UI thread).
    pub fn derive(passphrase: &str, salt: &[u8], iterations: u32) -> Result<Self, Error> {
        let iterations = NonZeroU32::new(iterations).ok_or(Error::Format("zero iterations"))?;
        let mut raw = Zeroizing::new([0u8; 32]);
        ring::pbkdf2::derive(ring::pbkdf2::PBKDF2_HMAC_SHA256, iterations, salt, passphrase.as_bytes(), &mut *raw);
        Self::from_bytes(*raw)
    }

    /// Rebuilds a key kept in secure storage.
    pub fn from_bytes(bytes: [u8; 32]) -> Result<Self, Error> {
        let raw = Zeroizing::new(bytes);
        let aead = LessSafeKey::new(UnboundKey::new(&AES_256_GCM, &*raw).map_err(|_| Error::Format("AES key"))?);
        Ok(Self { raw, aead })
    }

    /// The raw key, for secure storage only (Android Keystore, libsecret).
    pub fn to_bytes(&self) -> Zeroizing<[u8; 32]> {
        self.raw.clone()
    }

    /// Appends `nonce ‖ ciphertext ‖ tag` to `out`.
    fn seal_with(&self, nonce: [u8; NONCE_LEN], aad: &[u8], plain: &[u8], out: &mut Vec<u8>) -> Result<(), Error> {
        let mut body = plain.to_vec();
        self.aead
            .seal_in_place_append_tag(Nonce::assume_unique_for_key(nonce), Aad::from(aad), &mut body)
            .map_err(|_| Error::Format("AES-GCM seal"))?;
        out.extend_from_slice(&nonce);
        out.extend_from_slice(&body);
        Ok(())
    }

    fn seal(&self, aad: &[u8], plain: &[u8], out: &mut Vec<u8>) -> Result<(), Error> {
        self.seal_with(random()?, aad, plain, out)
    }

    fn open(&self, aad: &[u8], sealed: &[u8]) -> Result<Vec<u8>, Error> {
        if sealed.len() < NONCE_LEN + TAG_LEN {
            return Err(Error::Decrypt);
        }
        let (nonce, body) = sealed.split_at(NONCE_LEN);
        let nonce = Nonce::try_assume_unique_for_key(nonce).map_err(|_| Error::Decrypt)?;
        let mut body = body.to_vec();
        let plain = self.aead.open_in_place(nonce, Aad::from(aad), &mut body).map_err(|_| Error::Decrypt)?;
        Ok(plain.to_vec())
    }
}

/// Random bytes from the system's generator.
pub fn random<const N: usize>() -> Result<[u8; N], Error> {
    let mut b = [0u8; N];
    SystemRandom::new().fill(&mut b).map_err(|_| Error::Random)?;
    Ok(b)
}

/// A new random blob id: 32 lowercase hex characters. It says nothing about the file.
pub fn new_id() -> Result<String, Error> {
    Ok(hex(&random::<16>()?))
}

/// Lowercase hex.
pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// SHA-256 of `data`, in hex.
pub fn sha256_hex(data: &[u8]) -> String {
    hex(ring::digest::digest(&ring::digest::SHA256, data).as_ref())
}

fn chunk_aad(file_id: &str, index: u64, last: bool) -> Vec<u8> {
    format!("mahay-v1|{file_id}|{index}|{}", u8::from(last)).into_bytes()
}

/// Encrypts a whole file as a blob. `file_id` is bound into every chunk.
pub fn encrypt_file(key: &Key, file_id: &str, plain: &[u8]) -> Result<Vec<u8>, Error> {
    let chunks = plain.len().div_ceil(CHUNK_SIZE).max(1);
    let mut out = Vec::with_capacity(MAGIC.len() + plain.len() + chunks * (NONCE_LEN + TAG_LEN));
    out.extend_from_slice(MAGIC);
    for i in 0..chunks {
        let part = &plain[(i * CHUNK_SIZE).min(plain.len())..((i + 1) * CHUNK_SIZE).min(plain.len())];
        key.seal(&chunk_aad(file_id, i as u64, i + 1 == chunks), part, &mut out)?;
    }
    Ok(out)
}

/// Decrypts a blob made by [`encrypt_file`] for the same `file_id`.
pub fn decrypt_file(key: &Key, file_id: &str, blob: &[u8]) -> Result<Vec<u8>, Error> {
    let body = blob.strip_prefix(MAGIC.as_slice()).ok_or(Error::Format("not a Mahay blob"))?;
    let step = NONCE_LEN + CHUNK_SIZE + TAG_LEN;
    let mut out = Vec::with_capacity(body.len());
    let mut rest = body;
    let mut index = 0u64;
    loop {
        let last = rest.len() <= step;
        let (chunk, tail) = rest.split_at(rest.len().min(step));
        out.extend_from_slice(&key.open(&chunk_aad(file_id, index, last), chunk)?);
        if last {
            return Ok(out);
        }
        rest = tail;
        index += 1;
    }
}

/// The passphrase check block stored under `meta/check`: the salt, the iteration count and a
/// known text sealed with the key, so a wrong passphrase is caught before anything is decrypted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckBlock {
    pub v: u32,
    pub salt: String,
    pub iter: u32,
    pub check: String,
}

impl CheckBlock {
    /// First setup: a new salt, the key and its check block.
    pub fn create(passphrase: &str, iterations: u32) -> Result<(Self, Key), Error> {
        let salt = random::<SALT_LEN>()?;
        let key = Key::derive(passphrase, &salt, iterations)?;
        let mut sealed = Vec::new();
        key.seal(CHECK_AAD, CHECK_PLAIN, &mut sealed)?;
        Ok((Self { v: 1, salt: B64.encode(salt), iter: iterations, check: B64.encode(sealed) }, key))
    }

    /// Another device: derives the key and checks it. A wrong passphrase gives
    /// [`Error::BadPassphrase`].
    pub fn unlock(&self, passphrase: &str) -> Result<Key, Error> {
        if self.v != 1 {
            return Err(Error::Format("unknown check block version"));
        }
        let salt = B64.decode(&self.salt).map_err(|_| Error::Format("check block salt"))?;
        let sealed = B64.decode(&self.check).map_err(|_| Error::Format("check block"))?;
        let key = Key::derive(passphrase, &salt, self.iter)?;
        match key.open(CHECK_AAD, &sealed) {
            Ok(p) if p == CHECK_PLAIN => Ok(key),
            _ => Err(Error::BadPassphrase),
        }
    }

    pub fn to_json(&self) -> Result<String, Error> {
        serde_json::to_string(self).map_err(|_| Error::Format("check block JSON"))
    }

    pub fn from_json(s: &str) -> Result<Self, Error> {
        serde_json::from_str(s).map_err(|_| Error::Format("check block JSON"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unhex(s: &str) -> Vec<u8> {
        (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
    }

    // Vectors made independently with Python (hashlib + cryptography's AESGCM).
    const KEY: &str = "02169e674d80c0fc7292e8610c8f17be140df062af9aa33ea85eaabecffdf08d";
    const CHECK: &str = "010101010101010101010101da209e392f61edda1a0096db217923a0b30177aeba3d67afe978db1d8ba5fe69289cbd";
    const CHUNK: &str = "02020202020202020202020277ae7cb9b0c21ed852ac00715ba0568512719b91aa745c0291edc1";
    const FID: &str = "0123456789abcdef0123456789abcdef";

    fn test_key() -> Key {
        Key::derive("correct horse battery", &(0..16).collect::<Vec<u8>>(), 1000).unwrap()
    }

    #[test]
    fn pbkdf2_rfc7914_vectors() {
        let k = Key::derive("passwd", b"salt", 1).unwrap();
        assert_eq!(hex(&*k.to_bytes()), "55ac046e56e3089fec1691c22544b605f94185216dde0465e68b9d57c20dacbc");
        let k = Key::derive("Password", b"NaCl", 80_000).unwrap();
        assert_eq!(hex(&*k.to_bytes()), "4ddcd8f60b98be21830cee5ef22701f9641a4418d04c0414aeff08876b34ab56");
    }

    #[test]
    fn key_matches_python() {
        assert_eq!(hex(&*test_key().to_bytes()), KEY);
    }

    #[test]
    fn check_and_chunk_match_python() {
        let key = test_key();
        let mut out = Vec::new();
        key.seal_with([1; 12], CHECK_AAD, CHECK_PLAIN, &mut out).unwrap();
        assert_eq!(hex(&out), CHECK);
        let mut out = Vec::new();
        key.seal_with([2; 12], &chunk_aad(FID, 0, true), b"hello mahay", &mut out).unwrap();
        assert_eq!(hex(&out), CHUNK);
        // and the Python chunk opens as a one-chunk blob
        let blob = [MAGIC.as_slice(), &unhex(CHUNK)].concat();
        assert_eq!(decrypt_file(&key, FID, &blob).unwrap(), b"hello mahay");
    }

    #[test]
    fn check_block_round_trip_and_wrong_passphrase() {
        let (block, key) = CheckBlock::create("ny tenimiafina", 1000).unwrap();
        let block = CheckBlock::from_json(&block.to_json().unwrap()).unwrap();
        assert_eq!(*block.unlock("ny tenimiafina").unwrap().to_bytes(), *key.to_bytes());
        assert!(matches!(block.unlock("wrong"), Err(Error::BadPassphrase)));
    }

    fn sample(len: usize) -> Vec<u8> {
        (0..len).map(|i| (i * 31 % 251) as u8).collect()
    }

    #[test]
    fn round_trip_sizes() {
        let key = test_key();
        for len in [0, 1, CHUNK_SIZE - 1, CHUNK_SIZE, CHUNK_SIZE + 1, 3 * CHUNK_SIZE + 7] {
            let plain = sample(len);
            let blob = encrypt_file(&key, FID, &plain).unwrap();
            assert_eq!(decrypt_file(&key, FID, &blob).unwrap(), plain, "len {len}");
        }
    }

    #[test]
    fn wrong_key_or_file_id_rejected() {
        let blob = encrypt_file(&test_key(), FID, b"secret").unwrap();
        let other = Key::derive("other", b"0123456789abcdef", 1000).unwrap();
        assert!(matches!(decrypt_file(&other, FID, &blob), Err(Error::Decrypt)));
        assert!(matches!(decrypt_file(&test_key(), "ffffffffffffffffffffffffffffffff", &blob), Err(Error::Decrypt)));
    }

    #[test]
    fn tampered_reordered_truncated_rejected() {
        let key = test_key();
        let plain = sample(2 * CHUNK_SIZE + 100);
        let blob = encrypt_file(&key, FID, &plain).unwrap();
        let step = NONCE_LEN + CHUNK_SIZE + TAG_LEN;
        let (c0, c1, c2) = (&blob[4..4 + step], &blob[4 + step..4 + 2 * step], &blob[4 + 2 * step..]);

        let mut tampered = blob.clone();
        tampered[4 + step + 100] ^= 1;
        assert!(matches!(decrypt_file(&key, FID, &tampered), Err(Error::Decrypt)));

        let reordered = [MAGIC.as_slice(), c1, c0, c2].concat();
        assert!(matches!(decrypt_file(&key, FID, &reordered), Err(Error::Decrypt)));

        // cut at a chunk boundary: the new last chunk was not sealed as last
        let truncated = [MAGIC.as_slice(), c0, c1].concat();
        assert!(matches!(decrypt_file(&key, FID, &truncated), Err(Error::Decrypt)));
        // cut inside a chunk
        assert!(decrypt_file(&key, FID, &blob[..blob.len() - 1]).is_err());
        // last chunk dropped entirely after the first
        assert!(decrypt_file(&key, FID, &[MAGIC.as_slice(), c0].concat()).is_err());
        // chunks of another file
        let other = encrypt_file(&key, "ffffffffffffffffffffffffffffffff", &plain).unwrap();
        let mixed = [MAGIC.as_slice(), c0, &other[4 + step..4 + 2 * step], c2].concat();
        assert!(matches!(decrypt_file(&key, FID, &mixed), Err(Error::Decrypt)));
        assert!(matches!(decrypt_file(&key, FID, b"nope"), Err(Error::Format(_))));
    }

    #[test]
    fn nonces_differ() {
        let key = test_key();
        assert_ne!(encrypt_file(&key, FID, b"x").unwrap(), encrypt_file(&key, FID, b"x").unwrap());
    }
}
