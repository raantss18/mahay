//! The `entities` query (control channel `engine.execute`, MCP `query_entities`) rejects malformed
//! filters instead of ignoring them and returning everything.

use cadcraft_engine::Session;
use serde_json::json;

#[test]
fn malformed_filters_are_errors() {
    let mut s = Session::new();
    s.execute("circle", &json!({"center": [0, 0], "radius": 1})).unwrap();
    s.execute("circle", &json!({"center": [100, 100], "radius": 1})).unwrap();
    for p in [
        json!({"window": [-5, -5, 5, 5]}),
        json!({"window": [[-5, -5]]}),
        json!({"window": [[-5, -5], [5, 5], [9, 9]]}),
        json!({"window": {"min": [-5, -5], "max": [5, 5]}}),
        json!({"window": "x"}),
        json!({"type": 7}),
        json!({"layer": ["0"]}),
        json!({"limit": -1}),
        json!({"offset": "1"}),
        json!({"crossing": "yes"}),
    ] {
        assert!(s.execute("entities", &p).is_err(), "{p} must be rejected");
    }
    let r = s.execute("entities", &json!({"window": [[-5, -5], [5, 5]]})).unwrap();
    assert_eq!(r["count"], 1, "{r}");
}
