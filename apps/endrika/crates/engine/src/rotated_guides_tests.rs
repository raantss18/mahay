//! Guides on rotated frames: they run along the frame's own axes, page points convert into the
//! frame's coordinates, a ruler guide dropped on a turned frame takes its nearer axis, and layers in
//! the frame snap to its guides (and siblings, and edges) along those axes.

use figcraft_doc::NodeId;
use figcraft_geom::{Affine, Point, Vec2};
use serde_json::json;

use crate::Session;
use crate::guides::{frame_axis, offset_at, segment};
use crate::params::id_of;
use crate::snap::Targets;
use crate::tools::{PointerEvent, PointerKind};

fn id(v: serde_json::Value) -> NodeId {
    id_of(&v["id"]).unwrap()
}

fn near(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-6
}

/// A 300 × 200 frame at (100, 100) turned by `rotation`, with a 20 × 20 rectangle at (10, 10) in it
/// and a vertical guide at 40.
fn setup(rotation: f64) -> (Session, NodeId, NodeId) {
    let mut s = Session::new();
    let frame = id(s.execute("node.create", &json!({"type": "frame", "x": 100, "y": 100, "width": 300, "height": 200})).unwrap());
    let rect = id(s.execute("node.create", &json!({"type": "rectangle", "parent": frame, "x": 10, "y": 10, "width": 20, "height": 20})).unwrap());
    s.execute("node.set", &json!({"id": frame, "props": {"rotation": rotation}})).unwrap();
    s.execute("guide.add", &json!({"frame": frame, "vertical": true, "offset": 40})).unwrap();
    (s, frame, rect)
}

/// Page vector for `v` in `frame`'s coordinates.
fn page_vec(s: &Session, frame: NodeId, v: Vec2) -> Vec2 {
    let m = s.doc().unwrap().world_transform(frame);
    m * Point::new(v.x, v.y) - m * Point::ZERO
}

/// Page vector `v` in `frame`'s coordinates.
fn local_vec(s: &Session, frame: NodeId, v: Vec2) -> Vec2 {
    let m = s.doc().unwrap().world_transform(frame).inverse();
    m * (Point::ZERO + v) - m * Point::ZERO
}

/// `id`'s position in its parent's coordinates.
fn local_xy(s: &Session, id: NodeId) -> Point {
    s.doc().unwrap().node(id).unwrap().transform.translation().to_point()
}

#[test]
fn a_rotated_frames_guides_run_along_its_axes_and_span_it() {
    let (s, frame, _) = setup(30.0);
    let doc = s.doc().unwrap();
    let xf = doc.world_transform(frame);
    let [a, b] = segment(doc, frame, true, 40.0).unwrap();
    // From 40 along the frame's top edge, down its y axis, as long as the frame is tall.
    let top = xf * Point::new(300.0, 0.0) - xf * Point::ZERO;
    assert!(near((a - xf * Point::ZERO).hypot(), 40.0));
    assert!(near((a - xf * Point::ZERO).dot(top) / top.hypot(), 40.0), "on the top edge");
    assert!(near((b - a).hypot(), 200.0));
    assert!(near((b - a).dot(top), 0.0), "across the frame");
    assert!((b.x - a.x).abs() > 50.0, "not a page-vertical line");
    // Page points along it convert back to 40 in the frame; 5 further along the frame's x is 45.
    for t in [0.0, 0.3, 1.0] {
        assert!(near(offset_at(doc, frame, true, a.lerp(b, t)).unwrap(), 40.0));
    }
    let p = a.midpoint(b) + page_vec(&s, frame, Vec2::new(5.0, 0.0));
    assert!(near(offset_at(doc, frame, true, p).unwrap(), 45.0));
    let p = xf * Point::new(12.0, 34.0);
    assert!(near(offset_at(doc, frame, false, p).unwrap(), 34.0));
    // A page's guides are page lines across the canvas.
    let page = s.page().unwrap();
    assert!(segment(doc, page, true, 40.0).is_none());
    assert_eq!(offset_at(doc, page, true, Point::new(12.5, 7.0)), Some(12.5));
}

#[test]
fn a_ruler_guide_dropped_on_a_turned_frame_takes_its_nearer_axis() {
    for (rotation, same) in [(0.0, true), (30.0, true), (-40.0, true), (60.0, false), (90.0, false), (150.0, true), (180.0, true)] {
        let (s, frame, _) = setup(rotation);
        let doc = s.doc().unwrap();
        assert_eq!(frame_axis(doc, frame, true), same, "{rotation}°");
        assert_eq!(frame_axis(doc, frame, false), !same, "{rotation}°");
    }
}

#[test]
fn layers_in_a_rotated_frame_snap_to_its_guides_along_its_axes() {
    let (s, frame, rect) = setup(30.0);
    let doc = s.doc().unwrap();
    let page = s.page().unwrap();
    let t = Targets::gather(doc, page, &[rect]);
    assert_ne!(t.space, Affine::IDENTITY, "snapping runs in the frame's space");
    let m = t.moving.unwrap();
    assert!(near(m.x0, 10.0) && near(m.y0, 10.0) && near(m.x1, 30.0) && near(m.y1, 30.0), "{m:?}");
    // Moved by (29, 3) in the frame: its left edge lands 1 short of the guide at 40 and snaps.
    let d = page_vec(&s, frame, Vec2::new(29.0, 3.0));
    let (c, guides) = t.snap_move(d, 4.0);
    let moved = local_vec(&s, frame, d + c);
    assert!(near(moved.x, 30.0) && near(moved.y, 3.0), "{moved:?}");
    // The guide drawn runs along the frame guide.
    assert!(
        guides.iter().any(|g| [Point::new(g.x0, g.y0), Point::new(g.x1, g.y1)].iter().all(|p| near(offset_at(doc, frame, true, *p).unwrap(), 40.0))),
        "{guides:?}"
    );
    // A page guide doesn't run along the frame's axes: no target.
    let mut s = s;
    s.execute("guide.add", &json!({"vertical": true, "offset": 250})).unwrap();
    let t = Targets::gather(s.doc().unwrap(), page, &[rect]);
    assert!(!t.xs.iter().chain(&t.ys).any(|l| l.1 < -1e6), "no page-wide lines");
}

#[test]
fn moving_a_layer_in_a_rotated_frame_snaps_and_keeps_whole_pixels_there() {
    let (mut s, frame, rect) = setup(30.0);
    s.execute("select.set", &json!({"ids": [rect]})).unwrap();
    let doc = s.doc().unwrap();
    let xf = doc.world_transform(frame);
    let from = xf * Point::new(20.0, 20.0);
    // Off by fractions in the frame and 1.3 short of the guide.
    let to = from + page_vec(&s, frame, Vec2::new(28.7, 5.4));
    s.pointer(&PointerEvent::at(PointerKind::Down, from.x, from.y, 1.0)).unwrap();
    s.pointer(&PointerEvent::at(PointerKind::Move, to.x, to.y, 1.0)).unwrap();
    s.pointer(&PointerEvent::at(PointerKind::Up, to.x, to.y, 1.0)).unwrap();
    assert_eq!(s.doc().unwrap().node(rect).unwrap().parent, Some(frame));
    let p = local_xy(&s, rect);
    assert!(near(p.x, 40.0), "snapped to the guide: {p:?}");
    assert!(near(p.y, p.y.round()), "whole pixels in the frame: {p:?}");
    assert!(near(p.y, 15.0), "{p:?}");
}

#[test]
fn siblings_and_page_guides_line_up_inside_a_quarter_turned_frame() {
    let (mut s, frame, rect) = setup(90.0);
    let page = s.page().unwrap();
    s.execute("node.create", &json!({"type": "rectangle", "parent": frame, "x": 120, "y": 60, "width": 30, "height": 30})).unwrap();
    // A page-vertical guide runs along the turned frame's x axis.
    s.execute("guide.add", &json!({"vertical": true, "offset": 250})).unwrap();
    let doc = s.doc().unwrap();
    let t = Targets::gather(doc, page, &[rect]);
    let y = offset_at(doc, frame, false, Point::new(250.0, 0.0)).unwrap();
    assert!(t.ys.iter().any(|l| near(l.0, y) && l.1 < -1e6), "{y} in {:?}", t.ys);
    // The sibling's edges are snap lines in the frame's space.
    assert!(t.xs.iter().any(|l| near(l.0, 120.0)) && t.ys.iter().any(|l| near(l.0, 60.0)));
    // Moved by (109, 49) in the frame: 1 short of the sibling's left and top edges.
    let d = page_vec(&s, frame, Vec2::new(109.0, 49.0));
    let (c, _) = t.snap_move(d, 4.0);
    let moved = local_vec(&s, frame, d + c);
    assert!(near(moved.x, 110.0) && near(moved.y, 50.0), "{moved:?}");
}

#[test]
fn measuring_finds_a_quarter_turned_frames_guides() {
    let (s, frame, rect) = setup(90.0);
    let doc = s.doc().unwrap();
    let page = s.page().unwrap();
    let [a, b] = segment(doc, frame, true, 40.0).unwrap();
    // The frame's vertical guide is a page-horizontal line.
    assert!(near(a.y, b.y));
    let found = crate::distance::guide_at(doc, page, &[rect], Point::new(a.x, a.y + 2.0), 4.0);
    assert_eq!(found.map(|(v, at)| (v, (at * 1e6).round() / 1e6)), Some((false, (a.y * 1e6).round() / 1e6)));
}

#[test]
fn snapping_outside_rotated_frames_stays_on_the_page() {
    let (s, _, rect) = setup(0.0);
    let page = s.page().unwrap();
    let doc = s.doc().unwrap();
    assert_eq!(Targets::gather(doc, page, &[rect]).space, Affine::IDENTITY);
    let top = doc.node(page).unwrap().children[0];
    assert_eq!(Targets::gather(doc, page, &[top]).space, Affine::IDENTITY);
}
