//! `cargo xtask layers`: dependency layering rules (AGENTS.md "Layering", plan/architecture.md).
//!
//! The rule engine works on a small, metadata-independent model so it can be
//! unit-tested; `from_metadata` builds that model from `cargo metadata`.

use serde_json::Value;

/// Where a workspace crate sits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Class {
    /// Regular layered crate.
    Layer(u8),
    /// Binaries and build tooling: exempt from the rules.
    Exempt,
}

impl Class {
    fn layer(self) -> Option<u8> {
        match self {
            Class::Layer(l) => Some(l),
            Class::Exempt => None,
        }
    }
}

/// The layering table (AGENTS.md "Layering", plan/architecture.md). Names are package names
/// without the `tabilao-` prefix.
pub const TABLE: &[(&str, Class)] = &[
    ("core", Class::Layer(0)),
    ("numfmt", Class::Layer(0)),
    ("pdf", Class::Layer(0)),
    ("formula", Class::Layer(1)),
    ("functions", Class::Layer(1)),
    ("model", Class::Layer(2)),
    ("calc", Class::Layer(2)),
    ("xlsx", Class::Layer(2)),
    ("chart", Class::Layer(3)),
    ("engine", Class::Layer(5)),
    ("ui-egui", Class::Layer(6)),
    ("mcp", Class::Layer(6)),
    // L7 apps (exempt) and tooling (unchecked)
    ("tabilao", Class::Layer(7)),
    ("cli", Class::Layer(7)),
    ("web", Class::Layer(7)),
    ("xtask", Class::Exempt),
];

/// Explicit orderings *within* a layer (earlier may be used by later). Each chain is separate:
/// `calc` and `xlsx` both build on `model` but not on each other.
pub const INTRA_LAYER_ORDER: &[&[&str]] = &[&["core", "numfmt"], &["formula", "functions"], &["model", "calc"], &["model", "xlsx"]];

fn intra_layer_allowed(from: &str, to: &str) -> bool {
    let (from, to) = (short_name(from), short_name(to));
    INTRA_LAYER_ORDER.iter().any(|chain| match (chain.iter().position(|n| *n == from), chain.iter().position(|n| *n == to)) {
        (Some(f), Some(t)) => t < f,
        _ => false,
    })
}

/// Format crates may only depend on these workspace crates (on top of the layer rule), so a
/// file format never pulls in calculation or engine code.
pub const FORMAT_ALLOWED: &[(&str, &[&str])] = &[("xlsx", &["core", "formula", "model"])];

/// External crates that constitute a UI toolkit / windowing dependency.
/// Entries ending in `*` are prefixes.
pub const UI_CRATES: &[&str] = &["egui*", "eframe", "winit", "rfd", "wgpu*"];

/// First layer allowed to use UI crates.
pub const UI_MIN_LAYER: u8 = 6;

pub fn short_name(pkg: &str) -> &str {
    pkg.strip_prefix("tabilao-").unwrap_or(pkg)
}

pub fn classify(pkg: &str) -> Option<Class> {
    let s = short_name(pkg);
    TABLE.iter().find(|(n, _)| *n == s).map(|(_, c)| *c)
}

fn is_ui_crate(name: &str) -> bool {
    UI_CRATES.iter().any(|p| match p.strip_suffix('*') {
        Some(prefix) => name.starts_with(prefix),
        None => name == *p,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DepKind {
    Normal,
    Dev,
    Build,
}

#[derive(Debug, Clone)]
pub struct Dep {
    pub name: String,
    pub kind: DepKind,
    /// `true` if the dependency is a workspace member.
    pub workspace: bool,
}

#[derive(Debug, Clone)]
pub struct Crate {
    pub name: String,
    pub deps: Vec<Dep>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Violation {
    Unregistered { krate: String },
    Upward { krate: String, dep: String, from: u8, to: u8, kind: DepKind },
    FormatDep { krate: String, dep: String },
    UiBelowL6 { krate: String, dep: String, layer: u8 },
}

impl std::fmt::Display for Violation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Violation::Unregistered { krate } => {
                write!(f, "{krate}: unknown workspace crate; register it in xtask/src/layers.rs TABLE (see plan/architecture.md §3)")
            }
            Violation::Upward { krate, dep, from, to, kind } => {
                write!(f, "{krate} (L{from}) -> {dep} (L{to}) [{kind:?}]: may only depend on strictly lower layers")
            }
            Violation::FormatDep { krate, dep } => {
                write!(f, "{krate}: format crates may only depend on core/formula/model, not {dep}")
            }
            Violation::UiBelowL6 { krate, dep, layer } => {
                write!(f, "{krate} (L{layer}) depends on UI crate `{dep}`; UI toolkits are only allowed in L6+")
            }
        }
    }
}

/// Check all rules. Returns violations sorted for stable output.
pub fn check(crates: &[Crate]) -> Vec<Violation> {
    let mut out = Vec::new();
    for c in crates {
        let Some(class) = classify(&c.name) else {
            out.push(Violation::Unregistered { krate: c.name.clone() });
            continue;
        };
        // Tooling and the L7 apps sit on top of everything.
        let Some(layer) = class.layer().filter(|l| *l < 7) else {
            continue;
        };
        let format_allowed = FORMAT_ALLOWED.iter().find(|(n, _)| *n == short_name(&c.name)).map(|(_, a)| *a);
        for d in &c.deps {
            // Self dev-dependencies (e.g. to enable features in tests) are fine.
            if d.name == c.name {
                continue;
            }
            if d.workspace {
                if let Some(allowed) = format_allowed
                    && d.kind != DepKind::Dev
                    && !allowed.contains(&short_name(&d.name))
                {
                    out.push(Violation::FormatDep { krate: c.name.clone(), dep: d.name.clone() });
                }
                // Unregistered deps are reported on their own entry.
                let Some(dc) = classify(&d.name) else { continue };
                let to = dc.layer().unwrap_or(u8::MAX);
                if to >= layer && !(to == layer && intra_layer_allowed(&c.name, &d.name)) {
                    out.push(Violation::Upward {
                        krate: c.name.clone(),
                        dep: d.name.clone(),
                        from: layer,
                        to: if to == u8::MAX { 7 } else { to },
                        kind: d.kind,
                    });
                }
            } else if layer < UI_MIN_LAYER && is_ui_crate(&d.name) {
                out.push(Violation::UiBelowL6 { krate: c.name.clone(), dep: d.name.clone(), layer });
            }
        }
    }
    out.sort_by_key(|v| v.to_string());
    out.dedup();
    out
}

/// Build the model from `cargo metadata --format-version 1 --no-deps`.
pub fn from_metadata(meta: &Value) -> Result<Vec<Crate>, String> {
    let pkgs = meta["packages"].as_array().ok_or("metadata: no packages array")?;
    let members: Vec<&str> = pkgs.iter().filter_map(|p| p["name"].as_str()).collect();
    let mut out = Vec::new();
    for p in pkgs {
        let name = p["name"].as_str().ok_or("package without name")?.to_owned();
        let mut deps = Vec::new();
        for d in p["dependencies"].as_array().into_iter().flatten() {
            let dname = d["name"].as_str().unwrap_or_default().to_owned();
            let kind = match d["kind"].as_str() {
                Some("dev") => DepKind::Dev,
                Some("build") => DepKind::Build,
                _ => DepKind::Normal,
            };
            let workspace = members.contains(&dname.as_str()) || d["path"].is_string();
            deps.push(Dep { name: dname, kind, workspace });
        }
        out.push(Crate { name, deps });
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(out)
}

pub fn describe(class: Option<Class>) -> String {
    match class {
        Some(Class::Layer(7)) => "L7 app".into(),
        Some(Class::Layer(l)) => format!("L{l}"),
        Some(Class::Exempt) => "unchecked".into(),
        None => "UNREGISTERED".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(name: &str, deps: &[(&str, DepKind, bool)]) -> Crate {
        Crate { name: name.into(), deps: deps.iter().map(|(n, k, w)| Dep { name: (*n).into(), kind: *k, workspace: *w }).collect() }
    }
    use DepKind::*;

    #[test]
    fn clean_downward_graph_passes() {
        let g = [
            c("tabilao-core", &[("serde", Normal, false)]),
            c("tabilao-numfmt", &[("tabilao-core", Normal, true)]),
            c("tabilao-formula", &[("tabilao-core", Normal, true)]),
            c("tabilao-functions", &[("tabilao-core", Normal, true), ("tabilao-formula", Normal, true)]),
            c("tabilao-model", &[("tabilao-formula", Normal, true)]),
            c("tabilao-calc", &[("tabilao-model", Normal, true), ("tabilao-functions", Normal, true), ("tabilao-numfmt", Normal, true)]),
            c("tabilao-xlsx", &[("tabilao-core", Normal, true), ("tabilao-formula", Normal, true), ("tabilao-model", Normal, true)]),
            c("tabilao-chart", &[("tabilao-model", Normal, true)]),
            c("tabilao-engine", &[("tabilao-calc", Normal, true), ("tabilao-xlsx", Normal, true), ("tabilao-chart", Normal, true)]),
            c("tabilao-ui-egui", &[("tabilao-engine", Normal, true), ("egui", Normal, false), ("wgpu", Normal, false)]),
            c("tabilao-cli", &[("tabilao-ui-egui", Normal, true), ("tabilao-mcp", Normal, true)]),
        ];
        assert!(check(&g).is_empty(), "{:?}", check(&g));
    }

    #[test]
    fn upward_dependency_flagged() {
        let v = check(&[c("tabilao-formula", &[("tabilao-engine", Normal, true)])]);
        assert!(matches!(v[..], [Violation::Upward { from: 1, to: 5, .. }]));
    }

    #[test]
    fn sideways_dependency_flagged() {
        let v = check(&[c("tabilao-xlsx", &[("tabilao-calc", Normal, true)])]);
        assert!(v.iter().any(|x| matches!(x, Violation::Upward { from: 2, to: 2, .. })), "{v:?}");
        let v = check(&[c("tabilao-ui-egui", &[("tabilao-mcp", Normal, true)])]);
        assert!(matches!(v[..], [Violation::Upward { from: 6, to: 6, .. }]));
    }

    #[test]
    fn intra_layer_chains_allowed_one_way() {
        assert!(check(&[c("tabilao-numfmt", &[("tabilao-core", Normal, true)])]).is_empty());
        let v = check(&[c("tabilao-core", &[("tabilao-numfmt", Normal, true)])]);
        assert!(matches!(v[..], [Violation::Upward { from: 0, to: 0, .. }]));
        assert!(check(&[c("tabilao-calc", &[("tabilao-model", Normal, true)])]).is_empty());
        assert!(!check(&[c("tabilao-model", &[("tabilao-calc", Normal, true)])]).is_empty());
        assert!(!check(&[c("tabilao-calc", &[("tabilao-xlsx", Normal, true)])]).is_empty());
    }

    #[test]
    fn format_crate_limited_to_core_formula_model() {
        let v = check(&[c("tabilao-xlsx", &[("tabilao-numfmt", Normal, true)])]);
        assert!(matches!(&v[..], [Violation::FormatDep { dep, .. }] if dep == "tabilao-numfmt"), "{v:?}");
        // Dev-dependencies (round-trip tests through the calculator, say) are allowed if lower.
        assert!(check(&[c("tabilao-xlsx", &[("tabilao-numfmt", Dev, true)])]).is_empty());
    }

    #[test]
    fn self_dev_dependency_ignored() {
        assert!(check(&[c("tabilao-model", &[("tabilao-model", Dev, true)])]).is_empty());
    }

    #[test]
    fn upward_dev_dependency_flagged() {
        let v = check(&[c("tabilao-core", &[("tabilao-model", Dev, true)])]);
        assert!(matches!(v[..], [Violation::Upward { kind: Dev, .. }]));
    }

    #[test]
    fn ui_crates_below_l6_flagged() {
        for dep in ["egui", "eframe", "winit", "egui_kittest", "egui_extras", "rfd", "wgpu", "wgpu-core"] {
            let v = check(&[c("tabilao-engine", &[(dep, Normal, false)])]);
            assert!(matches!(v[..], [Violation::UiBelowL6 { layer: 5, .. }]), "{dep}");
        }
        assert!(check(&[c("tabilao-engine", &[("serde", Normal, false)])]).is_empty());
        assert!(check(&[c("tabilao-mcp", &[("winit", Normal, false)])]).is_empty());
    }

    #[test]
    fn unregistered_crate_is_error() {
        let v = check(&[c("tabilao-mystery", &[])]);
        assert!(matches!(&v[..], [Violation::Unregistered { krate }] if krate == "tabilao-mystery"));
        assert!(v[0].to_string().contains("register"));
    }

    #[test]
    fn apps_and_xtask_exempt() {
        for app in ["tabilao", "tabilao-cli", "tabilao-web", "xtask"] {
            assert!(check(&[c(app, &[("egui", Normal, false), ("tabilao-ui-egui", Normal, true)])]).is_empty());
        }
    }

    #[test]
    fn metadata_parsing() {
        let meta: Value = serde_json::from_str(
            r#"{"packages":[
                {"name":"tabilao-model","dependencies":[
                    {"name":"tabilao-core","kind":null,"path":"/x/crates/core"},
                    {"name":"serde","kind":null},
                    {"name":"proptest","kind":"dev"}]},
                {"name":"tabilao-core","dependencies":[]}
            ]}"#,
        )
        .unwrap();
        let g = from_metadata(&meta).unwrap();
        assert_eq!(g.len(), 2);
        let model = g.iter().find(|c| c.name == "tabilao-model").unwrap();
        assert!(model.deps[0].workspace && !model.deps[1].workspace);
        assert_eq!(model.deps[2].kind, Dev);
        assert!(check(&g).is_empty());
    }
}
