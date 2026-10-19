//! A layout audit that works on any egui context after a frame: no two clickable widgets of a
//! layer overlap, and none lies outside the window. Used by the apps' tests at 360, 400, 600 and
//! 840 points.

use egui::{Context, Id, Rect};

use super::command_bar::{overlays, scroll_regions};

/// What is wrong with a widget.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FindingKind {
    /// It overlaps another clickable widget (neither contains the other).
    Overlap,
    /// It lies partly or wholly outside the window and no scrolling bar holds it.
    OutsideWindow,
    /// It is smaller than the minimum touch target.
    TooSmall,
}

/// A problem found by [`audit`].
#[derive(Clone, Copy, Debug)]
pub struct Finding {
    pub kind: FindingKind,
    pub id: Id,
    pub rect: Rect,
    /// The other widget of an overlap.
    pub other: Option<Rect>,
}

/// How far two widgets may intrude on each other without counting as an overlap (borders touch).
const SLACK: f32 = 1.0;

fn near(a: &Rect, b: &Rect) -> bool {
    (a.min - b.min).abs().max_elem() < 1.0 && (a.max - b.max).abs().max_elem() < 1.0
}

/// Clickable widgets of the last frame, per layer, as `(id, full rect, visible rect)`.
fn clickables(ctx: &Context) -> Vec<(egui::LayerId, Id, Rect, Rect)> {
    ctx.viewport(|v| {
        v.prev_pass
            .widgets
            .layers()
            .flat_map(|(layer, ws)| ws.iter().map(move |w| (*layer, w)))
            .filter(|(_, w)| w.sense.senses_click() && w.rect.is_positive())
            .map(|(layer, w)| (layer, w.id, w.rect, w.interact_rect))
            .collect()
    })
}

/// Overlaps and widgets outside the window, in the last frame of `ctx`. A widget clipped away by a
/// scroll area is not on screen and is not examined; one cut by a scroll area that registered itself
/// ([`super::register_scroll_region`], done by [`super::command_bar`]) may run past the edge.
pub fn audit(ctx: &Context) -> Vec<Finding> {
    audit_excluding(ctx, &[])
}

/// Like [`audit`], leaving out everything that lies inside one of `skip` (the document page or
/// canvas, which scrolls and draws its own handles).
pub fn audit_excluding(ctx: &Context, skip: &[Rect]) -> Vec<Finding> {
    let screen = ctx.content_rect();
    let regions = scroll_regions(ctx);
    let overlays = overlays(ctx);
    let all: Vec<_> = clickables(ctx).into_iter().filter(|(_, _, _, v)| !skip.iter().any(|s| s.expand(1.0).contains_rect(*v))).collect();
    let mut found = Vec::new();
    for (i, (layer, id, rect, visible)) in all.iter().enumerate() {
        if !visible.is_positive() {
            continue;
        }
        let scrolled = regions.iter().any(|r| r.expand(1.0).contains_rect(*visible));
        let inside = screen.expand(SLACK).contains_rect(*rect);
        if !inside && !scrolled {
            found.push(Finding { kind: FindingKind::OutsideWindow, id: *id, rect: *rect, other: None });
        }
        for (layer_b, id_b, _, visible_b) in all.iter().skip(i + 1) {
            if layer != layer_b || id == id_b || !visible_b.is_positive() {
                continue;
            }
            let both = visible.intersect(*visible_b);
            let overlap = both.width() > SLACK && both.height() > SLACK;
            let nested = visible.contains_rect(*visible_b) || visible_b.contains_rect(*visible);
            let on_purpose = overlays.iter().any(|o| near(o, visible) || near(o, visible_b) || o.expand(1.0).contains_rect(*visible) || o.expand(1.0).contains_rect(*visible_b));
            if overlap && !nested && !on_purpose {
                found.push(Finding { kind: FindingKind::Overlap, id: *id, rect: *visible, other: Some(*visible_b) });
            }
        }
    }
    found
}

/// Clickable widgets smaller than `min` points in either direction (touch targets), most of
/// them wanting a bigger hit area on a phone. Containers (a widget holding another) are skipped.
pub fn small_targets(ctx: &Context, min: f32) -> Vec<Finding> {
    let all = clickables(ctx);
    all.iter()
        .filter(|(_, _, _, visible)| visible.is_positive() && (visible.width() < min || visible.height() < min))
        .filter(|(layer, id, _, visible)| !all.iter().any(|(l, i, _, v)| l == layer && i != id && visible.contains_rect(*v) && v != visible))
        .map(|(_, id, rect, _)| Finding { kind: FindingKind::TooSmall, id: *id, rect: *rect, other: None })
        .collect()
}

/// Runs `frame` for a window of each of `widths` points (height `height`) and returns the findings
/// per width. `frame(ctx, width)` must run one egui frame of the app at that width, twice if
/// the app needs a settling frame; this function only reads the result afterwards.
pub fn audit_widths(widths: &[f32], mut frame: impl FnMut(f32) -> Context) -> Vec<(f32, Vec<Finding>)> {
    widths.iter().map(|w| (*w, audit(&frame(*w)))).collect()
}
