//! Soratra on Android. The desktop app's file dialogs (rfd, with the Android backend vendored in
//! `libs/core/vendor/rfd`), clipboard (arboard, Android text backend) and region modules are
//! reused as they are; only the window, printing and desktop integration are left out.

#[cfg(target_os = "android")]
#[path = "../../soratra/src/file_dialogs.rs"]
mod file_dialogs;
#[cfg(target_os = "android")]
#[path = "../../soratra/src/clipboard.rs"]
mod clipboard;
#[cfg(target_os = "android")]
#[path = "../../soratra/src/region.rs"]
mod region;
#[cfg(target_os = "android")]
mod android;
