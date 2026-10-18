# Porting audit: Linux and Android

Audit of the upstream stack for the release-1 apps (Trano, Sary, Taratasy, Soratra, Tabilao,
Seho), written before and during the port (2026-10-18). Every row was checked against the code
and, where marked, against a real build or an emulator run.

## Upstream stack

| Layer            | What upstream uses                                     | Android status                                   |
|------------------|--------------------------------------------------------|--------------------------------------------------|
| UI               | egui / eframe 0.36 (immediate mode, pure Rust)         | supported by eframe (android-activity)           |
| Windowing        | winit 0.30.13 (Taratasy vendors a patched copy)        | supported; text input gap (see below)            |
| GPU              | wgpu 30: Vulkan, Metal, DX12, GLES                     | Vulkan, GLES 3 fallback chosen by wgpu           |
| Text             | parley, cosmic/own shaping, embedded fonts             | pure Rust, builds as is                          |
| File dialogs     | rfd 0.17 (XDG portal / GTK / Win32 / AppKit)           | **no Android backend** → vendored backend added  |
| Clipboard        | arboard 3.6                                            | **no Android backend** → vendored backend added  |
| Audio (Seho)     | cpal 0.16                                              | AAudio backend, builds                           |
| Video (Seho)     | own pure-Rust AV1/H.264/HEVC/VP9 decoders              | builds; CPU-heavy on phones                      |
| PDF (Taratasy)   | own COS/model + hayro renderer, krilla writer          | builds                                           |
| Crypto           | rustls + aws-lc-sys (C, Taratasy signatures)           | builds with the NDK's clang                      |
| OCR (Taratasy)   | ocrs (pure Rust); models downloaded on demand          | builds; models are large, see risks              |
| Accessibility    | accesskit                                              | only with GameActivity (not NativeActivity)      |
| Database         | none in these apps (sqlx only in the ArtCraft AI app)  | n/a for release 1                                |
| ffmpeg           | not used                                               | no LGPL/GPL codec licensing question             |

## Incompatibilities found, and what was done

| # | Problem                                                                                     | Risk   | Mitigation                                                                                                                                       | Verified                     |
|---|---------------------------------------------------------------------------------------------|--------|--------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------|
| 1 | No `android_main` entry point; eframe needs an `AndroidApp`                                  | high   | one small `<app>-android` cdylib per app, reusing the desktop app's modules by path                                                              | builds for arm64 and x86_64  |
| 2 | eframe's `accesskit` feature refuses NativeActivity                                         | medium | GameActivity (androidx.games 4.4.0) instead; it also brings a real IME                                                                           | build                        |
| 3 | winit 0.30 ignores GameActivity's `TextEvent`: soft-keyboard text was lost after one letter | high   | `EguiBridge` diffs the GameTextInput buffer into egui Text/Backspace/Enter events; multi-line IME so Enter arrives; no full-screen extract UI   | emulator: "Mahay", `=21*2`   |
| 4 | Android 15+ forces edge-to-edge: the ribbon was under the status bar                        | medium | `systemInsets()` (bars, cutout, keyboard) → the app is laid out inside them; bar icons follow the app's theme                                   | emulator screenshot          |
| 5 | rfd has no Android backend                                                                  | high   | `libs/core/vendor/rfd` adds one: Storage Access Framework through `mahay-android`; patched in every app workspace; desktop code paths unchanged | build                        |
| 6 | arboard has no Android backend                                                              | medium | `libs/core/vendor/arboard` adds a text backend on `ClipboardManager`; images and file lists report "not supported"                              | build                        |
| 7 | Scoped storage: pickers return `content://` URIs, not paths                                 | high   | copy into app storage, edit the copy, copy each save back to the URI (persisted grant); Save As asks `ACTION_CREATE_DOCUMENT` after first write | not yet on a device          |
| 8 | Blocking a picker on the native thread would deadlock the activity (its Java side waits)    | high   | pickers never block the app thread; answers come back through channels and `poll()`                                                             | design                       |
| 9 | Folder pickers (Taratasy batch OCR/export to folder) have no path equivalent                | low    | answer "cancelled" for now; listed in STATUS known gaps                                                                                          | —                            |
| 10| Printing (CUPS / Win32 spooler)                                                             | low    | not offered on Android in release 1; export to PDF and share instead                                                                            | —                            |
| 11| Single-instance IPC, Wayland/X11 detection, tray, desktop entries                           | low    | desktop-only modules are not compiled into the Android crates                                                                                    | build                        |
| 12| Control channel / MCP server (localhost TCP)                                                | low    | not started on Android (no CLI there); the CLIs and MCP servers stay on desktop                                                                  | —                            |
| 13| Desktop dirs (`dirs`, `directories`) find no home on Android                                | medium | `mahay-android::init` points `HOME`, `XDG_*`, `TMPDIR` at the app's private storage before anything starts; eframe persistence path set         | emulator                     |
| 14| Ribbon UIs (Soratra, Tabilao, Seho) on phones (compact width)                               | medium | tablets get the full layout; on phones the ribbon scrolls; a dedicated compact mode is planned (STATUS)                                         | tablet emulator              |
| 15| Memory limits: large images (Sary), long PDFs, video (Seho)                                  | medium | upstream already tiles and streams; no extra cap yet; to measure on a 4 GB device                                                                | —                            |
| 16| GPU: emulators and old devices only offer software Vulkan (SwiftShader) or GLES 3           | low    | wgpu picks GLES when Vulkan isn't usable; Sary keeps its CPU canvas on software adapters                                                         | emulator (SwiftShader)       |
| 17| Upstream release feed (`api.storyteller.ai`) and social links                               | high   | removed: Trano reads GitHub releases of `raantss18/mahay` only                                                                                   | code review                  |
| 18| APK size                                                                                    | medium | one APK per app per ABI, fat LTO + strip, compressed `.so`, R8 for Java                                                                          | Tabilao x86_64: 11.5 MB      |

## Platform notes

- **Minimum Android:** 8.0 (API 26). Vulkan needs API 28+ in practice; below that, GLES 3.
- **ABIs:** arm64-v8a (phones and tablets) and x86_64 (emulators, Chromebooks).
- **Tablets and foldables:** `resizeableActivity`, every configuration change handled in
  place (no restart on rotation, fold, keyboard or density change), multi-window allowed.
- **Keyboard and mouse on tablets:** winit delivers hardware keys and mouse events; Ctrl+V reads
  the Android clipboard through the bridge.
- **Stylus:** arrives as touch with pressure through winit; Sary's tablet pressure path is not
  wired to it yet.
- **Linux desktop:** unchanged upstream code paths (Wayland and X11, HiDPI, software rendering
  fallback through wgpu's GL/llvmpipe). Packaging reuses upstream's `packaging/linux` scripts
  (AppImage, .deb, .rpm, Flatpak manifests).
- **Windows and macOS:** code paths untouched; not built in this environment (see STATUS).

## WASM fallback

Every app also has an upstream `*-web` build. It was not needed on Android: the native build
works for all five apps, so the WebView path stays unused.
