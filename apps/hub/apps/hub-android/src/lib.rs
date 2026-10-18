//! Mahay HUB on Android: a light hub. Each Mahay app is its own APK (users install only what
//! they use); Mahay HUB lists the suite, opens the installed apps and links to the release page for
//! the others. The desktop launcher's installer and updater don't apply on Android, where the
//! system (or a store) installs and updates apps.

#[cfg(target_os = "android")]
mod android;
#[cfg(target_os = "android")]
mod text;
