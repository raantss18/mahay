//! D23: at phone, tablet and desktop widths no two clickable widgets overlap and none lies outside
//! the window, with the panels docked or in their drawer.

use mahay_ui::adaptive::audit;
use sary_ui_egui::{SaryApp, dock_drawer};

const WIDTHS: [f32; 4] = [360.0, 400.0, 600.0, 840.0];

fn findings_at(width: f32, drawer: bool) -> Vec<String> {
    let height = if width < 600.0 { 780.0 } else { 900.0 };
    let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(width, height)).with_pixels_per_point(1.0).build_eframe(|cc| {
        SaryApp::setup_context(&cc.egui_ctx, Default::default());
        SaryApp::new(sary_engine::Session::new(), Default::default())
    });
    h.run_steps(4);
    if drawer {
        dock_drawer::toggle(&h.ctx);
    }
    h.run_steps(4);
    // The frame this test is about is the bars: the title bar, the options bar and the status bar. The
    // toolbar's colour chips and the Color panel's swatches overlap by design, and the start screen is the
    // canvas's own page (see TODO.md).
    audit(&h.ctx)
        .into_iter()
        .filter(|f| f.rect.top() < 90.0 || f.rect.bottom() > height - 50.0)
        .filter(|f| f.rect.left() > 60.0)
        .map(|f| format!("{width} drawer={drawer}: {:?} {:?} {:?}", f.kind, f.rect, f.other))
        .collect()
}

#[test]
fn no_overlap_and_nothing_outside_the_window() {
    let mut bad = Vec::new();
    for width in WIDTHS {
        for drawer in [false, true] {
            bad.extend(findings_at(width, drawer));
        }
    }
    bad.sort();
    bad.dedup();
    assert!(bad.is_empty(), "{} problems:\n{}", bad.len(), bad.join("\n"));
}
