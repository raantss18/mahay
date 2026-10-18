//! Optional mahay-fonts build input (https://github.com/raantss18/mahay, recipe from its
//! `docs/integration.md`; rules in `mahay-rules/standards/fonts.md`).
//!
//! With `MAHAY_FONTS_DIR=<mahay-fonts checkout>`, every font in its `fonts/manifest.txt` is
//! embedded and exposed as `sary_text::MAHAY_FONTS`; unset, `MAHAY_FONTS` is empty and the
//! build is exactly as before (an empty value counts as unset). Use an absolute path: build
//! scripts run in the crate's directory, so a relative one resolves from `crates/text`. A bad
//! checkout is a warning, or an error with `MAHAY_FONTS_REQUIRED=1` (release builds).
//!
//! Sary difference from the recipe: the web build (wasm32) embeds nothing. The wasm must
//! stay under the 24 MiB gate in `packaging/web/package.sh` (#237; Cloudflare's per-file cap is
//! 25 MiB). Measured 2026-10-06 (`trunk build --release`): 24,190,076 bytes
//! without mahay-fonts, 28,867,488 with only the UI face (BIZ UDPGothic Regular) embedded, over
//! the 25,165,824-byte gate. The web build loads fonts served next to the wasm instead, on
//! demand (`fonts/manifest.txt`, see `served.rs` and packaging/web/README.md › Fonts).
//!
//! Also deflates the bundled UI fonts (`assets/fonts/*.ttf`, `fonts::BUNDLED`) into
//! `OUT_DIR/fonts/<file>.deflate`: 0.73 MB of binary instead of 1.5 MB, for the same size gate.
use std::fmt::Write as _;
use std::path::PathBuf;

fn main() {
    println!("cargo::rerun-if-env-changed=MAHAY_FONTS_DIR");
    println!("cargo::rerun-if-env-changed=MAHAY_FONTS_REQUIRED");
    let wasm = std::env::var("CARGO_CFG_TARGET_ARCH").is_ok_and(|a| a == "wasm32");
    let mut src = String::from("pub static MAHAY_FONTS: &[MahayFont] = &[\n");
    if let Some(dir) = std::env::var_os("MAHAY_FONTS_DIR").filter(|d| !d.is_empty() && !wasm).map(PathBuf::from) {
        match mahay_fonts(&dir) {
            Ok(entries) => src.push_str(&entries),
            Err(e) if std::env::var_os("MAHAY_FONTS_REQUIRED").is_some() => {
                println!("cargo::error=MAHAY_FONTS_DIR={}: {e}", dir.display());
            }
            Err(e) => println!("cargo::warning=building without mahay-fonts: MAHAY_FONTS_DIR={}: {e}", dir.display()),
        }
    }
    src.push_str("];\n");
    let out = PathBuf::from(std::env::var_os("OUT_DIR").unwrap_or_default()).join("mahay_fonts.rs");
    if let Err(e) = std::fs::write(&out, src) {
        println!("cargo::error=writing {}: {e}", out.display());
    }
    deflate_bundled_fonts();
}

/// Deflate the bundled fonts into `OUT_DIR/fonts/` (`fonts::BUNDLED` includes them).
fn deflate_bundled_fonts() {
    use std::io::Write as _;
    let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/fonts");
    let out = PathBuf::from(std::env::var_os("OUT_DIR").unwrap_or_default()).join("fonts");
    let result = (|| -> std::io::Result<()> {
        std::fs::create_dir_all(&out)?;
        for name in ["Inter-Regular.ttf", "Inter-Medium.ttf", "Inter-SemiBold.ttf", "JetBrainsMono-Regular.ttf"] {
            let path = src.join(name);
            println!("cargo::rerun-if-changed={}", path.display());
            let mut z = flate2::write::DeflateEncoder::new(Vec::new(), flate2::Compression::best());
            z.write_all(&std::fs::read(&path)?)?;
            std::fs::write(out.join(format!("{name}.deflate")), z.finish()?)?;
        }
        Ok(())
    })();
    if let Err(e) = result {
        println!("cargo::error=deflating the bundled fonts in {}: {e}", src.display());
    }
}

/// One `MahayFont { .. }` initialiser per manifest line.
fn mahay_fonts(dir: &std::path::Path) -> Result<String, String> {
    let manifest = dir.join("fonts/manifest.txt");
    println!("cargo::rerun-if-changed={}", manifest.display());
    let text = std::fs::read_to_string(&manifest).map_err(|e| format!("{}: {e}", manifest.display()))?;
    let mut out = String::new();
    for line in text.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')) {
        let f: Vec<&str> = line.split(" | ").map(str::trim).collect();
        let [family, style, file, scripts, ..] = f.as_slice() else {
            return Err(format!("malformed manifest line: {line}"));
        };
        let path = dir.join(file).canonicalize().map_err(|e| format!("{file}: {e}"))?;
        println!("cargo::rerun-if-changed={}", path.display());
        let scripts: Vec<String> = scripts.split(',').map(|s| format!("{:?}", s.trim())).collect();
        let _ = writeln!(
            out,
            "    MahayFont {{ family: {family:?}, style: {style:?}, scripts: &[{}], bytes: include_bytes!({:?}) }},",
            scripts.join(", "),
            path.display().to_string(),
        );
    }
    Ok(out)
}
