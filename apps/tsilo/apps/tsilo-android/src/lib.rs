//! Tsilo on Android. File dialogs run on their own thread (as on Linux) through rfd's Android
//! backend; the clipboard module is the desktop app's (arboard's Android text backend).

#[cfg(target_os = "android")]
#[path = "../../tsilo/src/clipboard.rs"]
mod clipboard;
#[cfg(target_os = "android")]
mod android;
