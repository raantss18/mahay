//! The suite's version (the repository's `VERSION` file) as `MAHAY_VERSION`.
fn main() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../VERSION");
    println!("cargo:rerun-if-changed={path}");
    let version = std::fs::read_to_string(path).map(|v| v.trim().to_string()).unwrap_or_else(|_| "0.0.0".into());
    println!("cargo:rustc-env=MAHAY_VERSION={version}");
}
