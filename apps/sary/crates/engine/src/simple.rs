//! The Simple workspace's non-destructive "Adjust" model (decision D27).
//!
//! Four sliders (Light, Contrast, Color, Warmth, each -100..=100) drive four named adjustment
//! layers on top of the picture. Nothing is destroyed: the layers show in the Layers panel (so Pro
//! sees what Simple did), survive saving to `.sary` and PSD, and a slider back at 0 removes its
//! layer. A drag is one history step (`coalesce`), and "hold to compare" hides the layers without
//! touching history.

use sary_doc::{Adjustment, Document, Layer, LayerContent};

use crate::{Result, Session};

/// The layer names, in stacking order from the bottom.
pub const LAYER_NAMES: [&str; 4] = ["Simple: Light", "Simple: Contrast", "Simple: Color", "Simple: Warmth"];

/// The slider range's end: sliders run from `-RANGE` to `RANGE`.
pub const RANGE: f32 = 100.0;

/// Where the four sliders stand.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Look {
    pub light: f32,
    pub contrast: f32,
    pub color: f32,
    pub warmth: f32,
}

impl Look {
    /// The values clamped to the slider range (NaN becomes 0).
    pub fn clamped(self) -> Self {
        let c = |v: f32| if v.is_nan() { 0.0 } else { v.clamp(-RANGE, RANGE) };
        Self { light: c(self.light), contrast: c(self.contrast), color: c(self.color), warmth: c(self.warmth) }
    }

    pub fn is_neutral(self) -> bool {
        self.values().iter().all(|v| v.abs() < 0.5)
    }

    fn values(self) -> [f32; 4] {
        [self.light, self.contrast, self.color, self.warmth]
    }
}

/// Exposure in stops at a full Light slider.
const MAX_STOPS: f32 = 1.5;
const WARM: [f32; 3] = [236.0 / 255.0, 138.0 / 255.0, 0.0];
const COOL: [f32; 3] = [0.0, 0.63, 1.0];
/// Photo-filter density at a full Warmth slider.
const MAX_DENSITY: f32 = 0.5;

/// The adjustment a slider value makes, for the layer `LAYER_NAMES[kind]`.
fn adjustment(kind: usize, v: f32) -> Adjustment {
    match kind {
        0 => Adjustment::Exposure { exposure: v / RANGE * MAX_STOPS, offset: 0.0, gamma: 1.0 },
        1 => Adjustment::BrightnessContrast { brightness: 0.0, contrast: v, legacy: false },
        2 => Adjustment::Vibrance { vibrance: v, saturation: v * 0.25 },
        _ => Adjustment::PhotoFilter { color: if v >= 0.0 { WARM } else { COOL }, density: v.abs() / RANGE * MAX_DENSITY, preserve_luminosity: true },
    }
}

/// The slider value an adjustment stands for (the inverse of [`adjustment`]), if it is one of ours.
fn value_of(kind: usize, adj: &Adjustment) -> Option<f32> {
    Some(match (kind, adj) {
        (0, Adjustment::Exposure { exposure, .. }) => exposure / MAX_STOPS * RANGE,
        (1, Adjustment::BrightnessContrast { contrast, .. }) => *contrast,
        (2, Adjustment::Vibrance { vibrance, .. }) => *vibrance,
        (3, Adjustment::PhotoFilter { color, density, .. }) => {
            let sign = if color[2] > color[0] { -1.0 } else { 1.0 };
            sign * density / MAX_DENSITY * RANGE
        }
        _ => return None,
    })
}

fn find(doc: &Document, name: &str) -> Option<sary_doc::LayerId> {
    doc.walk().into_iter().find(|(_, _, l)| l.name == name && matches!(l.content, LayerContent::Adjustment(_))).map(|(_, _, l)| l.id)
}

/// Where the sliders stand in `doc`, read back from the Simple layers (neutral without them).
pub fn look_of(doc: &Document) -> Look {
    let read = |kind: usize| -> f32 {
        let Some(id) = find(doc, LAYER_NAMES[kind]) else { return 0.0 };
        match doc.layer(id).map(|l| &l.content) {
            Some(LayerContent::Adjustment(a)) => value_of(kind, a).unwrap_or(0.0),
            _ => 0.0,
        }
    };
    Look { light: read(0), contrast: read(1), color: read(2), warmth: read(3) }
}

/// Whether `doc` has any Simple layer.
pub fn has_look(doc: &Document) -> bool {
    LAYER_NAMES.iter().any(|n| find(doc, n).is_some())
}

/// Set the sliders: create, change or remove the four layers in one edit. Edits that share a
/// `coalesce` key (one slider drag) are a single history step. The active layer does not change.
pub fn apply_look(s: &mut Session, look: Look, coalesce: Option<&str>) -> Result<()> {
    let look = look.clamped();
    s.coalesce_request = coalesce.map(str::to_string);
    let r = s.edit("Simple Adjust", |doc, _active| {
        for (kind, v) in look.values().into_iter().enumerate() {
            let existing = find(doc, LAYER_NAMES[kind]);
            if v.abs() < 0.5 {
                if let Some(id) = existing {
                    doc.remove(id);
                }
                continue;
            }
            let adj = adjustment(kind, v);
            match existing.and_then(|id| doc.layer_mut(id)) {
                Some(l) => l.content = LayerContent::Adjustment(adj),
                None => doc.layers.push(Layer::new(LAYER_NAMES[kind], LayerContent::Adjustment(adj))),
            }
        }
        Ok(())
    });
    s.coalesce_request = None;
    r
}

/// Show or hide the Simple layers to compare with the original. Not a history step and not an
/// edit: the document stays clean. Returns whether anything changed.
pub fn set_compare(s: &mut Session, original: bool) -> bool {
    let Some(st) = s.active_mut() else { return false };
    let wanted = !original;
    let differs = LAYER_NAMES.iter().filter_map(|n| find(&st.doc, n)).any(|id| st.doc.layer(id).is_some_and(|l| l.visible != wanted));
    if !differs {
        return false;
    }
    let mut doc = (*st.doc).clone();
    for name in LAYER_NAMES {
        if let Some(l) = find(&doc, name).and_then(|id| doc.layer_mut(id)) {
            l.visible = wanted;
        }
    }
    let clean = st.saved_revision == st.revision;
    st.doc = std::sync::Arc::new(doc);
    st.revision += 1;
    if clean {
        st.saved_revision = st.revision;
    }
    st.last_damage = None;
    true
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn session() -> Session {
        let mut s = Session::new();
        s.execute("file.new", json!({"width": 16, "height": 16})).unwrap();
        s
    }

    fn doc(s: &Session) -> &Document {
        &s.active().unwrap().doc
    }

    #[test]
    fn sliders_read_back_what_was_set() {
        let mut s = session();
        let look = Look { light: 40.0, contrast: -20.0, color: 55.0, warmth: -35.0 };
        apply_look(&mut s, look, None).unwrap();
        let back = look_of(doc(&s));
        for (a, b) in look.values().into_iter().zip(back.values()) {
            assert!((a - b).abs() < 0.01, "{a} vs {b}");
        }
    }

    #[test]
    fn neutral_sliders_leave_no_layers_and_zero_removes_them() {
        let mut s = session();
        let layers = doc(&s).walk().len();
        apply_look(&mut s, Look::default(), None).unwrap();
        assert_eq!(doc(&s).walk().len(), layers);
        apply_look(&mut s, Look { light: 30.0, ..Look::default() }, None).unwrap();
        assert_eq!(doc(&s).walk().len(), layers + 1);
        assert!(has_look(doc(&s)));
        apply_look(&mut s, Look::default(), None).unwrap();
        assert_eq!(doc(&s).walk().len(), layers);
        assert!(!has_look(doc(&s)));
    }

    #[test]
    fn a_drag_is_one_history_step_and_undo_restores_the_picture() {
        let mut s = session();
        let steps = s.active().unwrap().history.past_len();
        for v in [5.0, 20.0, 45.0, 60.0] {
            apply_look(&mut s, Look { light: v, warmth: v / 2.0, ..Look::default() }, Some("simple-drag")).unwrap();
        }
        assert_eq!(s.active().unwrap().history.past_len(), steps + 1);
        assert!(s.undo());
        assert!(!has_look(doc(&s)));
    }

    #[test]
    fn the_active_layer_is_not_changed() {
        let mut s = session();
        let active = s.active().unwrap().active_layer;
        apply_look(&mut s, Look { contrast: 50.0, color: 30.0, ..Look::default() }, None).unwrap();
        assert_eq!(s.active().unwrap().active_layer, active);
    }

    #[test]
    fn compare_hides_without_a_history_step_and_keeps_the_document_clean() {
        let mut s = session();
        apply_look(&mut s, Look { light: 50.0, ..Look::default() }, None).unwrap();
        let steps = s.active().unwrap().history.past_len();
        s.active_mut().unwrap().saved_revision = s.active().unwrap().revision;
        assert!(set_compare(&mut s, true));
        let id = find(doc(&s), LAYER_NAMES[0]).unwrap();
        assert!(!doc(&s).layer(id).unwrap().visible);
        assert!(!set_compare(&mut s, true));
        assert!(set_compare(&mut s, false));
        assert!(doc(&s).layer(id).unwrap().visible);
        assert_eq!(s.active().unwrap().history.past_len(), steps);
        assert_eq!(s.active().unwrap().saved_revision, s.active().unwrap().revision);
    }

    #[test]
    fn out_of_range_and_nan_are_clamped() {
        let mut s = session();
        apply_look(&mut s, Look { light: f32::NAN, contrast: 900.0, color: -900.0, warmth: f32::INFINITY }, None).unwrap();
        let back = look_of(doc(&s));
        assert_eq!(back.light, 0.0);
        assert!((back.contrast - 100.0).abs() < 0.01);
        assert!((back.color + 100.0).abs() < 0.01);
        assert!((back.warmth - 100.0).abs() < 0.01);
    }
}
