//! The version history in `.figcraft` files: round trips, files without versions, damaged entries.

use std::io::Write;
use std::sync::Arc;

use figcraft_doc::{Document, NodeKind};

use super::*;

fn png(r: u8) -> Vec<u8> {
    let img = image::RgbaImage::from_pixel(2, 2, image::Rgba([r, 0, 0, 255]));
    let mut out = Vec::new();
    img.write_to(&mut std::io::Cursor::new(&mut out), image::ImageFormat::Png).unwrap();
    out
}

fn zip_of(members: &[(&str, &[u8])]) -> Vec<u8> {
    let mut buf = std::io::Cursor::new(Vec::new());
    let mut z = zip::ZipWriter::new(&mut buf);
    for (name, bytes) in members {
        z.start_file(*name, zip::write::SimpleFileOptions::default()).unwrap();
        z.write_all(bytes).unwrap();
    }
    z.finish().unwrap();
    buf.into_inner()
}

fn doc_json(name: &str) -> Vec<u8> {
    serde_json::to_vec(&to_json(&Document::new(name)).unwrap()).unwrap()
}

#[test]
fn roundtrip_with_versions() {
    // The version has an image the current document no longer uses, and vice versa.
    let mut old = Document::new("File");
    let p = old.first_page().unwrap();
    old.create(p, NodeKind::Rectangle, "R", 0.0, 0.0, 10.0, 10.0).unwrap();
    let (old_img, ..) = add_image(&mut old, png(1)).unwrap();
    let mut cur = Document::new("File");
    let (cur_img, ..) = add_image(&mut cur, png(2)).unwrap();
    let versions = vec![Version::new("v1", "First", "with a box", 1_700_000_000, Arc::new(old.clone()))];

    let bytes = save_bytes_with_versions(&cur, None, &versions).unwrap();
    let l = load_bytes_with_versions(&bytes).unwrap();
    assert!(l.issues.is_empty(), "{:?}", l.issues);
    assert_eq!(l.versions.len(), 1);
    let v = &l.versions[0];
    assert_eq!((v.id.as_str(), v.name.as_str(), v.description.as_str(), v.time), ("v1", "First", "with a box", 1_700_000_000));
    let snap = v.document().unwrap();
    assert_eq!(snap.node_count(), old.node_count());
    assert!(snap.images.contains_key(&old_img));
    // Images only the history uses stay out of the document.
    assert!(l.doc.images.contains_key(&cur_img));
    assert!(!l.doc.images.contains_key(&old_img));

    // Saving what was read (not yet parsed) keeps it intact.
    let again = load_bytes_with_versions(&save_bytes_with_versions(&l.doc, None, &l.versions).unwrap()).unwrap();
    assert_eq!(again.versions.len(), 1);
    assert_eq!(again.versions[0].document().unwrap().node_count(), old.node_count());
    assert!(again.versions[0].images().contains_key(&old_img));

    // Readers that ignore versions still open the file.
    let (plain, _) = load_bytes(&bytes).unwrap();
    assert_eq!(plain.node_count(), cur.node_count());
}

#[test]
fn files_without_versions_load_unchanged() {
    let mut d = Document::new("Old");
    let (h, ..) = add_image(&mut d, png(3)).unwrap();
    let l = load_bytes_with_versions(&save_bytes(&d, None).unwrap()).unwrap();
    assert!(l.versions.is_empty() && l.issues.is_empty());
    assert!(l.doc.images.contains_key(&h));
    // A minimal archive (document.json only) and bare JSON.
    assert!(load_bytes_with_versions(&zip_of(&[("document.json", &doc_json("Min"))])).unwrap().versions.is_empty());
    assert_eq!(load_bytes_with_versions(&doc_json("Bare")).unwrap().doc.name, "Bare");
}

#[test]
fn damaged_versions_never_fail_the_load() {
    let doc = doc_json("D");
    // An unreadable index: the file opens without a history.
    let l = load_bytes_with_versions(&zip_of(&[("document.json", &doc), ("versions/index.json", b"{nope")])).unwrap();
    assert!(l.versions.is_empty());
    assert_eq!(l.issues.len(), 1);

    // Bad ids, a missing snapshot, duplicates, junk fields and a damaged snapshot.
    let index = serde_json::json!({"versions": [
        {"id": "../escape", "name": "bad id"},
        {"id": 7},
        {"id": "missing", "name": "no member"},
        {"id": "ok", "name": "Fine", "description": 5, "time": "soon", "images": [1, "unknown-hash"]},
        {"id": "ok", "name": "duplicate"},
        {"id": "broken", "name": "Broken"},
    ]});
    let bytes = zip_of(&[
        ("document.json", &doc),
        ("versions/index.json", &serde_json::to_vec(&index).unwrap()),
        ("versions/ok.json", &doc_json("Snap")),
        ("versions/broken.json", b"\xff\xfe not json"),
    ]);
    let l = load_bytes_with_versions(&bytes).unwrap();
    let names: Vec<&str> = l.versions.iter().map(|v| v.name.as_str()).collect();
    assert_eq!(names, ["Fine", "Broken"]);
    assert_eq!(l.issues.len(), 4, "{:?}", l.issues);
    let ok = &l.versions[0];
    assert_eq!((ok.description.as_str(), ok.time), ("", 0));
    assert!(ok.images().is_empty());
    assert_eq!(ok.document().unwrap().name, "Snap");
    // The damaged snapshot fails only when used.
    assert!(l.versions[1].document().is_err());
    // A snapshot from a newer format is refused too.
    let newer = zip_of(&[
        ("document.json", &doc),
        ("versions/index.json", br#"{"versions":[{"id":"n","name":"Newer"}]}"#),
        ("versions/n.json", br#"{"version": 999}"#),
    ]);
    assert!(matches!(load_bytes_with_versions(&newer).unwrap().versions[0].document(), Err(FormatError::Newer(999))));
}

#[test]
fn oversized_and_excess_versions_are_skipped() {
    let doc = doc_json("D");
    // Member size guard.
    let bytes = zip_of(&[("big", &[0u8; 64])]);
    let mut z = zip::ZipArchive::new(std::io::Cursor::new(bytes)).unwrap();
    assert!(read_member(&mut z, "big", 63).is_err());
    assert_eq!(read_member(&mut z, "big", 64).unwrap().len(), 64);
    assert!(read_member(&mut z, "absent", 64).is_err());

    // More entries than the cap: the first MAX_VERSIONS are kept.
    let n = MAX_VERSIONS + 3;
    let entries: Vec<serde_json::Value> = (0..n).map(|i| serde_json::json!({"id": format!("v{i}"), "name": format!("V{i}")})).collect();
    let index = serde_json::to_vec(&serde_json::json!({ "versions": entries })).unwrap();
    let names: Vec<String> = (0..n).map(|i| format!("versions/v{i}.json")).collect();
    let mut members: Vec<(&str, &[u8])> = vec![("document.json", &doc), ("versions/index.json", &index)];
    members.extend(names.iter().map(|m| (m.as_str(), doc.as_slice())));
    let l = load_bytes_with_versions(&zip_of(&members)).unwrap();
    assert_eq!(l.versions.len(), MAX_VERSIONS);
    assert_eq!(l.issues.len(), 1);
}

#[test]
fn saving_refuses_bad_version_ids() {
    let d = Arc::new(Document::new("x"));
    assert!(save_bytes_with_versions(&d, None, &[Version::new("a/b", "n", "", 0, d.clone())]).is_err());
    assert!(save_bytes_with_versions(&d, None, &[Version::new("a", "n", "", 0, d.clone()), Version::new("a", "m", "", 0, d.clone())]).is_err());
    // Names and descriptions are clipped.
    let v = Version::new("a", &"n".repeat(MAX_VERSION_NAME + 10), &"é".repeat(MAX_VERSION_DESCRIPTION + 1), 0, d);
    assert_eq!(v.name.chars().count(), MAX_VERSION_NAME);
    assert_eq!(v.description.chars().count(), MAX_VERSION_DESCRIPTION);
}
