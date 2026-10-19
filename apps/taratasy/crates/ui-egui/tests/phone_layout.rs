//! D23: at phone, tablet and desktop widths no two clickable widgets overlap and none lies outside
//! the window: on the Home screen and in every mode of an open document.

use egui_kittest::Harness;
use mahay_ui::adaptive::audit;
use taratasy_ui_egui::TaratasyApp;

const WIDTHS: [f32; 4] = [360.0, 400.0, 600.0, 840.0];

fn fixture() -> Vec<u8> {
    let mut objs: Vec<String> = vec!["<< /Type /Catalog /Pages 2 0 R >>".into()];
    objs.push("<< /Type /Pages /Kids [4 0 R] /Count 1 /MediaBox [0 0 200 300] >>".into());
    objs.push("<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".into());
    objs.push("<< /Type /Page /Parent 2 0 R /Contents 5 0 R /Resources << /Font << /F1 3 0 R >> >> >>".into());
    let body = "BT /F1 24 Tf 20 150 Td (Page 1) Tj ET";
    objs.push(format!("<< /Length {} >>\nstream\n{body}\nendstream", body.len()));
    let mut out = b"%PDF-1.7\n".to_vec();
    let mut offsets = Vec::new();
    for (i, o) in objs.iter().enumerate() {
        offsets.push(out.len());
        out.extend_from_slice(format!("{} 0 obj\n{o}\nendobj\n", i + 1).as_bytes());
    }
    let xref = out.len();
    out.extend_from_slice(format!("xref\n0 {}\n0000000000 65535 f \n", objs.len() + 1).as_bytes());
    for o in offsets {
        out.extend_from_slice(format!("{o:010} 00000 n \n").as_bytes());
    }
    out.extend_from_slice(format!("trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n", objs.len() + 1).as_bytes());
    out
}

fn findings_at(width: f32, mode: Option<&str>) -> Vec<String> {
    let height = if width < 600.0 { 780.0 } else { 900.0 };
    let mode = mode.map(str::to_owned);
    let mut h = Harness::builder().with_size(egui::vec2(width, height)).build_eframe(move |_cc| {
        let mut app = TaratasyApp::new();
        app.set_option("language", "en").unwrap();
        if let Some(mode) = &mode {
            app.open_bytes("doc.pdf", None, fixture()).unwrap();
            app.set_option("mode", mode).unwrap();
        }
        app
    });
    h.run_steps(6);
    audit(&h.ctx).into_iter().map(|f| format!("{width}: {:?} {:?} {:?}", f.kind, f.rect, f.other)).collect()
}

#[test]
fn no_overlap_and_nothing_outside_the_window_in_any_mode() {
    let mut bad = Vec::new();
    for width in WIDTHS {
        for mode in [None, Some("read"), Some("all"), Some("edit"), Some("convert"), Some("sign")] {
            bad.extend(findings_at(width, mode).into_iter().map(|f| format!("[{}] {f}", mode.unwrap_or("home"))));
        }
    }
    bad.sort();
    bad.dedup();
    assert!(bad.is_empty(), "{} problems:\n{}", bad.len(), bad.join("\n"));
}
