//! Widgets: layers on the canvas that a plugin draws (plugin API 3). A widget is a frame owned by
//! its plugin: the plugin describes it ([`WidgetRender`]: frames with auto layout, text, shapes and
//! images) and FigCraft builds the layers inside it, locked, so the widget moves as one layer.
//! Each widget keeps its plugin's id and a small state (a JSON object) in its own layer data, so it
//! is saved, copied, undone, exported and synced in a live session like any layer. Clicking a part
//! of it the plugin named as a target calls the plugin back with that state, in a fresh sandbox
//! (deterministic: the state is all it remembers), as one undo step; the plugin hands back the new
//! state and layers. A callback can only rebuild its own widget; other layers change only through
//! the commands its manifest permits. See `docs/plugins.md`.

use figcraft_color::{ImageScaleMode, Paint};
use figcraft_doc::{Document, Node, NodeId, NodeKind, TextData};
use figcraft_geom::{Affine, Point};
use figcraft_plugin::widget::{MAX_IMAGE_BYTES, MAX_STATE_BYTES};
use figcraft_plugin::{Instance, Manifest, RunOutput, WidgetKind, WidgetNode, WidgetRender};
use serde_json::{Map, Value, json};

use crate::cmd::CommandSpec;
use crate::params::{id, num, string};
use crate::plugins::{InstalledPlugin, call_plugin, installed, limits, module_of, plugin_error};
use crate::{Result, Session, bad, spec};

/// Layer data FigCraft keeps for widgets starts with this; plugins can't write it.
pub const DATA_PREFIX: &str = "figcraft.widget";
/// On a widget's own layer: its plugin's id, and its state (JSON).
const PLUGIN: &str = "figcraft.widget";
const STATE: &str = "figcraft.widget.state";
/// On a layer inside a widget: the name its plugin gets when it's clicked.
const TARGET: &str = "figcraft.widget.target";
/// Images in a widget are at most this many pixels on a side.
const MAX_IMAGE_SIDE: u32 = 4096;

/// Whether `n` is a widget's own layer.
pub fn is_widget(n: &Node) -> bool {
    matches!(n.kind, NodeKind::Frame) && n.plugin_data.contains_key(PLUGIN)
}

/// The widget layer `id` is, or is inside (the innermost).
pub fn widget_of(doc: &Document, id: NodeId) -> Option<NodeId> {
    std::iter::once(id).chain(doc.ancestors(id)).find(|a| doc.node(*a).is_some_and(is_widget))
}

/// The clickable part of a widget at page point `p`, given the layer a click there hits: the widget
/// and its target. The layers inside a widget are locked, so the hit stops at the widget itself;
/// the front-most target whose box holds the point wins.
pub fn target_at(doc: &Document, hit: NodeId, p: Point) -> Option<(NodeId, String)> {
    let w = widget_of(doc, hit)?;
    let target = doc
        .subtree(w)
        .into_iter()
        .filter(|i| doc.world_bounds(*i).is_some_and(|b| b.contains(p)))
        .filter_map(|i| doc.node(i).filter(|n| n.visible)?.plugin_data.get(TARGET).cloned())
        .last()?;
    Some((w, target))
}

/// A widget's state (an empty object when it has none or it's unusable).
fn state_of(doc: &Document, w: NodeId) -> Map<String, Value> {
    doc.node(w)
        .and_then(|n| n.plugin_data.get(STATE))
        .filter(|s| s.len() <= MAX_STATE_BYTES)
        .and_then(|s| serde_json::from_str::<Value>(s).ok())
        .and_then(|v| match v {
            Value::Object(m) => Some(m),
            _ => None,
        })
        .unwrap_or_default()
}

/// The targets inside widget `w`, each once, in layer order.
fn targets(doc: &Document, w: NodeId) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for t in doc.subtree(w).into_iter().filter_map(|i| doc.node(i)?.plugin_data.get(TARGET).cloned()) {
        if !out.contains(&t) {
            out.push(t);
        }
    }
    out
}

/// Installed widget plugin `plugin`, its manifest checked again (the preferences may have been
/// edited by hand).
fn widget_plugin(s: &Session, cmd: &str, plugin: &str) -> Result<InstalledPlugin> {
    let mut p = installed(s, cmd, plugin)?.clone();
    p.manifest = Manifest::from_value(&json!(p.manifest)).map_err(|e| bad(cmd, e))?;
    if !p.manifest.widget {
        return Err(bad(cmd, format!("{} doesn't draw a widget", p.manifest.name)));
    }
    Ok(p)
}

/// Call widget `w`'s plugin `p` back (`target`: what was clicked; none when it's inserted) and
/// build what it hands back. Part of the caller's undo step; a failure rolls the whole step back.
fn callback(s: &mut Session, p: &InstalledPlugin, w: NodeId, target: Option<&str>, fuel: Option<f64>) -> Result<RunOutput> {
    let m = &p.manifest;
    let wasm = module_of(s, p)?;
    let event = target.map(|t| json!({"target": t}));
    let state = state_of(s.doc()?, w);
    let input =
        json!({"api": m.api, "command": "widget", "params": {}, "selection": s.selection(), "widget": {"id": w, "state": state, "event": event}});
    let mut instance = Instance::new(&wasm, &limits(fuel)).map_err(|e| plugin_error(&m.name, e))?;
    let mut out = call_plugin(s, m, &m.name, &mut instance, &input.to_string()).map_err(|(e, _)| e)?;
    match out.widget.take() {
        Some(r) => build(s, &m.id, w, &r)?,
        None if target.is_none() => return Err(bad("widget.insert", format!("{} drew no widget", m.name))),
        None => {}
    }
    Ok(out)
}

/// Rebuild widget `w` of `plugin` from `r`: its own look and state, and new locked layers inside.
fn build(s: &mut Session, plugin: &str, w: NodeId, r: &WidgetRender) -> Result<()> {
    let cmd = "widget.render";
    let ds = s.ds_mut()?;
    let before = ds.doc.clone();
    let doc = ds.doc_mut();
    let n = doc.get(w).map_err(|_| bad(cmd, "the widget was removed while its plugin ran"))?;
    if !is_widget(n) || n.plugin_data.get(PLUGIN).map(String::as_str) != Some(plugin) {
        return Err(bad(cmd, "the layer is no longer this plugin's widget"));
    }
    for c in n.children.clone() {
        doc.remove(c)?;
    }
    crate::cmd::props::apply(cmd, doc, w, &props(&r.root, true))?;
    let n = doc.get_mut(w)?;
    n.locked = false;
    if !r.root.name.is_empty() {
        n.name = r.root.name.clone();
    }
    if let Some(state) = &r.state {
        n.plugin_data.insert(STATE.into(), Value::Object(state.clone()).to_string());
    }
    for c in &r.root.children {
        add(doc, w, c, cmd)?;
    }
    crate::normalize(doc, &before);
    Ok(())
}

/// Add layer `d` (and what it holds) inside `parent`, locked.
fn add(doc: &mut Document, parent: NodeId, d: &WidgetNode, cmd: &str) -> Result<()> {
    let (kind, name) = match d.kind {
        WidgetKind::Frame => (NodeKind::Frame, "Frame"),
        WidgetKind::Text => (NodeKind::Text(TextData { characters: d.text.clone(), ..TextData::default() }), "Text"),
        WidgetKind::Rectangle => (NodeKind::Rectangle, "Rectangle"),
        WidgetKind::Ellipse => (NodeKind::Ellipse { start: 0.0, sweep: std::f64::consts::TAU, inner: 0.0 }, "Ellipse"),
        WidgetKind::Image => (NodeKind::Rectangle, "Image"),
    };
    let name = if d.name.is_empty() { name.to_string() } else { d.name.clone() };
    let id = doc.alloc_id();
    let mut n = Node::new(id, kind, name, d.width.unwrap_or(100.0), d.height.unwrap_or(100.0));
    n.transform = Affine::translate((d.x.unwrap_or(0.0), d.y.unwrap_or(0.0)));
    n.locked = true;
    if !d.target.is_empty() {
        n.plugin_data.insert(TARGET.into(), d.target.clone());
    }
    if d.kind == WidgetKind::Image {
        use base64::Engine as _;
        let bytes = base64::engine::general_purpose::STANDARD.decode(d.data.trim()).map_err(|e| bad(cmd, format!("an image's `data`: {e}")))?;
        if bytes.len() > MAX_IMAGE_BYTES {
            return Err(bad(cmd, format!("an image is larger than {} KB", MAX_IMAGE_BYTES / 1024)));
        }
        let (hash, iw, ih) = figcraft_format::add_image(doc, bytes).map_err(|e| bad(cmd, e.to_string()))?;
        if iw > MAX_IMAGE_SIDE || ih > MAX_IMAGE_SIDE {
            return Err(bad(cmd, format!("an image is larger than {MAX_IMAGE_SIDE} pixels on a side")));
        }
        n.width = d.width.unwrap_or(f64::from(iw));
        n.height = d.height.unwrap_or(f64::from(ih));
        n.fills = vec![Paint::image(hash, ImageScaleMode::Fill)];
    }
    doc.insert(n, parent, None)?;
    crate::cmd::props::apply(cmd, doc, id, &props(d, false))?;
    d.children.iter().try_for_each(|c| add(doc, id, c, cmd))
}

/// `node.set` properties for described layer `d` (validated already); `root`: the widget itself,
/// which keeps its place and size unless the description gives one.
fn props(d: &WidgetNode, root: bool) -> Map<String, Value> {
    let mut p = Map::new();
    let mut set = |k: &str, v: Value| {
        p.insert(k.into(), v);
    };
    match d.kind {
        WidgetKind::Text => {
            if let Some(c) = &d.color {
                set("fills", json!([c]));
            }
            set("fontSize", json!(d.size.unwrap_or(14.0)));
            if d.bold {
                set("fontStyle", json!("Bold"));
            }
            // A text with a width wraps; one without grows with its text.
            set("textAutoResize", json!(if d.width.is_some() { "height" } else { "widthAndHeight" }));
        }
        WidgetKind::Image => {}
        _ => set("fills", json!(d.fill.iter().collect::<Vec<_>>())),
    }
    if let Some(c) = &d.stroke {
        set("strokes", json!([c]));
        set("strokeWeight", json!(d.stroke_weight.unwrap_or(1.0)));
    } else if root {
        set("strokes", json!([]));
    }
    if matches!(d.kind, WidgetKind::Frame | WidgetKind::Rectangle | WidgetKind::Image) {
        set("cornerRadius", json!(d.radius.unwrap_or(0.0)));
    }
    set("opacity", json!(d.opacity.unwrap_or(1.0)));
    if d.kind == WidgetKind::Frame {
        match d.layout {
            Some(l) => {
                set("layout", json!(l));
                set("gap", json!(d.gap.unwrap_or(0.0)));
                set("padding", json!(d.padding.unwrap_or(0.0)));
                set("crossAlign", json!(d.align.map_or(json!("center"), |a| json!(a))));
                set("mainAlign", json!(d.justify.map_or(json!("min"), |a| json!(a))));
                set("sizingH", json!(if d.width.is_some() { "fixed" } else { "hug" }));
                set("sizingV", json!(if d.height.is_some() { "fixed" } else { "hug" }));
            }
            None => set("layout", Value::Null),
        }
    }
    // A new layer got its size when it was made.
    if root {
        for (k, v) in [("width", d.width), ("height", d.height)] {
            if let Some(v) = v {
                set(k, json!(v));
            }
        }
    }
    p
}

/// What `widget.insert` and `widget.click` return.
fn report(s: &Session, w: NodeId, m: &Manifest, out: RunOutput) -> Result<Value> {
    let doc = s.doc()?;
    let output = serde_json::from_str::<Value>(&out.output).unwrap_or(Value::String(out.output));
    Ok(
        json!({"id": w, "plugin": m.id, "state": state_of(doc, w), "targets": targets(doc, w), "output": output, "logs": out.logs, "calls": out.calls, "fuel": out.fuel}),
    )
}

fn insert(s: &mut Session, p: &Value) -> Result<Value> {
    let cmd = "widget.insert";
    let fuel = num(p, "fuel");
    let plugin = string(p, "plugin").ok_or_else(|| bad(cmd, "`plugin` required (a widget plugin's id; see plugin.list)"))?;
    let wp = widget_plugin(s, cmd, &plugin)?;
    let m = &wp.manifest;
    let (x, y) = (num(p, "x").filter(|v| v.abs() < 1e7), num(p, "y").filter(|v| v.abs() < 1e7));
    let page = s.page()?;
    let (cx, cy) = {
        let v = &s.ds()?.view;
        (v.cx, v.cy)
    };
    let (w, out) = s.grouped(&m.name, |s| {
        let doc = s.ds_mut()?.doc_mut();
        let w = doc.alloc_id();
        let mut n = Node::new(w, NodeKind::Frame, m.name.clone(), 100.0, 100.0);
        n.transform = Affine::translate((x.unwrap_or(cx), y.unwrap_or(cy)));
        n.fills.clear();
        n.plugin_data.insert(PLUGIN.into(), m.id.clone());
        doc.insert(n, page, None)?;
        let out = callback(s, &wp, w, None, fuel)?;
        let ds = s.ds_mut()?;
        // Without a position, centred in the view on whole pixels.
        if x.is_none() && y.is_none() {
            let (ww, wh) = ds.doc.node(w).map_or((0.0, 0.0), |n| (n.width, n.height));
            ds.doc_mut().get_mut(w)?.transform = Affine::translate(((cx - ww / 2.0).round(), (cy - wh / 2.0).round()));
        }
        ds.selection = vec![w];
        Ok((w, out))
    })?;
    report(s, w, m, out)
}

fn click(s: &mut Session, p: &Value) -> Result<Value> {
    let cmd = "widget.click";
    let nid = id(p, "id").ok_or_else(|| bad(cmd, "`id` required (a widget, or a layer in one)"))?;
    let target = string(p, "target").ok_or_else(|| bad(cmd, "`target` required (see widget.list)"))?;
    let doc = s.doc()?;
    let w = widget_of(doc, nid).ok_or_else(|| bad(cmd, format!("{nid} isn't a widget or inside one")))?;
    let all = targets(doc, w);
    if !all.contains(&target) {
        return Err(bad(cmd, format!("the widget has no `{target}` to click (it has: {})", all.join(", "))));
    }
    let plugin = doc.node(w).and_then(|n| n.plugin_data.get(PLUGIN)).cloned().unwrap_or_default();
    let fuel = num(p, "fuel");
    let wp = widget_plugin(s, cmd, &plugin)?;
    let out = s.grouped(&wp.manifest.name, |s| callback(s, &wp, w, Some(&target), fuel))?;
    report(s, w, &wp.manifest, out)
}

/// Click `target` of the widget `id` is (or is in) on `doc`, a document that isn't one of the
/// session's: the prototype player's copy. The same callback as on the canvas, with the session's
/// installed plugins and the same sandbox limits, but the widget's state and layers are the
/// copy's, nothing reaches the session's documents and there is no undo step. Returns the
/// document as the callback left it (on failure the caller keeps its copy as it was) and what
/// `widget.click` returns.
pub fn click_on(s: &Session, doc: Document, id: NodeId, target: &str) -> Result<(Document, Value)> {
    let mut scratch = Session::with_document(doc);
    scratch.prefs.plugins.clone_from(&s.prefs.plugins);
    scratch.plugins.modules.clone_from(&s.plugins.modules);
    let out = click(&mut scratch, &json!({"id": id, "target": target}))?;
    let ds = scratch.docs.pop().ok_or(crate::EngineError::NoDocument)?;
    Ok((std::sync::Arc::unwrap_or_clone(ds.doc), out))
}

fn list(s: &mut Session, _p: &Value) -> Result<Value> {
    let doc = s.doc()?;
    let widgets: Vec<Value> = doc
        .ids()
        .filter_map(|i| {
            let n = doc.node(i).filter(|n| is_widget(n))?;
            let state = Value::Object(state_of(doc, i));
            Some(json!({"id": i, "plugin": n.plugin_data.get(PLUGIN), "name": n.name, "page": doc.page_of(i), "state": state, "targets": targets(doc, i)}))
        })
        .collect();
    Ok(json!({"widgets": widgets}))
}

pub fn specs() -> Vec<CommandSpec> {
    vec![
        spec!(
            "widget.insert",
            "Insert widget",
            "{plugin (a widget plugin's id), x?, y? (top-left; default: centred in the view), fuel?} the plugin draws it; one undo step → {id, plugin, state, targets, output, logs, calls, fuel}",
            false,
            insert
        ),
        spec!(
            "widget.click",
            "Click widget",
            "{id (a widget or a layer in one), target (see widget.list), fuel?} calls the widget's plugin back, which redraws it; one undo step → like widget.insert",
            false,
            click
        ),
        spec!("widget.list", "List widgets", "{} → {widgets: [{id, plugin, name, page, state, targets}]}", false, list),
    ]
}
