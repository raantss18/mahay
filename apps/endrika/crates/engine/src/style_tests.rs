//! Effect and grid styles through the session: create from a layer, apply, edits propagate,
//! detach.

use serde_json::json;

use crate::Session;
use crate::params::id_of;

#[test]
fn effect_and_grid_styles_apply_propagate_and_detach() {
    let mut s = Session::new();
    let a = id_of(&s.execute("frame.create", &json!({"x": 0, "y": 0, "width": 100, "height": 100})).unwrap()["id"]).unwrap();
    let b = id_of(&s.execute("frame.create", &json!({"x": 200, "y": 0, "width": 100, "height": 100})).unwrap()["id"]).unwrap();
    s.execute("effect.add", &json!({"ids": [a], "type": "dropShadow"})).unwrap();
    s.execute("grid.add", &json!({"ids": [a]})).unwrap();
    let fx = s.execute("style.create", &json!({"type": "effect", "name": "Shadow/md", "from": a})).unwrap();
    let fx = fx["id"].as_str().unwrap().to_string();
    let gr = s.execute("style.create", &json!({"type": "grid", "name": "Grid 8", "from": a})).unwrap();
    let gr = gr["id"].as_str().unwrap().to_string();
    s.execute("style.apply", &json!({"ids": [b], "style": fx})).unwrap();
    s.execute("style.apply", &json!({"ids": [b], "style": gr})).unwrap();
    let node = |s: &Session, id| s.doc().unwrap().node(id).unwrap().clone();
    assert_eq!(node(&s, b).effects.len(), 1);
    assert_eq!(node(&s, b).grids.len(), 1);
    assert_eq!(node(&s, b).styles.effect.as_deref(), Some(fx.as_str()));
    // Redefining the style from a layer without effects clears them on linked layers.
    let c = id_of(&s.execute("frame.create", &json!({"x": 400, "y": 0, "width": 10, "height": 10})).unwrap()["id"]).unwrap();
    s.execute("style.update", &json!({"style": fx, "from": c})).unwrap();
    assert!(node(&s, b).effects.is_empty(), "linked layers follow the style");
    // Detach keeps the values and drops the link.
    s.execute("style.detach", &json!({"ids": [b], "which": "grid"})).unwrap();
    assert!(node(&s, b).styles.grid.is_none());
    assert_eq!(node(&s, b).grids.len(), 1);
}
