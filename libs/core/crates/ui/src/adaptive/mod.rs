//! Layout by window width (decision D23): [`FormFactor`], touch sizes and a layout audit.
//!
//! The layout never depends on a "phone" or "tablet" flag or a setting, only on the width of the
//! window in egui points (dp on Android), so rotation, split screen, floating windows and
//! foldables change it by themselves. [`FormFactor::Expanded`] is today's desktop layout and
//! must stay exactly as it is.

mod audit;
mod command_bar;
mod form_factor;

pub use audit::{Finding, FindingKind, audit, audit_excluding, audit_widths, small_targets};
pub use command_bar::{command_bar, register_overlay, register_scroll_region};
pub use form_factor::{
    COMPACT_MAX, EXPANDED_MIN, FormFactor, HYSTERESIS, MIN_TOUCH, dialog_rect, dialog_window, touch_spacing,
};

#[cfg(test)]
mod tests;
