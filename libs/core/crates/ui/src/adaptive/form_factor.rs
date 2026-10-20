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

/// Window height below which the layout is [`HeightClass::Compact`] (a phone on its side).
pub const HEIGHT_COMPACT_MAX: f32 = 480.0;
/// Window height from which the layout is [`HeightClass::Expanded`].
pub const HEIGHT_EXPANDED_MIN: f32 = 900.0;

/// The height class of a window (Android's window size classes, the other axis).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HeightClass {
    /// Under 480: a phone on its side, a short floating window. Chrome gives its room to the content.
    Compact,
    /// 480 to 899.
    Medium,
    /// 900 and up.
    Expanded,
}

impl HeightClass {
    /// The class of a window `height` points high, with no memory.
    pub fn from_height(height: f32) -> Self {
        if height < HEIGHT_COMPACT_MAX {
            Self::Compact
        } else if height < HEIGHT_EXPANDED_MIN {
            Self::Medium
        } else {
            Self::Expanded
        }
    }

    /// The class after a window `height` points high that was `previous`, with the same
    /// [`HYSTERESIS`] as [`FormFactor::next`].
    pub fn next(previous: Self, height: f32) -> Self {
        let mut class = previous;
        loop {
            let moved = match class {
                Self::Compact if height >= HEIGHT_COMPACT_MAX + HYSTERESIS => Self::Medium,
                Self::Medium if height >= HEIGHT_EXPANDED_MIN + HYSTERESIS => Self::Expanded,
                Self::Medium if height < HEIGHT_COMPACT_MAX - HYSTERESIS => Self::Compact,
                Self::Expanded if height < HEIGHT_EXPANDED_MIN - HYSTERESIS => Self::Medium,
                other => other,
            };
            if moved == class {
                return class;
            }
            class = moved;
        }
    }
}

/// The size of the window the app lives in: both classes and the size itself, recomputed every
/// frame. Nothing about a layout is fixed at launch: a fold opening, split screen, a free-form
/// window, rotation and the keyboard all change it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowSize {
    /// Width class (what decides the ribbon, the panels and the dialogs).
    pub width_class: FormFactor,
    /// Height class (what decides how much chrome a short window can afford).
    pub height_class: HeightClass,
    /// The window's size in points.
    pub size: egui::Vec2,
}

impl WindowSize {
    /// The window's size now, with hysteresis on both classes.
    pub fn of(ctx: &Context) -> Self {
        let size = ctx.content_rect().size();
        let id = Id::new("mahay_height_class");
        let previous = ctx.data(|d| d.get_temp::<HeightClass>(id)).unwrap_or_else(|| HeightClass::from_height(size.y));
        let height_class = HeightClass::next(previous, size.y);
        ctx.data_mut(|d| d.insert_temp(id, height_class));
        Self { width_class: FormFactor::of(ctx), height_class, size }
    }

    /// A phone held on its side: wide enough for columns, too short for tall bars.
    pub fn is_short(&self) -> bool {
        self.height_class == HeightClass::Compact
    }
}

/// How many columns of at least `min_column` points (gaps of `gap`) fit in `width` points, between 1
/// and `max_columns`: a grid that follows the window smoothly instead of jumping between presets.
pub fn columns(width: f32, min_column: f32, gap: f32, max_columns: usize) -> usize {
    if !width.is_finite() || !min_column.is_finite() || min_column <= 0.0 {
        return 1;
    }
    let n = ((width + gap) / (min_column + gap)).floor();
    (n.max(1.0) as usize).min(max_columns.max(1))
}

/// How many of the side panels (their widths, most wanted first) can sit beside content that needs at
/// least `min_content` points in `available` points: the rest become drawers. Panels come and go one
/// at a time as the window changes, never all at once.
pub fn panels_beside(available: f32, min_content: f32, panels: &[f32]) -> usize {
    let mut left = available - min_content;
    let mut n = 0;
    for w in panels {
        if left >= *w {
            left -= *w;
            n += 1;
        } else {
            break;
        }
    }
    n
}
