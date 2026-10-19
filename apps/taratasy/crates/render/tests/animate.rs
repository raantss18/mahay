//! LaTeX `animate` players (#169): found in the document, and each frame drawn on its own.

use std::sync::Arc;

use taratasy_render::animate::{ControlAction, Player};
use taratasy_render::{PageRenderer, RenderConfig, RenderRequest, RequestKind, inspect};

/// A one-page PDF with an `animate` player of three frames (red, green, blue), its display
/// widget showing frame 0 as the package draws it, and a PlayRight button. `script` is the
/// display widget's page-open script.
fn animate_pdf(script: &str) -> Vec<u8> {
    let colour = |rgb: &str| {
        let body = format!("{rgb} rg 0 0 100 100 re f");
        format!("<< /Type /XObject /Subtype /Form /BBox [0 0 100 100] /Length {} >>\nstream\n{body}\nendstream", body.len())
    };
    let widget = |name: &str, ap: u32, flags: u32, rect: &str| {
        format!("<< /Type /Annot /Subtype /Widget /FT /Btn /Ff 65537 /T ({name}) /F {flags} /Rect [{rect}] /P 3 0 R /AP << /N {ap} 0 R >> >>")
    };
    let display = format!(
        "<< /Type /Annot /Subtype /Widget /FT /Btn /Ff 65537 /T (anm4) /F 4 /Rect [50 50 150 150] /P 3 0 R /AP << /N 5 0 R >> \
         /AA << /PO << /S /JavaScript /JS ({script}) >> >> >>"
    );
    let objs: Vec<String> = vec![
        "<< /Type /Catalog /Pages 2 0 R /AcroForm << /Fields [8 0 R 9 0 R 10 0 R 11 0 R 12 0 R] >> >>".into(),
        "<< /Type /Pages /Kids [3 0 R] /Count 1 >>".into(),
        "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 200 200] /Contents 4 0 R /Annots [8 0 R 9 0 R 10 0 R 11 0 R 12 0 R] >>".into(),
        "<< /Length 0 >>\nstream\n\nendstream".into(),
        colour("1 0 0"),
        colour("0 1 0"),
        colour("0 0 1"),
        display,
        widget("4.0", 5, 2, "50 50 150 150"),
        widget("4.1", 6, 2, "50 50 150 150"),
        widget("4.2", 7, 2, "50 50 150 150"),
        widget("4.PlayRight", 5, 4, "50 20 60 30"),
    ];
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

/// The colour at the middle of the player, drawn at scale 1 (RGB, premultiplied but opaque).
fn middle(renderer: &mut PageRenderer, visibility: Option<Vec<(i32, i32, bool)>>) -> [u8; 3] {
    let req = RenderRequest { page: 0, kind: RequestKind::Pixels, tile: None, scale: 1.0, tag: 0, preview: false };
    let page = match visibility {
        Some(v) => renderer.render_annotation_variant(req, Arc::new(v)),
        None => renderer.render(req),
    };
    assert!(page.error.is_none(), "{:?}", page.error);
    // (100, 100) in user space is (100, 100) on a 200-point page, y down.
    let at = (100 * page.width as usize + 100) * 4;
    [page.rgba[at], page.rgba[at + 1], page.rgba[at + 2]]
}

#[test]
fn an_animate_player_is_found_with_its_frames_rate_and_controls() {
    let bytes = Arc::new(animate_pdf("var a4_fps=12;a4_spd=1;"));
    let info = inspect(bytes, None).expect("opens");
    assert_eq!(info.animations.len(), 1, "{:?}", info.animations);
    let a = &info.animations[0];
    assert_eq!((a.page, a.number, a.display), (0, 4, (8, 0)));
    assert_eq!(a.frames, vec![(9, 0), (10, 0), (11, 0)]);
    assert_eq!(a.fps, 12.0);
    assert_eq!(a.rect, [50.0, 50.0, 150.0, 150.0]);
    assert_eq!(a.controls.len(), 1);
    assert_eq!(a.controls[0].action, ControlAction::Play { forward: true });
}

#[test]
fn without_a_rate_in_the_script_the_package_default_is_used() {
    let info = inspect(Arc::new(animate_pdf("nothing here")), None).expect("opens");
    assert_eq!(info.animations[0].fps, taratasy_render::animate::DEFAULT_FPS);
}

#[test]
fn each_frame_renders_in_place_of_the_display_widget() {
    let bytes = Arc::new(animate_pdf("a4_fps=12;"));
    let info = inspect(bytes.clone(), None).expect("opens");
    let a = &info.animations[0];
    let mut renderer = PageRenderer::new(bytes, RenderConfig::default());
    assert_eq!(middle(&mut renderer, None), [255, 0, 0], "the page as it is shows frame 0");
    let mut player = Player::default();
    player.click(a);
    player.advance(a);
    assert_eq!(middle(&mut renderer, Some(player.visibility(a))), [0, 255, 0], "frame 1");
    player.advance(a);
    assert_eq!(middle(&mut renderer, Some(player.visibility(a))), [0, 0, 255], "frame 2");
    // The renderer keeps nothing of a variant: the page as it is still shows frame 0.
    assert_eq!(middle(&mut renderer, None), [255, 0, 0]);
}
