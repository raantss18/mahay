//! Hazavana on Android. The photo library lives in the app's private storage; photos are
//! imported through the system picker (rfd's Android backend, on the app's own dialog thread);
//! exports are saved where the user chooses. Folder pickers and external editors aren't
//! available on Android yet.

#[cfg(target_os = "android")]
mod android;
