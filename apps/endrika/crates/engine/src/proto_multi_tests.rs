//! Interactions on several layers at once.

use serde_json::json;

use crate::Session;
use crate::params::id_of;

#[test]
fn edit_and_replace_interactions_on_several_layers() {
    let mut s = Session::new();
    let id = |v: serde_json::Value| id_of(&v["id"]).unwrap();
    let home = id(s.execute("frame.create", &json!({"x": 0, "y": 0, "width": 100, "height": 100})).unwrap());
    let next = id(s.execute("frame.create", &json!({"x": 200, "y": 0, "width": 100, "height": 100})).unwrap());
    let a = id(s.execute("shape.rectangle", &json!({"x": 0, "y": 0, "width": 10, "height": 10, "parent": home})).unwrap());
    let b = id(s.execute("shape.rectangle", &json!({"x": 20, "y": 0, "width": 10, "height": 10, "parent": home})).unwrap());
    let count = |s: &Session, i| s.doc().unwrap().node(i).unwrap().interactions.len();
    // Added to both, edited on both.
    s.execute("proto.interaction.add", &json!({"ids": [a, b], "dest": next})).unwrap();
    s.execute("proto.interaction.set", &json!({"ids": [a, b], "index": 0, "durationMs": 450})).unwrap();
    for i in [a, b] {
        assert_eq!(s.doc().unwrap().node(i).unwrap().interactions[0].duration_ms, 450);
    }
    // Mixed: replace both with one new interaction in one step.
    s.execute("proto.interaction.add", &json!({"ids": [a], "action": {"type": "back"}})).unwrap();
    assert_eq!((count(&s, a), count(&s, b)), (2, 1));
    let steps = json!([
        {"command": "proto.interaction.clear", "params": {"ids": [a, b]}},
        {"command": "proto.interaction.add", "params": {"ids": [a, b], "trigger": {"type": "onClick"}, "action": {"type": "back"}}},
    ]);
    s.execute("command.batch", &json!({"label": "Replace interactions", "commands": steps})).unwrap();
    assert_eq!((count(&s, a), count(&s, b)), (1, 1));
    assert_eq!(s.doc().unwrap().node(a).unwrap().interactions, s.doc().unwrap().node(b).unwrap().interactions);
    s.undo().unwrap();
    assert_eq!((count(&s, a), count(&s, b)), (2, 1), "one undo step");
    s.execute("proto.interaction.clear", &json!({"ids": [a, b]})).unwrap();
    assert_eq!((count(&s, a), count(&s, b)), (0, 0));
}
