//! The Supabase project's public values (`release-keys/accounts.env`, exported by the build scripts).
fn main() {
    for k in ["MAHAY_SUPABASE_URL", "MAHAY_SUPABASE_KEY"] {
        println!("cargo:rerun-if-env-changed={k}");
        println!("cargo:rustc-env={k}={}", std::env::var(k).unwrap_or_default().trim());
    }
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../VERSION");
    println!("cargo:rerun-if-changed={path}");
    let version = std::fs::read_to_string(path).map(|v| v.trim().to_string()).unwrap_or_else(|_| "0.0.0".into());
    println!("cargo:rustc-env=MAHAY_VERSION={version}");
}
