//! Paints: gradient handles exact under any transform, spreads unrolled, patterns as image fills.

use figcraft_color::{Gradient, GradientKind, ImageScaleMode, Paint, PaintKind};
use figcraft_doc::{Document, Node};

use super::import_svg_with_notes;

/// The layers of `svg`'s import (frame first), and its notes.
fn import(svg: &str) -> (Document, Vec<Node>, Vec<String>) {
    let mut d = Document::new("t");
    let p = d.first_page().unwrap();
    let i = import_svg_with_notes(&mut d, svg.as_bytes(), p, 0.0, 0.0, "x").unwrap();
    let nodes = d.subtree(i.frame).into_iter().filter_map(|id| d.node(id).cloned()).collect();
    (d, nodes, i.notes)
}

fn fill(svg: &str) -> Paint {
    import(svg).1.get(1).and_then(|n| n.fills.first().cloned()).unwrap()
}

fn gradient(svg: &str) -> Gradient {
    match fill(svg).kind {
        PaintKind::Gradient(g) => g,
        k => panic!("{k:?}"),
    }
}

fn near(a: (f64, f64), b: (f64, f64)) -> bool {
    (a.0 - b.0).abs() < 1e-4 && (a.1 - b.1).abs() < 1e-4
}

const DEFS: &str = r##"<stop offset="0" stop-color="#f00"/><stop offset="1" stop-color="#00f"/>"##;

#[test]
fn linear_handles_land_in_the_layers_own_space() {
    // Bounding-box units: the handles span the unit box (perpendicular in bounding-box space, as
    // SVG has it).
    let g = gradient(&format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="300" height="200"><linearGradient id="g">{DEFS}</linearGradient><rect x="10" y="20" width="200" height="100" fill="url(#g)"/></svg>"##
    ));
    assert_eq!(g.kind, GradientKind::Linear);
    assert!(near(g.from, (0.0, 0.0)) && near(g.to, (1.0, 0.0)) && near(g.width, (0.0, 1.0)), "{g:?}");
    assert_eq!(g.stops.len(), 2);
    // User space through a rotated group: the handles land in the rectangle's own space.
    let g = gradient(&format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="300" height="300"><linearGradient id="g" gradientUnits="userSpaceOnUse" x1="0" y1="0" x2="0" y2="50">{DEFS}</linearGradient><g transform="translate(100 100) rotate(90)"><rect width="100" height="50" fill="url(#g)"/></g></svg>"##
    ));
    assert!(near(g.from, (0.0, 0.0)) && near(g.to, (0.0, 1.0)), "{g:?}");
}

#[test]
fn radial_gradients_center_on_their_focal_point() {
    let g = gradient(&format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100"><radialGradient id="g" fx="0.25">{DEFS}</radialGradient><rect width="100" height="50" fill="url(#g)"/></svg>"##
    ));
    assert_eq!(g.kind, GradientKind::Radial);
    assert!(near(g.from, (0.25, 0.5)) && near(g.to, (0.75, 0.5)) && near(g.width, (0.25, 1.0)), "{g:?}");
}

#[test]
fn reflect_and_repeat_unroll_over_the_box() {
    let svg = |m: &str| {
        format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="10"><linearGradient id="g" x2="0.25" spreadMethod="{m}">{DEFS}</linearGradient><rect width="100" height="10" fill="url(#g)"/></svg>"##
        )
    };
    let r = gradient(&svg("reflect"));
    assert!(near(r.from, (0.0, 0.0)) && near(r.to, (1.0, 0.0)), "{r:?}");
    let pos: Vec<f32> = r.stops.iter().map(|s| s.position).collect();
    assert_eq!(pos, [0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0]);
    // Every other period runs backwards.
    assert!(r.stops[2].color.b > 0.9 && r.stops[3].color.r > 0.9);
    let p = gradient(&svg("repeat"));
    assert!(p.stops[2].color.r > 0.9 && p.stops[3].color.b > 0.9, "a hard edge at each period");
    // Pad stays as it is.
    assert_eq!(gradient(&svg("pad")).stops.len(), 2);
}

#[test]
fn opacity_and_stop_opacity_carry_over() {
    let p = fill(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="10" height="10"><linearGradient id="g"><stop offset="0" stop-color="#000" stop-opacity="0.25"/><stop offset="1" stop-color="#000"/></linearGradient><rect width="10" height="10" fill="url(#g)" fill-opacity="0.5"/></svg>"##,
    );
    assert_eq!(p.opacity, 0.5);
    let PaintKind::Gradient(g) = p.kind else { panic!() };
    assert_eq!((g.stops[0].color.a, g.stops[1].color.a), (0.25, 1.0));
}

const PNG: &str = "iVBORw0KGgoAAAANSUhEUgAAAAQAAAACCAIAAADwyuo0AAAAE0lEQVR4nGP4z8AARGDiPwMyBwBnsgf5Q2OaZwAAAABJRU5ErkJggg==";

fn image_paint(svg: &str) -> (figcraft_color::ImagePaint, Document) {
    let (d, nodes, _) = import(svg);
    match nodes.get(1).and_then(|n| n.fills.first()).map(|p| p.kind.clone()) {
        Some(PaintKind::Image(i)) => (i, d),
        k => panic!("{k:?}"),
    }
}

#[test]
fn an_image_pattern_shown_once_is_an_image_fill() {
    // The usual export of an image fill: bounding-box content units, the image scaled into it.
    let (i, d) = image_paint(&format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100"><defs><pattern id="p" patternContentUnits="objectBoundingBox" width="1" height="1"><use href="#img" transform="scale(0.25 0.5)"/></pattern><image id="img" width="4" height="2" href="data:image/png;base64,{PNG}"/></defs><rect width="80" height="40" fill="url(#p)"/></svg>"##
    ));
    assert_eq!(i.scale_mode, ImageScaleMode::Fill);
    assert_eq!(d.images.get(&i.image).map(|im| (im.width, im.height)), Some((4, 2)));
    // Placed off-centre: a crop that puts it exactly there.
    let (i, _) = image_paint(&format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100"><defs><pattern id="p" patternUnits="userSpaceOnUse" width="1000" height="1000"><image x="10" width="4" height="2" href="data:image/png;base64,{PNG}"/></pattern></defs><rect width="80" height="40" fill="url(#p)"/></svg>"##
    ));
    assert_eq!(i.scale_mode, ImageScaleMode::Crop);
    let c = i.crop.unwrap();
    assert!((c[0] - 20.0).abs() < 1e-6 && (c[4] + 2.5).abs() < 1e-6, "{c:?}");
}

#[test]
fn a_repeating_pattern_tiles() {
    let (i, _) = image_paint(&format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100"><defs><pattern id="p" patternUnits="userSpaceOnUse" width="8" height="4"><image width="8" height="4" href="data:image/png;base64,{PNG}"/></pattern></defs><rect width="80" height="40" fill="url(#p)"/></svg>"##
    ));
    assert_eq!((i.scale_mode, i.scale), (ImageScaleMode::Tile, 2.0));
    // A vector pattern is drawn into a tile image.
    let (i, d) = image_paint(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100"><defs><pattern id="dots" patternUnits="userSpaceOnUse" width="10" height="10"><circle cx="5" cy="5" r="3" fill="#000"/></pattern></defs><rect width="100" height="100" fill="url(#dots)"/></svg>"##,
    );
    assert_eq!((i.scale_mode, i.scale), (ImageScaleMode::Tile, 0.5));
    assert_eq!(d.images.get(&i.image).map(|im| (im.width, im.height)), Some((20, 20)));
}

#[test]
fn a_rotated_repeating_pattern_is_reported() {
    let (_, nodes, notes) = import(&format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100"><defs><pattern id="p" patternUnits="userSpaceOnUse" width="8" height="4" patternTransform="rotate(30)"><image width="8" height="4" href="data:image/png;base64,{PNG}"/></pattern></defs><rect width="80" height="40" fill="url(#p)"/></svg>"##
    ));
    assert!(nodes.get(1).is_some_and(|n| n.fills.is_empty()));
    assert!(notes.iter().any(|n| n.contains("pattern")), "{notes:?}");
}

#[test]
fn a_pattern_inside_its_own_tile_stops() {
    let (_, nodes, _) = import(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100"><defs><pattern id="p" patternUnits="userSpaceOnUse" width="10" height="10"><rect width="5" height="5" fill="url(#p)"/><rect x="5" width="5" height="5" fill="#000"/></pattern></defs><rect width="100" height="100" fill="url(#p)"/></svg>"##,
    );
    assert!(nodes.len() >= 2);
}
