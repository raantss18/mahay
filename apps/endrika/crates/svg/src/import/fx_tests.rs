//! Filters → effects: the primitive chains tools write for shadows and blurs, scaled with their
//! layer, and our own exports coming back as the effects they were.

use figcraft_color::{Color, Paint};
use figcraft_doc::{Blur, Document, Effect, EffectKind, NodeKind, Shadow};

use super::import_svg_with_notes;

/// The effects on the first layer of `svg` with filter `filter` (its `<filter>` content) on a
/// 40 × 40 rectangle, optionally inside a group with `transform`.
fn effects(filter: &str, transform: &str) -> (Vec<EffectKind>, Vec<String>) {
    let svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="200" height="200"><filter id="f" x="-1" y="-1" width="3" height="3">{filter}</filter><g transform="{transform}"><rect x="10" y="10" width="40" height="40" fill="red" filter="url(#f)"/></g></svg>"##
    );
    let mut d = Document::new("t");
    let p = d.first_page().unwrap();
    let i = import_svg_with_notes(&mut d, svg.as_bytes(), p, 0.0, 0.0, "x").unwrap();
    let rect = d.subtree(i.frame).into_iter().filter_map(|id| d.node(id)).find(|n| n.kind == NodeKind::Rectangle).unwrap().clone();
    (rect.effects.into_iter().map(|e| e.kind).collect(), i.notes)
}

fn shadow(k: &EffectKind) -> &Shadow {
    match k {
        EffectKind::DropShadow(s) | EffectKind::InnerShadow(s) => s,
        k => panic!("{k:?}"),
    }
}

#[test]
fn blurs_and_drop_shadows() {
    let (fx, _) = effects(r##"<feGaussianBlur stdDeviation="3"/>"##, "");
    assert!(matches!(&fx[..], [EffectKind::LayerBlur(b)] if b.radius == 6.0), "{fx:?}");
    let (fx, _) = effects(r##"<feDropShadow dx="2" dy="3" stdDeviation="1.5" flood-color="#00f" flood-opacity="0.4"/>"##, "");
    let s = shadow(&fx[0]);
    assert!(matches!(fx[0], EffectKind::DropShadow(_)));
    assert_eq!((s.offset, s.radius, s.spread), ((2.0, 3.0), 3.0, 0.0));
    assert!((s.color.b - 1.0).abs() < 1e-6 && (s.color.a - 0.4).abs() < 1e-6);
    assert!(s.show_behind, "an SVG shadow shows through the layer");
}

#[test]
fn effects_scale_with_their_layer() {
    let (fx, _) = effects(r##"<feDropShadow dx="2" dy="0" stdDeviation="1"/>"##, "scale(2)");
    let s = shadow(&fx[0]);
    assert_eq!((s.offset, s.radius), ((4.0, 0.0), 4.0));
    // Turned with the layer.
    let (fx, _) = effects(r##"<feDropShadow dx="2" dy="0" stdDeviation="1"/>"##, "rotate(90)");
    let s = shadow(&fx[0]);
    assert!(s.offset.0.abs() < 1e-9 && (s.offset.1 - 2.0).abs() < 1e-9, "{s:?}");
}

#[test]
fn offset_blur_flood_composite_merge_is_a_drop_shadow() {
    let (fx, _) = effects(
        r##"<feOffset in="SourceAlpha" dx="1" dy="2" result="o"/><feGaussianBlur in="o" stdDeviation="2" result="b"/><feFlood flood-color="#0f0" flood-opacity="0.5"/><feComposite in2="b" operator="in" result="s"/><feMerge><feMergeNode in="s"/><feMergeNode in="SourceGraphic"/></feMerge>"##,
        "",
    );
    let s = shadow(&fx[0]);
    assert_eq!((s.offset, s.radius), ((1.0, 2.0), 4.0));
    assert!((s.color.g - 1.0).abs() < 1e-6 && (s.color.a - 0.5).abs() < 1e-6);
    // Spread from a dilated alpha.
    let (fx, _) = effects(
        r##"<feMorphology in="SourceAlpha" operator="dilate" radius="3"/><feOffset dy="1"/><feGaussianBlur stdDeviation="1"/><feMerge><feMergeNode/><feMergeNode in="SourceGraphic"/></feMerge>"##,
        "",
    );
    assert_eq!(shadow(&fx[0]).spread, 3.0);
}

#[test]
fn colour_matrix_shadows_inner_and_drop() {
    let hard = r##"<feColorMatrix in="SourceAlpha" type="matrix" values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0" result="hard"/>"##;
    let (fx, _) = effects(
        &format!(
            r##"<feFlood flood-opacity="0" result="bg"/>{hard}<feOffset dy="4"/><feGaussianBlur stdDeviation="2"/><feComposite in2="hard" operator="out"/><feColorMatrix type="matrix" values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0.25 0"/><feBlend mode="normal" in2="bg" result="drop"/><feBlend mode="normal" in="SourceGraphic" in2="drop" result="shape"/>{}<feOffset dx="-1"/><feGaussianBlur stdDeviation="0.5"/><feComposite in2="hard2" operator="arithmetic" k2="-1" k3="1"/><feColorMatrix type="matrix" values="0 0 0 0 1 0 0 0 0 1 0 0 0 0 1 0 0 0 0.5 0"/><feBlend mode="normal" in2="shape"/>"##,
            hard.replace("hard", "hard2")
        ),
        "",
    );
    let [EffectKind::DropShadow(d), EffectKind::InnerShadow(i)] = &fx[..] else { panic!("{fx:?}") };
    assert_eq!((d.offset, d.radius, d.color.a, d.show_behind), ((0.0, 4.0), 4.0, 0.25, false));
    assert_eq!((i.offset, i.radius, i.color), ((-1.0, 0.0), 1.0, Color::new(1.0, 1.0, 1.0, 0.5)));
    // The flood-out-in way of writing an inner shadow.
    let (fx, _) = effects(
        r##"<feOffset in="SourceAlpha" dx="2" dy="2"/><feGaussianBlur stdDeviation="1" result="b"/><feFlood flood-color="#000" flood-opacity="0.6"/><feComposite in2="b" operator="out"/><feComposite in2="SourceAlpha" operator="in" result="inner"/><feMerge><feMergeNode in="SourceGraphic"/><feMergeNode in="inner"/></feMerge>"##,
        "",
    );
    assert!(matches!(&fx[..], [EffectKind::InnerShadow(s)] if s.offset == (2.0, 2.0) && (s.color.a - 0.6).abs() < 1e-6), "{fx:?}");
}

#[test]
fn filters_that_arent_shadows_or_blurs_are_left_out_and_reported() {
    for f in [
        r##"<feTurbulence baseFrequency="0.1"/>"##,
        r##"<feColorMatrix type="saturate" values="0"/>"##,
        // Just a shadow, without the layer.
        r##"<feOffset in="SourceAlpha" dx="2"/>"##,
        // A shadow drawn over the layer.
        r##"<feOffset in="SourceAlpha" dx="2" result="s"/><feMerge><feMergeNode in="SourceGraphic"/><feMergeNode in="s"/></feMerge>"##,
        r##"<feBlend mode="multiply" in="SourceGraphic" in2="SourceAlpha"/>"##,
    ] {
        let (fx, notes) = effects(f, "");
        assert!(fx.is_empty(), "{f}: {fx:?}");
        assert!(notes.iter().any(|n| n.contains("filter")), "{f}: {notes:?}");
    }
}

#[test]
fn exported_effects_come_back_as_they_were() {
    let mut d = Document::new("t");
    let p = d.first_page().unwrap();
    let r = d.create(p, NodeKind::Rectangle, "R", 20.0, 20.0, 60.0, 40.0).unwrap();
    let shadow = Shadow { color: Color::new(0.1, 0.2, 0.3, 0.4), offset: (3.0, 5.0), radius: 6.0, ..Shadow::default() };
    let node = d.get_mut(r).unwrap();
    node.fills = vec![Paint::solid(Color::from_hex("336699").unwrap())];
    node.effects = vec![
        Effect { kind: EffectKind::DropShadow(shadow.clone()), visible: true },
        Effect { kind: EffectKind::LayerBlur(Blur { radius: 4.0, ..Blur::default() }), visible: true },
    ];
    let svg = crate::export_svg(&d, &[r], &crate::SvgOptions::default()).unwrap();
    let mut d2 = Document::new("back");
    let p2 = d2.first_page().unwrap();
    let f = crate::import_svg(&mut d2, svg.as_bytes(), p2, 0.0, 0.0, "x").unwrap();
    let back = d2.subtree(f).into_iter().filter_map(|i| d2.node(i)).find(|n| n.kind == NodeKind::Rectangle).unwrap();
    let [EffectKind::DropShadow(s), EffectKind::LayerBlur(b)] = &back.effects.iter().map(|e| e.kind.clone()).collect::<Vec<_>>()[..] else {
        panic!("{:?}", back.effects)
    };
    assert_eq!((s.offset, s.radius, b.radius), (shadow.offset, shadow.radius, 4.0));
    let c = s.color;
    assert!((c.r - 0.1).abs() < 1e-3 && (c.g - 0.2).abs() < 1e-3 && (c.b - 0.3).abs() < 1e-3 && (c.a - 0.4).abs() < 1e-3, "{c:?}");
}
