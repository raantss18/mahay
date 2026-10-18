//! Text box sizes measured against documents re-saved by other software. Local only: runs when
//! `FIGCRAFT_FIG_SAMPLES` names a folder of `.fig` files, and only for text whose exact font is
//! installed. The rules themselves are tested on built-in fonts in `figcraft_text`.

use figcraft_doc::{LineHeight, Node, NodeKind, TextAutoResize};

/// `n` with its percent line height read as a share of the font's own line height (the import
/// reads every percent as a share of the font size; files can mean either).
fn of_own_line_height(n: &Node) -> Option<Node> {
    let mut m = n.clone();
    let NodeKind::Text(t) = &mut m.kind else { return None };
    let LineHeight::Percent(p) = t.style.line_height else { return None };
    let f = figcraft_text::find_face(&t.style.font_family, &t.style.font_style)?;
    t.style.line_height = LineHeight::Percent(p * (f.ascent + f.descent + f.line_gap.max(0.0)) / f.upem);
    Some(m)
}

/// Each text box that sizes itself comes out at the size stored in the file, to the pixel.
#[test]
fn local_samples_keep_their_text_box_sizes() {
    let Some(dir) = std::env::var_os("FIGCRAFT_FIG_SAMPLES") else { return };
    let Ok(dir) = std::fs::read_dir(dir) else { return };
    figcraft_text::scan_system_fonts();
    for p in dir.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "fig")) {
        let Ok((doc, _)) = figcraft_format::fig::import(&std::fs::read(&p).unwrap(), "sample") else { continue };
        for n in doc.nodes() {
            let NodeKind::Text(t) = &n.kind else { continue };
            // Stored sizes are whole pixels once the text was laid out by the software that saved it.
            let laid_out = n.height.fract() == 0.0 && (t.auto_resize != TextAutoResize::WidthAndHeight || n.width.fract() == 0.0);
            let own_font = t.spans().iter().all(|(_, _, s)| {
                figcraft_text::find_face(&s.font_family, &s.font_style).is_some_and(|f| f.family == s.font_family && f.style == s.font_style)
            });
            let Some((w, h)) = figcraft_text::auto_size(n).filter(|_| laid_out && own_font) else { continue };
            let h = if h == n.height { h } else { of_own_line_height(n).and_then(|m| figcraft_text::auto_size(&m)).map_or(h, |s| s.1) };
            assert_eq!((w, h), (n.width, n.height), "{}: {:?}", p.display(), t.characters);
        }
    }
}
