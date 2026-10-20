//! On a phone (D23) the panels (Color, Layers, History…) are a drawer: hidden until the status bar's
//! Layers button opens them over the whole window, which gives the canvas back its room. Narrow
//! windows that can hold the dock beside a usable canvas keep it docked.

use egui::{Context, Id};

/// Points a usable canvas needs beside the dock.
const MIN_CANVAS: f32 = 300.0;
/// The dock: its icon rail and its narrowest panel column (points).
const DOCK_TOTAL: f32 = 36.0 + 250.0;
/// The toolbar's width (points).
const TOOLBAR: f32 = 50.0;

fn key() -> Id {
    Id::new("sary_dock_drawer")
}

/// Whether the panels drawer is open.
pub fn open(ctx: &Context) -> bool {
    ctx.data(|d| d.get_temp::<bool>(key())).unwrap_or(false)
}

/// Opens or closes the drawer.
pub fn toggle(ctx: &Context) {
    let now = open(ctx);
    ctx.data_mut(|d| d.insert_temp(key(), !now));
}

/// Whether the dock sits beside the canvas at this window width: it does as soon as a usable canvas
/// still fits, and is a drawer below that.
pub fn docked(width: f32) -> bool {
    mahay_ui::adaptive::panels_beside(width - TOOLBAR, MIN_CANVAS, &[DOCK_TOTAL]) >= 1
}

/// The width a drawer's dock takes: the window less the toolbar and the dock's rail.
pub fn drawer_width(width: f32) -> f32 {
    (width - TOOLBAR - 36.0 - 4.0).max(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_dock_docks_as_soon_as_a_canvas_fits_beside_it() {
        assert!(!docked(360.0) && !docked(411.0) && !docked(600.0));
        assert!(docked(700.0) && docked(840.0) && docked(1280.0));
        assert!((docked(635.0)) != (docked(637.0)), "it changes once, at 636 points");
    }
}
