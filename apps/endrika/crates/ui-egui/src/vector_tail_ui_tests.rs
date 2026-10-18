//! The Design panel's Arc fields (named, valued, Mixed across ellipses, typed into), the width
//! point's side fields, and operands with an operation of their own in the layers panel.

use egui::accesskit::{Node, Role};
use egui::{Event, Key, Modifiers};
use figcraft_doc::NodeId;
use figcraft_engine::Session;
use serde_json::json;

use crate::icons::Icon;
use crate::layers_drag_tests::Ui;

fn id(v: serde_json::Value) -> NodeId {
    NodeId(v["id"].as_u64().unwrap())
}

fn tree(u: &mut Ui) -> Vec<Node> {
    u.ctx.enable_accesskit();
    u.frame(vec![]);
    let out = u.frame_output(vec![]);
    out.platform_output.accesskit_update.expect("an AccessKit update").nodes.into_iter().map(|(_, n)| n).collect()
}

fn value(nodes: &[Node], label: &str) -> Option<String> {
    nodes.iter().find(|n| n.role() == Role::SpinButton && n.label() == Some(label)).map(|n| n.value().unwrap_or_default().to_string())
}

fn press(u: &mut Ui, key: Key) {
    u.frame(vec![
        Event::Key { key, physical_key: None, pressed: true, repeat: false, modifiers: Modifiers::NONE },
        Event::Key { key, physical_key: None, pressed: false, repeat: false, modifiers: Modifiers::NONE },
    ]);
}

#[test]
fn arc_fields_show_and_set_the_ellipse_arc() {
    let mut s = Session::new();
    let e = id(s.execute("shape.ellipse", &json!({"x": 0, "y": 0, "width": 100, "height": 100})).unwrap());
    let r = id(s.execute("shape.rectangle", &json!({"x": 200, "y": 0, "width": 50, "height": 50})).unwrap());
    s.execute("node.set", &json!({"id": e, "props": {"arcStart": 90, "arcSweep": 270, "arcInner": 0.4}})).unwrap();
    s.execute("select.set", &json!({"ids": [e]})).unwrap();
    let mut u = Ui::new(s);
    let nodes = tree(&mut u);
    assert_eq!(value(&nodes, "Arc start").as_deref(), Some("90°"));
    assert_eq!(value(&nodes, "Arc sweep").as_deref(), Some("75%"));
    assert_eq!(value(&nodes, "Arc ratio").as_deref(), Some("40%"));
    // Two ellipses with different sweeps: Mixed; with a rectangle in the selection: no Arc fields.
    let e2 = id(u.app.session.execute("shape.ellipse", &json!({"x": 0, "y": 200, "width": 50, "height": 50})).unwrap());
    u.app.session.execute("node.set", &json!({"id": e2, "props": {"arcStart": 90, "arcInner": 0.4}})).unwrap();
    u.app.session.execute("select.set", &json!({"ids": [e, e2]})).unwrap();
    let nodes = tree(&mut u);
    assert_eq!(value(&nodes, "Arc start").as_deref(), Some("90°"));
    assert_eq!(value(&nodes, "Arc sweep").as_deref(), Some("Mixed"));
    u.app.session.execute("select.set", &json!({"ids": [e, r]})).unwrap();
    assert_eq!(value(&tree(&mut u), "Arc sweep"), None);
    // Typing into Sweep sets it on the ellipse (one step).
    u.app.session.execute("select.set", &json!({"ids": [e]})).unwrap();
    press(&mut u, Key::F6);
    for _ in 0..200 {
        if crate::a11y::focused_name(&u.ctx) == "Arc sweep" {
            break;
        }
        press(&mut u, Key::Tab);
    }
    assert_eq!(crate::a11y::focused_name(&u.ctx), "Arc sweep");
    press(&mut u, Key::Enter);
    u.frame(vec![Event::Text("50".into())]);
    press(&mut u, Key::Enter);
    u.frame(vec![]);
    let sweep = match u.app.session.doc().unwrap().node(e).unwrap().kind {
        figcraft_doc::NodeKind::Ellipse { sweep, .. } => sweep,
        _ => 0.0,
    };
    assert!((sweep - std::f64::consts::PI).abs() < 1e-9, "{sweep}");
}

#[test]
fn a_width_point_shows_its_sides() {
    let mut s = Session::new();
    let v = id(s.execute("vector.create", &json!({"points": [[0, 50], [100, 50]], "closed": false})).unwrap());
    s.execute("node.set", &json!({"id": v, "props": {"strokes": ["#000000"], "strokeWeight": 10, "strokeAlign": "center"}})).unwrap();
    s.execute("vector.edit.begin", &json!({"id": v})).unwrap();
    s.execute("vector.edit.tool", &json!({"tool": "width"})).unwrap();
    s.execute("vector.edit.widthPoint", &json!({"t": 0.5, "width": 20})).unwrap();
    s.execute("vector.edit.widthPoint", &json!({"index": 1, "side": "left", "width": 4})).unwrap();
    let mut u = Ui::new(s);
    let nodes = tree(&mut u);
    assert_eq!(value(&nodes, "Left side width").as_deref(), Some("4"));
    assert_eq!(value(&nodes, "Right side width").as_deref(), Some("10"));
    assert_eq!(value(&nodes, "Width").as_deref(), Some("14"));
}

#[test]
fn operands_with_their_own_operation_show_it_in_the_layers_panel() {
    let mut s = Session::new();
    let a = id(s.execute("shape.rectangle", &json!({"x": 0, "y": 0, "width": 100, "height": 100})).unwrap());
    let b = id(s.execute("shape.ellipse", &json!({"x": 50, "y": 50, "width": 100, "height": 100})).unwrap());
    let g = id(s.execute("bool.subtract", &json!({"ids": [a, b]})).unwrap());
    let doc = s.doc().unwrap();
    assert_eq!(crate::layers::node_icon(doc, b), Icon::Ellipse, "following the group: its own icon");
    assert_eq!(crate::layers::node_icon(doc, g), Icon::Subtract);
    s.execute("bool.intersect", &json!({"ids": [b]})).unwrap();
    assert_eq!(crate::layers::node_icon(s.doc().unwrap(), b), Icon::Intersect);
    assert_eq!(crate::layers::node_icon(s.doc().unwrap(), a), Icon::Rectangle, "the bottom operand's never shows");
}
