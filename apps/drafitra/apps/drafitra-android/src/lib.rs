//! Drafitra on Android. Open goes through the system document picker (the answer arrives on a
//! later frame and is opened by path, so saves are copied back to the picked file); Save As
//! writes into the app's storage, then asks where to put the drawing.

#[cfg(target_os = "android")]
mod android;
