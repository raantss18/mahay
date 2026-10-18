//! Windows only: embed the app icon and version info (VERSIONINFO) into `tselatra.exe`.
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `TSELATRA_REQUIRE_WINRES=1` turns it
//! into an error (for release builds).

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/tselatra.ico");
    println!("cargo:rerun-if-env-changed=TSELATRA_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("../../assets/app-icon/tselatra.ico")
        .set("ProductName", "Tselatra")
        .set("FileDescription", "Tselatra motion graphics and visual effects")
        .set("LegalCopyright", "Copyright (c) 2026 The Tselatra contributors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "tselatra.exe")
        .set("InternalName", "tselatra");
    if let Err(e) = res.compile() {
        if std::env::var_os("TSELATRA_REQUIRE_WINRES").is_some() {
            panic!("embedding Windows resources failed: {e}");
        }
        println!("cargo:warning=tselatra.exe built without icon/version resources: {e}");
    }
}
