//! Frame guides: guides kept on a frame in its own coordinates, moving with it, saved with the file,
//! copied with the frame, and snapped to by the layers inside it.

use figcraft_doc::NodeId;
use figcraft_geom::Rect;
use serde_json::json;

use crate::Session;
use crate::snap::Targets;

fn id(v: serde_json::Value) -> NodeId {
    NodeId(v["id"].as_u64().unwrap())
}

fn guides(s: &Session, owner: NodeId) -> Vec<(bool, f64)> {
    s.doc().unwrap().node(owner).unwrap().guides().iter().map(|g| (g.vertical, g.offset)).collect()
}

/// A frame at (100, 50) with a child rectangle.
fn setup() -> (Session, NodeId, NodeId) {
    let mut s = Session::new();
    let frame = id(s.execute("node.create", &json!({"type": "frame", "x": 100, "y": 50, "width": 300, "height": 200})).unwrap());
    let rect = id(s.execute("node.create", &json!({"type": "rectangle", "parent": frame, "x": 10, "y": 10, "width": 20, "height": 20})).unwrap());
    (s, frame, rect)
}

#[test]
fn guides_on_a_frame_are_kept_in_its_coordinates() {
    let (mut s, frame, _) = setup();
    let r = s.execute("guide.add", &json!({"frame": frame, "vertical": true, "offset": 40})).unwrap();
    assert_eq!(r["index"], json!(0));
    s.execute("guide.add", &json!({"frame": frame, "vertical": false, "offset": 25.5})).unwrap();
    assert_eq!(guides(&s, frame), vec![(true, 40.0), (false, 25.5)]);
    // The page's guides are separate.
    let page = s.page().unwrap();
    assert!(guides(&s, page).is_empty());
    s.execute("guide.move", &json!({"frame": frame, "index": 0, "offset": 60})).unwrap();
    assert_eq!(guides(&s, frame)[0], (true, 60.0));
    s.undo().unwrap();
    assert_eq!(guides(&s, frame)[0], (true, 40.0));
    s.execute("guide.remove", &json!({"frame": frame, "index": 1})).unwrap();
    assert_eq!(guides(&s, frame), vec![(true, 40.0)]);
    s.execute("guide.clear", &json!({"frame": frame})).unwrap();
    assert!(guides(&s, frame).is_empty());
}

#[test]
fn only_pages_and_frames_take_guides() {
    let (mut s, frame, rect) = setup();
    for bad in [
        json!({"frame": rect, "offset": 5}),
        json!({"frame": 999_999, "offset": 5}),
        json!({"frame": frame, "offset": "x"}),
        json!({"page": rect, "offset": 1}),
    ] {
        assert!(s.execute("guide.add", &bad).is_err(), "{bad}");
    }
    assert!(s.execute("guide.move", &json!({"frame": frame, "index": 0, "offset": 1})).is_err());
    assert!(s.execute("guide.remove", &json!({"frame": frame, "index": 0})).is_err());
    assert!(guides(&s, frame).is_empty());
}

#[test]
fn frame_guides_move_with_the_frame_and_snap_its_layers() {
    let (mut s, frame, rect) = setup();
    s.execute("guide.add", &json!({"frame": frame, "vertical": true, "offset": 40})).unwrap();
    let page = s.page().unwrap();
    // The guide is at page x 140: a box inside the frame whose left edge is 1 past it snaps back.
    let snap = |s: &Session| {
        let t = Targets::gather(s.doc().unwrap(), page, &[rect]);
        t.snap_rect(Rect::new(141.0, 300.0, 149.0, 310.0), 4.0).0
    };
    assert_eq!(snap(&s), -1.0);
    // Moving the frame moves its guide (still 40 in the frame).
    s.execute("node.set", &json!({"ids": [frame], "props": {"x": 200}})).unwrap();
    assert_eq!(guides(&s, frame), vec![(true, 40.0)]);
    let t = Targets::gather(s.doc().unwrap(), page, &[rect]);
    assert_eq!(t.snap_rect(Rect::new(241.0, 300.0, 249.0, 310.0), 4.0).0, -1.0);
    // Layers outside the frame don't snap to it.
    let outside = id(s.execute("node.create", &json!({"type": "rectangle", "x": 1000, "y": 1000, "width": 5, "height": 5})).unwrap());
    let t = Targets::gather(s.doc().unwrap(), page, &[outside]);
    assert_eq!(t.snap_rect(Rect::new(241.0, 1000.0, 249.0, 1010.0), 4.0).0, 0.0);
}

#[test]
fn frame_guides_are_saved_and_copied_with_the_frame() {
    let (mut s, frame, _) = setup();
    s.execute("guide.add", &json!({"frame": frame, "vertical": false, "offset": 12})).unwrap();
    let bytes = figcraft_format::save_bytes(s.doc().unwrap(), None).unwrap();
    let (doc, _) = figcraft_format::load_bytes(&bytes).unwrap();
    assert_eq!(doc.node(frame).unwrap().guides.iter().map(|g| g.offset).collect::<Vec<_>>(), vec![12.0]);
    s.execute("select.set", &json!({"ids": [frame]})).unwrap();
    s.execute("edit.duplicate", &json!({})).unwrap();
    let copy = *s.ds().unwrap().selection.first().unwrap();
    assert_ne!(copy, frame);
    assert_eq!(guides(&s, copy), vec![(false, 12.0)]);
}
