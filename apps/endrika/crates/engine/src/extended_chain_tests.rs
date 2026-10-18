//! Extended collections' tail: variables a library adds to a collection this file extends are an
//! update of their own, and a library collection that is itself an extended collection can be
//! extended again, values flowing down the chain as each file accepts updates.

use figcraft_doc::*;
use serde_json::{Value, json};

use crate::Session;
use crate::params::id_of;

fn ex(s: &mut Session, cmd: &str, p: Value) -> Value {
    s.execute(cmd, &p).unwrap_or_else(|e| panic!("{cmd} {p}: {e}"))
}

fn var<'a>(s: &'a Session, collection: &str, name: &str) -> &'a Variable {
    let doc = s.doc().unwrap();
    doc.variables.variables.iter().find(|v| v.collection == collection && v.name == name).unwrap()
}

/// `name`'s value in mode `mode` (by name) of `collection`, as text.
fn value(s: &Session, collection: &str, name: &str, mode: &str) -> String {
    let doc = s.doc().unwrap();
    let m = doc.variables.collection(collection).and_then(|c| c.find_mode(mode)).unwrap().id.clone();
    match var(s, collection, name).values.get(&m) {
        Some(VariableValue::Color(c)) => c.to_hex(),
        Some(VariableValue::Number(n)) => n.to_string(),
        other => format!("{other:?}"),
    }
}

fn healthy(s: &Session) {
    let doc = s.doc().unwrap();
    assert!(doc.validate().is_empty(), "{:?}", doc.validate());
}

fn dir(name: &str) -> std::path::PathBuf {
    let d = std::env::temp_dir().join(format!("figcraft-extchain-{}-{name}", std::process::id()));
    std::fs::create_dir_all(&d).unwrap();
    d
}

/// A library file with "Theme" (Light/Dark: `primary`, `radius`) and "Other" (`size`), published
/// and saved at `path`; returns (session, library id).
fn core(path: &str) -> (Session, String) {
    let mut lib = Session::new();
    ex(&mut lib, "variable.createCollection", json!({"name": "Theme", "modes": ["Light", "Dark"]}));
    ex(
        &mut lib,
        "variable.create",
        json!({"collection": "Theme", "name": "primary", "type": "color", "values": {"Light": "#0000ff", "Dark": "#ffffff"}}),
    );
    ex(&mut lib, "variable.create", json!({"collection": "Theme", "name": "radius", "type": "number", "value": 4}));
    ex(&mut lib, "variable.createCollection", json!({"name": "Other"}));
    ex(&mut lib, "variable.create", json!({"collection": "Other", "name": "size", "type": "number", "value": 1}));
    let id = ex(&mut lib, "library.publish", json!({}))["id"].as_str().unwrap().to_string();
    ex(&mut lib, "file.saveAs", json!({"path": path}));
    (lib, id)
}

fn republish(lib: &mut Session) {
    ex(lib, "library.publish", json!({}));
    ex(lib, "file.save", json!({}));
}

#[test]
fn variables_new_to_an_extended_collection_are_an_update_of_their_own() {
    let d = dir("new");
    let path = d.join("core.figcraft").to_string_lossy().into_owned();
    let (mut lib, lid) = core(&path);
    let mut s = Session::new();
    ex(&mut s, "library.add", json!({"path": path}));
    let ext = ex(&mut s, "library.extendCollection", json!({"library": lid, "collection": "Theme"}))["id"].as_str().unwrap().to_string();
    assert_eq!(ex(&mut s, "library.updates", json!({}))["count"], 0);

    // The library only adds a variable to the extended collection (and one to a collection this
    // file doesn't use): that alone is an update, listed by name.
    ex(&mut lib, "variable.create", json!({"collection": "Theme", "name": "gap", "type": "number", "value": 8}));
    ex(&mut lib, "variable.create", json!({"collection": "Other", "name": "unused", "type": "number", "value": 2}));
    republish(&mut lib);
    let u = ex(&mut s, "library.updates", json!({}));
    assert_eq!(u["count"], 1, "{u}");
    let row = &u["libraries"][0]["variables"][0];
    assert_eq!((&row["name"], &row["id"]), (&json!("gap"), &Value::Null), "no copy yet: no id");
    // Accepting brings it in, inheriting; then nothing is pending.
    let done = ex(&mut s, "library.acceptUpdates", json!({}));
    assert_eq!(done["variables"], 1);
    let gap = var(&s, &ext, "gap");
    assert!(gap.extends.is_some() && gap.overrides.is_empty());
    assert_eq!(value(&s, &ext, "gap", "Dark"), "8");
    assert_eq!(ex(&mut s, "library.updates", json!({}))["count"], 0);
    // Docs silent: the variable added to a collection nobody here uses stays out.
    assert!(!s.doc().unwrap().variables.variables.iter().any(|v| v.name == "unused"));
    healthy(&s);
    std::fs::remove_dir_all(&d).ok();
}

#[test]
fn an_extended_collection_can_be_extended_again() {
    let d = dir("chain");
    let core_path = d.join("core.figcraft").to_string_lossy().into_owned();
    let brand_path = d.join("brand.figcraft").to_string_lossy().into_owned();
    let (mut core_lib, core_id) = core(&core_path);

    // A brand library extends the core theme, overrides a value, adds a variable and publishes.
    let mut brand = Session::new();
    ex(&mut brand, "library.add", json!({"path": core_path}));
    let bext = ex(&mut brand, "library.extendCollection", json!({"library": core_id, "collection": "Theme", "name": "Brand"}))["id"]
        .as_str()
        .unwrap()
        .to_string();
    let bprimary = var(&brand, &bext, "primary").id.clone();
    ex(&mut brand, "variable.set", json!({"variable": bprimary, "mode": "Light", "value": "#ff0000"}));
    ex(&mut brand, "variable.create", json!({"collection": bext, "name": "logo", "type": "number", "value": 3}));
    let brand_id = ex(&mut brand, "library.publish", json!({}))["id"].as_str().unwrap().to_string();
    ex(&mut brand, "file.saveAs", json!({"path": brand_path}));

    // A product file extends the brand's extended collection: the brand's values, modes and
    // variables come along, and it overrides one of its own.
    let mut s = Session::new();
    ex(&mut s, "library.add", json!({"path": brand_path}));
    let pext = ex(&mut s, "library.extendCollection", json!({"library": brand_id, "collection": "Brand"}))["id"].as_str().unwrap().to_string();
    let modes: Vec<String> = s.doc().unwrap().variables.collection(&pext).unwrap().modes.iter().map(|m| m.name.clone()).collect();
    assert_eq!(modes, ["Light", "Dark"]);
    assert_eq!(value(&s, &pext, "primary", "Light"), "FF0000", "the brand's override is inherited");
    assert_eq!(value(&s, &pext, "primary", "Dark"), "FFFFFF", "the core value is inherited");
    assert_eq!(value(&s, &pext, "logo", "Light"), "3", "the brand's own variable is inherited");
    let radius = var(&s, &pext, "radius").id.clone();
    ex(&mut s, "variable.set", json!({"variable": radius, "mode": "Dark", "value": 12}));
    let r = id_of(&ex(&mut s, "shape.rectangle", json!({"x": 0, "y": 0, "width": 10, "height": 10}))["id"]).unwrap();
    let primary = var(&s, &pext, "primary").id.clone();
    ex(&mut s, "variable.bind", json!({"ids": [r], "field": "fill", "variable": primary}));

    // The core changes: the brand accepts and republishes, then the product accepts. Values
    // follow down the chain; each level's overrides stay.
    ex(&mut core_lib, "variable.set", json!({"variable": "primary", "mode": "Dark", "value": "#222222"}));
    ex(&mut core_lib, "variable.set", json!({"variable": "radius", "mode": "Light", "value": 6}));
    republish(&mut core_lib);
    ex(&mut brand, "library.acceptUpdates", json!({}));
    assert_eq!(value(&brand, &bext, "primary", "Dark"), "222222");
    assert_eq!(value(&brand, &bext, "primary", "Light"), "FF0000");
    republish(&mut brand);
    assert!(ex(&mut s, "library.updates", json!({}))["count"].as_u64().unwrap() > 0);
    ex(&mut s, "library.acceptUpdates", json!({}));
    assert_eq!(value(&s, &pext, "primary", "Dark"), "222222");
    assert_eq!(value(&s, &pext, "primary", "Light"), "FF0000");
    assert_eq!(value(&s, &pext, "radius", "Light"), "6");
    assert_eq!(value(&s, &pext, "radius", "Dark"), "12", "the product's override stays");
    healthy(&s);

    // Saved and reopened, the chain's end is the same.
    let user = d.join("product.figcraft").to_string_lossy().into_owned();
    ex(&mut s, "file.saveAs", json!({"path": user}));
    let mut t = Session::empty();
    ex(&mut t, "file.open", json!({"path": user}));
    assert_eq!(t.doc().unwrap().variables, s.doc().unwrap().variables);
    std::fs::remove_dir_all(&d).ok();
}
