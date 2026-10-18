//! Endrika on Android. The desktop app's services (file dialogs through rfd's Android backend,
//! clipboard through arboard's, preferences, crash recovery) are reused from `apps/endrika/src`;
//! the control channel, Wayland/X11 handling and the native title bar are left out.

#[cfg(target_os = "android")]
#[path = "../../endrika/src/services.rs"]
mod services;
#[cfg(target_os = "android")]
#[path = "../../endrika/src/recovery.rs"]
mod recovery;
#[cfg(target_os = "android")]
#[path = "../../endrika/src/gamepad.rs"]
mod gamepad;
#[cfg(target_os = "android")]
mod android;
