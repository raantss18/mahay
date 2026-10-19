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
