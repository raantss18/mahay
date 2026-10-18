//! Picture Format ▸ Color: Recolor presets and Set Transparent Color.

use deckcraft_color::{Rgba, SchemeSlot};
use deckcraft_model::style::PictureAdjust;
use deckcraft_model::{ColorRef, ShapeKind};
use serde_json::{Value, json};

use super::*;
use crate::{Result, Session};

pub fn specs() -> Vec<CommandSpec> {
    vec![
        cmd!(
            "picture.recolor",
            "Recolor",
            ["Picture Format", "Adjust", "Color"],
            None,
            "{preset: none|grayscale|sepia|washout|bw25|bw50|bw75|accent1Dark…accent6Dark|accent1Light…accent6Light|tx2Dark|tx2Light (see picture.recolors), ids?}",
            has_selection,
            recolor
        ),
        cmd!(query "picture.recolors", "Recolor Gallery", [], None, "{} → [{id, label}]", always, |_s, _p| Ok(json!(
            presets().into_iter().map(|(id, label)| json!({"id": id, "label": label})).collect::<Vec<_>>()
        ))),
        cmd!(
            "picture.transparentColor",
            "Set Transparent Color",
            ["Picture Format", "Adjust", "Color"],
            None,
            "{color: #RRGGBB | none: true, ids?} (pixels of that colour become transparent)",
            has_selection,
            transparent_color
        ),
    ]
}

const ACCENTS: [(&str, &str, SchemeSlot); 7] = [
    ("tx2", "Text color 2", SchemeSlot::Dk2),
    ("accent1", "Accent color 1", SchemeSlot::Accent1),
    ("accent2", "Accent color 2", SchemeSlot::Accent2),
    ("accent3", "Accent color 3", SchemeSlot::Accent3),
    ("accent4", "Accent color 4", SchemeSlot::Accent4),
    ("accent5", "Accent color 5", SchemeSlot::Accent5),
    ("accent6", "Accent color 6", SchemeSlot::Accent6),
];

/// (id, label), in PowerPoint's gallery order: colour modes, then dark and light variations.
pub fn presets() -> Vec<(String, String)> {
    let mut v: Vec<(String, String)> = [
        ("none", "No Recolor"),
        ("grayscale", "Grayscale"),
        ("sepia", "Sepia"),
        ("washout", "Washout"),
        ("bw25", "Black and White: 25%"),
        ("bw50", "Black and White: 50%"),
        ("bw75", "Black and White: 75%"),
    ]
    .iter()
    .map(|(a, b)| (a.to_string(), b.to_string()))
    .collect();
    for (id, label, _) in ACCENTS {
        v.push((format!("{id}Dark"), format!("{label} Dark")));
    }
    for (id, label, _) in ACCENTS {
        v.push((format!("{id}Light"), format!("{label} Light")));
    }
    v
}

/// Set `a`'s recolour to `preset`, clearing any earlier one. `false` for an unknown preset.
pub fn apply(a: &mut PictureAdjust, preset: &str) -> bool {
    let washed = a.brightness == 0.7 && a.contrast == -0.7;
    let mut n = PictureAdjust { grayscale: false, duotone: None, bilevel: None, ..a.clone() };
    if washed {
        n.brightness = 0.0;
        n.contrast = 0.0;
    }
    let rgb = |r, g, b| ColorRef::rgb(Rgba::rgb(r, g, b));
    match preset {
        "none" => {}
        "grayscale" => n.grayscale = true,
        // PowerPoint's sepia is a duotone from dark brown to light cream.
        "sepia" => n.duotone = Some((rgb(0x40, 0x2A, 0x10), rgb(0xF2, 0xE3, 0xC6))),
        // Washout is PowerPoint's +70% brightness, −70% contrast.
        "washout" => {
            n.brightness = 0.7;
            n.contrast = -0.7;
        }
        "bw25" => n.bilevel = Some(0.25),
        "bw50" => n.bilevel = Some(0.5),
        "bw75" => n.bilevel = Some(0.75),
        other => {
            let Some((slot, dark)) = ACCENTS.iter().find_map(|(id, _, slot)| {
                other.strip_prefix(id).and_then(|rest| match rest {
                    "Dark" => Some((*slot, true)),
                    "Light" => Some((*slot, false)),
                    _ => None,
                })
            }) else {
                return false;
            };
            let c = ColorRef::scheme(slot);
            n.duotone = Some(if dark { (rgb(0, 0, 0), c) } else { (c, rgb(0xFF, 0xFF, 0xFF)) });
        }
    }
    *a = n;
    true
}

fn recolor(s: &mut Session, p: &Value) -> Result<Value> {
    let preset = str_param(p, "preset").ok_or_else(|| bad("picture.recolor", "missing `preset`"))?.to_string();
    if !presets().iter().any(|(id, _)| *id == preset) {
        return Err(bad("picture.recolor", format!("unknown preset `{preset}`; see picture.recolors")));
    }
    edit_shapes(s, p, "picture.recolor", |sh| {
        if let ShapeKind::Picture { fill } = &mut sh.kind {
            apply(&mut fill.adjust, &preset);
        }
        Ok(())
    })
}

fn transparent_color(s: &mut Session, p: &Value) -> Result<Value> {
    let c = if bool_or(p, "none", false) {
        None
    } else {
        let hex = str_param(p, "color").ok_or_else(|| bad("picture.transparentColor", "give `color` (#RRGGBB) or `none: true`"))?;
        Some(Rgba::from_hex(hex.trim_start_matches('#')).ok_or_else(|| bad("picture.transparentColor", format!("bad colour `{hex}`")))?)
    };
    edit_shapes(s, p, "picture.transparentColor", |sh| {
        if let ShapeKind::Picture { fill } = &mut sh.kind {
            fill.adjust.clear_color = c;
        }
        Ok(())
    })
}
