//! The Animation Pane's advanced timeline driven with the mouse: dragging a bar changes the
//! delay, dragging its right edge changes the duration, each one undoable command.

use deckcraft_engine::Session;
use egui::Pos2;
use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use serde_json::json;

use crate::{Services, SlideApp};

/// A blank slide with two shapes: Fade on click (1 s), then Zoom after previous (2 s).
fn harness() -> Harness<'static, SlideApp> {
    let mut s = Session::with_new();
    s.execute("slide.new", &json!({"layout": "blank"})).unwrap();
    for (i, (effect, start, dur)) in [("fade", "onClick", 1000), ("zoom", "afterPrevious", 2000)].iter().enumerate() {
        let id =
            s.execute("shape.insert", &json!({"preset": "rect", "rect": [40.0 + 200.0 * i as f64, 80, 150, 100]})).unwrap()["id"].as_u64().unwrap();
        s.execute("edit.select", &json!({"ids": [id]})).unwrap();
        s.execute("animation.add", &json!({"effect": effect, "start": start, "duration": dur})).unwrap();
    }
    let mut app = SlideApp::new(s, Services::default());
    app.ui.pane = Some("animation".into());
    let mut h = Harness::builder().with_size(egui::vec2(1400.0, 900.0)).with_step_dt(1.0 / 60.0).build_ui_state(
        |ui, app: &mut SlideApp| {
            app.logic(ui.ctx());
            app.ui(ui);
        },
        app,
    );
    h.run_steps(3);
    h
}

fn timing(h: &Harness<'_, SlideApp>, i: usize) -> (u32, u32) {
    let a = &h.state().session.active().unwrap().current_slide().unwrap().animations[i];
    (a.delay_ms, a.duration_ms)
}

fn drag(h: &mut Harness<'_, SlideApp>, from: Pos2, to: Pos2) {
    h.hover_at(from);
    h.run_steps(1);
    h.drag_at(from);
    h.run_steps(1);
    for i in 1..=6 {
        h.hover_at(from + (to - from) * (i as f32 / 6.0));
        h.run_steps(1);
    }
    h.drop_at(to);
    h.run_steps(3);
}

#[test]
fn after_previous_bar_starts_where_the_first_ends() {
    let h = harness();
    let a = h.get_by_label("Timing bar 1").rect();
    let b = h.get_by_label("Timing bar 2").rect();
    assert!((b.left() - a.right()).abs() < 1.5, "{a:?} {b:?}");
    assert!((b.width() - 2.0 * a.width()).abs() < 1.5, "2 s bar twice the 1 s bar");
}

#[test]
fn dragging_a_bar_sets_the_delay_and_undoes() {
    let mut h = harness();
    let a = h.get_by_label("Timing bar 1").rect();
    // One second is the width of bar 1.
    drag(&mut h, a.center(), a.center() + egui::vec2(a.width() / 2.0, 0.0));
    assert_eq!(timing(&h, 0), (500, 1000));
    h.state_mut().session.execute("edit.undo", &json!({})).unwrap();
    assert_eq!(timing(&h, 0), (0, 1000));
}

#[test]
fn dragging_the_edge_sets_the_duration() {
    let mut h = harness();
    let b = h.get_by_label("Timing bar 2").rect();
    let one_sec = b.width() / 2.0;
    let edge = h.get_by_label("Duration handle 2").rect().center();
    drag(&mut h, edge, edge - egui::vec2(one_sec, 0.0));
    assert_eq!(timing(&h, 1), (0, 1000));
}

#[test]
#[ignore = "writes a screenshot for review"]
fn screenshot() {
    let mut h = harness();
    h.run_steps(2);
    let img = h.render().unwrap();
    img.save("/tmp/animpane.png").unwrap();
}
