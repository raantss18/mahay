//! Windows only: embed the app icon and version info into `drafitra.exe`. Elsewhere a no-op.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/drafitra.ico");
    println!("cargo:rerun-if-env-changed=DRAFITRA_REQUIRE_WINRES");
    println!("cargo:rerun-if-env-changed=DRAFITRA_BUILD_SHA");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    if std::path::Path::new("../../assets/app-icon/drafitra.ico").exists() {
        res.set_icon("../../assets/app-icon/drafitra.ico");
    }
    res.set("ProductName", "Drafitra")
        .set("FileDescription", "Drafitra computer-aided design")
        .set("CompanyName", "Learning Machines LLC")
        .set("LegalCopyright", "Copyright (c) 2026 Mahay Team and the Drafitra contributors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "drafitra.exe")
        .set("InternalName", "drafitra");
    if let Err(e) = res.compile() {
        if std::env::var_os("DRAFITRA_REQUIRE_WINRES").is_some() {
            eprintln!("embedding Windows resources failed: {e}");
            std::process::exit(1);
        }
        println!("cargo:warning=drafitra.exe built without icon/version resources: {e}");
    }
}
