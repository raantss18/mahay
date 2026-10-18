//! Tselatra on Android. The commands' file dialogs are non-blocking (as on Linux and macOS) and
//! go through rfd's Android backend on a thread of their own; sound plays through the desktop
//! app's cpal output (AAudio). Folder pickers aren't available on Android yet.

#[cfg(target_os = "android")]
#[path = "../../tselatra/src/audio_out.rs"]
mod audio_out;
#[cfg(target_os = "android")]
mod android;
