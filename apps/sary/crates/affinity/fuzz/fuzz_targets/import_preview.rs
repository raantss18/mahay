#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // The complete import: native document mapping, or the preview fallback.
    if sary_affinity::is_affinity(data) {
        let _ = sary_io::import("fuzz.af", data);
    }
});
