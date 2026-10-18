//! Branches in `.figcraft` files: round trips, files from before branches, damaged entries.

use std::io::Write;
use std::sync::Arc;

use figcraft_doc::{Document, NodeKind};

use super::*;

fn png() -> Vec<u8> {
    let img = image::RgbaImage::from_pixel(2, 2, image::Rgba([9, 0, 0, 255]));
    let mut out = Vec::new();
    img.write_to(&mut std::io::Cursor::new(&mut out), image::ImageFormat::Png).unwrap();
    out
}

fn branch(base: &Document) -> Branch {
    let mut head = base.clone();
    let p = head.first_page().unwrap();
    head.create(p, NodeKind::Rectangle, "On the branch", 0.0, 0.0, 10.0, 10.0).unwrap();
    add_image(&mut head, png()).unwrap();
    Branch {
        id: "b1".into(),
        name: "Explore".into(),
        time: 7,
        client: 3,
        state: BranchState::Merged,
        base: Arc::new(base.clone()),
        versions: vec![Version::new("v1", "On its own", "", 9, Arc::new(base.clone()))],
        head: Arc::new(head),
    }
}

#[test]
fn branches_round_trip_with_their_images() {
    let doc = Document::new("File");
    let b = branch(&doc);
    let bytes = save_bytes_with_history(&doc, None, &[], std::slice::from_ref(&b)).unwrap();
    let l = load_bytes_with_versions(&bytes).unwrap();
    assert!(l.issues.is_empty(), "{:?}", l.issues);
    let got = l.branches.first().unwrap();
    assert_eq!((got.id.as_str(), got.name.as_str(), got.time, got.client, got.state), ("b1", "Explore", 7, 3, BranchState::Merged));
    assert_eq!(*got.base, *b.base);
    assert_eq!(*got.head, *b.head);
    // Its own version history comes back with it.
    let v = got.versions.first().unwrap();
    assert_eq!((got.versions.len(), v.id.as_str(), v.name.as_str(), v.time), (1, "v1", "On its own", 9));
    assert_eq!(*v.document().unwrap(), *b.base);
    assert!(l.versions.is_empty(), "the branch's versions aren't the file's");
    // The image only the branch uses stays out of the document.
    assert!(l.doc.images.is_empty() && got.head.images.len() == 1);
    // Duplicate ids are refused when saving.
    assert!(save_bytes_with_history(&doc, None, &[], &[b.clone(), b]).is_err());
}

#[test]
fn files_without_branches_and_damaged_branches_load() {
    let doc = Document::new("Old");
    // A file from before branches (and one saved without any) has none.
    let l = load_bytes_with_versions(&save_bytes(&doc, None).unwrap()).unwrap();
    assert!(l.branches.is_empty() && l.issues.is_empty());
    // A branch whose snapshot is missing is skipped and reported; the file still opens.
    let mut buf = std::io::Cursor::new(Vec::new());
    {
        let mut z = zip::ZipWriter::new(&mut buf);
        let opts = zip::write::SimpleFileOptions::default();
        z.start_file("document.json", opts).unwrap();
        z.write_all(&serde_json::to_vec(&to_json(&doc).unwrap()).unwrap()).unwrap();
        z.start_file(BRANCH_INDEX, opts).unwrap();
        z.write_all(br#"{"branches": [{"id": "gone", "name": "x"}, {"id": "../bad"}]}"#).unwrap();
        z.finish().unwrap();
    }
    let l = load_bytes_with_versions(&buf.into_inner()).unwrap();
    assert!(l.branches.is_empty());
    assert_eq!(l.issues.len(), 2, "{:?}", l.issues);
    assert_eq!(l.doc.name, "Old");
}
