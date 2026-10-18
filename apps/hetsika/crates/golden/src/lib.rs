//! Golden-image tests for Hetsika's renderer (test-only crate; the library is empty).
//!
//! `tests/golden.rs` builds small procedural projects (no media files), renders one frame of each
//! through the CPU reference compositor (`hetsika-render`) and compares it with a committed PNG
//! in `goldens/` using `hetsika_testkit::golden`. The same scenes are composited on the GPU
//! (`hetsika-gpu`) when an adapter is available and compared with the CPU result.
//!
//! Regenerate references with `HETSIKA_BLESS=1 cargo test -p hetsika-golden`.
