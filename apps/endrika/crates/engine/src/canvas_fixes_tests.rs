//! Canvas fixes: boolean groups are clicked where their result is (a click in a subtracted hole
//! goes through), and a layer dragged inside a group stays in the group while frames still take
//! drops.

use serde_json::{Value, json};

use crate::Session;
use crate::params::id_of;
use crate::tools::{PointerEvent, PointerKind};
use figcraft_doc::NodeId;

fn ex(s: &mut Session, cmd: &str, p: Value) -> NodeId {
    id_of(&s.execute(cmd, &p).unwrap_or_else(|e| panic!("{cmd}: {e}"))["id"]).unwrap_or(NodeId(0))
}

fn ev(kind: PointerKind, x: f64, y: f64, cmd: bool) -> PointerEvent {
    PointerEvent { cmd, ..PointerEvent::at(kind, x, y, 1.0) }
}

fn click(s: &mut Session, x: f64, y: f64, cmd: bool) {
    s.pointer(&ev(PointerKind::Down, x, y, cmd)).unwrap();
    s.pointer(&ev(PointerKind::Up, x, y, cmd)).unwrap();
}

fn drag(s: &mut Session, from: (f64, f64), to: (f64, f64)) {
    s.pointer(&ev(PointerKind::Down, from.0, from.1, false)).unwrap();
    for k in 1..=4 {
        let t = f64::from(k) / 4.0;
        s.pointer(&ev(PointerKind::Move, from.0 + (to.0 - from.0) * t, from.1 + (to.1 - from.1) * t, false)).unwrap();
    }
    s.pointer(&ev(PointerKind::Up, to.0, to.1, false)).unwrap();
}

fn no_snapping(s: &mut Session) {
    let ds = s.ds_mut().unwrap();
    ds.view.snap_objects = false;
    ds.view.snap_pixel = false;
}

fn parent(s: &Session, id: NodeId) -> Option<NodeId> {
    s.doc().unwrap().node(id).unwrap().parent
}

/// A 200 × 200 ring (a square minus a square hole at 50..150) over a rectangle filling the hole,
/// and nothing behind the ring's top-right corner: (behind, ring, outer, hole).
fn ring_over_a_rectangle(s: &mut Session) -> (NodeId, NodeId, NodeId, NodeId) {
    let behind = ex(s, "shape.rectangle", json!({"x": 60, "y": 60, "width": 80, "height": 80}));
    let outer = ex(s, "shape.rectangle", json!({"x": 0, "y": 0, "width": 200, "height": 200}));
    let hole = ex(s, "shape.rectangle", json!({"x": 50, "y": 50, "width": 100, "height": 100}));
    let ring = ex(s, "bool.subtract", json!({"ids": [outer, hole]}));
    s.execute("select.none", &json!({})).unwrap();
    no_snapping(s);
    (behind, ring, outer, hole)
}

#[test]
fn a_click_in_a_booleans_hole_goes_through_to_what_is_behind() {
    let mut s = Session::new();
    let (behind, ring, outer, hole) = ring_over_a_rectangle(&mut s);
    click(&mut s, 100.0, 100.0, false);
    assert_eq!(s.selection(), vec![behind], "the shape seen through the hole");
    // (Away from the selected rectangle's corner handles.)
    s.execute("select.none", &json!({})).unwrap();
    click(&mut s, 145.0, 55.0, false);
    assert!(s.selection().is_empty(), "the hole's corner, with nothing behind it: nothing");
    click(&mut s, 25.0, 25.0, false);
    assert_eq!(s.selection(), vec![ring], "the ring itself");
    // Operand rules: Cmd/Ctrl-click reaches the operand under the pointer, and inside the group
    // (it or an operand selected) every operand can be picked, the hole's too.
    click(&mut s, 25.0, 25.0, true);
    assert_eq!(s.selection(), vec![outer]);
    click(&mut s, 100.0, 100.0, false);
    assert_eq!(s.selection(), vec![hole], "a sibling operand, in the hole");
    s.execute("select.set", &json!({"ids": [ring]})).unwrap();
    s.pointer(&ev(PointerKind::DoubleClick, 100.0, 100.0, false)).unwrap();
    assert_eq!(s.selection(), vec![hole], "a double click enters the group");
    // "Select layer" doesn't list the ring in its hole.
    s.execute("select.none", &json!({})).unwrap();
    let listed = s.execute("select.layersAt", &json!({"x": 100, "y": 100})).unwrap();
    let ids: Vec<NodeId> = listed["layers"].as_array().unwrap().iter().filter_map(|l| id_of(&l["id"])).collect();
    assert_eq!(ids, vec![behind]);
}

#[test]
fn a_layer_dragged_in_a_group_stays_in_it() {
    let mut s = Session::new();
    let frame = ex(&mut s, "frame.create", json!({"x": 400, "y": 0, "width": 300, "height": 300}));
    let a = ex(&mut s, "shape.rectangle", json!({"x": 0, "y": 0, "width": 50, "height": 50}));
    let b = ex(&mut s, "shape.rectangle", json!({"x": 100, "y": 0, "width": 50, "height": 50}));
    let g = ex(&mut s, "object.group", json!({"ids": [a, b]}));
    no_snapping(&mut s);
    // Into the frame, then out onto the empty page: the layer stays in its group each time.
    s.execute("select.set", &json!({"ids": [a]})).unwrap();
    drag(&mut s, (25.0, 25.0), (525.0, 125.0));
    assert_eq!(parent(&s, a), Some(g), "dropped over a frame: still in its group");
    assert_eq!(s.doc().unwrap().world_bounds(a).unwrap().x0, 500.0, "it moved");
    drag(&mut s, (525.0, 125.0), (225.0, 425.0));
    assert_eq!(parent(&s, a), Some(g), "dropped on the page: still in its group");
    let gb = s.doc().unwrap().world_bounds(g).unwrap();
    assert_eq!((gb.x0, gb.y0, gb.x1, gb.y1), (100.0, 0.0, 250.0, 450.0), "the group's box follows its layers");
    // A layer that isn't in a group still drops into the frame.
    s.execute("select.set", &json!({"ids": [g]})).unwrap();
    s.execute("object.ungroup", &json!({})).unwrap();
    s.execute("select.set", &json!({"ids": [a]})).unwrap();
    drag(&mut s, (225.0, 425.0), (525.0, 125.0));
    assert_eq!(parent(&s, a), Some(frame), "outside a group, a frame takes the drop");
}
