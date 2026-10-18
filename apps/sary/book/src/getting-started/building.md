# Building

Sary uses the Rust workspace at the repository root. Install the stable Rust toolchain, clone the repository, and run commands from that root.

On Linux, install the windowing, GL and GTK development packages first. The development guide's
[Prerequisites](https://github.com/raantss18/mahay/blob/main/docs/development.md#prerequisites)
list them for Debian/Ubuntu, Fedora, Arch and openSUSE.

```sh
cargo build --workspace
cargo run --release -p sary -- image.psd
cargo run -p sary-cli -- --help
```

The standard repository gates are:

```sh
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo xtask layers
cargo xtask wasm
```

`cargo xtask wasm` requires the `wasm32-unknown-unknown` target. UI work should also be checked using the offscreen snapshot example described in the [development guide](https://github.com/raantss18/mahay/blob/main/docs/development.md).

## Build the documentation

Install mdBook, then build from the repository root:

```sh
cargo install mdbook --locked
mdbook build book
```

The generated site is written to `book/output/` and is not source documentation.
