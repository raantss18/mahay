//! Seho on Android. Presentations open through the system document picker by path, so saves are
//! copied back to the picked file; pictures, audio and video are read into the app's inbox as the
//! web build does. Sound plays through the desktop app's cpal output (AAudio on Android).

#[cfg(target_os = "android")]
mod android;
#[cfg(target_os = "android")]
#[path = "../../seho/src/audio.rs"]
mod audio;
