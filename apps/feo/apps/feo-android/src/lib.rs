//! Feo on Android. Sessions, audio and MIDI open through the system document picker (opened by
//! path on a later frame, so saved sessions go back to the picked file); sound plays through
//! cpal (AAudio). Folder pickers aren't available on Android yet.

#[cfg(target_os = "android")]
mod android;
