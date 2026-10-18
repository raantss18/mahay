//! Retained renders after edits inside transformed and clipping containers: every render must match
//! a full render of the same view (the engine's random walk drives whole documents through it).

use std::sync::Arc;

use figcraft_color::{Color, Paint};
use figcraft_doc::{Blur, Document, Effect, EffectKind, NodeId, NodeKind};
use figcraft_geom::Affine;

use crate::retained_tests::assert_same;
use crate::{Options, Retained, View, Work, render_view};

fn check(r: &mut Retained, doc: &Arc<Document>, page: NodeId, v: &View, what: &str) {
    let (shown, pix, work) = r.render(doc, page, v, &Options::default()).unwrap();
    let pix = pix.clone();
    let full = render_view(doc, page, &shown, &Options::default()).unwrap();
    assert_same(&pix, &full, &format!("{what} ({work:?})"));
}

#[test]
fn moving_a_child_of_a_rotated_frame() {
    let mut d = Document::new("rot");
    let page = d.first_page().unwrap();
    let f = d.create(page, NodeKind::Frame, "F", 100.0, 100.0, 300.0, 200.0).unwrap();
    d.get_mut(f).unwrap().transform = figcraft_geom::set_rotation_about_center(d.node(f).unwrap().transform, 300.0, 200.0, 25.0);
    let c = d.create(f, NodeKind::Rectangle, "C", 40.0, 40.0, 80.0, 60.0).unwrap();
    d.get_mut(c).unwrap().fills = vec![Paint::solid(Color::from_hex("E03030").unwrap())];
    let doc = Arc::new(d);
    let v = View { origin: (0.0, 0.0), scale: 1.0, width: 500, height: 450 };
    let mut r = Retained::default();
    check(&mut r, &doc, page, &v, "first");
    let mut d2 = (*doc).clone();
    let pw = d2.world_transform(f);
    let n = d2.get_mut(c).unwrap();
    n.transform = pw.inverse() * Affine::translate((150.0, 60.0)) * pw * n.transform;
    check(&mut r, &Arc::new(d2), page, &v, "moved");
}

/// A clipping frame whose right edge lands exactly on a pixel boundary at 40 %: float noise in
/// one render's origin and not another's put the edge a hair past the boundary, and the clip,
/// rounded outwards, let a whole column of the child through.
#[test]
fn a_clip_edge_on_a_pixel_boundary_clips_the_same_in_every_render() {
    let mut d = Document::new("clip");
    let page = d.first_page().unwrap();
    let f = d.create(page, NodeKind::Frame, "F", 212.0, 198.0, 220.0, 70.0).unwrap();
    d.get_mut(f).unwrap().fills = vec![Paint::solid(Color::from_hex("FFE0B0").unwrap())];
    let c = d.create(f, NodeKind::Rectangle, "C", 58.0, 10.0, 40.0, 40.0).unwrap();
    d.get_mut(c).unwrap().fills = vec![Paint::solid(Color::from_hex("3070E0").unwrap())];
    d.get_mut(c).unwrap().effects.push(Effect { kind: EffectKind::LayerBlur(Blur { radius: 3.0, ..Blur::default() }), visible: true });
    let doc = Arc::new(d);
    // The frame's new right edge (x 280) is device pixel 172 exactly.
    let v = View { origin: (-150.0, 132.8), scale: 0.4, width: 320, height: 240 };
    for lod in [false, true] {
        let opts = Options { lod, ..Default::default() };
        let mut r = Retained::default();
        r.render(&doc, page, &v, &opts).unwrap();
        let mut smaller = (*doc).clone();
        smaller.get_mut(f).unwrap().width = 68.0;
        smaller.get_mut(f).unwrap().height = 54.0;
        let smaller = Arc::new(smaller);
        let (shown, pix, work) = r.render(&smaller, page, &v, &opts).unwrap();
        assert!(matches!(work, Work::Partial { .. }), "{work:?}");
        let pix = pix.clone();
        assert_same(&pix, &render_view(&smaller, page, &shown, &opts).unwrap(), &format!("lod {lod}"));
    }
}
