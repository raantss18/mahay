//! Pixel preview: toggling it, and its 1x / 2x resolution.

use serde_json::json;

use crate::Session;

fn preview(s: &Session) -> (bool, u8) {
    let v = &s.ds().unwrap().view;
    (v.pixel_preview, v.pixel_scale)
}

#[test]
fn toggles_and_sets_its_resolution() {
    let mut s = Session::new();
    assert_eq!(preview(&s), (false, 1));
    s.execute("view.pixelPreview", &json!({})).unwrap();
    assert_eq!(preview(&s), (true, 1));
    s.execute("view.pixelPreview", &json!({})).unwrap();
    assert_eq!(preview(&s), (false, 1));
    // A resolution turns it on.
    let r = s.execute("view.pixelPreview", &json!({"scale": 2})).unwrap();
    assert_eq!(r, json!({"on": true, "scale": 2}));
    assert_eq!(preview(&s), (true, 2));
    // Turning it off keeps the resolution for next time.
    s.execute("view.pixelPreview", &json!({"on": false})).unwrap();
    assert_eq!(preview(&s), (false, 2));
    s.execute("view.pixelPreview", &json!({"on": true, "scale": 1})).unwrap();
    assert_eq!(preview(&s), (true, 1));
}

#[test]
fn rejects_other_resolutions() {
    let mut s = Session::new();
    for bad in [json!(3), json!(0), json!(1.5), json!("2"), json!(-1)] {
        assert!(s.execute("view.pixelPreview", &json!({"scale": bad})).is_err(), "{bad}");
    }
    assert_eq!(preview(&s), (false, 1));
}
