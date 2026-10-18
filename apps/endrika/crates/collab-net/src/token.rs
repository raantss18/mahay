//! Session secrets from the operating system's random number generator.

use base64::Engine as _;

/// A session token: 128 random bits, URL-safe base64 (22 characters).
pub fn secret() -> Result<String, String> {
    let mut b = [0u8; 16];
    getrandom::fill(&mut b).map_err(|e| format!("no secure random numbers: {e}"))?;
    Ok(base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(b))
}

/// A file's session id: 64 random bits as hex (not a secret; it tells sessions apart).
pub fn file_id() -> Result<String, String> {
    let mut b = [0u8; 8];
    getrandom::fill(&mut b).map_err(|e| format!("no secure random numbers: {e}"))?;
    Ok(b.iter().map(|x| format!("{x:02x}")).collect())
}

/// 64 random bits (join codes are made from them).
pub fn random() -> Result<u64, String> {
    let mut b = [0u8; 8];
    getrandom::fill(&mut b).map_err(|e| format!("no secure random numbers: {e}"))?;
    Ok(u64::from_le_bytes(b))
}
