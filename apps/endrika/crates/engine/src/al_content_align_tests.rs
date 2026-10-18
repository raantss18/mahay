//! Wrapped-line alignment through commands: the `contentAlign` prop and its alias, undo, the CSS
//! and inspect output, instance overrides, the REST and `.fig` mappings, and junk values.

use serde_json::{Value, json};

use crate::params::id_of;
use crate::{EngineError, Session};
use figcraft_doc::{ContentAlign, NodeId};

fn id(v: Value) -> NodeId {
    id_of(&v["id"]).unwrap()
}

/// A 100 × 100 wrapping frame of three 40 × 20 rectangles (gap 10, row gap 5): two rows.
fn wrapping(s: &mut Session) -> (NodeId, Vec<NodeId>) {
    let props = json!({"layout": "horizontal", "wrap": true, "padding": 0, "gap": 10, "crossGap": 5});
    let f = id(s.execute("frame.create", &json!({"x": 0, "y": 0, "width": 100, "height": 100, "props": props})).unwrap());
    let kids = (0..3).map(|_| id(s.execute("shape.rectangle", &json!({"parent": f, "x": 0, "y": 0, "width": 40, "height": 20})).unwrap())).collect();
    (f, kids)
}

fn ys(s: &Session, kids: &[NodeId]) -> Vec<f64> {
    kids.iter().map(|k| s.doc().unwrap().node(*k).unwrap().transform.as_coeffs()[5]).collect()
}

fn content(s: &Session, f: NodeId) -> ContentAlign {
    s.doc().unwrap().node(f).unwrap().layout.as_ref().unwrap().content_align
}

#[test]
fn content_align_spreads_the_rows_and_undoes() {
    let mut s = Session::new();
    let (f, kids) = wrapping(&mut s);
    assert_eq!(ys(&s, &kids), [0.0, 0.0, 25.0]);
    s.execute("node.set", &json!({"id": f, "props": {"contentAlign": "spaceBetween"}})).unwrap();
    assert_eq!(ys(&s, &kids), [0.0, 0.0, 80.0]);
    s.execute("edit.undo", &json!({})).unwrap();
    assert_eq!((content(&s, f), ys(&s, &kids)), (ContentAlign::Auto, vec![0.0, 0.0, 25.0]));
    // The file format's name and spelling work too; packed rows follow the cross alignment.
    s.execute("node.set", &json!({"id": f, "props": {"counterAxisAlignContent": "SPACE_BETWEEN"}})).unwrap();
    assert_eq!(content(&s, f), ContentAlign::SpaceBetween);
    s.execute("node.set", &json!({"id": f, "props": {"contentAlign": "auto", "crossAlign": "max"}})).unwrap();
    assert_eq!(ys(&s, &kids), [55.0, 55.0, 80.0]);
}

#[test]
fn css_and_inspect_name_the_lines() {
    let mut s = Session::new();
    let (f, _) = wrapping(&mut s);
    let css = |s: &mut Session| s.execute("inspect.code", &json!({"id": f})).unwrap().to_string();
    assert!(css(&mut s).contains("align-content: flex-start;"), "{}", css(&mut s));
    s.execute("node.set", &json!({"id": f, "props": {"crossAlign": "center"}})).unwrap();
    assert!(css(&mut s).contains("align-content: center;"));
    s.execute("node.set", &json!({"id": f, "props": {"contentAlign": "spaceBetween"}})).unwrap();
    assert!(css(&mut s).contains("align-content: space-between;"));
    let rows = s.execute("inspect.properties", &json!({"id": f})).unwrap().to_string();
    assert!(rows.contains("Auto (space between)"), "{rows}");
    // One line has no lines to align.
    s.execute("node.set", &json!({"id": f, "props": {"wrap": false}})).unwrap();
    assert!(!css(&mut s).contains("align-content"));
}

#[test]
fn an_instance_keeps_its_own_line_alignment() {
    let mut s = Session::new();
    let (f, _) = wrapping(&mut s);
    let comp = id(s.execute("component.create", &json!({"ids": [f]})).unwrap());
    let inst = id(s.execute("instance.create", &json!({"component": comp, "x": 200, "y": 0})).unwrap());
    s.execute("node.set", &json!({"id": inst, "props": {"contentAlign": "spaceBetween"}})).unwrap();
    // A change to the main reaches the instance; its own alignment stays.
    s.execute("node.set", &json!({"id": comp, "props": {"padding": 2}})).unwrap();
    assert_eq!((content(&s, comp), content(&s, inst)), (ContentAlign::Auto, ContentAlign::SpaceBetween));
}

#[test]
fn rest_and_fig_carry_the_setting() {
    let frame = |extra: Value| {
        let mut f = json!({"id": "1:2", "name": "Wrap", "type": "FRAME", "relativeTransform": [[1, 0, 0], [0, 1, 0]], "size": {"x": 100, "y": 100},
            "layoutMode": "HORIZONTAL", "layoutWrap": "WRAP", "children": []});
        f.as_object_mut().unwrap().extend(extra.as_object().unwrap().clone());
        json!({"name": "File", "document": {"id": "0:0", "type": "DOCUMENT", "children": [{"id": "0:1", "name": "Page", "type": "CANVAS", "children": [f]}]}})
    };
    let read = |file: &Value| {
        let (doc, _) = figcraft_format::rest::import(file, &figcraft_format::rest::no_images).unwrap();
        doc
    };
    let of = |doc: &figcraft_doc::Document| doc.nodes().find(|n| n.name == "Wrap").unwrap().layout.as_ref().unwrap().content_align;
    assert_eq!(of(&read(&frame(json!({})))), ContentAlign::Auto);
    assert_eq!(of(&read(&frame(json!({"counterAxisAlignContent": "AUTO"})))), ContentAlign::Auto);
    let spread = read(&frame(json!({"counterAxisAlignContent": "SPACE_BETWEEN"})));
    assert_eq!(of(&spread), ContentAlign::SpaceBetween);
    // Written to `.fig` and read back, both values survive.
    for doc in [spread, read(&frame(json!({})))] {
        let (bytes, _) = figcraft_format::fig::export(&doc, None).unwrap();
        let (back, _) = figcraft_format::fig::import(&bytes, "x").unwrap();
        assert_eq!(of(&back), of(&doc));
    }
}

#[test]
fn junk_values_are_refused_not_fatal() {
    let junk =
        [json!(null), json!(true), json!(-1), json!(1e308), json!(""), json!("sideways"), json!([1, 2]), json!({"a": 1}), json!("x".repeat(10_000))];
    for k in ["contentAlign", "counterAxisAlignContent"] {
        for j in &junk {
            let mut s = Session::new();
            let (f, kids) = wrapping(&mut s);
            for target in [f, kids[0]] {
                let r = s.execute("node.set", &json!({"id": target, "props": {k: j}}));
                assert!(!matches!(r, Err(EngineError::Internal { .. })), "{k} = {j}: {r:?}");
            }
            assert_eq!(content(&s, f), ContentAlign::Auto, "{k} = {j}");
        }
    }
}
