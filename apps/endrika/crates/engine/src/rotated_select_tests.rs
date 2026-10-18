//! Handle boxes for several layers that share a rotation.

use serde_json::json;

use crate::Session;
use crate::params::id_of;
use crate::tools::{PointerEvent, PointerKind, selection_box, selection_frame};
use figcraft_doc::NodeId;
use figcraft_geom::{Affine, Point};

fn rect(s: &mut Session, x: f64, y: f64, rotation: f64) -> NodeId {
    let id = id_of(&s.execute("shape.rectangle", &json!({"x": x, "y": y, "width": 100, "height": 50})).unwrap()["id"]).unwrap();
    s.execute("node.set", &json!({"id": id, "props": {"rotation": rotation}})).unwrap();
    id
}

fn near(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-6
}

/// The layer's corners in the box's axes.
fn corners_in(s: &Session, frame: Affine, id: NodeId) -> Vec<Point> {
    let doc = s.doc().unwrap();
    let n = doc.node(id).unwrap();
    let m = frame.inverse() * doc.world_transform(id);
    [(0.0, 0.0), (n.width, 0.0), (n.width, n.height), (0.0, n.height)].iter().map(|(x, y)| m * Point::new(*x, *y)).collect()
}

#[test]
fn shared_rotation_turns_the_box_with_the_layers() {
    let mut s = Session::new();
    let ids = [rect(&mut s, 0.0, 0.0, 30.0), rect(&mut s, 40.0, 120.0, 30.0)];
    let (frame, w, h) = selection_frame(s.doc().unwrap(), &ids).unwrap();
    // The box's axes follow the layers': each sits upright in it, and the box hugs them.
    let pts: Vec<Point> = ids.iter().flat_map(|i| corners_in(&s, frame, *i)).collect();
    let fold = |f: fn(f64, f64) -> f64, init: f64, k: fn(&Point) -> f64| pts.iter().map(k).fold(init, f);
    assert!(near(fold(f64::min, f64::INFINITY, |p| p.x), 0.0) && near(fold(f64::min, f64::INFINITY, |p| p.y), 0.0));
    assert!(near(fold(f64::max, f64::NEG_INFINITY, |p| p.x), w) && near(fold(f64::max, f64::NEG_INFINITY, |p| p.y), h));
    let c = (frame.inverse() * s.doc().unwrap().world_transform(ids[0])).as_coeffs();
    assert!(near(c[0], 1.0) && near(c[1], 0.0) && near(c[2], 0.0) && near(c[3], 1.0), "{c:?}");
    // Tighter than the page-aligned bounds.
    let b = s.doc().unwrap().bounds_of(&ids).unwrap();
    assert!(w * h < b.width() * b.height() - 1.0);
}

#[test]
fn different_or_right_angle_rotations_keep_the_page_aligned_box() {
    let mut s = Session::new();
    for (r0, r1) in [(30.0, 45.0), (90.0, 90.0), (0.0, 180.0)] {
        let ids = [rect(&mut s, 0.0, 0.0, r0), rect(&mut s, 200.0, 50.0, r1)];
        let doc = s.doc().unwrap();
        let b = doc.bounds_of(&ids).unwrap();
        let q = selection_box(doc, &ids).unwrap();
        assert_eq!(q, [Point::new(b.x0, b.y0), Point::new(b.x1, b.y0), Point::new(b.x1, b.y1), Point::new(b.x0, b.y1)], "{r0} {r1}");
    }
}

#[test]
fn resizing_a_rotated_multi_selection_scales_along_its_axes() {
    let mut s = Session::new();
    let ids = [rect(&mut s, 0.0, 0.0, 30.0), rect(&mut s, 40.0, 120.0, 30.0)];
    s.execute("select.set", &json!({"ids": ids})).unwrap();
    let (frame, w, h) = selection_frame(s.doc().unwrap(), &ids).unwrap();
    // Drag the bottom-right handle to make the box 1.5× wider and 2× taller.
    let (from, to) = (frame * Point::new(w, h), frame * Point::new(w * 1.5, h * 2.0));
    s.pointer(&PointerEvent::at(PointerKind::Down, from.x, from.y, 1.0)).unwrap();
    s.pointer(&PointerEvent::at(PointerKind::Move, to.x, to.y, 1.0)).unwrap();
    s.pointer(&PointerEvent::at(PointerKind::Up, to.x, to.y, 1.0)).unwrap();
    let doc = s.doc().unwrap();
    let (f2, w2, h2) = selection_frame(doc, &ids).unwrap();
    assert!(near(w2, w * 1.5) && near(h2, h * 2.0), "{w2} {h2}");
    // The opposite corner stays put and the layers keep their rotation and grow with the box.
    assert!((f2 * Point::ZERO - frame * Point::ZERO).hypot() < 1e-6);
    for i in ids {
        let n = doc.node(i).unwrap();
        assert!(near(n.width, 150.0) && near(n.height, 100.0), "{} {}", n.width, n.height);
        assert!(near(figcraft_geom::rotation_degrees(n.transform), 30.0));
    }
    // One undo step restores both.
    s.execute("edit.undo", &json!({})).unwrap();
    assert!(near(s.doc().unwrap().node(ids[0]).unwrap().width, 100.0));
}
