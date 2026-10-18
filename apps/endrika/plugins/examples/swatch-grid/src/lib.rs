//! Swatch grid: a grid of colour swatches, centred in the view, each named after its colour and
//! optionally labelled. Two commands: a panel to pick one of the built-in palettes and Generate
//! (the palette picked last is remembered for next time), and a form taking hex colours.

use figcraft_plugin_sdk::{Event, Input, Result, Value, call, close_ui, entry, json, log, notify, set_setting, setting, show_ui};

/// The labels' colour unless the panel picks another.
const LABEL: &str = "#000000";

/// Swatches made in one run, at most.
const MAX_SWATCHES: usize = 256;

/// The panel's palettes: (name, colours).
const PALETTES: &[(&str, &str)] = &[
    ("Sunset", "#FF5A5F, #FFB400, #00A699, #007A87, #7B0051, #3D3D3D"),
    ("Ocean", "#0A2463, #1E5AA8, #3E92CC, #8CC7E8, #D8F1FF"),
    ("Forest", "#183A2B, #2F6B4F, #4E9F6E, #8CCB9B, #D3EED8"),
    ("Greys", "#111111, #444444, #777777, #AAAAAA, #DDDDDD"),
];

/// `#RGB` or `#RRGGBB` (the `#` optional) as `#RRGGBB`.
fn hex(s: &str) -> Option<String> {
    let h = s.trim().trim_start_matches('#');
    if !h.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    match h.len() {
        3 => Some(h.chars().flat_map(|c| [c, c]).collect::<String>()),
        6 => Some(h.to_string()),
        _ => None,
    }
    .map(|h| format!("#{}", h.to_uppercase()))
}

/// The palette's valid colours, logging the rest.
fn palette(text: &str) -> Vec<String> {
    text.split([',', ' ', ';', '\n'])
        .filter(|s| !s.trim().is_empty())
        .filter_map(|s| {
            let c = hex(s);
            if c.is_none() {
                log(&format!("skipped `{s}`: not a hex colour"));
            }
            c
        })
        .take(MAX_SWATCHES)
        .collect()
}

/// The colours of the built-in palette `name` (empty when there's none).
fn named(name: &str) -> &'static str {
    PALETTES.iter().find(|(n, _)| *n == name).map_or("", |(_, c)| c)
}

/// Swatches of `colors` in a grid centred in the view (labelled in colour `labels` when given),
/// then selected.
fn grid(colors: &[String], columns: f64, size: f64, gap: f64, labels: Option<&str>) -> Result<Value> {
    if colors.is_empty() {
        return Err("The palette has no hex colours (like #FF5A5F, #00A699)".into());
    }
    let columns = columns.clamp(1.0, 24.0) as usize;
    let (size, gap) = (size.clamp(8.0, 512.0), gap.clamp(0.0, 256.0));
    let rows = colors.len().div_ceil(columns);
    let cell_h = if labels.is_some() { size + 20.0 } else { size };
    let (w, h) = (columns as f64 * (size + gap) - gap, rows as f64 * (cell_h + gap) - gap);
    // Centred in the view, on whole pixels.
    let view = call("view.get", &json!({}))?;
    let (x0, y0) = ((view["cx"].as_f64().unwrap_or(0.0) - w / 2.0).round(), (view["cy"].as_f64().unwrap_or(0.0) - h / 2.0).round());
    let mut made = Vec::new();
    for (i, color) in colors.iter().enumerate() {
        let (x, y) = (x0 + (i % columns) as f64 * (size + gap), y0 + (i / columns) as f64 * (cell_h + gap));
        let r = call("shape.rectangle", &json!({"x": x, "y": y, "width": size, "height": size, "props": {"name": color, "fill": color}}))?;
        made.extend(r["id"].as_u64());
        if let Some(label) = labels {
            let t = call("text.create", &json!({"text": color, "x": x, "y": y + size + 4.0, "props": {"fontSize": 11, "fill": label}}))?;
            made.extend(t["id"].as_u64());
        }
    }
    call("select.set", &json!({"ids": made}))?;
    notify(&format!("Made {} swatches", colors.len()));
    Ok(json!({"swatches": colors.len(), "ids": made}))
}

/// Show the panel with palette `name` picked and the other fields as in `values` (null: their
/// defaults).
fn panel(name: &str, values: &Value) -> Result<Value> {
    let names: Vec<&str> = PALETTES.iter().map(|(n, _)| *n).collect();
    show_ui(&json!({"title": "Swatch grid", "controls": [
        {"type": "list", "name": "palette", "label": "Palette", "options": names, "value": name},
        {"type": "note", "label": named(name)},
        {"type": "number", "name": "columns", "label": "Columns", "min": 1, "max": 24, "value": values["columns"].as_f64().unwrap_or(3.0)},
        {"type": "boolean", "name": "labels", "label": "Label each swatch", "value": values["labels"].as_bool().unwrap_or(true)},
        {"type": "color", "name": "labelColor", "label": "Label colour", "value": values["labelColor"].as_str().unwrap_or(LABEL)},
        {"type": "button", "name": "generate", "label": "Generate"},
        {"type": "button", "name": "close", "label": "Close"}
    ]}))?;
    Ok(Value::Null)
}

/// A button or a changed value in the panel.
fn on_event(e: Event) -> Result<Value> {
    let name = e.values["palette"].as_str().unwrap_or("Sunset");
    match e.control.as_str() {
        // Show the picked palette's colours, and remember it for next time.
        "palette" => {
            set_setting("palette", &json!(name))?;
            panel(name, &e.values)
        }
        "generate" => {
            let labels = e.values["labels"].as_bool().unwrap_or(true).then(|| e.values["labelColor"].as_str().unwrap_or(LABEL));
            grid(&palette(named(name)), e.values["columns"].as_f64().unwrap_or(3.0), 64.0, 16.0, labels)
        }
        "close" => {
            close_ui();
            Ok(Value::Null)
        }
        _ => Ok(Value::Null),
    }
}

fn run(input: Input) -> Result<Value> {
    if let Some(e) = input.event {
        return on_event(e);
    }
    if input.command == "panel" {
        let last = setting("palette").and_then(|v| v.as_str().map(str::to_string)).filter(|n| !named(n).is_empty());
        return panel(last.as_deref().unwrap_or("Sunset"), &Value::Null);
    }
    let colors = palette(input.str("palette").unwrap_or_default());
    grid(
        &colors,
        input.f64("columns").unwrap_or(3.0),
        input.f64("size").unwrap_or(64.0),
        input.f64("gap").unwrap_or(16.0),
        input.bool("labels").unwrap_or(true).then_some(LABEL),
    )
}

entry!(run);

#[cfg(test)]
mod tests {
    use super::{PALETTES, hex, palette};

    #[test]
    fn palettes_parse() {
        assert_eq!(hex("#f0a"), Some("#FF00AA".into()));
        assert_eq!(hex(" 00a699 "), Some("#00A699".into()));
        assert_eq!(hex("#12345"), None);
        assert_eq!(hex("red"), None);
        assert_eq!(palette("#FF5A5F, nope;#0f0\n00A699"), vec!["#FF5A5F", "#00FF00", "#00A699"]);
        for (name, colors) in PALETTES {
            assert_eq!(palette(colors).len(), colors.split(',').count(), "{name}");
        }
    }
}
