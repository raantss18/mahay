//! The branch review dialog, driven headless (the harness is `present_tests`'): it opens on a
//! branch with a conflict, keeps chosen sides while they still name a conflict, and closes with
//! Escape.

use egui::{Key, pos2};
use figcraft_engine::Session;
use figcraft_engine::branches::Side;
use serde_json::json;

use crate::present_tests::{Ui, id};

#[test]
fn review_dialog_opens_keeps_choices_and_closes() {
    let mut s = Session::new();
    let f = id(&s.execute("frame.create", &json!({"x": 0, "y": 0, "width": 300, "height": 200})).unwrap());
    let r = id(&s.execute("shape.rectangle", &json!({"x": 20, "y": 20, "width": 50, "height": 50, "parent": f})).unwrap());
    let b = s.execute("branch.create", &json!({"name": "Try"})).unwrap()["id"].as_str().unwrap().to_string();
    s.execute("fill.set", &json!({"ids": [r], "color": "#FF0000"})).unwrap();
    s.execute("doc.switch", &json!({"index": 0})).unwrap();
    s.execute("fill.set", &json!({"ids": [r], "color": "#0000FF"})).unwrap();
    s.execute("doc.switch", &json!({"index": 1})).unwrap();
    let mut u = Ui::new(s);
    u.frame(vec![]);
    crate::branch_ui::open(&mut u.app, None);
    let key = format!("{r}/fills");
    if let Some(v) = u.app.ui.branch_review.as_mut() {
        v.picks.insert(key.clone(), Side::Branch);
        v.picks.insert("stale/key".into(), Side::Main);
    }
    u.frames(2, Vec::new);
    let state = &u.app.inspect()["branchReview"];
    assert_eq!(state["id"], json!(b));
    assert_eq!(state["picks"], json!({key: "branch"}), "choices for conflicts that no longer exist are dropped");
    u.key(pos2(700.0, 450.0), Key::Escape);
    u.frame(vec![]);
    assert!(u.app.ui.branch_review.is_none());
}
