# AGENTS.md: guide for AI agents and contributors

Mahay Trano installs, updates and launches the **Mahay App Suite** (Sary, Hazavana,
Hetsika, Pejy, Tsilo, Taratasy, Tselatra, Soratra, Seho, Tabilao,
Drafitra, Feo, Endrika). It is written in **Rust only** (no JavaScript or TypeScript): the desktop
app is native egui/eframe, and the web build is the same Rust compiled to WebAssembly (trunk +
wasm-bindgen). **No Tauri, Electron or webview shells**, ever.

The product name is always **Mahay Trano** in user-facing text: UI, window titles, About,
tray menu, notifications, installers, release names, docs prose. The suite is the **Mahay App
Suite**. Machine names stay lowercase: crates and binaries (`trano`,
`trano-*`), file names, and the app id `org.mahay.trano`. Note the
GitHub repo is `raantss18/mahay-launcher`, while every file and package is `trano-*`.

Standards and learnings shared across the Mahay apps live in `../mahay-rules` (read its
`README.md` and `AGENTS.md`). Contribute reusable learnings there, never code.

## 1. Workspace map

```text
crates/
  core      trano-core      pure and wasm-safe: the app catalog, versions, GitHub
                                        release parsing, picking the right asset per platform
  native    trano-native    desktop only: HTTP, detecting installed apps, download,
                                        verify and install
  ui-egui   trano-ui-egui   the shared egui UI (desktop and web)
apps/
  trano                     desktop shell (eframe): window, tray icon, notifications
  trano-web                 the same UI in the browser (trunk; catalog and downloads only)
xtask/                                  cargo xtask ci | wasm | version | ico
packaging/                              macOS, Windows, Linux, FreeBSD and web packaging scripts
assets/app-icon/                        the launcher's own icon (master SVG, renders, tray icons)
assets/app-icons/                       the Mahay apps' icons, copied from each app's repo
assets/fonts/                           Archivo, Inter, Geist Mono (OFL-1.1): mahay.com's faces
```

Layering: `core` depends on no other workspace crate and must build for `wasm32-unknown-unknown`.
`native` may use `core`. `ui-egui` may use `core` (and `native` only behind
`cfg(not(target_arch = "wasm32"))` or through a trait the desktop shell implements). Nothing below
the apps opens windows, tray icons or notifications.

## 2. Golden rules

### Never crash (outranks feature work)

A bad network response, a malformed release, a half-downloaded file, a full disk or a corrupt
settings file must produce an error the user can act on, never a panic. Shared standard:
`../mahay-rules/standards/never-crash.md`.

- **Non-test code never panics.** No `unwrap()`, `expect()`, `panic!`, `unreachable!`, `todo!` or
  `unimplemented!`. Return the crate's error type and propagate with `?`.
- Every production crate root (`lib.rs`, each binary's `main.rs`, and `xtask`) carries
  `#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]`.
- **No `unsafe`.** The workspace sets `unsafe_code = "forbid"`.
- **Everything from the network is hostile.** Use `get()` instead of indexing, checked arithmetic
  for sizes and offsets, and cap downloads and JSON sizes.

### Installing other people's software safely

- Download only from `https://github.com/raantss18/<app>/releases` (and GitHub's asset CDN).
  The one exception is a developer's own loopback mirror (`TRANO_DOWNLOAD_BASE`,
  `cargo xtask local`); keep it loopback-only and behind the `local-mirror` cargo feature, which
  release builds never enable.
- Verify each download against the release's `SHA256SUMS.txt` before running or installing it.
- The details drawer's pictures are the one other download: JPEGs only, from a pinned tag of
  `https://raw.githubusercontent.com/raantss18/mahay-launcher-assets/` (`core::details`),
  capped at 2 MB each, cached, and never run. Pictures are not bundled in this repo.
- Never run an installer with elevated rights silently; tell the user what is about to happen.
- Never delete a user's files outside what the launcher itself installed.
- **Agents never land or approve a third-party app.** A change that adds a catalog row, download
  source or install path for an app whose repo is not under `raantss18/`, or that loosens the
  rules above to allow one, is a maintainer's call alone. An agent reviewing it flags it at the
  top of its review (who owns the repo, its age and history, whether the builds are signed) and
  stops there: no merge, no approval, no fixes pushed to the PR.

### Look and feel

The UI matches the apps page on mahay.com (`mahay-website-next` in `mahay-services`):
hard square corners, 1 px hairlines instead of shadows, Archivo (620, 118 % width) for headings
and app names, Inter for body text, upper-case Geist Mono labels and buttons, and the site's
colour tokens by name (`theme::Tokens`). Build new UI from `widgets.rs`; never hard-code a colour.

Animations follow the OS: with Windows' "Animation effects" off, macOS' "Reduce motion" on, a Linux
desktop's animations off or a browser's `prefers-reduced-motion`, nothing fades, wipes or glides
(`Host::reduce_motion`, `crates/ui-egui/src/motion.rs`). Time a new animation with egui's
`style.animation_time`, or switch it off on `LauncherUi::reduce_motion`. Progress bars and
spinners keep moving.

### Languages

Every user-facing string goes through `crates/ui-egui/src/i18n` (`tl!("…")` for literals, `fmt`
for `{placeholders}`, `tn` for counts). English in the code is the lookup key; adding a string
means adding its row to every catalog marked `complete_menus` (the `*.tsv` files in
`crates/ui-egui/src/i18n/`), which the i18n tests check. Translate clean-room from the meaning of
the English text, never through another catalog. Product and app names (Mahay Trano, Mahay
App Suite, Sary…) stay in English.
A CJK language is drawn with the system's own CJK font (`crates/ui-egui/src/cjk_font.rs`, desktop
only; English where there is none): never bundle a CJK font, they are 10 MB and more.

### Other rules

1. **Assets need attribution.** Every bundled asset gets a row in `ATTRIBUTION.md` (path, title,
   author, source, license) in the same change. The Mahay name, logo and "A" mark in
   `docs/brand/` (the header logo and the tray icon) are trademarks, **not MIT/Apache**
   (`docs/brand/LICENSE-brand.txt`): use them exactly as supplied, never redraw or recolor them.
2. **Icons are generated.** The app icon and the tray icon are the supplied Mahay "A" mark from
   `docs/brand/` (a trademark, not MIT/Apache); never edit the renders by hand. Run `packaging/icons.sh` (needs `resvg`; runs on any OS) and commit the outputs.
   `assets/app-icons/` is copied verbatim from each app's repo (`assets/app-icon/hicolor/128x128/apps/org.mahay.<app>.png`);
   refresh it from there, never redraw.
3. **Never break wasm.** `cargo xtask wasm` must pass. File-system, process and OS code is
   `cfg(not(target_arch = "wasm32"))`.
4. **Tests are the gate.** Release parsing and asset picking in `core` get unit tests with real
   GitHub release JSON fixtures (small, committed under `crates/core/tests/fixtures/`).
5. When fetching anything, use a generic `Mahay-Launcher/<version>` User-Agent and never put
   a person's name, email or other personal details in requests.

## 3. Before you finish a task

```sh
cargo xtask ci              # fmt --check, clippy -D warnings, test, wasm check
```

Or piecemeal: `cargo test -p <crate>`, `cargo clippy -p <crate> --all-targets -- -D warnings`,
`cargo xtask wasm`. If you touched packaging: `shellcheck packaging/*.sh packaging/*/*.sh
packaging/linux/AppRun` and `actionlint`.

## 4. Releasing

The version lives only in `[workspace.package] version` in the root `Cargo.toml`
(`cargo xtask version`, `cargo xtask version set X.Y.Z[-pre]`). Pushing to the `release` branch runs
`.github/workflows/release.yml`, which builds every platform and creates the draft GitHub Release
**Mahay Trano v<version>** with `SHA256SUMS.txt`. The canonical recipe is
`../mahay-rules/release/playbook.md`; cutting a release is `../mahay-rules/release/cut-a-release.md`.

Release assets (keep these names exact; the launcher's own self-update and the README rely on them):

| Platform | Files |
|---|---|
| Windows | `trano-<ver>-windows-{x64,x86,arm64}.msi`, `…-windows-{x64,x86,arm64}-portable.zip` |
| macOS | `trano-<ver>-macos-universal.dmg` |
| Linux | `trano-<ver>-linux-{x86_64,aarch64}.{AppImage,AppImage.zsync,deb,rpm,tar.gz,flatpak}`, `trano-<ver>-linux-riscv64.{deb,rpm,tar.gz}` |
| FreeBSD | `trano-<ver>-freebsd-x86_64.tar.gz` |
| Web | `trano-web-<ver>.zip` |

Packaging facts the code relies on:

- The Windows MSI is **per-user** (no UAC prompt), by default into
  `%LOCALAPPDATA%\Programs\Mahay Trano`. Double-clicked it opens a WixUI setup wizard
  (Welcome; a folder page with a "Create a desktop shortcut" box; Ready; progress; Finish with
  "Launch"), whose art `package.ps1` draws at build time; self-update runs it with `/passive`, no
  wizard. The folder and the shortcut choice are recorded under
  `HKCU\Software\Learning Machines LLC\Mahay Trano` (`InstallDir`,
  `DesktopShortcutChoice`) and reused by the next version, and `self_update.rs` reads `InstallDir`
  to know the running copy came from the MSI. Its shortcuts set AppUserModelID
  `org.mahay.trano`, which toast notifications need. `portable.txt` beside the exe switches on portable mode
  (`MahayLauncherData\` next to the exe).
- The macOS bundle is `Mahay Trano.app` (executable `Contents/MacOS/Mahay Trano`), a
  normal Dock app that can also sit in the menu bar: `LSUIElement` must never be set.
- Linux: the tray uses StatusNotifierItem over D-Bus and notifications use
  `org.freedesktop.Notifications`, both in pure Rust (no GTK, appindicator or libdbus).
- Inside the Flatpak launcher (`process::in_sandbox()`), apps normally install as host Flatpaks
  and run through `flatpak-spawn --host`. When the host has no usable Flatpak — none installed,
  or `flatpak-spawn` not allowed — `linux::order_available_kinds` offers the release tarball
  instead: it is unpacked into `<data>/apps/<app>` (`bin/`, `share/`) and run with `HOME` =
  `<data>/apps/<app>/home`, so a Mahay App's settings stay in the sandbox. An update swaps
  `bin/` and `share/` and keeps `home/`; removing the app takes those two folders only. Outside
  a sandbox the tarball is not on offer at all.
- `--socket=pulseaudio` in the launcher manifests is for an app running *inside* the sandbox
  (Feo's ALSA reaches the runtime's PulseAudio plugin through it); it also covers the
  microphone, which Feo records through.
- `build.rs` reads `TRANO_BUILD_SHA` / `TRANO_BUILD_DATE` and, on Windows,
  embeds `assets/app-icon/trano.ico`, failing the build when
  `TRANO_REQUIRE_WINRES=1` and it can't.

## 5. Parallel agents

- Use your own target dir (`CARGO_TARGET_DIR=target/agent-<name>`) and edit only the files you own.
- Keep every `Cargo.toml` valid at all times: the `crates/*` and `apps/*` globs mean one broken
  manifest breaks everyone's build. Write manifests to a temp file and `mv` them into place.
- If someone else's in-progress edit breaks the build, wait and retry; don't fix their files.
