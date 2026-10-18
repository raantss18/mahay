//! `.fig` export through `file.export {format: "fig"}`: files written by the command open again
//! with the same layers, linked instances and auto layout. The export's own round-trip tests
//! (generated documents, fuzzing) are in `figcraft_format::fig`; the local check here runs only
//! when `FIGCRAFT_FIG_SAMPLES` names a folder of `.fig` files.

use base64::Engine as _;
use figcraft_doc::{Document, NodeKind};
use serde_json::{Value, json};

use crate::{EngineError, Session};

fn id(v: &Value) -> u64 {
    v["id"].as_u64().unwrap()
}

fn temp(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("figcraft-fig-export-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    dir.join(name)
}

/// The layers depth-first: kind, name, position, size, text and fill count, and whether an
/// instance's layers are linked to its main's.
fn outline(doc: &Document) -> Vec<String> {
    doc.pages
        .iter()
        .flat_map(|p| doc.subtree(*p))
        .filter_map(|i| doc.node(i))
        .map(|n| {
            let t = n.transform.translation();
            let text = n.kind.text().map(|t| t.characters.clone()).unwrap_or_default();
            let inside = doc.ancestors(n.id).iter().any(|a| matches!(doc.node(*a).map(|x| &x.kind), Some(NodeKind::Instance { .. })));
            format!(
                "{} {:?} ({:.1}, {:.1}) {:.1}×{:.1} {text:?} fills {} linked {} layout {:?}",
                n.kind.type_name(),
                n.name,
                t.x,
                t.y,
                n.width,
                n.height,
                n.fills.len(),
                !inside || n.source.is_some(),
                n.layout.as_ref().map(|l| (l.mode, l.gap, l.padding))
            )
        })
        .collect()
}

/// A small file: a frame with auto layout holding text and a rectangle, a component with a text
/// property and an instance of it setting the property, and a click from the frame to the
/// instance.
fn build(s: &mut Session) {
    let frame = id(&s.execute("frame.create", &json!({"x": 0, "y": 0, "width": 200, "height": 100})).unwrap());
    s.execute("text.create", &json!({"text": "Hello 😀", "x": 10, "y": 10, "parent": frame})).unwrap();
    s.execute("shape.rectangle", &json!({"x": 10, "y": 40, "width": 50, "height": 20, "parent": frame})).unwrap();
    s.execute("layout.add", &json!({"ids": [frame], "direction": "vertical"})).unwrap();
    let card = id(&s.execute("frame.create", &json!({"x": 300, "y": 0, "width": 120, "height": 60})).unwrap());
    let label = id(&s.execute("text.create", &json!({"text": "Label", "x": 10, "y": 10, "parent": card})).unwrap());
    let comp = id(&s.execute("component.create", &json!({"ids": [card]})).unwrap());
    s.execute("component.addProperty", &json!({"id": comp, "name": "Title", "type": "text", "default": "Label", "layer": label})).unwrap();
    let inst = id(&s.execute("instance.create", &json!({"component": comp, "x": 300, "y": 200})).unwrap());
    s.execute("instance.setProperties", &json!({"ids": [inst], "properties": {"Title": "Changed"}})).unwrap();
    s.execute("proto.interaction.add", &json!({"id": frame, "dest": inst})).unwrap();
}

#[test]
fn exported_fig_files_open_with_the_same_layers() {
    let mut s = Session::new();
    build(&mut s);
    let path = temp("roundtrip.fig");
    let r = s.execute("file.export", &json!({"format": "fig", "path": path.to_string_lossy()})).unwrap();
    assert_eq!(r["format"], "fig");
    assert!(r["warnings"].is_array());
    let before = outline(s.doc().unwrap());
    let r = s.execute("file.open", &json!({"path": path.to_string_lossy()})).unwrap();
    assert!(r["warnings"].is_array());
    let doc = s.doc().unwrap();
    assert_eq!(outline(doc), before);
    let inst = doc.nodes().find(|n| matches!(n.kind, NodeKind::Instance { .. })).unwrap();
    assert!(inst.children.iter().all(|c| doc.node(*c).is_some_and(|c| c.source.is_some())));
    assert_eq!(doc.nodes().filter(|n| !n.interactions.is_empty()).count(), 1);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn fig_export_without_a_path_returns_the_file() {
    let mut s = Session::new();
    build(&mut s);
    let r = s.execute("file.export", &json!({"format": "FIG"})).unwrap();
    let bytes = base64::engine::general_purpose::STANDARD.decode(r["data"].as_str().unwrap()).unwrap();
    assert!(figcraft_format::fig::is_fig(&bytes));
    let (doc, _) = crate::cmd::file::load_bytes(&bytes, "x.fig").unwrap();
    assert_eq!(outline(&doc), outline(s.doc().unwrap()));
    // A path ending in .fig picks the format.
    let path = temp("by-extension.fig");
    let r = s.execute("file.export", &json!({"path": path.to_string_lossy()})).unwrap();
    assert_eq!(r["format"], "fig");
    let _ = std::fs::remove_file(&path);
}

#[test]
fn fig_export_survives_junk_params() {
    let junk = [
        json!({"format": "fig", "path": [5]}),
        json!({"format": "fig", "path": ""}),
        json!({"format": "fig", "path": "\u{0}/x.fig"}),
        json!({"format": "fig", "ids": ["x"], "scale": "NaN", "size": {"by": 1}}),
        json!({"format": ["fig"]}),
        json!({"path": "nowhere/../\u{0}.fig"}),
    ];
    for j in &junk {
        let mut s = Session::new();
        build(&mut s);
        if let Err(EngineError::Internal { cmd, msg }) = s.execute("file.export", j) {
            panic!("{cmd} panicked on {j}: {msg}");
        }
    }
    // A failed write leaves no temporary file behind.
    assert!(!std::path::Path::new(".tmp-figcraft").exists());
    // An empty document exports too.
    let mut s = Session::new();
    assert!(s.execute("file.export", &json!({"format": "fig"})).is_ok());
}

/// Local only (`FIGCRAFT_FIG_SAMPLES`): each sample opened, exported and opened again has the
/// same layers, with its instances linked.
#[test]
fn local_samples_export_and_open_again() {
    let Some(dir) = std::env::var_os("FIGCRAFT_FIG_SAMPLES") else { return };
    let Ok(dir) = std::fs::read_dir(dir) else { return };
    for p in dir.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "fig")) {
        let mut s = Session::new();
        s.execute("file.open", &json!({"path": p.to_string_lossy()})).unwrap();
        let before = outline(s.doc().unwrap());
        let out = temp(&format!("again-{}", p.file_name().unwrap().to_string_lossy()));
        s.execute("file.export", &json!({"format": "fig", "path": out.to_string_lossy()})).unwrap();
        s.execute("file.open", &json!({"path": out.to_string_lossy()})).unwrap();
        assert_eq!(outline(s.doc().unwrap()), before, "{}", p.display());
        let _ = std::fs::remove_file(&out);
    }
}
