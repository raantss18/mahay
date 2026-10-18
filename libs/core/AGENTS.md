# AGENTS.md

Shared Rust libraries for the Mahay Apps. The shared standards live in
[mahay-rules](https://github.com/raantss18/mahay-rules) (`../../mahay-rules` on a dev machine): read its
`AGENTS.md`, then `standards/never-crash.md`, `standards/engineering.md` and `standards/licensing.md`.

- **Never crash.** Every crate's `lib.rs` starts with
  `#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]`.
  No `unsafe` (the workspace forbids it).
- **New crates** go in `crates/<name>`, named `mahay-<name>`, inherit the workspace package fields
  and lints, and are listed in `[workspace.dependencies]`.
- **No app code.** A crate here must be useful to more than one app and must not depend on any app.
- **Before merging:** `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`.
