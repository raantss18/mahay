//! Which graphics backend an app starts with, and a way out when one does not work.
//!
//! wgpu can draw through Vulkan or OpenGL ES. On some devices the first fails (a Mali driver that
//! refuses the device, an emulator whose Vulkan cannot present) and on others asking for both breaks
//! the second: Vulkan probes the window, then OpenGL's `eglCreateWindowSurface` fails with
//! `BadAlloc` ("already connected to another API") and wgpu panics on an invalid surface. The app
//! then crashes before its first frame, at every start.
//!
//! So the choice is remembered: [`graphics_backend`] writes "`<backend> trying`" before the app
//! starts, and the first frame drawn ([`graphics_ok`]) turns it into "`<backend> ok`". A start that
//! finds "trying" knows the last one never drew, and moves on to the next backend in
//! [`next_after_failure`]. Android before 11 starts on OpenGL ES alone.

#[cfg(target_os = "android")]
use std::path::{Path, PathBuf};
#[cfg(target_os = "android")]
use std::sync::Mutex;

/// What wgpu may use.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Backend {
    /// Vulkan and OpenGL ES: wgpu picks (the default on Android 11 and later).
    Auto,
    /// OpenGL ES only.
    Gl,
    /// Vulkan only.
    Vulkan,
}

#[cfg_attr(not(target_os = "android"), allow(dead_code))]
impl Backend {
    fn name(self) -> &'static str {
        match self {
            Backend::Auto => "auto",
            Backend::Gl => "gl",
            Backend::Vulkan => "vulkan",
        }
    }

    fn parse(s: &str) -> Option<Self> {
        match s {
            "auto" => Some(Backend::Auto),
            "gl" => Some(Backend::Gl),
            "vulkan" => Some(Backend::Vulkan),
            _ => None,
        }
    }
}

/// The backend a first start uses on Android `sdk`.
pub fn default_backend(sdk: i32) -> Backend {
    if sdk < 30 { Backend::Gl } else { Backend::Auto }
}

/// The backend to try after `failed` never drew a frame.
pub fn next_after_failure(failed: Backend) -> Backend {
    match failed {
        Backend::Auto => Backend::Gl,
        Backend::Gl => Backend::Vulkan,
        Backend::Vulkan => Backend::Gl,
    }
}

/// The backend this start uses, from the marker left by the last one (`None`: first start or no file).
pub fn decide(marker: Option<&str>, sdk: i32) -> Backend {
    let Some(marker) = marker else { return default_backend(sdk) };
    let mut parts = marker.split_whitespace();
    match (parts.next().and_then(Backend::parse), parts.next()) {
        (Some(backend), Some("ok")) => backend,
        (Some(backend), Some("trying")) => next_after_failure(backend),
        _ => default_backend(sdk),
    }
}

#[cfg(target_os = "android")]
mod io {
    use super::*;

    pub(super) fn marker_path() -> Option<PathBuf> {
        crate::data_dir().map(|d| d.join("graphics-backend"))
    }

    fn read_marker(path: &Path) -> Option<String> {
        std::fs::read_to_string(path).ok()
    }

    static CHOSEN: Mutex<Option<Backend>> = Mutex::new(None);

    /// The backend to start with. Call once, before building the app's options.
    pub fn graphics_backend() -> Backend {
        let sdk = crate::sdk_int();
        let backend = match marker_path() {
            Some(path) => {
                let backend = decide(read_marker(&path).as_deref(), sdk);
                if let Err(e) = std::fs::write(&path, format!("{} trying", backend.name())) {
                    log::warn!("couldn't remember the graphics backend: {e}");
                }
                backend
            }
            None => default_backend(sdk),
        };
        log::info!("graphics: {} (Android {sdk})", backend.name());
        if let Ok(mut chosen) = CHOSEN.lock() {
            *chosen = Some(backend);
        }
        backend
    }

    /// The first frame was drawn: remember that this backend works.
    pub fn graphics_ok() {
        let chosen = CHOSEN.lock().ok().and_then(|c| *c);
        if let (Some(backend), Some(path)) = (chosen, marker_path()) {
            let _ = std::fs::write(path, format!("{} ok", backend.name()));
        }
    }
}

#[cfg(target_os = "android")]
pub use io::{graphics_backend, graphics_ok};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn android_before_11_starts_on_opengl_es() {
        assert_eq!(decide(None, 29), Backend::Gl);
        assert_eq!(decide(None, 26), Backend::Gl);
        assert_eq!(decide(None, 30), Backend::Auto);
        assert_eq!(decide(None, 36), Backend::Auto);
    }

    #[test]
    fn a_backend_that_drew_a_frame_stays() {
        assert_eq!(decide(Some("gl ok"), 36), Backend::Gl);
        assert_eq!(decide(Some("vulkan ok"), 29), Backend::Vulkan);
    }

    #[test]
    fn a_backend_that_never_drew_gives_way_to_the_next() {
        assert_eq!(decide(Some("auto trying"), 36), Backend::Gl);
        assert_eq!(decide(Some("gl trying"), 36), Backend::Vulkan);
        assert_eq!(decide(Some("vulkan trying"), 36), Backend::Gl);
    }

    #[test]
    fn a_damaged_marker_means_the_default() {
        assert_eq!(decide(Some(""), 36), Backend::Auto);
        assert_eq!(decide(Some("banana"), 29), Backend::Gl);
        assert_eq!(decide(Some("gl"), 36), Backend::Auto);
    }
}
