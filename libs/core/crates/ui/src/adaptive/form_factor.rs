use egui::{Context, Id, Rect, Ui};

/// Window width below which the layout is [`FormFactor::Compact`] (Android's window size classes).
pub const COMPACT_MAX: f32 = 600.0;
/// Window width from which the layout is [`FormFactor::Expanded`].
pub const EXPANDED_MIN: f32 = 840.0;
/// How far past a threshold the width must go before the class changes, so a width resting
/// exactly on a threshold never flickers.
pub const HYSTERESIS: f32 = 8.0;
/// Smallest touch target, in points (dp on Android).
pub const MIN_TOUCH: f32 = 48.0;

/// The layout class of a window, from its width alone.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FormFactor {
    /// Under 600: a phone upright, a narrow split screen. One column, drawers, full-screen dialogs.
    Compact,
    /// 600 to 839: a tablet upright, a phone-sized foldable open. A lighter version of Compact.
    Medium,
    /// 840 and up: the desktop layout, unchanged.
    Expanded,
}

impl FormFactor {
    /// The class of a window `width` points wide, with no memory (no hysteresis).
    pub fn from_width(width: f32) -> Self {
        if width < COMPACT_MAX {
            Self::Compact
        } else if width < EXPANDED_MIN {
            Self::Medium
        } else {
            Self::Expanded
        }
    }

    /// The class of a window `width` points wide that was `previous`: it changes only once the width
    /// is [`HYSTERESIS`] past the threshold. Applying it to its own result changes nothing.
    pub fn next(previous: Self, width: f32) -> Self {
        let mut class = previous;
        loop {
            let moved = match class {
                Self::Compact if width >= COMPACT_MAX + HYSTERESIS => Self::Medium,
                Self::Medium if width >= EXPANDED_MIN + HYSTERESIS => Self::Expanded,
                Self::Medium if width < COMPACT_MAX - HYSTERESIS => Self::Compact,
                Self::Expanded if width < EXPANDED_MIN - HYSTERESIS => Self::Medium,
                other => other,
            };
            if moved == class {
                return class;
            }
            class = moved;
        }
    }

    /// The class of the window right now, recomputed from its width every frame. Calling it
    /// several times in a frame gives the same answer.
    pub fn of(ctx: &Context) -> Self {
        let width = ctx.content_rect().width();
        let id = Id::new("mahay_form_factor");
        let previous = ctx.data(|d| d.get_temp::<Self>(id)).unwrap_or_else(|| Self::from_width(width));
        let class = Self::next(previous, width);
        if class != previous {
            ctx.data_mut(|d| d.insert_temp(id, class));
        } else if ctx.data(|d| d.get_temp::<Self>(id)).is_none() {
            ctx.data_mut(|d| d.insert_temp(id, class));
        }
        class
    }

    /// The class of the window `ui` is in.
    pub fn of_ui(ui: &Ui) -> Self {
        Self::of(ui.ctx())
    }

    /// Whether this is the phone layout.
    pub fn is_compact(self) -> bool {
        self == Self::Compact
    }

    /// Whether this is anything but the desktop layout.
    pub fn is_touch_layout(self) -> bool {
        self != Self::Expanded
    }

    /// Smallest height of a command, in points: [`MIN_TOUCH`] in Compact, 40 in Medium, `desktop`
    /// in Expanded (the app's own size, unchanged).
    pub fn command_height(self, desktop: f32) -> f32 {
        match self {
            Self::Compact => MIN_TOUCH,
            Self::Medium => 40.0_f32.max(desktop),
            Self::Expanded => desktop,
        }
    }
}

/// Roomier spacing for touch inside `ui`: the minimum target of this class and a wider gap
/// between commands. Does nothing in Expanded.
pub fn touch_spacing(ui: &mut Ui, form: FormFactor) {
    if form == FormFactor::Expanded {
        return;
    }
    let spacing = ui.spacing_mut();
    spacing.interact_size.y = spacing.interact_size.y.max(form.command_height(0.0));
    spacing.item_spacing.x = spacing.item_spacing.x.max(if form.is_compact() { 6.0 } else { 4.0 });
    spacing.button_padding.x = spacing.button_padding.x.max(if form.is_compact() { 10.0 } else { 8.0 });
    spacing.icon_width = spacing.icon_width.max(24.0);
}

/// Where a dialog goes: in Compact it fills the window (`None` means "the app's own size and
/// place"); elsewhere it keeps its own place.
pub fn dialog_rect(ctx: &Context) -> Option<Rect> {
    FormFactor::of(ctx).is_compact().then(|| ctx.content_rect())
}

/// `window` made to fill the screen in Compact (no moving, no collapsing, no resizing) and left
/// alone in Medium and Expanded, apart from never being wider than the screen.
pub fn dialog_window<'a>(ctx: &Context, window: egui::Window<'a>) -> egui::Window<'a> {
    let screen = ctx.content_rect();
    match dialog_rect(ctx) {
        Some(rect) => window.fixed_rect(rect).collapsible(false).resizable(false).movable(false),
        None => window.constrain_to(screen).max_size(screen.size()),
    }
}
