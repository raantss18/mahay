//! The Design panel's Arc fields drive the same `node.set` keys as the canvas handles: Start in
//! degrees, Sweep and Ratio as percentages, and number variables bound to them (sweep and ratio
//! variables hold percentages, like the fields).

use std::f64::consts::{PI, TAU};

use serde_json::{Value, json};

use crate::Session;
use crate::params::id_of;
use figcraft_doc::{NodeId, NodeKind};

fn ex(s: &mut Session, cmd: &str, p: Value) -> Value {
    s.execute(cmd, &p).unwrap_or_else(|e| panic!("{cmd}: {e}"))
}

fn arc(s: &Session, id: NodeId) -> (f64, f64, f64) {
    match s.doc().unwrap().node(id).unwrap().kind {
        NodeKind::Ellipse { start, sweep, inner } => (start, sweep, inner),
        _ => panic!("not an ellipse"),
    }
}

fn ellipse(s: &mut Session) -> NodeId {
    id_of(&ex(s, "shape.ellipse", json!({"x": 0, "y": 0, "width": 100, "height": 100}))["id"]).unwrap()
}

#[test]
fn fields_write_degrees_and_percentages() {
    let mut s = Session::new();
    let e = ellipse(&mut s);
    // What the fields send for Start 90°, Sweep 75% and Ratio 40%.
    ex(&mut s, "node.set", json!({"id": e, "props": {"arcStart": 90, "arcSweep": 75.0 * 3.6, "arcInner": 0.4}}));
    let (start, sweep, inner) = arc(&s, e);
    assert!((start - PI / 2.0).abs() < 1e-9 && (sweep - 0.75 * TAU).abs() < 1e-9 && (inner - 0.4).abs() < 1e-9);
    // A negative sweep runs the other way; past a full turn it holds at one.
    ex(&mut s, "node.set", json!({"id": e, "props": {"arcSweep": -25.0 * 3.6}}));
    assert!((arc(&s, e).1 + 0.25 * TAU).abs() < 1e-9);
    ex(&mut s, "node.set", json!({"id": e, "props": {"arcSweep": 900}}));
    assert!((arc(&s, e).1 - TAU).abs() < 1e-9);
    // Inspect reports the arc the fields show.
    ex(&mut s, "node.set", json!({"id": e, "props": {"arcSweep": 180}}));
    let n = ex(&mut s, "node.get", json!({"id": e}));
    assert!((n["arc"]["sweep"].as_f64().unwrap() - 180.0).abs() < 1e-9);
    // Arc keys are for ellipses only.
    let r = id_of(&ex(&mut s, "shape.rectangle", json!({"x": 0, "y": 0, "width": 10, "height": 10}))["id"]).unwrap();
    assert!(s.execute("node.set", &json!({"id": r, "props": {"arcSweep": 90}})).is_err());
}

#[test]
fn variables_bound_to_the_arc_hold_what_the_fields_show() {
    let mut s = Session::new();
    let e = ellipse(&mut s);
    ex(&mut s, "variable.createCollection", json!({"name": "Shape", "modes": ["A", "B"]}));
    ex(&mut s, "variable.create", json!({"collection": "Shape", "name": "start", "type": "number", "value": 45}));
    ex(&mut s, "variable.create", json!({"collection": "Shape", "name": "sweep", "type": "number", "value": 50}));
    ex(&mut s, "variable.create", json!({"collection": "Shape", "name": "ratio", "type": "number", "value": 25}));
    for (field, var) in [("arcStart", "start"), ("arcSweep", "sweep"), ("arcInner", "ratio")] {
        ex(&mut s, "variable.bind", json!({"ids": [e], "field": field, "variable": var}));
    }
    let (start, sweep, inner) = arc(&s, e);
    assert!((start - PI / 4.0).abs() < 1e-9, "degrees: {start}");
    assert!((sweep - PI).abs() < 1e-9, "50% of a turn: {sweep}");
    assert!((inner - 0.25).abs() < 1e-9, "25%: {inner}");
    // A small ratio stays a percentage.
    ex(&mut s, "variable.set", json!({"variable": "ratio", "value": 1}));
    assert!((arc(&s, e).2 - 0.01).abs() < 1e-9, "{:?}", arc(&s, e));
}
