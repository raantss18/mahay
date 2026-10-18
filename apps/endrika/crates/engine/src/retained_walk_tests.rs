//! A seeded random walk of edits, undo/redo, pans and zooms over a page of nested, rotated and
//! clipping frames, groups, masks, booleans, auto layout, effects, text, instances, variables and
//! styles: after every step the retained canvas render must match a full render of the same view.

use std::sync::{Arc, Once};

use figcraft_doc::{Blur, BlurType, Document, Effect, EffectKind, NodeId, Shadow};
use figcraft_render::{Options, Pixmap, Retained, View, Work, render_view};
use serde_json::{Value, json};

use crate::Session;
use crate::params::id_of;

/// splitmix64: a fixed seed gives the same walk everywhere.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n.max(1) as u64) as usize
    }
    fn range(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
    fn chance(&mut self, p: f64) -> bool {
        self.range(0.0, 1.0) < p
    }
    fn color(&mut self) -> String {
        format!("#{:06X}", self.next() & 0xFF_FFFF)
    }
}

fn fonts() {
    static ONCE: Once = Once::new();
    ONCE.call_once(figcraft_text::scan_system_fonts);
}

fn id(v: Value) -> NodeId {
    id_of(&v["id"]).unwrap()
}

fn run(s: &mut Session, cmd: &str, p: Value) -> Option<Value> {
    s.execute(cmd, &p).ok()
}

fn shape(s: &mut Session, kind: &str, parent: Option<NodeId>, r: [f64; 4], fill: &str) -> NodeId {
    let mut p = json!({"x": r[0], "y": r[1], "width": r[2], "height": r[3], "props": {"fill": fill}});
    if let Some(parent) = parent {
        p["parent"] = json!(parent);
    }
    id(s.execute(kind, &p).unwrap())
}

fn shadow(offset: (f64, f64), radius: f64, spread: f64) -> Value {
    serde_json::to_value(Effect { kind: EffectKind::DropShadow(Shadow { offset, radius, spread, ..Shadow::default() }), visible: true }).unwrap()
}

/// The page: shapes on it, a rotated clipping frame holding a rotated nested frame (not clipping)
/// with text, a group with a drop shadow, a masked group, a boolean, an auto layout row with a
/// blurred child, overflowing text, a layer with an inner shadow and an outside stroke, and a
/// component with two instances, a bound colour variable and a paint style.
fn scene() -> (Session, NodeId) {
    fonts();
    let mut s = Session::new();
    shape(&mut s, "shape.rectangle", None, [10.0, 10.0, 70.0, 40.0], "#E03030");
    shape(&mut s, "shape.ellipse", None, [120.0, 20.0, 50.0, 50.0], "#3070E0");
    let outer = id(s.execute("frame.create", &json!({"x": 40, "y": 90, "width": 200, "height": 140, "props": {"fill": "#F0F0E0"}})).unwrap());
    s.execute("node.set", &json!({"id": outer, "rotation": 25})).unwrap();
    shape(&mut s, "shape.rectangle", Some(outer), [150.0, 20.0, 90.0, 40.0], "#20A060");
    let inner =
        id(s.execute("frame.create", &json!({"x": 20, "y": 60, "width": 90, "height": 60, "parent": outer, "props": {"fill": "#D0E0FF"}})).unwrap());
    s.execute("node.set", &json!({"id": inner, "rotation": -15, "clip": false})).unwrap();
    shape(&mut s, "shape.star", Some(inner), [50.0, 30.0, 60.0, 50.0], "#A040C0");
    s.execute("text.create", &json!({"text": "Nested text", "x": 4, "y": 4, "parent": inner, "props": {"fontSize": 12}})).unwrap();
    let g1 = shape(&mut s, "shape.rectangle", None, [280.0, 30.0, 40.0, 40.0], "#F0B020");
    let g2 = shape(&mut s, "shape.ellipse", None, [300.0, 60.0, 40.0, 30.0], "#208080");
    let g = id(s.execute("object.group", &json!({"ids": [g1, g2]})).unwrap());
    s.execute("node.set", &json!({"id": g, "effects": [shadow((10.0, 12.0), 6.0, 0.0)]})).unwrap();
    let m = shape(&mut s, "shape.ellipse", None, [260.0, 140.0, 60.0, 60.0], "#000000");
    let mr = shape(&mut s, "shape.rectangle", None, [250.0, 150.0, 90.0, 30.0], "#C04040");
    s.execute("object.mask", &json!({"ids": [m, mr]})).unwrap();
    let b1 = shape(&mut s, "shape.rectangle", None, [20.0, 260.0, 50.0, 40.0], "#6060C0");
    let b2 = shape(&mut s, "shape.ellipse", None, [45.0, 270.0, 50.0, 40.0], "#6060C0");
    s.execute("bool.union", &json!({"ids": [b1, b2]})).unwrap();
    let al = id(s.execute("frame.create", &json!({"x": 130, "y": 250, "width": 220, "height": 70, "props": {"fill": "#FFE0B0"}})).unwrap());
    s.execute("node.set", &json!({"id": al, "layout": "horizontal", "gap": 8, "padding": 10})).unwrap();
    for c in ["#E03030", "#3070E0", "#A040C0"] {
        shape(&mut s, "shape.rectangle", Some(al), [0.0, 0.0, 40.0, 40.0], c);
    }
    let blurred = s.doc().unwrap().node(al).unwrap().children[1];
    let blur = serde_json::to_value(Effect { kind: EffectKind::LayerBlur(Blur { radius: 3.0, ..Blur::default() }), visible: true }).unwrap();
    s.execute("node.set", &json!({"id": blurred, "effects": [blur]})).unwrap();
    s.execute("text.create", &json!({"text": "A line of text that overflows its box", "x": 200, "y": 120, "width": 70, "props": {"fontSize": 13, "textAutoResize": "none", "height": 18}})).unwrap();
    let inset = shape(&mut s, "shape.rectangle", None, [190.0, 200.0, 50.0, 30.0], "#FFFFFF");
    let inner_shadow = serde_json::to_value(Effect::inner_shadow()).unwrap();
    s.execute("node.set", &json!({"id": inset, "effects": [inner_shadow], "stroke": "#202020", "strokeWeight": 3, "strokeAlign": "outside"}))
        .unwrap();
    // A component with instances (one in the rotated frame), a bound colour variable and a paint
    // style: edits to the main, the variable or the style repaint other layers.
    let main = id(s.execute("frame.create", &json!({"x": 20, "y": 340, "width": 80, "height": 50, "props": {"fill": "#E0F0E0"}})).unwrap());
    let part = shape(&mut s, "shape.ellipse", Some(main), [10.0, 10.0, 30.0, 30.0], "#C02060");
    let comp = s.execute("component.create", &json!({"ids": [main]})).unwrap()["id"].clone();
    s.execute("instance.create", &json!({"component": comp, "x": 130, "y": 340})).unwrap();
    s.execute("instance.create", &json!({"component": comp, "x": 100, "y": 10, "parent": outer})).unwrap();
    s.execute("variable.createCollection", &json!({"name": "T"})).unwrap();
    s.execute("variable.create", &json!({"collection": "T", "name": "brand", "type": "color", "value": "#FF8000"})).unwrap();
    s.execute("variable.bind", &json!({"ids": [part, g1], "field": "fill", "variable": "brand"})).unwrap();
    let style = s.execute("style.create", &json!({"type": "paint", "name": "Accent", "from": g2})).unwrap()["id"].clone();
    s.execute("style.apply", &json!({"ids": [mr, blurred], "style": style})).unwrap();
    s.execute("select.none", &json!({})).unwrap();
    let page = s.ds().unwrap().page;
    (s, page)
}

/// Layers on the page (pages excluded), ascending ids.
fn layers(doc: &Document, page: NodeId) -> Vec<NodeId> {
    doc.ids().filter(|i| *i != page && doc.page_of(*i) == Some(page)).collect()
}

/// One random step; returns what it did.
fn step(s: &mut Session, page: NodeId, view: &mut View, rng: &mut Rng) -> String {
    let doc = s.ds().unwrap().doc.clone();
    let all = layers(&doc, page);
    let Some(&n) = all.get(rng.below(all.len())) else { return "nothing left".into() };
    let frames: Vec<NodeId> = all.iter().copied().filter(|i| doc.node(*i).is_some_and(|x| x.kind.is_frame_like())).collect();
    let (cmd, p) = match rng.below(24) {
        0 | 1 => ("node.set", json!({"id": n, "x": rng.range(-40.0, 360.0).round(), "y": rng.range(-40.0, 320.0).round()})),
        2 => ("node.set", json!({"id": n, "width": rng.range(4.0, 160.0).round(), "height": rng.range(4.0, 120.0).round()})),
        3 => ("node.set", json!({"id": n, "rotation": rng.range(-90.0, 90.0).round()})),
        4 => ("node.set", json!({"id": n, "opacity": (rng.range(0.1, 1.0) * 100.0).round() / 100.0})),
        5 => ("node.set", json!({"id": n, "visible": !doc.node(n).is_some_and(|x| x.visible)})),
        6 => {
            let siblings = doc.node(n).and_then(|x| x.parent).and_then(|p| doc.node(p)).map_or(1, |p| p.children.len());
            ("node.reorder", json!({"id": n, "index": rng.below(siblings)}))
        }
        7 if all.len() > 10 => ("edit.delete", json!({"ids": [n]})),
        8 => {
            let _ = s.undo();
            return "undo".into();
        }
        9 => {
            let _ = s.redo();
            return "redo".into();
        }
        10 => {
            let far = rng.chance(0.1);
            let reach = if far { 400.0 } else { 90.0 } / view.scale;
            // Around the layers: an empty view would only ever be reused.
            let (x, y) = (view.origin.0 + rng.range(-reach, reach), view.origin.1 + rng.range(-reach, reach));
            view.origin = (x.clamp(-150.0, 300.0), y.clamp(-120.0, 280.0));
            return format!("pan to {:?}", view.origin);
        }
        11 => {
            view.scale = [0.4, 0.75, 1.0, 1.0, 1.5, 2.25][rng.below(6)];
            return format!("zoom to {}", view.scale);
        }
        12 => ("node.set", json!({"id": n, "fill": rng.color()})),
        13 => {
            let fx = match rng.below(3) {
                0 => json!([]),
                1 => json!([shadow(
                    (rng.range(-20.0, 20.0).round(), rng.range(-20.0, 20.0).round()),
                    rng.range(0.0, 16.0).round(),
                    rng.range(0.0, 6.0).round()
                )]),
                _ => {
                    // Uniform or progressive (the radius ramping across the layer).
                    let blur_type = if rng.chance(0.5) { BlurType::Uniform } else { BlurType::Progressive };
                    let blur = Blur { radius: rng.range(1.0, 10.0).round(), blur_type, ..Blur::default() };
                    json!([serde_json::to_value(Effect { kind: EffectKind::LayerBlur(blur), visible: true }).unwrap()])
                }
            };
            ("node.set", json!({"id": n, "effects": fx}))
        }
        14 => match frames.get(rng.below(frames.len())) {
            Some(f) if rng.chance(0.5) => {
                ("node.set", json!({"id": f, "gap": rng.range(0.0, 30.0).round(), "padding": rng.range(0.0, 20.0).round()}))
            }
            Some(f) => ("node.set", json!({"id": f, "clip": !doc.node(*f).is_some_and(|x| x.clip)})),
            None => return "no frame".into(),
        },
        15 => {
            let texts: Vec<NodeId> =
                all.iter().copied().filter(|i| doc.node(*i).is_some_and(|x| matches!(x.kind, figcraft_doc::NodeKind::Text(_)))).collect();
            let Some(t) = texts.get(rng.below(texts.len())) else { return "no text".into() };
            let words = ["glyph", "a", "longer word", "W", "overflowing text runs on"];
            let text: Vec<&str> = (0..1 + rng.below(4)).map(|_| words[rng.below(words.len())]).collect();
            ("text.set", json!({"id": t, "characters": text.join(" ")}))
        }
        17 => match frames.get(rng.below(frames.len())) {
            Some(f) if *f != n => ("node.reparent", json!({"ids": [n], "parent": f})),
            _ => ("node.reparent", json!({"ids": [n], "parent": page})),
        },
        18 => ("node.set", json!({"id": n, "isMask": !doc.node(n).is_some_and(|x| x.is_mask)})),
        19 => ("node.set", json!({"id": n, "cornerRadius": rng.range(0.0, 30.0).round()})),
        20 => {
            let blend = ["normal", "multiply", "screen", "overlay", "difference"][rng.below(5)];
            ("node.set", json!({"id": n, "blend": blend}))
        }
        21 => {
            let parent = doc.node(n).and_then(|x| x.parent).and_then(|p| doc.node(p));
            let Some(sib) = parent.and_then(|p| p.children.get(rng.below(p.children.len())).copied()) else { return "no sibling".into() };
            if rng.chance(0.5) && sib != n { ("object.group", json!({"ids": [n, sib]})) } else { ("object.ungroup", json!({"ids": [n]})) }
        }
        22 => ("variable.set", json!({"variable": "brand", "value": rng.color()})),
        23 => {
            let styles = &doc.styles;
            let Some(st) = styles.get(rng.below(styles.len())) else { return "no style".into() };
            ("style.update", json!({"style": st.id, "from": n}))
        }
        _ => {
            let align = ["inside", "center", "outside"][rng.below(3)];
            ("node.set", json!({"id": n, "stroke": rng.color(), "strokeWeight": rng.range(0.0, 8.0).round(), "strokeAlign": align}))
        }
    };
    let ok = run(s, cmd, p.clone()).is_some();
    format!("{cmd} {p}{}", if ok { "" } else { " (refused)" })
}

/// Pixels where `a` differs from `b` beyond anti-aliasing, and the first of them. A part rendered
/// on its own sees curves clipped (so flattened) at other places, which moves the coverage of edge
/// pixels a little; where both images are flat around a pixel, any difference is stale or missing
/// paint, and so is a big difference anywhere.
fn stale(a: &Pixmap, b: &Pixmap) -> (usize, Option<(u32, u32)>) {
    let (w, h) = (b.width() as usize, b.height() as usize);
    let (pa, pb) = (a.data().as_chunks::<4>().0, b.data().as_chunks::<4>().0);
    let d = |p: &[u8; 4], q: &[u8; 4]| p.iter().zip(q).map(|(x, y)| x.abs_diff(*y)).max().unwrap_or(0);
    let edge = |px: &[[u8; 4]], x: usize, y: usize| {
        let c = &px[y * w + x];
        (y.saturating_sub(1)..(y + 2).min(h)).any(|ny| (x.saturating_sub(1)..(x + 2).min(w)).any(|nx| d(&px[ny * w + nx], c) > 2))
    };
    let (mut n, mut first) = (0, None);
    for (i, (p, q)) in pa.iter().zip(pb).enumerate() {
        let diff = d(p, q);
        let (x, y) = (i % w, i / w);
        if diff > 96 || (diff > 2 && !edge(pa, x, y) && !edge(pb, x, y)) {
            n += 1;
            first.get_or_insert((x as u32, y as u32));
        }
    }
    (n, first)
}

/// Walk `steps` random steps from `seed`; panics with the steps so far on a divergence.
fn walk(seed: u64, steps: usize) -> (usize, usize) {
    let (mut s, page) = scene();
    let mut rng = Rng(seed);
    let mut view = View { origin: (-20.0, -10.0), scale: 1.0, width: 320, height: 240 };
    let opts = Options { lod: true, ..Default::default() };
    let mut retained = Retained::default();
    let mut log: Vec<String> = Vec::new();
    let mut partial = 0;
    for k in 0..=steps {
        if k > 0 {
            log.push(step(&mut s, page, &mut view, &mut rng));
        }
        let doc: Arc<Document> = s.ds().unwrap().doc.clone();
        let (shown, pix, work) = retained.render(&doc, page, &view, &opts).unwrap();
        let pix = pix.clone();
        partial += usize::from(matches!(work, Work::Partial { .. }));
        let full = render_view(&doc, page, &shown, &opts).unwrap();
        assert_eq!((pix.width(), pix.height()), (full.width(), full.height()));
        let (n, first) = stale(&pix, &full);
        assert!(
            n == 0,
            "seed {seed}, step {k} ({work:?}): retained differs from a full render ({n} pixels, first at {first:?})\nsteps:\n{}",
            log.join("\n")
        );
    }
    (steps, partial)
}

#[test]
fn retained_renders_match_full_renders_along_a_random_walk() {
    for seed in [1, 2, 3] {
        let (steps, partial) = walk(seed, 120);
        // The walk exercises the partial path, not only full renders.
        assert!(partial > steps / 5, "seed {seed}: only {partial} partial renders in {steps} steps");
    }
}
