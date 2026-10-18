#![no_main]

use libfuzzer_sys::fuzz_target;

// The whole native reader (archive, object stream, model) and the preview, on arbitrary bytes.
fuzz_target!(|bytes: &[u8]| {
    let _ = tsipika_affinity::container::header(bytes);
    let _ = tsipika_affinity::preview(bytes);
    let _ = tsipika_affinity::read(bytes, tsipika_affinity::Limits { max_entry: 16 << 20, max_total: 64 << 20 });
});
