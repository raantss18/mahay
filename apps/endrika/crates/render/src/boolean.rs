//! Boolean group results for drawing, cached so editing one operand stays cheap.
//!
//! A boolean group's result is its operands folded bottom to top, each combined with the result
//! of those below it by its own operation (`Node::bool_op`, else the group's). Every prefix of the
//! fold is cached under a rolling hash of the operands it covers, so when an operand changes (a
//! drag, a point edit) the fold restarts from the longest unchanged prefix: the operands below the
//! edited one are never recombined, and groups whose operands didn't change aren't recomputed at
//! all. The caches drop their least recently used half when full, so a long drag doesn't evict
//! every other group's result.

use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::{Arc, Mutex, OnceLock};

use figcraft_doc::BoolOp;
use figcraft_geom::BezPath;

use crate::draw::hash_path;

/// A bounded map of computed paths; the least recently used half goes when it is full.
pub(crate) struct PathCache {
    map: HashMap<u64, (Arc<BezPath>, u64)>,
    tick: u64,
    cap: usize,
}

impl PathCache {
    pub(crate) fn new(cap: usize) -> Self {
        Self { map: HashMap::new(), tick: 0, cap: cap.max(2) }
    }

    pub(crate) fn get(&mut self, key: u64) -> Option<Arc<BezPath>> {
        self.tick += 1;
        let tick = self.tick;
        self.map.get_mut(&key).map(|(p, used)| {
            *used = tick;
            p.clone()
        })
    }

    pub(crate) fn insert(&mut self, key: u64, path: Arc<BezPath>) {
        if self.map.len() >= self.cap {
            let mut ticks: Vec<u64> = self.map.values().map(|(_, t)| *t).collect();
            // Keep the newer half: drop every entry used at or before the median tick.
            let mid = (ticks.len() / 2).saturating_sub(1);
            let (_, cut, _) = ticks.select_nth_unstable(mid);
            let cut = *cut;
            self.map.retain(|_, (_, t)| *t > cut);
        }
        self.tick += 1;
        self.map.insert(key, (path, self.tick));
    }
}

/// The shared cache of a cache-returning `OnceLock`, recovering from a poisoned lock.
pub(crate) fn locked(c: &Mutex<PathCache>) -> std::sync::MutexGuard<'_, PathCache> {
    c.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

fn cache() -> &'static Mutex<PathCache> {
    static C: OnceLock<Mutex<PathCache>> = OnceLock::new();
    C.get_or_init(|| Mutex::new(PathCache::new(4096)))
}

thread_local! {
    static STEPS: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// How many boolean steps (one operand combined with the result below it) this thread has
/// computed: what a cache miss costs. For tests and benchmarks.
pub fn boolean_steps() -> u64 {
    STEPS.with(std::cell::Cell::get)
}

/// The result of folding `operands` (bottom to top, each with its operation; the bottom one's is
/// unused), reusing the longest cached prefix. Invalid geometry gives an empty path.
pub fn fold_cached(operands: &[(BezPath, BoolOp)]) -> BezPath {
    if operands.len() < 2 {
        return figcraft_pathops::boolean_ops(operands.iter().map(|(p, op)| (p, *op))).unwrap_or_default();
    }
    // keys[i]: the fold of operands 0..=i.
    let mut h = DefaultHasher::new();
    let keys: Vec<u64> = operands
        .iter()
        .enumerate()
        .map(|(i, (p, op))| {
            if i > 0 {
                (*op as u8).hash(&mut h);
            }
            hash_path(p, &mut h);
            9u8.hash(&mut h);
            h.finish()
        })
        .collect();
    let hit = {
        let mut c = locked(cache());
        keys.iter().enumerate().skip(1).rev().find_map(|(i, k)| c.get(*k).map(|p| (i, p)))
    };
    let (start, mut acc) = match hit {
        Some((i, p)) if i + 1 == operands.len() => return (*p).clone(),
        Some((i, p)) => (i, (*p).clone()),
        None => match operands.first() {
            Some((p, _)) => (0, p.clone()),
            None => return BezPath::new(),
        },
    };
    let mut fresh = Vec::new();
    for ((p, op), key) in operands.iter().zip(&keys).skip(start + 1) {
        STEPS.with(|s| s.set(s.get() + 1));
        acc = match figcraft_pathops::boolean_step(&acc, p, *op) {
            Ok(r) => r,
            Err(_) => return BezPath::new(),
        };
        fresh.push((*key, Arc::new(acc.clone())));
    }
    let mut c = locked(cache());
    for (k, p) in fresh {
        c.insert(k, p);
    }
    acc
}
