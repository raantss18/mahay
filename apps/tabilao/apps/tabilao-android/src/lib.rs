//! Tabilao on Android. Desktop-only services (printers, the XDG clipboard, rfd) are replaced by
//! `mahay-android`: the system document picker, write-back to the picked document, and the
//! Android clipboard.

#[cfg(target_os = "android")]
mod android;
