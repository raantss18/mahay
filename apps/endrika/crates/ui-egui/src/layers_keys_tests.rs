//! Arrow keys in the layers panel once a row has been clicked: up/down through the rows (Shift
//! extends), left/right to collapse, expand, and go to the parent or first child; a press on the
//! canvas gives arrows back to nudging.

use egui::{Event, Key, Modifiers, pos2};
use figcraft_doc::NodeId;
use figcraft_engine::Session;
use serde_json::json;

use crate::layers_drag_tests::Ui;

fn id(v: serde_json::Value) -> NodeId {
    NodeId(v["id"].as_u64().unwrap())
}

fn key(u: &mut Ui, k: Key, shift: bool) {
    let modifiers = Modifiers { shift, ..Default::default() };
    u.frame(vec![
        Event::Key { key: k, physical_key: None, pressed: true, repeat: false, modifiers },
        Event::Key { key: k, physical_key: None, pressed: false, repeat: false, modifiers },
    ]);
}

fn sel(u: &Ui) -> Vec<NodeId> {
    u.app.session.selection()
}

/// Rows top to bottom: `c` (rectangle), `frame` (collapsed, holding `child`), `a` (rectangle).
fn setup() -> (Ui, NodeId, NodeId, NodeId, NodeId) {
    let mut s = Session::new();
    let a = id(s.execute("node.create", &json!({"type": "rectangle", "x": 0, "y": 0, "width": 10, "height": 10})).unwrap());
    let frame = id(s.execute("node.create", &json!({"type": "frame", "x": 50, "y": 0, "width": 100, "height": 100})).unwrap());
    let child = id(s.execute("node.create", &json!({"type": "ellipse", "parent": frame, "x": 10, "y": 10, "width": 10, "height": 10})).unwrap());
    let c = id(s.execute("node.create", &json!({"type": "rectangle", "x": 200, "y": 0, "width": 10, "height": 10})).unwrap());
    s.execute("select.none", &json!({})).unwrap();
    let mut u = Ui::new(s);
    u.app.ui.expanded.clear();
    u.frame(vec![]);
    (u, a, frame, child, c)
}

fn click_row(u: &mut Ui, id: NodeId) {
    let p = u.row_at(id);
    u.button(p, true);
    u.button(p, false);
    assert_eq!(sel(u), vec![id]);
}

#[test]
fn up_and_down_walk_the_rows_and_shift_extends() {
    let (mut u, a, frame, _, c) = setup();
    click_row(&mut u, c);
    key(&mut u, Key::ArrowDown, false);
    assert_eq!(sel(&u), vec![frame]);
    key(&mut u, Key::ArrowDown, false);
    assert_eq!(sel(&u), vec![a]);
    // The last row stays put.
    key(&mut u, Key::ArrowDown, false);
    assert_eq!(sel(&u), vec![a]);
    key(&mut u, Key::ArrowUp, true);
    key(&mut u, Key::ArrowUp, true);
    let mut got = sel(&u);
    got.sort();
    let mut want = vec![a, frame, c];
    want.sort();
    assert_eq!(got, want, "Shift+Up from the bottom row selects up to the top");
    // Back down with Shift shrinks the range towards the anchor.
    key(&mut u, Key::ArrowDown, true);
    let mut got = sel(&u);
    got.sort();
    let mut want = vec![a, frame];
    want.sort();
    assert_eq!(got, want);
    // Nothing moved on the canvas.
    let doc = u.app.session.doc().unwrap();
    assert_eq!(doc.node(c).unwrap().transform.translation().x, 200.0);
}

#[test]
fn left_and_right_open_close_and_move_between_levels() {
    let (mut u, _, frame, child, _) = setup();
    click_row(&mut u, frame);
    key(&mut u, Key::ArrowRight, false);
    assert!(u.app.ui.expanded.contains(&frame), "Right opens a closed row");
    assert_eq!(sel(&u), vec![frame]);
    key(&mut u, Key::ArrowRight, false);
    assert_eq!(sel(&u), vec![child], "Right on an open row goes to its first child");
    key(&mut u, Key::ArrowLeft, false);
    assert_eq!(sel(&u), vec![frame], "Left on a leaf goes to its parent");
    key(&mut u, Key::ArrowLeft, false);
    assert!(!u.app.ui.expanded.contains(&frame), "Left closes an open row");
    assert_eq!(sel(&u), vec![frame]);
}

#[test]
fn a_canvas_press_gives_arrows_back_to_nudging() {
    let (mut u, _, _, _, c) = setup();
    click_row(&mut u, c);
    assert!(u.app.ui.layers_focus);
    // Press on empty canvas (deselects), then select c from the canvas side and nudge it.
    let p = pos2(900.0, 700.0);
    u.button(p, true);
    u.button(p, false);
    assert!(!u.app.ui.layers_focus);
    u.app.run("select.set", json!({"ids": [c]}));
    key(&mut u, Key::ArrowRight, false);
    let doc = u.app.session.doc().unwrap();
    assert_eq!(doc.node(c).unwrap().transform.translation().x, 201.0);
}
