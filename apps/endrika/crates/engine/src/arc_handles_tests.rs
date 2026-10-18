//! Arc handles on a selected ellipse: where they show, the sweep, start and ratio drags (Shift
//! snapping, closing, the other part), rotated and flipped ellipses, zoom, and one undo step per
//! drag.

use std::f64::consts::{PI, TAU};

use serde_json::{Value, json};

use crate::Session;
use crate::arc_handles::{Arc, ArcHandle, MIN_PX, arc_of, drag, handles, hit};
use crate::params::id_of;
use crate::tools::{PointerEvent, PointerKind};
use figcraft_doc::NodeId;
use figcraft_geom::Point;

fn ex(s: &mut Session, cmd: &str, p: Value) -> Value {
    s.execute(cmd, &p).unwrap_or_else(|e| panic!("{cmd}: {e}"))
}

/// A selected 100 × 100 ellipse at the origin.
fn ellipse(s: &mut Session) -> NodeId {
    let id = id_of(&ex(s, "shape.ellipse", json!({"x": 0, "y": 0, "width": 100, "height": 100}))["id"]).unwrap();
    ex(s, "select.set", json!({"ids": [id]}));
    id
}

fn near(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-6
}

fn assert_at(p: Option<[f64; 2]>, x: f64, y: f64) {
    let p = p.unwrap_or_else(|| panic!("no handle where ({x}, {y}) was expected"));
    assert!((p[0] - x).abs() < 1e-6 && (p[1] - y).abs() < 1e-6, "{p:?} != ({x}, {y})");
}

fn arc(s: &Session, id: NodeId) -> Arc {
    arc_of(s.doc().unwrap(), id).unwrap()
}

fn ev(kind: PointerKind, p: (f64, f64), zoom: f64, shift: bool) -> PointerEvent {
    let mut e = PointerEvent::at(kind, p.0, p.1, zoom);
    e.shift = shift;
    e
}

/// Drag with the Move tool from `a` through `via` to `to` at `zoom`; the overlay's readout seen
/// mid-drag.
fn drag_handle(s: &mut Session, a: (f64, f64), via: &[(f64, f64)], zoom: f64, shift: bool) -> Option<String> {
    s.pointer(&ev(PointerKind::Down, a, zoom, shift)).unwrap();
    let mut label = None;
    for p in via {
        s.pointer(&ev(PointerKind::Move, *p, zoom, shift)).unwrap();
        label = s.tool.overlay.arc.as_ref().map(|h| h.label.clone());
    }
    let last = via.last().copied().unwrap_or(a);
    s.pointer(&ev(PointerKind::Up, last, zoom, shift)).unwrap();
    assert!(s.tool.overlay.arc.is_none(), "the readout goes when the drag ends");
    label
}

/// The point at `deg` degrees (clockwise from the right) `r` from the 100 × 100 ellipse's centre.
fn at_angle(deg: f64, r: f64) -> (f64, f64) {
    let a = deg.to_radians();
    (50.0 + r * a.cos(), 50.0 + r * a.sin())
}

fn undo_len(s: &Session) -> usize {
    s.ds().unwrap().history.undo.len()
}

#[test]
fn a_selected_ellipse_shows_its_sweep_handle_inside_the_right_edge() {
    let mut s = Session::new();
    let id = ellipse(&mut s);
    let doc = s.doc().unwrap().clone();
    let h = handles(&doc, &[id], 1.0).expect("handles");
    assert_eq!(h.id, id);
    assert_at(Some(h.sweep), 90.0, 50.0);
    assert!(h.start.is_none() && h.ratio.is_none(), "a full ellipse has only the sweep handle: {h:?}");
    // The inset is in screen pixels: closer to the edge zoomed in.
    assert_at(handles(&doc, &[id], 4.0).map(|h| h.sweep), 97.5, 50.0);
    // Too small on screen, several layers, or not an ellipse: none.
    assert!(handles(&doc, &[id], (MIN_PX - 1.0) / 100.0).is_none());
    let r = id_of(&ex(&mut s, "shape.rectangle", json!({"x": 200, "y": 0, "width": 100, "height": 100}))["id"]).unwrap();
    let doc = s.doc().unwrap().clone();
    assert!(handles(&doc, &[id, r], 1.0).is_none() && handles(&doc, &[r], 1.0).is_none());
    // Locked ellipses show none.
    ex(&mut s, "node.set", json!({"ids": [id], "props": {"locked": true}}));
    assert!(handles(s.doc().unwrap(), &[id], 1.0).is_none());
    // The session shows them with the Move tool only.
    ex(&mut s, "node.set", json!({"ids": [id], "props": {"locked": false}}));
    ex(&mut s, "select.set", json!({"ids": [id]}));
    assert!(s.arc_handles().is_some() && s.tool_info()["arcHandles"]["sweep"].is_array());
    ex(&mut s, "tool.set", json!({"tool": "scale"}));
    assert!(s.arc_handles().is_none());
}

#[test]
fn dragging_the_sweep_handle_up_opens_a_positive_arc_in_one_undo_step() {
    let mut s = Session::new();
    let id = ellipse(&mut s);
    let undo = undo_len(&s);
    // Hovering it highlights it.
    s.pointer(&ev(PointerKind::Hover, (90.0, 50.0), 1.0, false)).unwrap();
    assert_eq!(s.tool.overlay.arc_hover, Some(ArcHandle::Sweep));
    // Up from the right edge, round to the top: three quarters of a turn.
    let label = drag_handle(&mut s, (90.0, 50.0), &[at_angle(-20.0, 40.0), at_angle(-60.0, 40.0), at_angle(-90.0, 40.0)], 1.0, false);
    assert_eq!(label.as_deref(), Some("Sweep 75%"));
    let a = arc(&s, id);
    assert!(near(a.start, 0.0) && near(a.sweep, 1.5 * PI) && near(a.inner, 0.0), "{a:?}");
    assert_eq!(undo_len(&s), undo + 1, "one undo step");
    // Now the start and ratio handles show; the sweep handle sits at the arc's end.
    let h = handles(s.doc().unwrap(), &[id], 1.0).unwrap();
    assert_at(Some(h.sweep), 50.0, 10.0);
    assert_at(h.start, 90.0, 50.0);
    assert_at(h.ratio, 50.0, 50.0);
    ex(&mut s, "edit.undo", json!({}));
    assert!(near(arc(&s, id).sweep, TAU));
}

#[test]
fn dragging_down_gives_a_negative_sweep_and_back_to_the_start_closes_it() {
    let mut s = Session::new();
    let id = ellipse(&mut s);
    drag_handle(&mut s, (90.0, 50.0), &[at_angle(20.0, 40.0), at_angle(60.0, 40.0), at_angle(90.0, 40.0)], 1.0, false);
    assert!(near(arc(&s, id).sweep, -1.5 * PI), "{:?}", arc(&s, id));
    // Back up round to the start: it snaps shut there, then opens the other way.
    let label = drag_handle(&mut s, (50.0, 90.0), &[at_angle(45.0, 40.0), at_angle(10.0, 40.0), at_angle(1.0, 40.0)], 1.0, false);
    assert_eq!(label.as_deref(), Some("Sweep -100%"));
    assert!(near(arc(&s, id).sweep, -TAU));
    drag_handle(&mut s, (90.0, 50.0), &[at_angle(-10.0, 40.0), at_angle(-45.0, 40.0)], 1.0, false);
    assert!(near(arc(&s, id).sweep, 1.75 * PI), "through a full turn the sweep carries on: {:?}", arc(&s, id));
}

#[test]
fn shift_snaps_the_sweep_and_start_to_15_degrees_and_the_ratio_to_5_percent() {
    let mut s = Session::new();
    let id = ellipse(&mut s);
    drag_handle(&mut s, (90.0, 50.0), &[at_angle(-30.0, 40.0), at_angle(-50.0, 40.0)], 1.0, true);
    assert!(near(arc(&s, id).sweep, 315f64.to_radians()), "{:?}", arc(&s, id));
    // The start handle turns the arc and keeps its sweep.
    drag_handle(&mut s, (90.0, 50.0), &[at_angle(40.0, 40.0), at_angle(52.0, 40.0)], 1.0, true);
    let a = arc(&s, id);
    assert!(near(a.start, 45f64.to_radians()) && near(a.sweep, 315f64.to_radians()), "{a:?}");
    let label = drag_handle(&mut s, (50.0, 50.0), &[at_angle(120.0, 20.0), at_angle(120.0, 31.0)], 1.0, true);
    assert_eq!(label.as_deref(), Some("Ratio 60%"));
    assert!(near(arc(&s, id).inner, 0.6));
}

#[test]
fn the_ratio_handle_makes_a_ring_and_shows_the_other_part_from_the_gap() {
    let mut s = Session::new();
    let id = ellipse(&mut s);
    // A three-quarter arc from the right edge, clockwise round to the top.
    ex(&mut s, "node.set", json!({"ids": [id], "props": {"arcSweep": 270}}));
    let label = drag_handle(&mut s, (50.0, 50.0), &[at_angle(135.0, 10.0), at_angle(135.0, 25.0)], 1.0, false);
    assert_eq!(label.as_deref(), Some("Ratio 50%"));
    let a = arc(&s, id);
    assert!(near(a.inner, 0.5) && near(a.sweep, 1.5 * PI), "{a:?}");
    // On a ring the ratio handle sits on the inner edge, halfway round the arc.
    let h = handles(s.doc().unwrap(), &[id], 1.0).unwrap();
    let (x, y) = at_angle(135.0, 25.0);
    assert_at(h.ratio, x, y);
    // Into the gap (the top right quarter): the quarter shows instead.
    drag_handle(&mut s, (x, y), &[at_angle(-45.0, 30.0)], 1.0, false);
    let a = arc(&s, id);
    assert!(near(a.start, 1.5 * PI) && near(a.sweep, 0.5 * PI) && near(a.inner, 0.6), "{a:?}");
    // A full ring keeps its ratio handle (and no start handle).
    ex(&mut s, "node.set", json!({"ids": [id], "props": {"arcSweep": 360, "arcStart": 0}}));
    let h = handles(s.doc().unwrap(), &[id], 1.0).unwrap();
    assert!(h.start.is_none() && h.ratio.is_some(), "{h:?}");
}

#[test]
fn handles_follow_rotated_and_flipped_ellipses() {
    let mut s = Session::new();
    let id = ellipse(&mut s);
    ex(&mut s, "node.set", json!({"ids": [id], "props": {"rotation": 30}}));
    ex(&mut s, "object.flipH", json!({"ids": [id]}));
    let doc = s.doc().unwrap().clone();
    let w = doc.world_transform(id);
    let local = |x: f64, y: f64| {
        let p = w * Point::new(x, y);
        (p.x, p.y)
    };
    let (sx, sy) = local(90.0, 50.0);
    assert_at(handles(&doc, &[id], 1.0).map(|h| h.sweep), sx, sy);
    assert_eq!(hit(&doc, &[id], Point::new(sx, sy), 1.0), Some((id, ArcHandle::Sweep)));
    // Dragged to the ellipse's own top (in its local space): three quarters, as unrotated.
    drag_handle(&mut s, (sx, sy), &[local(80.0, 20.0), local(50.0, 10.0)], 1.0, false);
    assert!(near(arc(&s, id).sweep, 1.5 * PI), "{:?}", arc(&s, id));
    assert_at(handles(s.doc().unwrap(), &[id], 1.0).map(|h| h.sweep), local(50.0, 10.0).0, local(50.0, 10.0).1);
}

#[test]
fn zoomed_out_the_handles_still_pick_and_escape_cancels_a_drag() {
    let mut s = Session::new();
    let id = ellipse(&mut s);
    // At 50% the inset is 20 page units and the pick distance 14.
    let doc = s.doc().unwrap().clone();
    assert_at(handles(&doc, &[id], 0.5).map(|h| h.sweep), 80.0, 50.0);
    assert_eq!(hit(&doc, &[id], Point::new(80.0, 62.0), 0.5), Some((id, ArcHandle::Sweep)));
    assert_eq!(hit(&doc, &[id], Point::new(80.0, 62.0), 2.0), None);
    let undo = undo_len(&s);
    s.pointer(&ev(PointerKind::Down, (80.0, 50.0), 0.5, false)).unwrap();
    s.pointer(&ev(PointerKind::Move, at_angle(-90.0, 30.0), 0.5, false)).unwrap();
    assert!(near(arc(&s, id).sweep, 1.5 * PI));
    s.escape().unwrap();
    assert!(near(arc(&s, id).sweep, TAU) && undo_len(&s) == undo, "escape restores the ellipse");
    // A click on a handle without moving changes nothing.
    drag_handle(&mut s, (80.0, 50.0), &[], 0.5, false);
    assert_eq!(undo_len(&s), undo);
}

#[test]
fn drag_math_survives_junk() {
    let mut s = Session::new();
    let id = ellipse(&mut s);
    let doc = s.doc().unwrap().clone();
    let a = arc(&s, id);
    for p in [Point::new(f64::NAN, 0.0), Point::new(f64::INFINITY, 1.0), Point::new(50.0, 50.0)] {
        for h in [ArcHandle::Sweep, ArcHandle::Start, ArcHandle::Ratio] {
            for zoom in [0.0, -1.0, f64::NAN, 1.0, 1e9] {
                if let Some(out) = drag(&doc, id, h, a, p, true, zoom) {
                    assert!(out.start.is_finite() && out.sweep.abs() <= TAU && (0.0..=0.99).contains(&out.inner), "{out:?}");
                }
                assert!(hit(&doc, &[id], p, zoom).is_none_or(|(i, _)| i == id));
            }
        }
    }
    // A zero-sized ellipse has no handles.
    ex(&mut s, "node.set", json!({"ids": [id], "props": {"width": 0}}));
    assert!(handles(s.doc().unwrap(), &[id], 1.0).is_none());
}
