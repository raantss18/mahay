//! Hostile lens database files (deep nesting, huge attribute or entity counts) are refused or
//! parsed in bounded time.

use std::time::{Duration, Instant};

use hazavana_lensdb::LensDb;

const BUDGET: Duration = Duration::from_secs(3);

#[test]
fn hostile_lens_databases_parse_in_bounded_time() {
    let attrs: String = (0..200_000).map(|i| format!(" a{i}=\"&amp;\"")).collect();
    let cases = [
        ("deep nesting", format!("<lensdatabase>{}{}</lensdatabase>", "<a>".repeat(100_000), "</a>".repeat(100_000))),
        ("many attributes", format!("<lensdatabase><lens{attrs}/></lensdatabase>")),
        ("many entities", format!("<lensdatabase><lens><model>{}</model></lens></lensdatabase>", "&amp;&#x41;".repeat(300_000))),
        ("many elements", format!("<lensdatabase>{}</lensdatabase>", "<lens/>".repeat(600_000))),
    ];
    for (what, xml) in cases {
        let t = Instant::now();
        let _ = LensDb::parse(&xml);
        assert!(t.elapsed() < BUDGET, "{what} took {:?}", t.elapsed());
    }
}
