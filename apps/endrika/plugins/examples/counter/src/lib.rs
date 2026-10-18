//! Counter: a widget on the canvas with − and + buttons around a count. Each click calls the
//! plugin back with the widget's state; it hands back the new count and the redrawn widget.

use figcraft_plugin_sdk::{Input, Result, Value, entry, json, render_widget};

/// The count stays within ± this.
const LIMIT: i64 = 999_999;

/// The count after clicking `target` (none: the widget was just inserted).
fn next(count: i64, target: Option<&str>) -> i64 {
    let n = match target {
        Some("increment") => count.saturating_add(1),
        Some("decrement") => count.saturating_sub(1),
        _ => count,
    };
    n.clamp(-LIMIT, LIMIT)
}

/// A square button that calls back with `target`.
fn button(target: &str, name: &str, label: &str) -> Value {
    json!({"type": "frame", "name": name, "target": target, "layout": "horizontal", "justify": "center", "width": 32, "height": 32, "radius": 8, "fill": "#F0F0F0",
           "children": [{"type": "text", "text": label, "size": 16, "bold": true, "color": "#1E1E1E"}]})
}

/// The widget showing `count`.
fn view(count: i64) -> Value {
    json!({"type": "frame", "name": "Counter", "layout": "horizontal", "gap": 12, "padding": 12, "radius": 12,
           "fill": "#FFFFFF", "stroke": "#D9D9D9", "children": [
        button("decrement", "Decrease", "−"),
        {"type": "text", "name": "Count", "text": count.to_string(), "size": 24, "bold": true, "color": "#1E1E1E"},
        button("increment", "Increase", "+"),
    ]})
}

fn run(input: Input) -> Result<Value> {
    let w = input.widget.ok_or("Insert the counter from Plugins ▸ Widgets")?;
    let count = next(w.state.get("count").and_then(Value::as_i64).unwrap_or(0), w.target.as_deref());
    render_widget(&json!({"state": {"count": count}, "root": view(count)}))?;
    Ok(json!({"count": count}))
}

entry!(run);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_within_its_limit() {
        assert_eq!(next(0, None), 0);
        assert_eq!(next(4, Some("increment")), 5);
        assert_eq!(next(4, Some("decrement")), 3);
        assert_eq!(next(LIMIT, Some("increment")), LIMIT);
        assert_eq!(next(i64::MIN, Some("decrement")), -LIMIT);
        assert_eq!(next(2, Some("other")), 2);
        assert_eq!(view(3)["children"][1]["text"], "3");
        assert!(run(Input::default()).is_err(), "it only runs as a widget");
    }
}
