//! A gesture's live preview makes a new document only when it shows something new: a button held
//! still (the same move every frame) or a preview that changes nothing keeps the document, so the
//! canvas doesn't re-render every frame for nothing.

use std::sync::Arc;

use serde_json::json;

use crate::Session;
use crate::params::id_of;
use crate::tools::{PointerEvent, PointerKind};
use figcraft_doc::{Document, NodeId};

fn rect(s: &mut Session, parent: Option<NodeId>) -> NodeId {
    let mut p = json!({"x": 40, "y": 40, "width": 80, "height": 60});
    if let Some(parent) = parent {
        p["parent"] = json!(parent);
    }
    id_of(&s.execute("shape.rectangle", &p).unwrap()["id"]).unwrap()
}

fn doc(s: &Session) -> Arc<Document> {
    s.ds().unwrap().doc.clone()
}

fn ev(kind: PointerKind, x: f64, y: f64) -> PointerEvent {
    PointerEvent::at(kind, x, y, 1.0)
}

#[test]
fn a_drag_held_still_keeps_its_preview() {
    let mut s = Session::new();
    // A child of a rotated frame, the case the canvas showed stale while dragging.
    let f = id_of(&s.execute("frame.create", &json!({"x": 100, "y": 100, "width": 300, "height": 200})).unwrap()["id"]).unwrap();
    s.execute("node.set", &json!({"id": f, "rotation": 25})).unwrap();
    let r = rect(&mut s, Some(f));
    // Snapping would pull the layer onto the frame's edges and centre.
    s.ds_mut().unwrap().view.snap_objects = false;
    s.execute("select.set", &json!({"ids": [r]})).unwrap();
    let c = s.doc().unwrap().world_bounds(r).unwrap().center();
    s.pointer(&ev(PointerKind::Down, c.x, c.y)).unwrap();
    s.pointer(&ev(PointerKind::Move, c.x + 60.0, c.y + 30.0)).unwrap();
    let moved = doc(&s);
    let rev = s.ds().unwrap().revision;
    for _ in 0..5 {
        s.pointer(&ev(PointerKind::Move, c.x + 60.0, c.y + 30.0)).unwrap();
    }
    assert!(Arc::ptr_eq(&moved, &doc(&s)), "the same move again keeps the document");
    assert_eq!(s.ds().unwrap().revision, rev);
    // The layer moved with the pointer, in page space.
    let b = s.doc().unwrap().world_bounds(r).unwrap().center();
    assert!((b.x - c.x - 60.0).abs() < 1.0 && (b.y - c.y - 30.0).abs() < 1.0, "{b:?} from {c:?}");
    // Shift (a modifier change without a move) and a real move still update the preview.
    s.pointer(&PointerEvent { shift: true, ..ev(PointerKind::Move, c.x + 60.0, c.y + 30.0) }).unwrap();
    assert!(!Arc::ptr_eq(&moved, &doc(&s)));
    let shifted = doc(&s);
    s.pointer(&ev(PointerKind::Move, c.x + 90.0, c.y + 30.0)).unwrap();
    assert!(!Arc::ptr_eq(&shifted, &doc(&s)));
    s.pointer(&ev(PointerKind::Up, c.x + 90.0, c.y + 30.0)).unwrap();
    // A new drag to the same spot as the last move of the previous one is not mistaken for it.
    let c = s.doc().unwrap().world_bounds(r).unwrap().center();
    let before = doc(&s);
    s.pointer(&ev(PointerKind::Down, c.x, c.y)).unwrap();
    s.pointer(&ev(PointerKind::Move, c.x + 20.0, c.y)).unwrap();
    assert!(!Arc::ptr_eq(&before, &doc(&s)));
}

#[test]
fn a_preview_that_changes_nothing_keeps_the_document() {
    let mut s = Session::new();
    let r = rect(&mut s, None);
    s.begin_interaction("Edit").unwrap();
    let set = |s: &mut Session, x: f64| {
        let props = serde_json::Map::from_iter([("x".to_string(), json!(x))]);
        s.preview(|d, _| crate::cmd::props::apply("node.set", d, r, &props)).unwrap();
    };
    set(&mut s, 10.0);
    let first = doc(&s);
    set(&mut s, 10.0);
    assert!(Arc::ptr_eq(&first, &doc(&s)));
    set(&mut s, 12.0);
    assert!(!Arc::ptr_eq(&first, &doc(&s)));
    s.commit_interaction().unwrap();
    assert_eq!(s.doc().unwrap().node(r).unwrap().transform.as_coeffs()[4], 12.0);
}
