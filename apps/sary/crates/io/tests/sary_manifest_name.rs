//! A `.sary` bundle's manifest reports the file it is in (#3312): writing a document to a new
//! name must not leave the previous one inside, on every path that produces `.sary` bytes.

mod common;

use common::*;
use sary_color::{ColorMode, SampleType};
use sary_format::zip::ZipReader;
use sary_io::*;

fn doc() -> sary_doc::Document {
    let mut d = gen_doc(ColorMode::Rgb, SampleType::U8, Features::PIXELS);
    d.name = "FirstTest.psd".into();
    d
}

/// `document.name` as the bundle actually stores it.
fn manifest_name(bytes: &[u8]) -> String {
    let archive = ZipReader::new(bytes).expect("zip");
    let manifest: serde_json::Value = serde_json::from_slice(&archive.read_by_name("manifest.json", usize::MAX).expect("manifest")).expect("manifest json");
    manifest["document"]["name"].as_str().expect("document.name").to_string()
}

/// Save As to a new name: the written file names itself, so reopening it agrees with the session
/// that wrote it instead of reporting the file it came from.
#[test]
fn export_names_the_manifest_after_the_file() {
    let r = export(&doc(), "/tmp/somewhere/_nametest.sary", &ExportOptions::default()).expect("export");
    assert_eq!(manifest_name(&r.bytes), "_nametest.sary");
    assert_eq!(sary_format::load_from_bytes(&r.bytes).expect("load").name, "_nametest.sary");
}

/// A bare extension names no file and leaves the document's own name alone.
#[test]
fn a_bare_extension_names_no_file() {
    let r = export(&doc(), "sary", &ExportOptions::default()).expect("export");
    assert_eq!(manifest_name(&r.bytes), "FirstTest.psd");
}
