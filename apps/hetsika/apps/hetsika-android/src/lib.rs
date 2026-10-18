//! Hetsika on Android. File dialogs run on their own thread (as on Linux) through rfd's Android
//! backend; sound plays and records through the desktop app's cpal modules (AAudio). Folder
//! pickers aren't available on Android yet.

#[cfg(target_os = "android")]
#[path = "../../hetsika/src/audio.rs"]
mod audio;
#[cfg(target_os = "android")]
#[path = "../../hetsika/src/audio_in.rs"]
mod audio_in;
#[cfg(target_os = "android")]
mod android;
