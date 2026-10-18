//! Taratasy on Android. The file dialogs go through rfd's Android backend (vendored in
//! `libs/core/vendor/rfd`), which uses the system document picker; `mahay-android` copies saves
//! back to the picked documents.

#[cfg(target_os = "android")]
mod android;
