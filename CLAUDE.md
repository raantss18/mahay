# CLAUDE.md

Guidance for Claude Code (and other agents) working in this repository.

## What this is

**Mahay**: a fork of the ArtCraft crafting apps, renamed (Malagasy names) and ported to Android
phones and tablets, plus Linux. Native Rust (egui/eframe + wgpu), no WebView. Trano (hub) + 13 apps; one version for
the suite (`VERSION`), automatic signed updates (`libs/core/crates/update`), shared About page
and update bar (`libs/core/crates/about`). Trano is the only file users download by hand. Read `docs/DECISIONS.md`,
`docs/STATUS.md` and `docs/PORTING_AUDIT.md` before changing anything structural.

## Layout

```
apps/<app>/                 one upstream app, renamed; its own Cargo workspace and Cargo.lock
apps/<app>/AGENTS.md        upstream's rules for that app: follow them inside that tree
apps/<app>/apps/<app>-android/   the Android entry point (cdylib, android_main)
libs/core/                  shared crates (mahay-ui, -print, -quit, ...) and mahay-android
libs/core/vendor/{rfd,arboard}   third-party crates patched with Android backends
android/                    Gradle project: one flavour per app; android/shell = MahayActivity
brand/                      icon family generator (generate.py) and applier (apply.py)
scripts/rebrand.py          the upstream → Mahay rename; rerun it after importing upstream
scripts/android-build.sh    Rust .so + Gradle APK/AAB per app
scripts/bump-version.py     one version for every app
scripts/stage-release.sh    release files, signed SHA256SUMS.txt, release page (release-notes.py)
release-keys/               signing keys: LOCAL ONLY, git-ignored, never push
docs/                       DECISIONS, STATUS, PORTING_AUDIT, APPS_INVENTORY, BRAND, BUILDING
```

## Commands

```bash
cd apps/<app> && cargo build --release -p <app>          # Linux desktop
scripts/android-build.sh --abi x86_64 <app>              # Android APK for the emulator
python3 -I brand/generate.py && python3 -I brand/apply.py
```

Scope cargo builds and tests to the crate you touched (`-p`, `-j 8`): the full workspaces of
Sary and Taratasy are very large.

## Rules

- **Legal:** never edit `LICENSE-*`, `NOTICE` or `ATTRIBUTION*` text of upstream trees except to
  add our own lines. No ArtCraft name, logo, domain, social link or backend anywhere except the
  "Forked from ArtCraft" attribution and links to upstream issues.
- **Android:** never block the app thread waiting for Java (pickers answer through `poll()` or a
  channel waited on by a worker thread). Keep desktop code paths unchanged; gate Android code
  with `cfg(target_os = "android")`.
- **Light:** no new heavy dependency without measuring the APK size change (`dist/android`).
- **Docs:** keep `docs/STATUS.md` true (never claim something works without running it), add a
  `docs/DECISIONS.md` entry for each structural choice, update `CHANGELOG.md`.
- **i18n:** user-facing text in English, French and Malagasy where Mahay adds text.
- Commits: Conventional Commits; never push to `main` with a failing build.
