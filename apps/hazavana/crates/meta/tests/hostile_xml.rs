//! Hostile XMP packets (deep nesting, huge attribute, namespace or entity counts) are refused
//! or parsed in bounded time, never stall a worker (RUSTSEC-2026-0194/0195 class of attacks).

use std::time::{Duration, Instant};

use hazavana_meta::{MergeRules, XmpError, merge_xmp, parse_xmp, write_xmp};

/// Generous for debug builds on a slow CI runner; the quadratic paths take minutes.
const BUDGET: Duration = Duration::from_secs(3);

const HEAD: &str = r#"<x:xmpmeta xmlns:x="adobe:ns:meta/"><rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">"#;
const TAIL: &str = "</rdf:RDF></x:xmpmeta>";

fn packet(desc_attrs: &str, body: &str) -> String {
    format!("{HEAD}<rdf:Description rdf:about=\"\" {desc_attrs}>{body}</rdf:Description>{TAIL}")
}

fn deep_nesting(n: usize) -> String {
    packet("", &format!("{}{}", "<a>".repeat(n), "</a>".repeat(n)))
}

fn many_attributes(n: usize) -> String {
    let attrs: String = (0..n).map(|i| format!("a{i}=\"v\" ")).collect();
    packet(&attrs, "")
}

/// One tag declaring `n` prefixes and using each of them: name resolution is per attribute.
fn many_namespaces(n: usize) -> String {
    let decls: String = (0..n).map(|i| format!("xmlns:p{i}=\"urn:x:{i}\" p{i}:k=\"v\" ")).collect();
    packet(&decls, "")
}

/// Every level of a deep chain declares many prefixes, so the in-scope set grows with depth.
fn stacked_namespaces(depth: usize, per_level: usize) -> String {
    let open: String = (0..depth)
        .map(|d| {
            let decls: String = (0..per_level).map(|i| format!(" xmlns:p{d}_{i}=\"urn:x:{d}:{i}\"")).collect();
            let uses: String = (0..per_level).map(|i| format!(" p0_{i}:k{d}=\"v\"")).collect();
            format!("<e{decls}{uses}>")
        })
        .collect();
    packet("", &format!("{open}{}", "</e>".repeat(depth)))
}

fn many_entities(n: usize) -> String {
    packet("xmlns:dc=\"http://purl.org/dc/elements/1.1/\"", &format!("<dc:title>{}</dc:title>", "&amp;&#x41;&unknown;".repeat(n)))
}

fn many_elements(n: usize) -> String {
    packet("xmlns:dc=\"http://purl.org/dc/elements/1.1/\"", &"<dc:subject/>".repeat(n))
}

fn hostile() -> Vec<(&'static str, String)> {
    vec![
        ("deep nesting", deep_nesting(100_000)),
        ("many attributes", many_attributes(200_000)),
        ("many namespaces", many_namespaces(100_000)),
        ("stacked namespaces", stacked_namespaces(60, 2_000)),
        ("many entities", many_entities(300_000)),
        ("many elements", many_elements(400_000)),
    ]
}

fn timed<T>(what: &str, f: impl FnOnce() -> T) -> T {
    let t = Instant::now();
    let out = f();
    let took = t.elapsed();
    assert!(took < BUDGET, "{what} took {took:?}");
    out
}

#[test]
fn hostile_packets_parse_in_bounded_time() {
    for (what, s) in hostile() {
        let r = timed(what, || parse_xmp(&s));
        if let Err(e) = &r {
            assert!(matches!(e, XmpError::TooComplex | XmpError::Xml(_)), "{what}: {e}");
        }
    }
}

#[test]
fn hostile_packets_merge_in_bounded_time() {
    let fresh = write_xmp(&Default::default(), Some("{}"));
    let rules = MergeRules { owned: &["xmp:Rating", "lc:*"], owned_if_present: &[] };
    for (what, s) in hostile() {
        let _ = timed(what, || merge_xmp(&s, &fresh, rules));
    }
}

#[test]
fn size_limits_are_reported_as_too_complex() {
    for (what, s) in [
        ("deep nesting", deep_nesting(100_000)),
        ("many elements", many_elements(400_000)),
        ("many namespaces", many_namespaces(100_000)),
        ("stacked namespaces", stacked_namespaces(60, 2_000)),
    ] {
        assert!(matches!(parse_xmp(&s), Err(XmpError::TooComplex)), "{what}");
    }
}

#[test]
fn many_plain_attributes_are_still_read() {
    let d = timed("many attributes", || parse_xmp(&many_attributes(200_000))).unwrap();
    assert_eq!(d.properties.len(), 200_000);
}

#[test]
fn a_typical_number_of_namespaces_is_accepted() {
    let d = parse_xmp(&many_namespaces(200)).unwrap();
    assert_eq!(d.properties.len(), 200);
}
