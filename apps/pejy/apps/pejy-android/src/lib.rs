//! Pejy on Android. Documents open through the system document picker (by path, on a later
//! frame, so saves go back to the picked file); Save As writes into the app's storage, then asks
//! where to put the document. Crash recovery lives in the app's private storage.

#[cfg(target_os = "android")]
mod android;
