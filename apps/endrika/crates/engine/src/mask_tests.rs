//! Mask types through the session: the prop, inspection, undo, junk.

use serde_json::json;

use crate::Session;
use crate::params::id_of;

#[test]
fn mask_type_prop_inspects_and_undoes() {
    let mut s = Session::new();
    let m = id_of(&s.execute("shape.ellipse", &json!({"x": 0, "y": 0, "width": 50, "height": 50})).unwrap()["id"]).unwrap();
    let r = id_of(&s.execute("shape.rectangle", &json!({"x": 0, "y": 0, "width": 100, "height": 100})).unwrap()["id"]).unwrap();
    s.execute("object.mask", &json!({"ids": [m, r]})).unwrap();
    s.execute("node.set", &json!({"id": m, "props": {"maskType": "luminance"}})).unwrap();
    assert_eq!(s.doc().unwrap().node(m).unwrap().mask_type, figcraft_doc::MaskType::Luminance);
    let v = s.execute("node.get", &json!({"id": m})).unwrap();
    assert_eq!(v["maskType"], json!("luminance"), "{v}");
    s.undo().unwrap();
    assert_eq!(s.doc().unwrap().node(m).unwrap().mask_type, figcraft_doc::MaskType::Alpha);
    for bad in [json!("sparkly"), json!(3), json!(null)] {
        assert!(s.execute("node.set", &json!({"id": m, "props": {"maskType": bad}})).is_err());
    }
    // Old documents (no field) load as alpha masks; alpha isn't written back out.
    let json = serde_json::to_string(s.doc().unwrap().node(m).unwrap()).unwrap();
    assert!(!json.contains("maskType"));
}
