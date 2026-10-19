//! D23: at phone, tablet and desktop widths no two clickable widgets overlap and none lies outside
//! the window, on every ribbon tab.

use mahay_ui::adaptive::audit_excluding;
use seho_engine::Session;
use seho_ui_egui::{Services, SlideApp};

const WIDTHS: [f32; 4] = [360.0, 400.0, 600.0, 840.0];
const TABS: [&str; 10] = ["Home", "Insert", "Draw", "Design", "Transitions", "Animations", "Slide Show", "Record", "Review", "View"];

fn findings_at(width: f32, tab: &str) -> Vec<String> {
    let mut app = SlideApp::new(Session::with_new(), Services::default());
    app.ui.tab = tab.to_string();
    let height = if width < 600.0 { 780.0 } else { 900.0 };
    let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(width, height)).build_ui_state(
        |ui, app: &mut SlideApp| {
            app.logic(ui.ctx());
            app.ui(ui);
        },
        app,
    );
    h.run_steps(5);
    // The slide itself is the canvas: it draws its own handles.
    let skip: Vec<egui::Rect> = h.state().canvas_rect.into_iter().collect();
    audit_excluding(&h.ctx, &skip).into_iter().map(|f| format!("{width} {tab}: {:?} {:?} {:?}", f.kind, f.rect, f.other)).collect()
}

#[test]
fn no_overlap_and_nothing_outside_the_window_on_any_tab() {
    let mut bad = Vec::new();
    for width in WIDTHS {
        for tab in TABS {
            bad.extend(findings_at(width, tab));
        }
    }
    bad.sort();
    bad.dedup();
    assert!(bad.is_empty(), "{} problems:\n{}", bad.len(), bad.join("\n"));
}
