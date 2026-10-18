//! The component playground: a component tried with other variant, boolean and text property
//! values on a copy. The code and the preview follow the values; the document, the selection and
//! the undo history don't change.

use std::sync::Arc;

use serde_json::{Value, json};

use crate::params::id_of;
use crate::{EngineError, Session};
use figcraft_doc::NodeId;

fn id(v: Value) -> NodeId {
    id_of(&v["id"]).unwrap()
}

fn run(s: &mut Session, cmd: &str, p: Value) -> Value {
    s.execute(cmd, &p).unwrap_or_else(|e| panic!("{cmd} {p}: {e}"))
}

/// A "Button" set with variants State=Rest (blue) and State=Hover (red), each holding a label
/// (text property "Label") and an icon (boolean property "Icon"); an instance of it. Returns
/// (set, instance).
fn button(s: &mut Session) -> (NodeId, NodeId) {
    let mut comps = Vec::new();
    for (i, (state, fill)) in [("Rest", "#0000FF"), ("Hover", "#FF0000")].into_iter().enumerate() {
        let f = id(run(
            s,
            "frame.create",
            json!({"x": 200 * i, "y": 0, "width": 120, "height": 40, "name": format!("State={state}"), "props": {"fill": fill}}),
        ));
        let label = id(run(s, "text.create", json!({"text": "Buy", "x": 40, "y": 10, "parent": f, "name": "Label"})));
        let icon = id(run(
            s,
            "shape.ellipse",
            json!({"x": 8, "y": 8, "width": 24, "height": 24, "parent": f, "props": {"fill": "#FFFFFF", "name": "Icon"}}),
        ));
        let c = id(run(s, "component.create", json!({"ids": [f]})));
        run(s, "component.addProperty", json!({"id": c, "name": "Label", "type": "text", "default": "Buy"}));
        run(s, "component.addProperty", json!({"id": c, "name": "Icon", "type": "boolean", "default": true}));
        run(s, "component.bindProperty", json!({"id": label, "property": "Label", "field": "characters"}));
        run(s, "component.bindProperty", json!({"id": icon, "property": "Icon", "field": "visible"}));
        comps.push(c);
    }
    let set = id(run(s, "component.combineVariants", json!({"ids": comps})));
    let inst = id(run(s, "instance.create", json!({"component": set, "x": 0, "y": 300})));
    (set, inst)
}

fn value<'a>(out: &'a Value, name: &str) -> &'a Value {
    out["properties"].as_array().unwrap().iter().find(|p| p["name"] == name).map(|p| &p["value"]).unwrap_or(&Value::Null)
}

#[test]
fn the_playground_tries_values_without_touching_the_document() {
    let mut s = Session::new();
    let (set, inst) = button(&mut s);
    let (before, undo, selection) = (s.ds().unwrap().doc.clone(), s.undo_depth(), s.selection().to_vec());
    // As the instance is: the default variant, its label, its icon.
    let out = run(&mut s, "component.playground", json!({"id": inst}));
    assert_eq!((value(&out, "State"), value(&out, "Label"), value(&out, "Icon")), (&json!("Rest"), &json!("Buy"), &json!(true)));
    let rest = out["code"].as_str().unwrap().to_string();
    // Other values: the reported values and the code follow.
    let props = json!({"State": "Hover", "Label": "Pay now", "Icon": false});
    let out = run(&mut s, "component.playground", json!({"id": inst, "properties": props}));
    assert_eq!((value(&out, "State"), value(&out, "Label"), value(&out, "Icon")), (&json!("Hover"), &json!("Pay now"), &json!(false)));
    let code = out["code"].as_str().unwrap();
    assert!(code != rest && code.contains("Pay now"), "{code}");
    // From the set or one of its variants: the same playground, starting from the defaults.
    for target in [set, s.doc().unwrap().node(set).unwrap().children[1]] {
        let out = run(&mut s, "component.playground", json!({"id": target, "properties": {"Label": "Hi"}}));
        assert_eq!(value(&out, "Label"), &json!("Hi"), "{target}");
    }
    // An instance with values of its own starts from them.
    run(&mut s, "instance.setProperties", json!({"ids": [inst], "properties": {"Label": "Mine"}}));
    let (before2, undo2) = (s.ds().unwrap().doc.clone(), s.undo_depth());
    assert_eq!(value(&run(&mut s, "component.playground", json!({"id": inst, "properties": {"Icon": false}})), "Label"), &json!("Mine"));
    // Nothing changed: the same document, undo history and selection.
    assert!(Arc::ptr_eq(&before2, &s.ds().unwrap().doc) && undo2 == s.undo_depth());
    assert!(!Arc::ptr_eq(&before, &before2) && undo != undo2, "(only the instance edit above did)");
    assert_eq!(s.selection(), selection);
}

#[test]
fn the_preview_shows_the_values() {
    let mut s = Session::new();
    let (_, inst) = button(&mut s);
    let doc = s.doc().unwrap().clone();
    let pixel = |values: Value, x: u32, y: u32| {
        let (copy, shown) = crate::playground::build(&doc, inst, values.as_object().unwrap()).unwrap();
        let pix = figcraft_render::render_node(&copy, shown, 1.0).unwrap();
        let i = ((y * pix.width() + x) * 4) as usize;
        [pix.data()[i], pix.data()[i + 1], pix.data()[i + 2]]
    };
    // (values, pixel, colour, why)
    for (values, at, want, why) in [
        (json!({}), (100, 5), [0, 0, 255], "the rest variant is blue"),
        (json!({"State": "Hover"}), (100, 5), [255, 0, 0], "the hover variant is red"),
        (json!({}), (20, 20), [255, 255, 255], "the icon shows"),
        (json!({"Icon": false}), (20, 20), [0, 0, 255], "the icon is hidden: the fill shows"),
    ] {
        assert_eq!(pixel(values.clone(), at.0, at.1), want, "{values}: {why}");
    }
    // A PNG on request.
    let dir = std::env::temp_dir().join(format!("figcraft-playground-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("p.png");
    let out = run(&mut s, "component.playground", json!({"id": inst, "path": path.to_string_lossy(), "scale": 1}));
    assert_eq!((out["width"].as_f64(), out["height"].as_f64()), (Some(120.0), Some(40.0)));
    assert!(std::fs::read(&path).unwrap().starts_with(b"\x89PNG"));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn what_is_no_component_or_no_value_is_refused() {
    let mut s = Session::new();
    let (_, inst) = button(&mut s);
    let r = id(run(&mut s, "shape.rectangle", json!({"x": 500, "y": 500, "width": 10, "height": 10})));
    assert!(s.execute("component.playground", &json!({"id": r})).unwrap_err().to_string().contains("select an instance"));
    assert!(s.execute("component.playground", &json!({"id": inst, "properties": {"State": "Nope"}})).is_err());
    assert!(s.execute("component.playground", &json!({"id": inst, "language": "cobol"})).is_err());
    let before = s.ds().unwrap().doc.clone();
    for j in [json!(null), json!(true), json!(-1), json!("x"), json!([1, [2]]), json!({"a": {"b": 1}}), json!("y".repeat(5000))] {
        for p in [
            json!({"id": j}),
            json!({"id": inst, "properties": j}),
            json!({"id": inst, "properties": {"Label": j, "Icon": j, "State": j}}),
            json!({"id": inst, "language": j, "scale": j}),
        ] {
            let r = s.execute("component.playground", &p);
            assert!(!matches!(r, Err(EngineError::Internal { .. })), "{p}: {r:?}");
        }
    }
    assert!(Arc::ptr_eq(&before, &s.ds().unwrap().doc));
}
