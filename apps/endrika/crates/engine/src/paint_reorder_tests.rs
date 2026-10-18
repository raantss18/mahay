//! Reordering paints, effects and layout grids (`fill.move`, `stroke.move`, `effect.move`,
//! `grid.move`): one undo step each, items keep everything, variable bindings and style links
//! follow; layout grid fields and instances' component properties bound to variables.

use figcraft_color::{Color, Gradient, GradientKind, ImageScaleMode, Paint};
use figcraft_doc::{GridKind, LayoutGrid, Node, NodeId, NodeKind};
use serde_json::{Value, json};

use crate::Session;

fn id(v: Value) -> NodeId {
    NodeId(v["id"].as_u64().unwrap())
}

fn ex(s: &mut Session, cmd: &str, p: Value) -> Value {
    s.execute(cmd, &p).unwrap_or_else(|e| panic!("{cmd} {p}: {e}"))
}

fn node(s: &Session, id: NodeId) -> Node {
    s.doc().unwrap().node(id).unwrap().clone()
}

fn rect(s: &mut Session) -> NodeId {
    id(ex(s, "shape.rectangle", json!({"x": 0, "y": 0, "width": 10, "height": 10})))
}

fn hexes(paints: &[Paint]) -> Vec<String> {
    paints.iter().map(|p| p.solid_color().map(|c| c.to_hex()).unwrap_or_else(|| "-".into())).collect()
}

fn vars(s: &mut Session) {
    ex(s, "variable.createCollection", json!({"name": "T", "modes": ["Light", "Dark"]}));
    for (name, kind, value) in [("brand", "color", json!("#FF0000")), ("gap", "number", json!(12)), ("cols", "number", json!(6))] {
        ex(s, "variable.create", json!({"collection": "T", "name": name, "type": kind, "value": value}));
    }
}

#[test]
fn fill_move_reorders_as_one_undo_step_and_keeps_every_paint() {
    let mut s = Session::new();
    let r = rect(&mut s);
    let gradient = Paint::gradient(Gradient::default_for(GradientKind::Radial, Color::from_hex("#00FF00").unwrap()));
    let mut image = Paint::image("synthetic-hash", ImageScaleMode::Fit);
    image.opacity = 0.4;
    let mut red = Paint::solid(Color::from_hex("#FF0000").unwrap());
    red.visible = false;
    let fills = vec![red.clone(), gradient.clone(), image.clone()];
    ex(&mut s, "node.set", json!({"ids": [r], "props": {"fills": fills}}));
    let undo = s.ds().unwrap().history.undo.len();
    // Bottom (0) to top (2).
    ex(&mut s, "fill.move", json!({"ids": [r], "from": 0, "to": 2}));
    assert_eq!(node(&s, r).fills, vec![gradient.clone(), image.clone(), red.clone()], "image, gradient and hidden paint survive whole");
    assert_eq!(s.ds().unwrap().history.undo.len(), undo + 1, "one undo step");
    ex(&mut s, "fill.move", json!({"ids": [r], "from": 2, "to": 1}));
    assert_eq!(node(&s, r).fills, vec![gradient.clone(), red.clone(), image.clone()]);
    ex(&mut s, "edit.undo", json!({}));
    ex(&mut s, "edit.undo", json!({}));
    assert_eq!(node(&s, r).fills, fills);
    ex(&mut s, "edit.redo", json!({}));
    assert_eq!(node(&s, r).fills, vec![gradient, image, red]);
}

#[test]
fn moves_out_of_range_change_nothing_and_junk_is_refused() {
    let mut s = Session::new();
    let r = rect(&mut s);
    ex(&mut s, "fill.add", json!({"ids": [r], "color": "#00FF00"}));
    let before = node(&s, r).fills;
    let undo = s.ds().unwrap().history.undo.len();
    for cmd in ["fill.move", "stroke.move", "effect.move", "grid.move"] {
        // Nothing to move: no change.
        ex(&mut s, cmd, json!({"ids": [r], "from": 0, "to": 9}));
        ex(&mut s, cmd, json!({"ids": [r], "from": 1, "to": 1}));
        for bad in [json!({"ids": [r], "from": -1, "to": 0}), json!({"ids": [r], "from": 0.5, "to": 1}), json!({"ids": [r], "from": 0})] {
            assert!(s.execute(cmd, &bad).is_err(), "{cmd} {bad}");
        }
    }
    assert_eq!(node(&s, r).fills, before);
    assert_eq!(s.ds().unwrap().history.undo.len(), undo, "no-op moves add no undo steps");
    // A negative index never removes the first paint or grid.
    assert!(s.execute("fill.remove", &json!({"ids": [r], "index": -1})).is_err());
    ex(&mut s, "node.set", json!({"ids": [r], "props": {"grids": [LayoutGrid::default()]}}));
    assert!(s.execute("grid.remove", &json!({"ids": [r], "index": -1})).is_err());
    assert_eq!(node(&s, r).fills, before);
    assert_eq!(node(&s, r).grids.len(), 1);
}

#[test]
fn a_bound_paint_colour_follows_its_paint() {
    let mut s = Session::new();
    let r = rect(&mut s);
    vars(&mut s);
    ex(&mut s, "fill.set", json!({"ids": [r], "index": 0, "color": "#0000FF"}));
    ex(&mut s, "fill.add", json!({"ids": [r], "color": "#00FF00"}));
    ex(&mut s, "fill.add", json!({"ids": [r], "color": "#FFFFFF"}));
    ex(&mut s, "variable.bind", json!({"ids": [r], "field": "fill", "variable": "brand"}));
    assert_eq!(hexes(&node(&s, r).fills), ["FF0000", "00FF00", "FFFFFF"]);
    // The bound bottom paint goes to the top: the binding goes with it.
    ex(&mut s, "fill.move", json!({"ids": [r], "from": 0, "to": 2}));
    assert_eq!(node(&s, r).bound.keys().collect::<Vec<_>>(), ["fills/2/color"]);
    ex(&mut s, "variable.set", json!({"variable": "brand", "value": "#123456"}));
    assert_eq!(hexes(&node(&s, r).fills), ["00FF00", "FFFFFF", "123456"], "the neighbours aren't recoloured");
    // Modes recolour the moved paint too.
    ex(&mut s, "variable.set", json!({"variable": "brand", "mode": "Dark", "value": "#ABCDEF"}));
    ex(&mut s, "variable.setMode", json!({"collection": "T", "mode": "Dark", "ids": [r]}));
    assert_eq!(hexes(&node(&s, r).fills), ["00FF00", "FFFFFF", "ABCDEF"]);
    // Removing a paint below shifts the binding down with its paint.
    ex(&mut s, "fill.remove", json!({"ids": [r], "index": 0}));
    assert_eq!(node(&s, r).bound.keys().collect::<Vec<_>>(), ["fills/1/color"]);
    ex(&mut s, "variable.set", json!({"variable": "brand", "mode": "Dark", "value": "#010203"}));
    assert_eq!(hexes(&node(&s, r).fills), ["FFFFFF", "010203"]);
    // Undo brings back the binding where it was.
    ex(&mut s, "edit.undo", json!({}));
    ex(&mut s, "edit.undo", json!({}));
    ex(&mut s, "edit.undo", json!({}));
    ex(&mut s, "edit.undo", json!({}));
    assert_eq!(node(&s, r).bound.keys().collect::<Vec<_>>(), ["fills/2/color"]);
    // Removing the bound paint drops its binding.
    ex(&mut s, "fill.remove", json!({"ids": [r], "index": 2}));
    assert!(node(&s, r).bound.is_empty());
}

#[test]
fn strokes_move_with_their_bindings_and_moves_unlink_styles() {
    let mut s = Session::new();
    let (a, b) = (rect(&mut s), rect(&mut s));
    vars(&mut s);
    for r in [a, b] {
        ex(&mut s, "stroke.add", json!({"ids": [r], "color": "#000000"}));
        ex(&mut s, "stroke.add", json!({"ids": [r], "color": "#00FF00"}));
    }
    ex(&mut s, "variable.bind", json!({"ids": [a, b], "field": "stroke", "variable": "brand"}));
    // (Adding a fill to a styled layer would unlink its style: add it first.)
    ex(&mut s, "fill.add", json!({"ids": [a, b], "color": "#FFFFFF"}));
    ex(&mut s, "style.create", json!({"type": "paint", "name": "Two", "from": a}));
    ex(&mut s, "style.apply", json!({"ids": [a, b], "style": "Two"}));
    assert!(node(&s, a).styles.fill.is_some());
    // One command moves the strokes of every selected layer.
    ex(&mut s, "stroke.move", json!({"ids": [a, b], "from": 0, "to": 1}));
    for r in [a, b] {
        assert_eq!(hexes(&node(&s, r).strokes), ["00FF00", "FF0000"]);
        assert_eq!(node(&s, r).bound.keys().collect::<Vec<_>>(), ["strokes/1/color"]);
    }
    ex(&mut s, "variable.set", json!({"variable": "brand", "value": "#0000FF"}));
    assert_eq!(hexes(&node(&s, b).strokes), ["00FF00", "0000FF"]);
    // A paint style defined the whole fill list: reordering it unlinks the style.
    let fills = node(&s, a).fills.len();
    assert!(fills >= 2);
    ex(&mut s, "fill.move", json!({"ids": [a], "from": 0, "to": fills - 1}));
    assert!(node(&s, a).styles.fill.is_none());
    assert!(node(&s, b).styles.fill.is_some(), "only the moved layer unlinks");
}

#[test]
fn bound_paint_keys_resolve_and_scope_by_index() {
    use crate::cmd::lists::{BoundList, list_binding};
    assert_eq!(list_binding("fill"), Some((BoundList::Fills, 0, "color")));
    assert_eq!(list_binding("stroke"), Some((BoundList::Strokes, 0, "color")));
    assert_eq!(list_binding("fills/3/color"), Some((BoundList::Fills, 3, "color")));
    assert_eq!(list_binding("layoutGrids/1/gutter"), Some((BoundList::Grids, 1, "gutter")));
    for junk in
        ["fills/-1/color", "fills/+1/color", "fills//color", "fills/1/", "effects/x/radius", "grids/0/size", "fills/99999999999999999999999/color"]
    {
        assert_eq!(list_binding(junk), None, "{junk}");
    }
    let mut s = Session::new();
    let r = rect(&mut s);
    let doc = s.doc().unwrap();
    use crate::cmd::variables::field_scope;
    assert_eq!(field_scope(doc, r, "fills/2/color"), Some("SHAPE_FILL"));
    assert_eq!(field_scope(doc, r, "strokes/1/color"), Some("STROKE_COLOR"));
    assert_eq!(field_scope(doc, r, "effects/0/color"), Some("EFFECT_COLOR"));
    assert_eq!(field_scope(doc, r, "layoutGrids/0/count"), None);
}

#[test]
fn inspect_names_the_variable_of_each_paint() {
    let mut s = Session::new();
    let r = rect(&mut s);
    vars(&mut s);
    ex(&mut s, "fill.add", json!({"ids": [r], "color": "#00FF00"}));
    ex(&mut s, "variable.bind", json!({"ids": [r], "field": "fill", "variable": "brand"}));
    ex(&mut s, "fill.move", json!({"ids": [r], "from": 0, "to": 1}));
    let props = crate::devinspect::properties(s.doc().unwrap(), r);
    let fills = props.iter().find(|p| p.title == "Fills").unwrap();
    let tokens: Vec<Option<&str>> = fills.rows.iter().map(|p| p.token.as_deref()).collect();
    assert_eq!(tokens, [None, Some("brand")], "the token follows the moved paint");
}

#[test]
fn layout_grid_fields_bind_to_number_variables_and_follow_moves() {
    let mut s = Session::new();
    let f = id(ex(&mut s, "frame.create", json!({"x": 0, "y": 0, "width": 400, "height": 300})));
    vars(&mut s);
    ex(&mut s, "grid.add", json!({"ids": [f], "grid": {"kind": "columns", "count": 4, "gutter": 20}}));
    ex(&mut s, "grid.add", json!({"ids": [f], "grid": {"kind": "rows", "count": 3}}));
    ex(&mut s, "variable.bind", json!({"ids": [f], "field": "layoutGrids/0/gutter", "variable": "gap"}));
    ex(&mut s, "variable.bind", json!({"ids": [f], "field": "layoutGrids/0/count", "variable": "cols"}));
    let g = node(&s, f).grids[0].clone();
    assert_eq!((g.kind, g.count, g.gutter), (GridKind::Columns, 6, 12.0));
    // Values and modes flow in.
    ex(&mut s, "variable.set", json!({"variable": "gap", "mode": "Dark", "value": 32}));
    ex(&mut s, "variable.setMode", json!({"collection": "T", "mode": "Dark", "ids": [f]}));
    assert_eq!(node(&s, f).grids[0].gutter, 32.0);
    // Junk numbers are kept in range.
    ex(&mut s, "variable.set", json!({"variable": "cols", "mode": "Dark", "value": -5}));
    assert_eq!(node(&s, f).grids[0].count, 1);
    ex(&mut s, "variable.set", json!({"variable": "cols", "mode": "Dark", "value": 1e12}));
    assert_eq!(node(&s, f).grids[0].count, 1000);
    ex(&mut s, "variable.set", json!({"variable": "cols", "mode": "Dark", "value": 5}));
    // Move the bound grid on top: its bindings go with it, the rows grid isn't touched.
    ex(&mut s, "grid.move", json!({"ids": [f], "from": 0, "to": 1}));
    let keys: Vec<String> = node(&s, f).bound.keys().cloned().collect();
    assert_eq!(keys, ["layoutGrids/1/count", "layoutGrids/1/gutter"]);
    ex(&mut s, "variable.set", json!({"variable": "gap", "mode": "Dark", "value": 8}));
    let gs = node(&s, f).grids;
    assert_eq!((gs[0].kind, gs[0].count), (GridKind::Rows, 3));
    assert_eq!((gs[1].kind, gs[1].count, gs[1].gutter), (GridKind::Columns, 5, 8.0));
    // Removing the grid below shifts them down; removing the bound grid drops them.
    ex(&mut s, "grid.remove", json!({"ids": [f], "index": 0}));
    assert!(node(&s, f).bound.keys().all(|k| k.starts_with("layoutGrids/0/")));
    ex(&mut s, "grid.remove", json!({"ids": [f], "index": 0}));
    assert!(node(&s, f).bound.is_empty());
    ex(&mut s, "edit.undo", json!({}));
    assert_eq!(node(&s, f).bound.len(), 2);
}

#[test]
fn instance_properties_bind_to_boolean_and_string_variables() {
    let mut s = Session::new();
    let f = id(ex(&mut s, "frame.create", json!({"x": 0, "y": 0, "width": 200, "height": 100, "name": "Card"})));
    let title = id(ex(&mut s, "text.create", json!({"text": "Hello", "x": 10, "y": 10, "parent": f})));
    let dot = id(ex(&mut s, "shape.ellipse", json!({"x": 170, "y": 10, "width": 20, "height": 20, "parent": f})));
    let c = id(ex(&mut s, "component.create", json!({"ids": [f]})));
    ex(&mut s, "component.addProperty", json!({"id": dot, "name": "Show dot", "type": "boolean", "default": true}));
    ex(&mut s, "component.addProperty", json!({"id": c, "name": "Title", "type": "text", "layer": title}));
    let i = id(ex(&mut s, "instance.create", json!({"component": c, "x": 300, "y": 0})));
    let copy = |s: &Session, src: NodeId| {
        let doc = s.doc().unwrap();
        doc.subtree(i).into_iter().find(|x| doc.node(*x).and_then(|n| n.source) == Some(src)).unwrap()
    };
    let (ititle, idot) = (copy(&s, title), copy(&s, dot));
    ex(&mut s, "variable.createCollection", json!({"name": "Flags", "modes": ["Short", "Long"]}));
    ex(&mut s, "variable.create", json!({"collection": "Flags", "name": "badge", "type": "boolean", "value": false}));
    ex(&mut s, "variable.create", json!({"collection": "Flags", "name": "label", "type": "string", "value": "Hi"}));
    ex(&mut s, "variable.set", json!({"variable": "label", "mode": "Long", "value": "Hello there"}));
    // Types must fit, and only instances take them.
    assert!(s.execute("variable.bind", &json!({"ids": [i], "field": "componentProperties/Show dot", "variable": "label"})).is_err());
    assert!(s.execute("variable.bind", &json!({"ids": [i], "field": "componentProperties/Title", "variable": "badge"})).is_err());
    assert!(s.execute("variable.bind", &json!({"ids": [i], "field": "componentProperties/Nope", "variable": "badge"})).is_err());
    assert!(s.execute("variable.bind", &json!({"ids": [c], "field": "componentProperties/Show dot", "variable": "badge"})).is_err());
    ex(&mut s, "variable.bind", json!({"ids": [i], "field": "componentProperties/Show dot", "variable": "badge"}));
    ex(&mut s, "variable.bind", json!({"ids": [i], "field": "componentProperties/Title", "variable": "label"}));
    assert!(!node(&s, idot).visible, "the bound boolean hides the dot");
    assert_eq!(node(&s, ititle).kind.text().unwrap().characters, "Hi");
    // Values and modes flow in.
    ex(&mut s, "variable.set", json!({"variable": "badge", "value": true}));
    assert!(node(&s, idot).visible);
    ex(&mut s, "variable.setMode", json!({"collection": "Flags", "mode": "Long", "ids": [i]}));
    assert_eq!(node(&s, ititle).kind.text().unwrap().characters, "Hello there");
    assert!(node(&s, ititle).overrides.is_empty(), "property values aren't overrides");
    // A main edit rebuilds the instance: its bindings stay.
    ex(&mut s, "node.set", json!({"id": dot, "props": {"fill": "#00FF00"}}));
    assert!(node(&s, i).bound.contains_key("componentProperties/Title"));
    ex(&mut s, "variable.set", json!({"variable": "label", "mode": "Long", "value": "After edit"}));
    assert_eq!(node(&s, ititle).kind.text().unwrap().characters, "After edit");
    // Renaming the property keeps the binding; deleting it drops the binding.
    ex(&mut s, "component.editProperty", json!({"id": c, "name": "Title", "newName": "Heading"}));
    assert!(node(&s, i).bound.contains_key("componentProperties/Heading"));
    ex(&mut s, "variable.set", json!({"variable": "label", "mode": "Long", "value": "Renamed"}));
    assert_eq!(node(&s, ititle).kind.text().unwrap().characters, "Renamed");
    ex(&mut s, "component.deleteProperty", json!({"id": c, "name": "Heading"}));
    assert!(!node(&s, i).bound.keys().any(|k| k.contains("Heading")));
    assert!(matches!(node(&s, i).kind, NodeKind::Instance { .. }));
    // Unbinding keeps the value (the Long mode's `false`).
    assert!(!node(&s, idot).visible);
    ex(&mut s, "variable.bind", json!({"ids": [i], "field": "componentProperties/Show dot", "variable": null}));
    ex(&mut s, "variable.set", json!({"variable": "badge", "mode": "Long", "value": true}));
    assert!(!node(&s, idot).visible);
    ex(&mut s, "variable.set", json!({"variable": "badge", "mode": "Long", "value": false}));
    // Resetting the instance drops its property bindings with its values; detaching too.
    ex(&mut s, "variable.bind", json!({"ids": [i], "field": "componentProperties/Show dot", "variable": "badge"}));
    assert!(!node(&s, idot).visible);
    ex(&mut s, "component.resetInstance", json!({"ids": [i]}));
    assert!(node(&s, i).bound.is_empty());
    assert!(node(&s, idot).visible, "back to the default");
    ex(&mut s, "variable.bind", json!({"ids": [i], "field": "componentProperties/Show dot", "variable": "badge"}));
    ex(&mut s, "component.detach", json!({"ids": [i]}));
    assert!(node(&s, i).bound.is_empty());
}
