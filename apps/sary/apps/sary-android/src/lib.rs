//! Sary on Android. The desktop app's platform services (file dialogs through rfd's Android
//! backend, import/export, autosave, presets) are reused from `apps/sary/src`; the window,
//! single-instance, monitor profiles, tablets drivers and printing are left out.

#[cfg(target_os = "android")]
#[path = "../../sary/src/services.rs"]
mod services;
#[cfg(target_os = "android")]
#[path = "../../sary/src/app_dirs.rs"]
mod app_dirs;
#[cfg(target_os = "android")]
#[path = "../../sary/src/appearance.rs"]
mod appearance;
#[cfg(target_os = "android")]
#[path = "../../sary/src/crash_guard.rs"]
mod crash_guard;
#[cfg(target_os = "android")]
#[path = "../../sary/src/file_dialog_probe.rs"]
mod file_dialog_probe;
#[cfg(target_os = "android")]
#[path = "../../sary/src/gpu_startup.rs"]
mod gpu_startup;
#[cfg(target_os = "android")]
#[path = "../../sary/src/photoshop_settings.rs"]
mod photoshop_settings;
#[cfg(target_os = "android")]
mod android;
