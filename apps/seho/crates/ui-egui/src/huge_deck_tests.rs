//! Large presentations in the editor: views that list every slide lay out only what is in view.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use egui::accesskit::Role;
use egui_kittest::kittest::Queryable;
use serde_json::json;

use crate::{Services, SlideApp};
use deckcraft_engine::Session;

fn harness(n: usize) -> egui_kittest::Harness<'static, SlideApp> {
    let mut s = Session::with_new();
    for _ in 1..n {
        s.execute("slide.new", &json!({})).unwrap();
    }
    egui_kittest::Harness::builder().with_size(egui::vec2(1400.0, 900.0)).build_ui_state(
        |ui, app: &mut SlideApp| {
            app.logic(ui.ctx());
            app.ui(ui);
        },
        SlideApp::new(s, Services::default()),
    )
}

/// Outline view of 2000 slides lays out the slides in view (and the current one), not all 2000
/// title editors every frame (40 ms a frame in a release build, growing with the text).
#[test]
fn the_outline_of_a_huge_deck_lays_out_only_what_is_in_view() {
    let mut h = harness(2000);
    h.state_mut().run("view.outline", json!({})).unwrap();
    h.run_steps(3);
    let editors = h.query_all_by_role(Role::TextInput).count();
    assert!(editors > 5 && editors < 200, "{editors} text editors laid out");
    // The current slide is always laid out, wherever it is.
    h.state_mut().run("slide.go", json!({"index": 1500})).unwrap();
    h.run_steps(3);
    assert!(h.query_all_by_role(Role::TextInput).count() < 200);
    assert_eq!(h.state().session.doc().unwrap().selection.slide, 1500);
}
