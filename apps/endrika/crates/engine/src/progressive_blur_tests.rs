//! Progressive blur through the session: adding and editing it, scaling, variable bindings on its
//! radii, Inspect and generated CSS, and hostile values that still render.

use serde_json::json;

use crate::Session;
use crate::codegen::{CodeLang, code_for};
use crate::params::id_of;
use figcraft_doc::{Blur, BlurType, EffectKind, MAX_BLUR_RADIUS};

fn rect(s: &mut Session) -> figcraft_doc::NodeId {
    id_of(&s.execute("shape.rectangle", &json!({"x": 0, "y": 0, "width": 100, "height": 60})).unwrap()["id"]).unwrap()
}

fn blur(s: &Session, id: figcraft_doc::NodeId, i: usize) -> Blur {
    s.doc().unwrap().node(id).unwrap().effects[i].blur().cloned().unwrap()
}

const PROGRESSIVE: &str = "progressive";

#[test]
fn effect_add_and_set_take_progressive_blurs() {
    let mut s = Session::new();
    let r = rect(&mut s);
    s.execute(
        "effect.add",
        &json!({"ids": [r], "effect": {"type": "layerBlur", "radius": 12, "blurType": PROGRESSIVE, "startRadius": 2, "startOffset": [0.5, 0.1], "endOffset": [0.5, 0.9]}}),
    )
    .unwrap();
    let b = blur(&s, r, 0);
    assert_eq!((b.blur_type, b.radius, b.start_radius, b.start, b.end), (BlurType::Progressive, 12.0, 2.0, (0.5, 0.1), (0.5, 0.9)));
    // A background blur, then back to uniform: one undo step each.
    s.execute("effect.set", &json!({"ids": [r], "index": 0, "effect": {"type": "backgroundBlur", "radius": 8, "blur_type": PROGRESSIVE}})).unwrap();
    let n = s.doc().unwrap().node(r).unwrap();
    assert!(matches!(&n.effects[0].kind, EffectKind::BackgroundBlur(b) if b.is_progressive() && b.start_radius == 0.0));
    s.undo().unwrap();
    assert_eq!(blur(&s, r, 0).start_radius, 2.0);
    // Junk is refused, not stored.
    assert!(s.execute("effect.set", &json!({"ids": [r], "index": 0, "effect": {"type": "layerBlur", "blurType": "wavy"}})).is_err());
    assert!(s.execute("effect.set", &json!({"ids": [r], "index": 0, "effect": {"type": "layerBlur", "startOffset": "top"}})).is_err());
    assert_eq!(blur(&s, r, 0).start_radius, 2.0);
}

#[test]
fn scaling_scales_both_radii_and_keeps_the_line() {
    let mut s = Session::new();
    let r = rect(&mut s);
    s.execute("effect.add", &json!({"ids": [r], "effect": {"type": "layerBlur", "radius": 10, "blurType": PROGRESSIVE, "startRadius": 3}})).unwrap();
    s.execute("transform.scale", &json!({"ids": [r], "factor": 2})).unwrap();
    let b = blur(&s, r, 0);
    assert_eq!((b.radius, b.start_radius, b.start, b.end), (20.0, 6.0, Blur::default().start, Blur::default().end));
}

#[test]
fn both_radii_bind_to_number_variables() {
    let mut s = Session::new();
    let r = rect(&mut s);
    s.execute("effect.add", &json!({"ids": [r], "effect": {"type": "layerBlur", "radius": 10, "blurType": PROGRESSIVE}})).unwrap();
    s.execute("variable.createCollection", &json!({"name": "Tokens"})).unwrap();
    for (name, v) in [("soft", 4), ("strong", 24)] {
        s.execute("variable.create", &json!({"collection": "Tokens", "name": name, "type": "number", "value": v})).unwrap();
    }
    s.execute("variable.bind", &json!({"ids": [r], "field": "effects/0/startRadius", "variable": "soft"})).unwrap();
    s.execute("variable.bind", &json!({"ids": [r], "field": "effects/0/radius", "variable": "strong"})).unwrap();
    let b = blur(&s, r, 0);
    assert_eq!((b.start_radius, b.radius), (4.0, 24.0));
    s.execute("variable.set", &json!({"variable": "soft", "value": -3})).unwrap();
    assert_eq!(blur(&s, r, 0).start_radius, 0.0, "clamped");
}

#[test]
fn inspect_shows_the_ramp_and_css_leaves_it_out() {
    let mut s = Session::new();
    let r = rect(&mut s);
    s.execute("effect.add", &json!({"ids": [r], "effect": {"type": "backgroundBlur", "radius": 16, "blurType": PROGRESSIVE, "startRadius": 2}}))
        .unwrap();
    s.execute("effect.add", &json!({"ids": [r], "effect": {"type": "layerBlur", "radius": 6}})).unwrap();
    let doc = s.doc().unwrap();
    let sections = crate::devinspect::properties(doc, r);
    let fx = sections.iter().find(|s| s.title == "Effects").unwrap();
    assert_eq!(fx.rows[0].label, "Background blur");
    assert_eq!(fx.rows[0].value, "Progressive 2 → 16 from 50% 0% to 50% 100%");
    assert_eq!(fx.rows[1].value, "6");
    // CSS blurs are uniform: the progressive one has no CSS, the uniform one does.
    let css = code_for(doc, r, CodeLang::Css);
    assert!(!css.contains("backdrop-filter"), "{css}");
    assert!(css.contains("filter: blur(3px);"), "{css}");
}

#[test]
fn hostile_progressive_values_are_capped_and_render() {
    let mut s = Session::new();
    let r = rect(&mut s);
    for effect in [
        json!({"type": "layerBlur", "radius": 1e308, "blurType": PROGRESSIVE, "startRadius": -1e308, "startOffset": [1e308, -1e308], "endOffset": [0, 0]}),
        json!({"type": "backgroundBlur", "radius": -5, "blurType": PROGRESSIVE, "startRadius": 1e12, "startOffset": [0.5, 0.5], "endOffset": [0.5, 0.5]}),
    ] {
        s.execute("effect.add", &json!({"ids": [r], "effect": effect})).unwrap();
    }
    let doc = s.doc().unwrap();
    for e in &doc.node(r).unwrap().effects {
        assert!(e.blur().unwrap().max_radius() <= MAX_BLUR_RADIUS);
    }
    let page = doc.first_page().unwrap();
    let view = figcraft_render::View { origin: (-20.0, -20.0), scale: 1.0, width: 160, height: 120 };
    assert!(figcraft_render::render_view(doc, page, &view, &figcraft_render::Options::default()).is_some());
    let props = crate::devinspect::properties(doc, r);
    assert!(props.iter().any(|s| s.title == "Effects"));
}
