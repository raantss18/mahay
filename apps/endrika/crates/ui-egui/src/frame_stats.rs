//! Frame statistics for spotting wasted redraws: how many frames the UI drew and what asked for
//! them (the `file:line` of each repaint request), read through `ui.inspect` (`framesDrawn`,
//! `repaintCauses`) and reset with `ui.set {resetFrameStats: true}`. An idle window should draw
//! no frames at all.

use std::collections::BTreeMap;

use serde_json::{Value, json};

/// How many of the most frequent repaint causes `ui.inspect` lists.
const TOP_CAUSES: usize = 12;

#[derive(Default)]
pub struct FrameStats {
    /// Frames drawn since the app started (or the last reset).
    pub frames: u64,
    /// Repaint requests by call site, counted once per frame they asked for.
    causes: BTreeMap<(&'static str, u32), u64>,
}

impl FrameStats {
    /// Count this frame and what asked for it (call once a frame).
    pub fn record(&mut self, ctx: &egui::Context) {
        self.frames += 1;
        for c in ctx.repaint_causes() {
            *self.causes.entry((c.file, c.line)).or_default() += 1;
        }
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// The most frequent repaint causes, most frequent first: `[{at: "file:line", count}]`.
    pub fn causes(&self) -> Value {
        let mut top: Vec<(&(&'static str, u32), &u64)> = self.causes.iter().collect();
        top.sort_by(|a, b| b.1.cmp(a.1));
        top.into_iter().take(TOP_CAUSES).map(|((file, line), n)| json!({"at": format!("{file}:{line}"), "count": n})).collect()
    }
}
