//! Windows only: embed the app icon and version info (VERSIONINFO) into `hazavana.exe`, so
//! Explorer, the taskbar, the Start menu and Alt-Tab show the lynx.
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `HAZAVANA_REQUIRE_WINRES=1` turns it
//! into an error (for release builds).

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/hazavana.ico");
    println!("cargo:rerun-if-env-changed=HAZAVANA_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("../../assets/app-icon/hazavana.ico")
        .set("ProductName", "Hazavana")
        .set("FileDescription", "Hazavana photo library and raw developer")
        .set("LegalCopyright", "Copyright (c) the Hazavana authors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "hazavana.exe")
        .set("InternalName", "hazavana");
    if let Err(e) = res.compile() {
        if std::env::var_os("HAZAVANA_REQUIRE_WINRES").is_some() {
            panic!("embedding Windows resources failed: {e}");
        }
        println!("cargo:warning=hazavana.exe built without icon/version resources: {e}");
    }
}
