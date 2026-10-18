//! Windows only: embed version info (VERSIONINFO) into `taratasy-cli.exe`, so Explorer's
//! Details tab, `(Get-Item taratasy-cli.exe).VersionInfo` and inventory tools can read the version
//! and publisher, as they can for `taratasy.exe` (`apps/taratasy/build.rs`).
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `TARATASY_REQUIRE_WINRES=1` turns it
//! into an error (for release builds).

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=TARATASY_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set("ProductName", "Taratasy")
        .set("FileDescription", "Taratasy command-line tool")
        .set("LegalCopyright", "Copyright (c) the Taratasy contributors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "taratasy-cli.exe")
        .set("InternalName", "taratasy-cli");
    if let Err(e) = res.compile() {
        if std::env::var_os("TARATASY_REQUIRE_WINRES").is_some() {
            println!("cargo::error=embedding Windows resources failed: {e}");
            return;
        }
        println!("cargo:warning=taratasy-cli.exe built without version resources: {e}");
    }
}
