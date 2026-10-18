//! Range fills on text: set on a range, cleared back to the layer's fill.

use serde_json::json;

use crate::Session;
use crate::params::id_of;

#[test]
fn range_fill_sets_and_clears() {
    let mut s = Session::new();
    let t = id_of(&s.execute("text.create", &json!({"text": "Hello brand world", "x": 0, "y": 0})).unwrap()["id"]).unwrap();
    s.execute("text.style", &json!({"id": t, "start": 6, "end": 11, "textFills": ["#0D99FF"]})).unwrap();
    let fill_at = |s: &Session, i: usize| {
        let n = s.doc().unwrap().node(t).unwrap().clone();
        let tx = n.kind.text().unwrap().clone();
        tx.style_at(i).fills.clone().unwrap_or(n.fills).first().and_then(|p| p.solid_color()).map(|c| c.to_hex())
    };
    assert_eq!(fill_at(&s, 7), Some("0D99FF".into()));
    assert_ne!(fill_at(&s, 1), Some("0D99FF".into()));
    s.execute("text.style", &json!({"id": t, "start": 6, "end": 11, "textFills": null})).unwrap();
    assert_eq!(fill_at(&s, 7), fill_at(&s, 1), "back to the layer's fill");
    s.undo().unwrap();
    assert_eq!(fill_at(&s, 7), Some("0D99FF".into()));
}
