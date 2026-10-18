# Status

Last updated: 2026-10-18 (release 0.1.0, alpha). Only what was actually run is marked as
verified; everything else says how far it was checked.

## Verified

| What                                              | How                                                                                         |
|---------------------------------------------------|---------------------------------------------------------------------------------------------|
| Upstream PdfCraft builds on Linux (baseline)      | `cargo build --release -p pdfcraft` before renaming: 9 min, OK                              |
| Tabilao builds and starts on Linux after renaming | `cargo build --release -p tabilao -p tabilao-cli`; `tabilao --version` → `Tabilao 0.8.0`    |
| All six Android crates compile for arm64-v8a      | `cargo ndk -t arm64-v8a check -p <app>-android`                                             |
| Tabilao on Android tablet (2560×1600, API 36)     | x86_64 emulator: starts, ribbon below the status bar, typing `Mahay` and `=21*2` → `42`, Enter moves down |
| Tabilao on Android phone (1080×2400)              | x86_64 emulator: starts, grid and formula bar usable; ribbon clipped (see known gaps)       |
| Taratasy on Android tablet                        | x86_64 emulator: starts on its Home screen                                                  |
| Soratra on Android tablet                         | x86_64 emulator: new blank document, typing with the soft keyboard (spell check works), Enter makes a paragraph, the page moves above the keyboard |
| Save As to shared storage (Soratra)               | File ▸ Save As ▸ Browse opens Android's picker; `Document1.docx` written to Downloads with the typed text |
| Write-back to the original document               | edit, File ▸ Save: the copy in Downloads gets the new paragraph                            |
| Release signing                                   | local keystore (`~/.mahay/release.jks`, outside git); `apksigner verify` on the APKs: SHA-256 `1e0abb5d…c777` |
| Seho on Android tablet                            | x86_64 emulator: Home screen, new presentation from a theme, editor with ribbon and slide panel |
| Sary on Android tablet                            | launched from Trano; New Document dialog; drawing with touch on the canvas                |
| Trano on Android tablet                           | lists the suite, sees installed apps (4/5), opens Sary                                    |
| Linux builds of all six apps                      | GitHub Actions CI on Ubuntu 24.04 (green), and local release builds                       |
| Linux packages                                    | AppImage and .deb for all six apps; Tabilao's AppImage extracted and `--version` run     |
| Security audit                                    | `cargo audit` on all workspaces, APK manifests checked: see `docs/SECURITY_AUDIT.md`     |

## Built, not yet exercised on a device

- arm64-v8a APKs: built and signed; the emulator here is x86_64, so they ran on no device.
  **A test on a real phone and tablet is the next step.**
- Linux AppImage/.deb packaging: upstream scripts, renamed; produced by the release workflow.

## Not built here

- Windows and macOS: code paths untouched, no toolchains in this environment.
- The eight later apps (Tsipika, Endrika, Pejy, Hazavana, Feo, Hetsika, Tselatra, Drafitra):
  inventoried (`docs/APPS_INVENTORY.md`), not imported yet.
- The ArtCraft AI app (Tauri + React), sync (phase C) and the model hub (phase D).

## Known gaps

| Area                     | Gap                                                                                                  | Plan                                                       |
|--------------------------|------------------------------------------------------------------------------------------------------|------------------------------------------------------------|
| Phones (compact width)   | Desktop ribbons (Soratra, Tabilao, Seho) are clipped on the right in portrait                         | a compact ribbon/toolbar per app; landscape works better meanwhile |
| APK size (arm64)         | Sary 23.2, Taratasy 17.6, Soratra 17.1, Seho 12.2, Tabilao 8.5, Trano 4.8 MB                         | optional OCR/JS features off on Android, `cargo bloat`     |
| Touch drawing            | a finger draws with near-zero pressure in Sary (thin lines)                                          | pressure 1.0 for fingers, real pressure for a stylus       |
| Dialogs and keyboard     | a dialog doesn't move above the soft keyboard                                                        | apply the keyboard inset to egui windows                   |
| Folders                  | folder pickers (batch export, OCR to a folder, pinned folders) answer "cancelled" on Android         | `ACTION_OPEN_DOCUMENT_TREE` support in `mahay-android`     |
| Printing                 | no printing on Android                                                                               | Android print framework via the shell                      |
| Clipboard                | text only on Android (no images or rich text)                                                         | `ClipData` with HTML/URI                                   |
| Stylus                   | pressure isn't passed to Sary's brushes on Android                                                    | read pressure from the motion events                        |
| Soft keyboard            | the app repaints every 80 ms while the keyboard is open (winit doesn't wake it for IME text)          | upstream fix in winit, then drop the polling               |
| Translations             | strings changed for Mahay (Discussions instead of Discord) fall back to English in other languages   | update the catalogues                                       |
| Upstream tests           | not run after the rename                                                                              | run per crate in CI                                         |

## Next steps

1. Run the arm64 APKs on a real Android phone and tablet; fix what breaks.
2. Compact (phone) layouts for the ribbon apps.
3. Shrink Taratasy and Soratra.
4. Import the next apps (Tsipika, Endrika, Pejy).
