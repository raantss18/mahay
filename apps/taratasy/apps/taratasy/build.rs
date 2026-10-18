//! Windows only: embed the app icon and version info (VERSIONINFO) into `taratasy.exe`, so it
//! shows in Explorer, the taskbar, the Start menu and Alt-Tab. A separate PDF document icon
//! uses resource ID 2; the installer references it as `taratasy.exe,-2`.
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `TARATASY_REQUIRE_WINRES=1` turns it
//! into an error (for release builds).

#[path = "src/windows_manifest.rs"]
mod windows_manifest;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/taratasy.ico");
    println!("cargo:rerun-if-changed=../../assets/app-icon/taratasy-document.ico");
    println!("cargo:rerun-if-changed=src/windows_manifest.rs");
    println!("cargo:rerun-if-env-changed=TARATASY_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("../../assets/app-icon/taratasy.ico")
        .set_icon_with_id("../../assets/app-icon/taratasy-document.ico", "2")
        .set_manifest(windows_manifest::WINDOWS_MANIFEST)
        .set("ProductName", "Taratasy")
        .set("FileDescription", "Taratasy PDF workbench")
        .set("LegalCopyright", "Copyright (c) the Taratasy contributors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "taratasy.exe")
        .set("InternalName", "taratasy");
    if let Err(e) = res.compile() {
        if std::env::var_os("TARATASY_REQUIRE_WINRES").is_some() {
            println!("cargo::error=embedding Windows resources failed: {e}");
            return;
        }
        println!("cargo:warning=taratasy.exe built without icon/version resources: {e}");
    }
}
