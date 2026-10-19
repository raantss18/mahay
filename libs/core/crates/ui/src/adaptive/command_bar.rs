use egui::{Context, Id, Rect, ScrollArea, Ui, scroll_area::ScrollBarVisibility};

use super::form_factor::FormFactor;

const REGIONS: &str = "mahay_scroll_regions";

/// Remembers that `rect` is a scrolling region: what it holds may run past its edge (and the
/// window's) without being a layout fault. Cleared at the start of every frame by the first call.
pub fn register_scroll_region(ctx: &Context, rect: Rect) {
    let id = Id::new(REGIONS);
    let frame = ctx.cumulative_pass_nr();
    ctx.data_mut(|d| {
        let (at, rects) = d.get_temp_mut_or_insert_with::<(u64, Vec<Rect>)>(id, || (frame, Vec::new()));
        if *at != frame {
            *at = frame;
            rects.clear();
        }
        rects.push(rect);
    });
}

/// The scrolling regions drawn in the last frame.
pub(super) fn scroll_regions(ctx: &Context) -> Vec<Rect> {
    ctx.data(|d| d.get_temp::<(u64, Vec<Rect>)>(Id::new(REGIONS))).map(|(_, r)| r).unwrap_or_default()
}

const OVERLAYS: &str = "mahay_overlays";

/// Remembers that `rect` is an overlay drawn on purpose over what is under it (a scroll chevron on the
/// edge of a bar): the audit does not report overlaps with it. Cleared like the scroll regions.
pub fn register_overlay(ctx: &Context, rect: Rect) {
    let id = Id::new(OVERLAYS);
    let frame = ctx.cumulative_pass_nr();
    ctx.data_mut(|d| {
        let (at, rects) = d.get_temp_mut_or_insert_with::<(u64, Vec<Rect>)>(id, || (frame, Vec::new()));
        if *at != frame {
            *at = frame;
            rects.clear();
        }
        rects.push(rect);
    });
}

/// The overlays drawn in the last frame.
pub(super) fn overlays(ctx: &Context) -> Vec<Rect> {
    ctx.data(|d| d.get_temp::<(u64, Vec<Rect>)>(Id::new(OVERLAYS))).map(|(_, r)| r).unwrap_or_default()
}

/// One horizontally scrolling row of commands that never wraps and never leaves the window: drag to
/// scroll, no visible bar. In Expanded it is a plain row (the desktop layout, which sizes itself).
pub fn command_bar<R>(ui: &mut Ui, id: impl std::hash::Hash + std::fmt::Debug, add: impl FnOnce(&mut Ui) -> R) -> R {
    let form = FormFactor::of_ui(ui);
    if form == FormFactor::Expanded {
        return ui.horizontal(add).inner;
    }
    let out = ScrollArea::horizontal()
        .id_salt(id)
        .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
        .show(ui, |ui| {
            super::form_factor::touch_spacing(ui, form);
            ui.horizontal_centered(add).inner
        });
    register_scroll_region(ui.ctx(), out.inner_rect);
    out.inner
}
