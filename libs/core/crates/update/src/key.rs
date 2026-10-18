//! Mahay's update-signing public key (Ed25519). Its private half is `release-keys/update-signing.pem`,
//! kept off the repository; the PEM public key is `.github/update-signing.pub` (base64:
//! `RGucVJyiOeXOMjDj1NMEq8zvdq0biHqdkv2ksDsseXk=`). Changing it locks every installed app out of
//! updates: never rotate it without shipping both keys first.

/// The 32-byte Ed25519 public key that must sign `SHA256SUMS.txt`.
pub const PUBLIC_KEY: [u8; 32] = [68, 107, 156, 84, 156, 162, 57, 229, 206, 50, 48, 227, 212, 211, 4, 171, 204, 239, 118, 173, 27, 136, 122, 157, 146, 253, 164, 176, 59, 44, 121, 121];
