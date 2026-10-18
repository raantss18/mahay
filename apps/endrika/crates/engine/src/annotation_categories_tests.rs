//! Annotation categories: defaults, user-defined ones with a colour, filtering, files.

use crate::components::tests::{ex, idv, node};
use crate::{EngineError, Session};
use figcraft_doc::*;
use serde_json::{Value, json};

fn rect(s: &mut Session) -> NodeId {
    idv(&ex(s, "shape.rectangle", json!({"x": 10, "y": 20, "width": 120, "height": 40, "props": {"name": "Button"}})))
}

fn names(v: &Value) -> Vec<String> {
    v["categories"].as_array().map(|a| a.iter().filter_map(|c| c["name"].as_str().map(String::from)).collect()).unwrap_or_default()
}

#[test]
fn defaults_until_the_file_changes_them() {
    let mut s = Session::new();
    let v = ex(&mut s, "annotation.categories", json!({}));
    assert_eq!(names(&v), vec!["Development", "Interaction", "Accessibility", "Content"]);
    assert_eq!(v["categories"][0], json!({"id": "development", "name": "Development", "color": "#2F6FEB", "count": 0}));
    assert!(s.doc().unwrap().annotation_categories.is_none(), "defaults aren't stored");
    let json = serde_json::to_value(s.doc().unwrap()).unwrap();
    assert!(json.get("annotation_categories").is_none() && json.get("annotationCategories").is_none());
}

#[test]
fn categories_are_added_renamed_recoloured_and_deleted_with_undo() {
    let mut s = Session::new();
    let r = rect(&mut s);
    let id = ex(&mut s, "annotation.addCategory", json!({"name": "  Motion  "}))["id"].as_str().unwrap().to_string();
    assert_eq!(id, "custom-1");
    let list = crate::annotations::categories(s.doc().unwrap()).to_vec();
    let motion = list.last().unwrap();
    assert_eq!(motion.name, "Motion");
    assert!(AnnotationCategory::COLORS[4..].iter().any(|c| c.to_hex() == motion.color.to_hex()), "the first colour no default uses");
    ex(&mut s, "annotation.editCategory", json!({"id": "motion", "name": "Animation", "color": "#123456"}));
    let cat = crate::annotations::categories(s.doc().unwrap()).last().cloned().unwrap();
    assert_eq!((cat.id.as_str(), cat.name.as_str(), cat.color.to_hex().as_str()), ("custom-1", "Animation", "123456"));
    ex(&mut s, "edit.undo", json!({}));
    assert_eq!(crate::annotations::categories(s.doc().unwrap()).last().unwrap().name, "Motion", "one undo step");
    // Annotations take a category by id or name; null clears it.
    ex(&mut s, "annotation.add", json!({"id": r, "label": "Fade in", "category": "Motion"}));
    ex(&mut s, "annotation.add", json!({"id": r, "label": "Label it", "category": "accessibility"}));
    ex(&mut s, "annotation.add", json!({"id": r, "label": "Plain"}));
    assert_eq!(node(&s, r).annotations[0].category.as_deref(), Some("custom-1"));
    ex(&mut s, "annotation.edit", json!({"id": r, "index": 2, "category": "content"}));
    assert_eq!(node(&s, r).annotations[2].category.as_deref(), Some("content"));
    ex(&mut s, "annotation.edit", json!({"id": r, "index": 2, "category": null}));
    assert_eq!(node(&s, r).annotations[2].category, None);
    let list = ex(&mut s, "annotation.list", json!({}));
    assert_eq!(list["annotations"][0]["category"], json!({"id": "custom-1", "name": "Motion", "color": motion.color.to_css_hex()}));
    assert!(list["annotations"][2].get("category").is_none());
    // Filtering the list by category.
    let only = |s: &mut Session, c: &str| -> Vec<String> {
        ex(s, "annotation.list", json!({"category": c}))["annotations"]
            .as_array()
            .map(|a| a.iter().filter_map(|x| x["label"].as_str().map(String::from)).collect())
            .unwrap_or_default()
    };
    assert_eq!(only(&mut s, "Accessibility"), vec!["Label it"]);
    assert_eq!(only(&mut s, "none"), vec!["Plain"]);
    assert!(s.execute("annotation.list", &json!({"category": "Nope"})).is_err());
    assert_eq!(ex(&mut s, "annotation.categories", json!({}))["categories"][2]["count"], json!(1));
    // Deleting a category keeps its annotations, uncategorised; undo brings both back.
    let v = ex(&mut s, "annotation.removeCategory", json!({"id": "Motion"}));
    assert_eq!(v, json!({"id": "custom-1", "uncategorized": 1}));
    assert_eq!(node(&s, r).annotations[0].category, None);
    assert_eq!(node(&s, r).annotations[0].label, "Fade in");
    ex(&mut s, "edit.undo", json!({}));
    assert_eq!(node(&s, r).annotations[0].category.as_deref(), Some("custom-1"));
    // A default can be deleted too: the file then stores its own list.
    ex(&mut s, "annotation.removeCategory", json!({"id": "interaction"}));
    assert_eq!(names(&ex(&mut s, "annotation.categories", json!({}))), vec!["Development", "Accessibility", "Content", "Motion"]);
    // Refused: duplicates, empty names, unknown categories, bad colours; nothing changes.
    let before = s.doc().unwrap().clone();
    for (cmd, p) in [
        ("annotation.addCategory", json!({"name": "development"})),
        ("annotation.addCategory", json!({"name": "   "})),
        ("annotation.addCategory", json!({})),
        ("annotation.addCategory", json!({"name": "X", "color": "not a colour"})),
        ("annotation.editCategory", json!({"id": "nope", "name": "Y"})),
        ("annotation.editCategory", json!({"id": "content"})),
        ("annotation.editCategory", json!({"id": "content", "name": "Motion"})),
        ("annotation.removeCategory", json!({"id": "nope"})),
        ("annotation.add", json!({"id": r, "label": "x", "category": "nope"})),
        ("annotation.add", json!({"id": r, "label": "x", "category": 5})),
    ] {
        assert!(s.execute(cmd, &p).is_err(), "{cmd} {p}");
    }
    assert_eq!(*s.doc().unwrap(), before);
    // Renaming to its own name in another case is fine.
    ex(&mut s, "annotation.editCategory", json!({"id": "content", "name": "CONTENT"}));
}

#[test]
fn categories_are_capped() {
    let mut s = Session::new();
    for k in 4..crate::annotations::MAX_CATEGORIES {
        ex(&mut s, "annotation.addCategory", json!({"name": format!("C{k}")}));
    }
    assert!(s.execute("annotation.addCategory", &json!({"name": "one too many"})).is_err());
    let long = "n".repeat(500);
    let mut s = Session::new();
    let id = ex(&mut s, "annotation.addCategory", json!({"name": long}))["id"].as_str().unwrap().to_string();
    let c = crate::annotations::categories(s.doc().unwrap()).iter().find(|c| c.id == id).cloned().unwrap();
    assert_eq!(c.name.chars().count(), crate::annotations::MAX_CATEGORY_NAME);
}

#[test]
fn categories_are_saved_and_old_files_open() {
    let mut s = Session::new();
    let r = rect(&mut s);
    ex(&mut s, "annotation.addCategory", json!({"name": "Motion", "color": "#FF00AA"}));
    ex(&mut s, "annotation.add", json!({"id": r, "label": "Fade", "category": "Motion"}));
    let bytes = figcraft_format::save_bytes(s.doc().unwrap(), None).unwrap();
    let (back, issues) = figcraft_format::load_bytes(&bytes).unwrap();
    assert!(issues.is_empty());
    assert_eq!(back.annotation_categories, s.doc().unwrap().annotation_categories);
    assert_eq!(back.node(r).unwrap().annotations[0].category.as_deref(), Some("custom-1"));
    // Annotations from older files have no category; ids a file doesn't define read as none.
    let a: Annotation = serde_json::from_value(json!({"label": "Old"})).unwrap();
    assert_eq!(a.category, None);
    assert!(serde_json::to_value(&a).unwrap().get("category").is_none());
    let stray = Annotation { label: "x".into(), properties: vec![], category: Some("gone".into()) };
    assert!(crate::annotations::category_of(&back, &stray).is_none());
}

#[test]
fn category_commands_survive_junk() {
    let junk = [
        json!({}),
        json!(null),
        json!({"id": [1], "name": {"a": 1}, "color": 7}),
        json!({"id": "development", "name": "\u{0}\u{2028}".repeat(4000), "color": "#GGGGGG"}),
        json!({"id": "", "name": "", "color": null, "category": [null]}),
        json!({"index": 0, "category": {"x": 1}, "label": "n"}),
    ];
    for cmd in [
        "annotation.categories",
        "annotation.addCategory",
        "annotation.editCategory",
        "annotation.removeCategory",
        "annotation.add",
        "annotation.edit",
        "annotation.list",
    ] {
        for j in &junk {
            let mut s = Session::new();
            let r = rect(&mut s);
            ex(&mut s, "annotation.add", json!({"id": r, "label": "n", "category": "content"}));
            ex(&mut s, "select.set", json!({"ids": [r]}));
            if let Err(EngineError::Internal { cmd, msg }) = s.execute(cmd, j) {
                panic!("{cmd} panicked on {j}: {msg}");
            }
        }
    }
}
