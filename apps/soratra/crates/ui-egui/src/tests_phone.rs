//! D23: at phone, tablet and desktop widths no two clickable widgets overlap and none lies outside
//! the window, on every tab of the ribbon.

use mahay_ui::adaptive::audit_excluding;
use serde_json::json;
use soratra_engine::Session;

use crate::WordApp;

const WIDTHS: [f32; 4] = [360.0, 400.0, 600.0, 840.0];

fn findings_at(width: f32, tab: &str) -> Vec<String> {
    let app = WordApp::new(Session::new(soratra_engine::sample::sample_document()), crate::Services::default());
    let height = if width < 600.0 { 780.0 } else { 900.0 };
    let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(width, height)).build_ui_state(
        |ui, app: &mut WordApp| {
            let ctx = ui.ctx().clone();
            app.logic(&ctx);
            app.ui(ui);
        },
        app,
    );
    h.state_mut().run("ui.tab", json!({"tab": tab})).unwrap();
    h.run_steps(4);
    let skip: Vec<egui::Rect> = h.state().canvas.viewport.into_iter().collect();
    let names = |r: egui::Rect| -> String {
        use egui_kittest::kittest::NodeT;
        h.root()
            .children_recursive()
            .filter(|n| {
                n.accesskit_node().bounding_box().is_some_and(|b| {
                    [(b.x0 as f32 - r.min.x), (b.y0 as f32 - r.min.y), (b.x1 as f32 - r.max.x), (b.y1 as f32 - r.max.y)].iter().all(|d| d.abs() < 1.5)
                })
            })
            .filter_map(|n| n.accesskit_node().label().map(|l| l.to_string()))
            .collect::<Vec<_>>()
            .join("|")
    };
    audit_excluding(&h.ctx, &skip)
        .into_iter()
        .map(|f| format!("{width} {tab}: {:?} {:?} [{}] {:?} [{}]", f.kind, f.rect, names(f.rect), f.other, f.other.map(names).unwrap_or_default()))
        .collect()
}

#[test]
fn no_overlap_and_nothing_outside_the_window_on_any_tab() {
    let mut bad = Vec::new();
    for width in WIDTHS {
        for tab in crate::ribbon::TABS.iter().filter(|t| **t != "File") {
            bad.extend(findings_at(width, tab));
        }
    }
    bad.sort();
    bad.dedup();
    assert!(bad.is_empty(), "{} problems:\n{}", bad.len(), bad.join("\n"));
}

fn touch(phase: egui::TouchPhase, pos: egui::Pos2) -> egui::Event {
    egui::Event::Touch { device_id: egui::TouchDeviceId(0), id: egui::TouchId(0), phase, pos, force: None }
}

fn button(pos: egui::Pos2, pressed: bool) -> egui::Event {
    egui::Event::PointerButton { pos, button: egui::PointerButton::Primary, pressed, modifiers: Default::default() }
}

/// A finger dragged over the text scrolls the page and selects nothing; a long press first would select.
#[test]
fn a_finger_drag_scrolls_the_page_and_does_not_select() {
    let app = WordApp::new(Session::new(soratra_engine::sample::sample_document()), crate::Services::default());
    let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(400.0, 780.0)).build_ui_state(
        |ui, app: &mut WordApp| {
            let ctx = ui.ctx().clone();
            app.logic(&ctx);
            app.ui(ui);
        },
        app,
    );
    h.run_steps(6);
    let before = h.state().canvas.scroll_offset;
    let start = egui::pos2(200.0, 600.0);
    let events = |h: &mut egui_kittest::Harness<'_, WordApp>, e: Vec<egui::Event>| {
        h.input_mut().events.extend(e);
        h.run_steps(1);
    };
    events(&mut h, vec![touch(egui::TouchPhase::Start, start), egui::Event::PointerMoved(start), button(start, true)]);
    let mut at = start;
    for _ in 0..10 {
        at.y -= 25.0;
        events(&mut h, vec![touch(egui::TouchPhase::Move, at), egui::Event::PointerMoved(at)]);
    }
    events(&mut h, vec![touch(egui::TouchPhase::End, at), button(at, false)]);
    h.run_steps(3);
    let after = h.state().canvas.scroll_offset;
    assert!(after.y > before.y + 100.0, "the page followed the finger: {before:?} → {after:?}");
    assert!(h.state().session.sel.is_collapsed(), "dragging with a finger selected text");
}
