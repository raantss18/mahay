//! Styles and variables on strokes: a colour style applied to a layer's strokes (linked, following
//! edits of the style, one undo step, detached), and stroke weights, each side on its own too,
//! bound to number variables that follow the frame's mode.

use serde_json::{Value, json};

use crate::Session;
use crate::params::id_of;
use figcraft_color::Color;
use figcraft_doc::NodeId;

fn ex(s: &mut Session, cmd: &str, p: Value) -> Value {
    s.execute(cmd, &p).unwrap_or_else(|e| panic!("{cmd}: {e}"))
}

fn node(s: &Session, id: NodeId) -> figcraft_doc::Node {
    s.doc().unwrap().node(id).unwrap().clone()
}

fn stroke_hex(s: &Session, id: NodeId) -> Option<String> {
    node(s, id).strokes.first().and_then(|p| p.solid_color()).map(Color::to_hex)
}

#[test]
fn a_colour_style_on_strokes_links_follows_edits_and_detaches() {
    let mut s = Session::new();
    let r = id_of(&ex(&mut s, "shape.rectangle", json!({"x": 0, "y": 0, "width": 50, "height": 50}))["id"]).unwrap();
    let fill = node(&s, r).fills;
    let st = ex(&mut s, "style.create", json!({"type": "paint", "name": "Brand/Line", "value": ["#ff0000"]}));
    let st = st["id"].as_str().unwrap().to_string();
    ex(&mut s, "style.apply", json!({"ids": [r], "style": "Brand/Line", "target": "stroke"}));
    assert_eq!(node(&s, r).styles.stroke.as_deref(), Some(st.as_str()));
    assert!(node(&s, r).styles.fill.is_none());
    assert_eq!(node(&s, r).fills, fill, "the fill keeps its own paint");
    let red = stroke_hex(&s, r);
    assert!(red.as_deref().is_some_and(|h| h.eq_ignore_ascii_case("FF0000")), "{red:?}");
    // One undo step takes the style off again.
    ex(&mut s, "edit.undo", json!({}));
    assert!(node(&s, r).styles.stroke.is_none());
    ex(&mut s, "edit.redo", json!({}));
    ex(&mut s, "style.update", json!({"style": st, "value": ["#0000ff"]}));
    let blue = stroke_hex(&s, r);
    assert!(blue.as_deref().is_some_and(|h| h.eq_ignore_ascii_case("0000FF")), "linked strokes follow the style: {blue:?}");
    ex(&mut s, "style.detach", json!({"ids": [r], "which": "stroke"}));
    assert!(node(&s, r).styles.stroke.is_none());
    assert_eq!(stroke_hex(&s, r), blue, "detaching keeps the colour");
}

#[test]
fn stroke_weights_and_each_side_bind_to_number_variables() {
    let mut s = Session::new();
    let f = id_of(&ex(&mut s, "frame.create", json!({"x": 0, "y": 0, "width": 200, "height": 200}))["id"]).unwrap();
    let r = id_of(&ex(&mut s, "node.create", json!({"type": "rectangle", "parent": f, "x": 10, "y": 10, "width": 50, "height": 50}))["id"]).unwrap();
    ex(&mut s, "node.set", json!({"ids": [r], "props": {"strokes": ["#000000"], "strokeWeight": 2}}));
    let c = ex(&mut s, "variable.createCollection", json!({"name": "Borders", "modes": ["Thin", "Thick"]}));
    let c = c["id"].as_str().unwrap().to_string();
    ex(&mut s, "variable.create", json!({"collection": c, "name": "border/top", "type": "number", "values": {"Thin": 1, "Thick": 6}}));
    ex(&mut s, "variable.create", json!({"collection": c, "name": "border/all", "type": "number", "values": {"Thin": 3, "Thick": 5}}));
    // One side: individual strokes turn on, the other sides keep the weight.
    ex(&mut s, "variable.bind", json!({"ids": [r], "field": "strokeTopWeight", "variable": "border/top"}));
    assert_eq!(node(&s, r).stroke.sides, Some([1.0, 2.0, 2.0, 2.0]));
    ex(&mut s, "variable.setMode", json!({"ids": [f], "collection": c, "mode": "Thick"}));
    assert_eq!(node(&s, r).stroke.sides, Some([6.0, 2.0, 2.0, 2.0]), "the frame's mode picks the value");
    // The whole weight binds too; per-side keys set one side.
    ex(&mut s, "variable.bind", json!({"ids": [r], "field": "strokeWeight", "variable": "border/all"}));
    assert_eq!(node(&s, r).stroke.weight, 5.0);
    ex(&mut s, "node.set", json!({"ids": [r], "props": {"strokeLeftWeight": 4}}));
    assert_eq!(node(&s, r).stroke.sides.map(|x| x[3]), Some(4.0));
    assert!(s.execute("node.set", &json!({"ids": [r], "props": {"strokeRightWeight": "thick"}})).is_err());
    // Only stroke-weight variables are offered for these fields, and instances count them as
    // stroke overrides.
    let doc = s.doc().unwrap();
    for field in crate::cmd::props::STROKE_SIDES.into_iter().chain(["strokeWeight"]) {
        assert_eq!(crate::cmd::variables::field_scope(doc, r, field), Some("STROKE_FLOAT"), "{field}");
        assert_eq!(crate::cmd::props::override_group(field), "strokes", "{field}");
    }
}
