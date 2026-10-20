# tex-m0: can Tectonic (XeTeX) run Mahay's LaTeX editor? (spike M0)

Throwaway Cargo workspace, **not** part of the Mahay suite (its own `[workspace]`, own lockfile).
It answers the engine question of `docs/NEW_APPS.md` / D25 with measurements. The report is in
`docs/NEW_APPS.md` ("M0 spike results"); the raw numbers are in `results/`.

`tex-m0` is a ~150-line program on the `tectonic` crate 0.17.0 (MIT) and `tectonic_bundles`
0.4.2 (MIT): `tex-m0 <bundle-dir|online> input.tex outdir [--format-cache DIR] [--synctex] [--record DIR]`.
It prints one `RESULT` line (success, milliseconds, peak resident memory from `VmHWM`, sizes).
`--record DIR` (with `online`) saves a copy of every file the engine asks the bundle for, which
is how the directory bundle is made. The `online` Cargo feature (default) exists only for that.

## Recipe (Linux, Arch; needs `freetype2 fontconfig graphite2 icu libpng` dev packages)

```bash
cd experiments/tex-m0
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=$PWD/target
cargo build --release -j 4                       # with the online bundle, to record
./record.sh                                      # compiles samples/*.tex online, copies the fetched files to work/rec-*
python3 -I tools/mkbundle.py                     # work/bundle (directory bundle) + work/packs/<Pack> + sizes
cargo build --release --no-default-features -j 4 # the offline-only engine (what the app would embed)
./run-linux.sh                                   # all samples, cold then warm, network cut with `unshare -rn`
python3 -I tools/licences.py                     # licence scan of the bundle files, by pack
```

Notes: `record.sh` needs the network once (default bundle, `relay.fullyjustified.net`);
nothing else does. `unshare -rn` needs unprivileged user namespaces. Fonts: Tectonic finds
bundle fonts by **file name** (`\setmainfont{lmroman10-regular.otf}[...]`), see `samples/e_fontspec.tex`.

## Recipe (Android; NDK 28.2.13676358, cargo-ndk, rustup targets, `ANDROID_HOME` set by the scripts)

```bash
tools/android-deps.sh x86_64      # and aarch64: freetype (CMake), libpng, stubs, pkg-config files -> work/android-deps/<abi>
./build-android.sh x86_64         # cargo ndk -P 31 build --release --no-default-features (and aarch64)
adb -s emulator-5554 shell mkdir -p /data/local/tmp/texm0
adb -s emulator-5554 push target/x86_64-linux-android/release/tex-m0 /data/local/tmp/texm0/tex-m0-x86_64
adb -s emulator-5554 push work/bundle /data/local/tmp/texm0/bundle
adb -s emulator-5554 push samples /data/local/tmp/texm0/samples
./run-android.sh x86_64 emulator-5554
adb -s emulator-5554 shell rm -rf /data/local/tmp/texm0   # clean up
```

`build-android.sh` puts rustup's cargo first in `PATH` (the distribution's `rustc` has no Android
standard library), points `pkg-config` at `work/android-deps/<abi>/lib/pkgconfig`, and links libc++
statically. Obstacles and their fixes are listed in `docs/NEW_APPS.md`.

## Layout

`samples/` the documents (a–f asked for, g–m extra packages) and `refs.bib`; `tools/` bundle,
licence and table scripts, `tools/android-deps/stubs/` the C stand-ins; `results/` the measured
output (committed); `target/` and `work/` are git-ignored and deleted at the end of the spike
(about 4 GB).
