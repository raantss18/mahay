//! Tidy names: cleans up the selected layers' names (extra spaces, the "copy" left by duplicating)
//! and optionally their letter case, inside the selection too.

use figcraft_plugin_sdk::{Input, Result, Value, call, entry, json, notify};

/// Layers looked at in one run, at most.
const MAX_LAYERS: usize = 10_000;

/// `name` with single spaces, no trailing "copy" / "copy 2", in `case`.
fn tidy(name: &str, case: &str) -> String {
    let mut words: Vec<&str> = name.split_whitespace().collect();
    loop {
        match words.as_slice() {
            [_, .., "copy"] => {
                words.pop();
            }
            [_, .., "copy", n] if n.chars().all(|c| c.is_ascii_digit()) => {
                words.truncate(words.len() - 2);
            }
            _ => break,
        }
    }
    let joined = words.join(" ");
    match case {
        "Title Case" => words.iter().map(|w| title(w)).collect::<Vec<_>>().join(" "),
        "lower case" => joined.to_lowercase(),
        "UPPER CASE" => joined.to_uppercase(),
        _ => joined,
    }
}

fn title(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars.flat_map(char::to_lowercase)).collect(),
        None => String::new(),
    }
}

fn run(input: Input) -> Result<Value> {
    if input.selection.is_empty() {
        return Err("Select the layers to tidy first".into());
    }
    let case = if input.command == "quick" { "Keep" } else { input.str("case").unwrap_or("Keep") };
    let inside = input.command == "quick" || input.bool("children").unwrap_or(true);
    let mut stack = input.selection.clone();
    let (mut seen, mut renamed) = (0, 0);
    while let Some(id) = stack.pop() {
        seen += 1;
        if seen > MAX_LAYERS {
            break;
        }
        let node = call("node.get", &json!({"id": id}))?;
        if inside && let Some(children) = node["children"].as_array() {
            stack.extend(children.iter().filter_map(|c| c["id"].as_u64()));
        }
        let name = node["name"].as_str().unwrap_or_default();
        let clean = tidy(name, case);
        if !clean.is_empty() && clean != name {
            call("object.rename", &json!({"ids": [id], "name": clean}))?;
            renamed += 1;
        }
    }
    notify(&match renamed {
        0 => "The names were already tidy".to_string(),
        1 => "Renamed 1 layer".to_string(),
        n => format!("Renamed {n} layers"),
    });
    Ok(json!({"renamed": renamed}))
}

entry!(run);

#[cfg(test)]
mod tests {
    use super::tidy;

    #[test]
    fn names_are_tidied() {
        assert_eq!(tidy("  Card   title copy 2 ", "Keep"), "Card title");
        assert_eq!(tidy("Button copy copy", "Keep"), "Button");
        assert_eq!(tidy("copy", "Keep"), "copy", "a name that is only `copy` stays");
        assert_eq!(tidy("primary BUTTON", "Title Case"), "Primary Button");
        assert_eq!(tidy("Hero Image", "lower case"), "hero image");
        assert_eq!(tidy("nav", "UPPER CASE"), "NAV");
    }
}
