//! Resize to fit and handoff status.

use serde_json::json;

use crate::Session;
use crate::params::id_of;

fn id(v: serde_json::Value) -> figcraft_doc::NodeId {
    id_of(&v["id"]).unwrap()
}

#[test]
fn resize_to_fit_keeps_contents_in_place() {
    let mut s = Session::new();
    let f = id(s.execute("frame.create", &json!({"x": 0, "y": 0, "width": 400, "height": 300})).unwrap());
    let a = id(s.execute("shape.rectangle", &json!({"x": 50, "y": 40, "width": 100, "height": 50, "parent": f})).unwrap());
    let b = id(s.execute("shape.ellipse", &json!({"x": 120, "y": 100, "width": 30, "height": 30, "parent": f})).unwrap());
    let before = (s.doc().unwrap().world_bounds(a).unwrap(), s.doc().unwrap().world_bounds(b).unwrap());
    s.execute("object.resizeToFit", &json!({"ids": [f]})).unwrap();
    let doc = s.doc().unwrap();
    let fb = doc.world_bounds(f).unwrap();
    assert_eq!((fb.x0, fb.y0, fb.width(), fb.height()), (50.0, 40.0, 100.0, 90.0));
    assert_eq!((doc.world_bounds(a).unwrap(), doc.world_bounds(b).unwrap()), before, "contents don't move on the page");
    s.undo().unwrap();
    assert_eq!(s.doc().unwrap().world_bounds(f).unwrap().width(), 400.0);
    // Sections too; empty containers and plain shapes are left alone.
    let sec = id(s.execute("node.create", &json!({"type": "section", "x": 600, "y": 0, "width": 500, "height": 500})).unwrap());
    s.execute("node.reparent", &json!({"ids": [f], "parent": sec})).unwrap();
    let empty = id(s.execute("frame.create", &json!({"x": 0, "y": 600, "width": 80, "height": 80})).unwrap());
    let r = s.execute("object.resizeToFit", &json!({"ids": [sec, empty, a]})).unwrap();
    assert_eq!(r["ids"].as_array().unwrap().len(), 1);
    assert_eq!(s.doc().unwrap().world_bounds(sec).unwrap().width(), 400.0, "section wraps the frame");
    // Auto layout frames hug instead.
    s.execute("layout.add", &json!({"ids": [f]})).unwrap();
    s.execute("node.set", &json!({"id": f, "props": {"width": 600}})).unwrap();
    s.execute("object.resizeToFit", &json!({"ids": [f]})).unwrap();
    let n = s.doc().unwrap().node(f).unwrap().clone();
    assert_eq!(n.child_layout.sizing_h, figcraft_doc::Sizing::Hug);
    assert!(n.width < 600.0);
}

#[test]
fn dev_status_prop() {
    let mut s = Session::new();
    let f = id(s.execute("frame.create", &json!({"x": 0, "y": 0, "width": 40, "height": 40})).unwrap());
    s.execute("node.set", &json!({"id": f, "props": {"devStatus": "readyForDev"}})).unwrap();
    assert_eq!(s.doc().unwrap().node(f).unwrap().dev_status, figcraft_doc::DevStatus::ReadyForDev);
    assert_eq!(s.execute("node.get", &json!({"id": f})).unwrap()["devStatus"], json!("readyForDev"));
    s.execute("node.set", &json!({"id": f, "props": {"devStatus": null}})).unwrap();
    assert!(s.doc().unwrap().node(f).unwrap().dev_status.is_none());
    assert!(s.execute("node.set", &json!({"id": f, "props": {"devStatus": "shipped"}})).is_err());
}
