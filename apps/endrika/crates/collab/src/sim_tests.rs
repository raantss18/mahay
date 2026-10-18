//! Convergence: several clients editing at once over links with random delays end up with exactly
//! the server's state (synthetic edits; the engine's tests drive real commands).

use std::collections::BTreeSet;

use figcraft_color::{Color, Paint};
use figcraft_doc::{Document, NodeId, NodeKind, Style, StyleKind};

use crate::sim::{Rng, Sim};
use crate::{NoRecompute, Recompute};

/// A toy derived value: a frame's width is ten times its number of children.
pub(crate) struct Width;

impl Recompute for Width {
    fn recompute(&self, doc: &mut Document, _: &Document, touched: &BTreeSet<NodeId>) {
        for id in touched {
            let want = doc.node(*id).filter(|n| matches!(n.kind, NodeKind::Frame)).map(|n| n.children.len() as f64 * 10.0);
            if let Some(w) = want
                && doc.node(*id).is_some_and(|n| n.width != w)
                && let Some(n) = doc.node_mut(*id)
            {
                n.width = w;
            }
        }
    }
}

pub(crate) fn base() -> Document {
    let mut d = Document::new("Shared");
    let page = d.first_page().unwrap();
    for i in 0..3 {
        let f = d.create(page, NodeKind::Frame, &format!("F{i}"), i as f64 * 200.0, 0.0, 0.0, 100.0).unwrap();
        for j in 0..3 {
            d.create(f, NodeKind::Rectangle, &format!("R{i}{j}"), 0.0, j as f64 * 20.0, 10.0, 10.0).unwrap();
        }
        d.get_mut(f).unwrap().width = 30.0;
    }
    d.take_dirty();
    d
}

/// One random edit of `doc`, the way a command would make it (derived data recomputed after).
pub(crate) fn random_edit(doc: &mut Document, rng: &mut Rng, derive: &dyn Recompute) {
    let before = doc.clone();
    doc.take_dirty();
    let ids: Vec<NodeId> = doc.ids().collect();
    let containers: Vec<NodeId> = ids.iter().copied().filter(|i| doc.node(*i).is_some_and(|n| n.kind.is_container())).collect();
    let layers: Vec<NodeId> = ids.iter().copied().filter(|i| doc.node(*i).is_some_and(|n| !n.is_page())).collect();
    let pick = |rng: &mut Rng, v: &[NodeId]| rng.pick(v).copied();
    match rng.below(14) {
        0 | 1 | 12 => {
            if let Some(p) = pick(rng, &containers) {
                let kind = if rng.chance(0.3) { NodeKind::Frame } else { NodeKind::Rectangle };
                let _ = doc.create(p, kind, "New", rng.below(500) as f64, rng.below(500) as f64, 10.0, 10.0);
            }
        }
        2 => {
            // Reparent: cycles are refused locally (and concurrent ones by the server).
            if let (Some(x), Some(p)) = (pick(rng, &layers), pick(rng, &containers)) {
                let at = rng.below(4);
                let _ = doc.place(x, p, at);
            }
        }
        3 => {
            if let Some(x) = pick(rng, &layers) {
                let at = rng.below(5);
                let _ = doc.reorder(x, at);
            }
        }
        4 => {
            // Deletions take whole subtrees: keep them rarer than creations.
            if let Some(x) = pick(rng, &layers).filter(|_| rng.chance(0.5)) {
                let _ = doc.remove(x);
            }
        }
        5..=7 => {
            if let Some(n) = pick(rng, &layers).and_then(|x| doc.node_mut(x)) {
                n.transform = figcraft_geom::Affine::translate((rng.below(800) as f64 * 0.5, rng.below(800) as f64 * 0.25));
            }
        }
        8 => {
            if let Some(n) = pick(rng, &layers).and_then(|x| doc.node_mut(x)) {
                n.name = format!("Name {}", rng.below(100));
                n.opacity = rng.below(11) as f32 / 10.0;
            }
        }
        9 => {
            if let Some(n) = pick(rng, &layers).and_then(|x| doc.node_mut(x)) {
                n.fills = vec![Paint::solid(Color::from_rgb8_const(rng.below(256) as u8, 0, 0))];
            }
        }
        10 => {
            if rng.chance(0.5) || doc.pages.len() < 2 {
                doc.add_page("Page");
            } else if let Some(p) = pick(rng, &doc.pages.clone()) {
                let _ = doc.remove(p);
            }
        }
        11 => {
            if doc.pages.len() > 1 {
                doc.pages.rotate_left(1);
            }
        }
        _ => {
            let id = format!("style{}", rng.below(6));
            match doc.styles.iter().position(|s| s.id == id) {
                Some(i) if rng.chance(0.4) => {
                    doc.styles.remove(i);
                }
                Some(i) => doc.styles[i].name = format!("Renamed {}", rng.below(100)),
                None => doc.styles.push(Style {
                    id,
                    name: "S".into(),
                    description: String::new(),
                    kind: StyleKind::Paint { paints: vec![] },
                    bound: Default::default(),
                }),
            }
        }
    }
    let touched = doc.take_dirty();
    derive.recompute(doc, &before, &touched);
    doc.take_dirty();
}

fn run(seed: u64, clients: usize, rounds: usize, derive: &dyn Recompute) -> Sim {
    let mut sim = Sim::new(base(), clients, seed, 12).unwrap();
    for _ in 0..rounds {
        let i = sim.rng().below(clients);
        let mut doc = sim.document(i).unwrap().clone();
        let mut rng = Rng::new(sim.rng().next_u64());
        random_edit(&mut doc, &mut rng, derive);
        sim.edit(i, &doc).unwrap();
        for _ in 0..sim.rng().below(4) {
            sim.step(derive).unwrap();
        }
    }
    sim.settle(derive, 100_000).unwrap();
    if let Err(e) = sim.converged() {
        panic!("seed {seed}: {e}");
    }
    sim
}

#[test]
fn clients_converge_on_the_server_state() {
    for seed in 0..12 {
        let sim = run(seed, 4, 60, &NoRecompute);
        assert!(sim.delivered > 60);
        assert!(sim.server.document().validate().is_empty());
    }
}

#[test]
fn derived_data_is_repaired_after_concurrent_edits() {
    for seed in 100..110 {
        let sim = run(seed, 3, 50, &Width);
        let doc = sim.server.document();
        for n in doc.nodes().filter(|n| matches!(n.kind, NodeKind::Frame)) {
            assert_eq!(n.width, n.children.len() as f64 * 10.0, "seed {seed}: frame {} has {} children", n.id, n.children.len());
        }
    }
}

#[test]
#[ignore = "longer run for development: cargo test -p figcraft-collab -- --ignored"]
fn many_seeds_converge() {
    for seed in 1000..1300 {
        run(seed, 4, 120, &Width);
    }
}
