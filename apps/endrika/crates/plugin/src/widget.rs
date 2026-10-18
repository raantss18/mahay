//! Widgets (ABI v3): a plugin draws a layer on the canvas by describing it, and FigCraft builds the
//! layers. The description comes from the plugin, so it is untrusted: [`WidgetRender::parse`]
//! checks its size, node count, depth, numbers, strings, colours and image sizes before the engine
//! sees it. The engine decides where the layers go (only inside the widget's own layer).

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::manifest::{check_id, clean};

/// Largest description a plugin may pass to `widget_render` (images included, as base64).
pub const MAX_WIDGET_BYTES: usize = 4 * 1024 * 1024;
/// Largest widget state (its JSON), and its longest key.
pub const MAX_STATE_BYTES: usize = 16 * 1024;
const MAX_STATE_KEY: usize = 128;
/// Most layers in one widget, and how deep they nest.
pub const MAX_NODES: usize = 256;
const MAX_DEPTH: usize = 16;
/// Longest text in a text layer.
const MAX_TEXT: usize = 1000;
const MAX_NAME: usize = 64;
/// Sizes, positions, gaps and paddings stay within this.
const MAX_SIZE: f64 = 4000.0;
/// Largest image (decoded bytes), and all images of one widget together.
pub const MAX_IMAGE_BYTES: usize = 1024 * 1024;
const MAX_IMAGES_BYTES: usize = 2 * 1024 * 1024;

/// What a widget is made of: FigCraft's own layer kinds.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WidgetKind {
    #[default]
    Frame,
    Text,
    Rectangle,
    Ellipse,
    /// A rectangle filled with an image (PNG, JPEG…) the plugin carries in its module.
    Image,
}

/// Auto layout direction of a frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WidgetLayout {
    Horizontal,
    Vertical,
}

/// Alignment of an auto layout frame's children, along (`justify`) or across (`align`) it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WidgetAlign {
    Min,
    Center,
    Max,
}

/// One layer of a widget's description.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WidgetNode {
    #[serde(rename = "type", default)]
    pub kind: WidgetKind,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Clicking this layer calls the plugin back with this name (empty: not clickable).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub target: String,
    /// Position in the parent frame (ignored inside auto layout and for the widget itself).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    /// Size; a frame with auto layout hugs its contents on an axis without one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    /// Hex colours (`#RRGGBB` or `#RRGGBBAA`); for text, `color` is the text's.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fill: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stroke: Option<String>,
    #[serde(rename = "strokeWeight", default, skip_serializing_if = "Option::is_none")]
    pub stroke_weight: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layout: Option<WidgetLayout>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gap: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub padding: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub align: Option<WidgetAlign>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub justify: Option<WidgetAlign>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub text: String,
    /// Font size of a text layer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub bold: bool,
    /// An image layer's bytes, base64.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub data: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<WidgetNode>,
}

/// What a widget callback hands back with `widget_render`: the widget's new state (`None`: keep
/// it) and its layers, the root a frame.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WidgetRender {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<Map<String, Value>>,
    pub root: WidgetNode,
}

/// What one description may still use while it's checked.
struct Budget {
    nodes: usize,
    image_bytes: usize,
}

impl WidgetRender {
    /// Parse and validate a widget description.
    pub fn parse(text: &str) -> Result<WidgetRender, String> {
        if text.len() > MAX_WIDGET_BYTES {
            return Err(format!("the widget is larger than {} MB", MAX_WIDGET_BYTES / (1024 * 1024)));
        }
        let mut w: WidgetRender = serde_json::from_str(text).map_err(|e| format!("widget: {e}"))?;
        if let Some(state) = &w.state {
            check_state(state)?;
        }
        if w.root.kind != WidgetKind::Frame {
            return Err("widget: the root must be a frame".into());
        }
        w.root.check(0, &mut Budget { nodes: 0, image_bytes: 0 }).map_err(|e| format!("widget: {e}"))?;
        Ok(w)
    }
}

/// A widget's state: a JSON object of at most [`MAX_STATE_BYTES`] with short keys.
pub fn check_state(state: &Map<String, Value>) -> Result<(), String> {
    if state.keys().any(|k| k.is_empty() || k.chars().count() > MAX_STATE_KEY || k.chars().any(char::is_control)) {
        return Err(format!("widget: a state key must be 1–{MAX_STATE_KEY} characters without control characters"));
    }
    if serde_json::to_string(state).map_or(usize::MAX, |s| s.len()) > MAX_STATE_BYTES {
        return Err(format!("widget: the state is larger than {} KB", MAX_STATE_BYTES / 1024));
    }
    Ok(())
}

/// `v` when it's finite and within `lo..=hi`.
fn within(what: &str, v: Option<f64>, lo: f64, hi: f64) -> Result<(), String> {
    match v {
        Some(n) if !n.is_finite() || n < lo || n > hi => Err(format!("`{what}` must be a number from {lo} to {hi}")),
        _ => Ok(()),
    }
}

/// A colour as `#RRGGBB(AA)`.
fn colour(what: &str, c: &mut Option<String>) -> Result<(), String> {
    if let Some(s) = c {
        let parsed = Some(s.as_str()).filter(|s| s.len() <= 9).and_then(figcraft_color::Color::from_hex);
        *s = parsed.ok_or_else(|| format!("`{what}` must be a hex colour like #FF5A5F"))?.to_css_hex();
    }
    Ok(())
}

impl WidgetNode {
    fn check(&mut self, depth: usize, budget: &mut Budget) -> Result<(), String> {
        budget.nodes += 1;
        if budget.nodes > MAX_NODES {
            return Err(format!("more than {MAX_NODES} layers"));
        }
        if depth >= MAX_DEPTH {
            return Err(format!("layers nest more than {MAX_DEPTH} deep"));
        }
        self.name = clean("name", &self.name, MAX_NAME)?;
        if !self.target.is_empty() {
            check_id("target", &self.target, MAX_NAME)?;
        }
        for (what, v) in [("x", self.x), ("y", self.y)] {
            within(what, v, -MAX_SIZE, MAX_SIZE)?;
        }
        for (what, v) in [("width", self.width), ("height", self.height), ("gap", self.gap), ("padding", self.padding), ("radius", self.radius)] {
            within(what, v, 0.0, MAX_SIZE)?;
        }
        within("strokeWeight", self.stroke_weight, 0.0, 64.0)?;
        within("opacity", self.opacity, 0.0, 1.0)?;
        within("size", self.size, 1.0, 256.0)?;
        colour("fill", &mut self.fill)?;
        colour("stroke", &mut self.stroke)?;
        colour("color", &mut self.color)?;
        let kind = self.kind;
        let only = |what: &str, used: bool, allowed: bool| {
            if used && !allowed { Err(format!("`{what}` doesn't apply to a {kind:?} layer")) } else { Ok(()) }
        };
        only("children", !self.children.is_empty(), kind == WidgetKind::Frame)?;
        only(
            "layout",
            self.layout.is_some() || self.gap.is_some() || self.padding.is_some() || self.align.is_some() || self.justify.is_some(),
            kind == WidgetKind::Frame,
        )?;
        only("text", !self.text.is_empty() || self.size.is_some() || self.color.is_some() || self.bold, kind == WidgetKind::Text)?;
        only("data", !self.data.is_empty(), kind == WidgetKind::Image)?;
        if self.text.chars().count() > MAX_TEXT {
            return Err(format!("a text is longer than {MAX_TEXT} characters"));
        }
        if self.text.chars().any(|c| c.is_control() && c != '\n') {
            return Err("a text contains control characters".into());
        }
        if kind == WidgetKind::Image {
            // Base64: four characters for three bytes (checked again when decoded).
            let bytes = self.data.trim().len() / 4 * 3;
            if self.data.trim().is_empty() || bytes > MAX_IMAGE_BYTES + 3 {
                return Err(format!("an image needs `data` (base64) of at most {} KB", MAX_IMAGE_BYTES / 1024));
            }
            budget.image_bytes += bytes;
            if budget.image_bytes > MAX_IMAGES_BYTES + 3 {
                return Err(format!("the images are larger than {} MB together", MAX_IMAGES_BYTES / (1024 * 1024)));
            }
        }
        self.children.iter_mut().try_for_each(|c| c.check(depth + 1, budget))
    }
}
