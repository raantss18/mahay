//! SVG through the file commands: imported centred where it's pasted, with warnings for what it
//! left out, opened with them, undone in one step; clipboard text recognised as SVG.

use base64::Engine as _;
use serde_json::json;

use crate::Session;
use crate::params::id_of;

const SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="40" height="20"><filter id="noise"><feTurbulence baseFrequency="0.1"/></filter><rect id="Card" width="40" height="20" rx="4" fill="#3366cc"/><circle cx="10" cy="10" r="5" filter="url(#noise)"/></svg>"##;

fn data(svg: &str) -> String {
    base64::engine::general_purpose::STANDARD.encode(svg)
}

#[test]
fn imported_svg_is_editable_centred_and_reports_what_it_left_out() {
    let mut s = Session::new();
    let r = s.execute("file.import", &json!({"data": data(SVG), "name": "Pasted SVG.svg", "x": 100, "y": 50, "center": true})).unwrap();
    let id = id_of(&r["id"]).unwrap();
    assert_eq!(s.selection(), [id]);
    let doc = s.doc().unwrap();
    let frame = doc.node(id).unwrap();
    assert_eq!((frame.name.as_str(), frame.width, frame.height), ("Pasted SVG", 40.0, 20.0));
    let o = doc.world_transform(id).translation();
    assert_eq!((o.x, o.y), (80.0, 40.0), "centred on the point");
    let card = doc.subtree(id).into_iter().filter_map(|i| doc.node(i)).find(|n| n.name == "Card").unwrap();
    assert_eq!((card.kind.type_name(), card.corners.radii[0]), ("RECTANGLE", 4.0));
    assert!(r["warnings"][0].as_str().is_some_and(|w| w.contains("noise")), "{r}");
    // Top-left at the point without `center`; one undo takes it away.
    let r = s.execute("file.import", &json!({"data": data(SVG), "name": "x.svg", "x": 100, "y": 50})).unwrap();
    let id2 = id_of(&r["id"]).unwrap();
    assert_eq!(s.doc().unwrap().world_transform(id2).translation().x, 100.0);
    s.execute("edit.undo", &json!({})).unwrap();
    assert!(s.doc().unwrap().node(id2).is_none());
    // Not SVG: an error, nothing added.
    let before = s.doc().unwrap().node_count();
    assert!(s.execute("file.import", &json!({"data": data("<svg"), "name": "bad.svg"})).is_err());
    assert_eq!(s.doc().unwrap().node_count(), before);
}

#[test]
fn opened_svg_lists_its_warnings() {
    let dir = std::env::temp_dir().join(format!("figcraft-svg-open-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("icon.svg");
    std::fs::write(&path, SVG).unwrap();
    let mut s = Session::new();
    let r = s.execute("file.open", &json!({"path": path.to_string_lossy()})).unwrap();
    assert!(r["warnings"].as_array().is_some_and(|w| w.len() == 1), "{r}");
    assert!(r.get("repairs").is_none(), "{r}");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn clipboard_text_is_recognised_as_svg() {
    for yes in [
        "<svg/>",
        "  \n<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>",
        "\u{feff}<?xml version=\"1.0\"?>\n<!-- Generator: x -->\n<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"x.dtd\">\n<svg>",
    ] {
        assert!(crate::looks_like_svg(yes), "{yes}");
    }
    for no in ["", "svg", "<html><svg/></html>", "<?xml version=\"1.0\"?><note/>", "<!-- unclosed", "Hello <svg>"] {
        assert!(!crate::looks_like_svg(no), "{no}");
    }
}
