//! D23: at phone, tablet and desktop widths no two clickable widgets overlap and none lies outside
//! the window, on every ribbon tab.

use mahay_ui::adaptive::audit;
use tabilao_engine::Session;
use tabilao_ui_egui::SheetApp;

const WIDTHS: [f32; 4] = [360.0, 400.0, 600.0, 840.0];

fn findings_at(width: f32, tab: &str) -> Vec<String> {
    let mut session = Session::new();
    session.new_workbook();
    let mut app = SheetApp::new(session, Default::default());
    app.ui.ribbon_tab = tab.to_string();
    let height = if width < 600.0 { 780.0 } else { 900.0 };
    let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(width, height)).build_ui_state(
        |ui, app: &mut SheetApp| {
            app.logic(ui.ctx());
            app.ui(ui);
        },
        app,
    );
    h.run_steps(5);
    audit(&h.ctx).into_iter().map(|f| format!("{width} {tab}: {:?} {:?} {:?}", f.kind, f.rect, f.other)).collect()
}

#[test]
fn no_overlap_and_nothing_outside_the_window_on_any_tab() {
    let mut bad = Vec::new();
    for width in WIDTHS {
        for tab in tabilao_ui_egui::ribbon::TABS {
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

fn press(pos: egui::Pos2, pressed: bool) -> egui::Event {
    egui::Event::PointerButton { pos, button: egui::PointerButton::Primary, pressed, modifiers: Default::default() }
}

/// A finger dragged over the grid scrolls the sheet and selects no range (it used to select
/// D1:D20 on a phone); a tap still selects a cell.
#[test]
fn a_finger_drag_scrolls_the_grid_and_does_not_select_a_range() {
    let mut session = Session::new();
    session.new_workbook();
    let app = SheetApp::new(session, Default::default());
    let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(400.0, 780.0)).build_ui_state(
        |ui, app: &mut SheetApp| {
            app.logic(ui.ctx());
            app.ui(ui);
        },
        app,
    );
    h.run_steps(6);
    let before = h.state().view().scroll;
    let start = egui::pos2(200.0, 600.0);
    let mut send = |h: &mut egui_kittest::Harness<'_, SheetApp>, e: Vec<egui::Event>| {
        h.input_mut().events.extend(e);
        h.run_steps(1);
    };
    send(&mut h, vec![touch(egui::TouchPhase::Start, start), egui::Event::PointerMoved(start), press(start, true)]);
    let mut at = start;
    for _ in 0..12 {
        at.y -= 25.0;
        send(&mut h, vec![touch(egui::TouchPhase::Move, at), egui::Event::PointerMoved(at)]);
    }
    send(&mut h, vec![touch(egui::TouchPhase::End, at), press(at, false)]);
    h.run_steps(3);
    let after = h.state().view().scroll;
    assert!(after.y > before.y + 100.0, "the sheet followed the finger: {before:?} → {after:?}");
    let ranges = h.state().session.active().map(|d| d.selection.ranges.len()).unwrap_or(0);
    let single = h.state().session.active().is_some_and(|d| d.selection.is_single_cell());
    assert!(ranges <= 1 && single, "dragging with a finger selected a range");
}
