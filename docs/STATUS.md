# Status

Last updated: 2026-10-19 (release 0.2.0 in preparation, alpha). Only what was actually run is marked as
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
| Phone layout by window width (D23), Soratra, Tabilao, Taratasy | x86_64 emulator `mahay_phone` (1080×2400 at 420 dpi = **411 dp** wide), 0.2.3 branch: touch title bar, scrolling tab strip with a menu, scrolling ribbon, status bar; Soratra new document, Tabilao empty book, Taratasy Home and mode bar. Landscape (914 dp) gives the desktop layout. Screenshots: `docs/img/phone-*.png` |
| Phone layout, Seho                                | built and started on the phone emulator: title bar and Home screen seen; editor, tab row and slide drawer **only tested headlessly** (`tests/phone_layout.rs`) |
| Finger scroll in Soratra (0.2.3 branch)           | `mahay_phone` emulator: a swipe (`adb input swipe`) scrolls the page and selects nothing; `tests_phone::a_finger_drag_scrolls_the_page_and_does_not_select` failed without the fix |
| Layout audit at 360, 400, 600 and 840 pt          | headless (egui_kittest), one test per app on every tab or mode: Soratra, Tabilao, Seho, Taratasy. Each failed on the old layout (70 to 580 findings) and passes now |
| Save As to shared storage (Soratra)               | File ▸ Save As ▸ Browse opens Android's picker; `Document1.docx` written to Downloads with the typed text |
| Write-back to the original document               | edit, File ▸ Save: the copy in Downloads gets the new paragraph                            |
| Release signing                                   | local keystore (`~/.mahay/release.jks`, outside git); `apksigner verify` on the APKs: SHA-256 `1e0abb5d…c777` |
| Seho on Android tablet                            | x86_64 emulator: Home screen, new presentation from a theme, editor with ribbon and slide panel |
| Sary on Android tablet                            | launched from Mahay HUB; New Document dialog; drawing with touch on the canvas                |
| Mahay HUB on Android tablet                           | lists the suite, sees installed apps (4/5), opens Sary                                    |
| Linux builds of all six apps                      | GitHub Actions CI on Ubuntu 24.04 (green), and local release builds                       |
| Linux packages                                    | AppImage and .deb for all six apps; Tabilao's AppImage extracted and `--version` run     |
| Security audit                                    | `cargo audit` on all workspaces, APK manifests checked: see `docs/SECURITY_AUDIT.md`     |
| The eight later apps on Android tablet (0.2.0)    | Tsilo, Endrika, Pejy, Hazavana, Feo, Hetsika, Tselatra, Drafitra: x86_64 emulator, each starts on its home screen, no crash in the log |
| Mahay HUB 0.2.0 on Android tablet                 | lists the 13 apps, sees all 13 installed, Open on each                                   |
| Linux packages of all 14 apps (0.2.0)             | `scripts/linux-build.sh`: AppImage and .deb for each                                      |
| Typing latency on Android (after the fix)        | x86_64 tablet emulator, Soratra and Tabilao: each letter shows in the next frame, the keyboard opened by Android or by a tap; Backspace, Enter, dismiss and reopen |
| Translation catalogues after the relabel          | `cargo test -p <app>-ui-egui -- i18n` green in all 13 ui crates                           |

## Built, not yet exercised on a device

- arm64-v8a APKs: built and signed; the emulator here is x86_64, so they ran on no device.
  **A test on a real phone and tablet is the next step.**
- Linux AppImage/.deb packaging: upstream scripts, renamed; produced by the release workflow.
- Direct install and self-update from Mahay HUB: unit-tested (signatures, versions), needs a
  published 0.2.0 to exercise end to end.
- "Open with" on Android for the eight later apps: intent filters generated, not yet tried.
- Touch pressure on Android (next version): the shell tells Rust each pointer's tool type, and
  a finger now draws at pressure 1.0 while a stylus or its eraser keeps its real pressure. Built
  (`mahay-android` and the Sary APK for x86_64), not tried with a finger or a stylus on a device.
- Backups (next version): Android backups and device transfers leave out the document copies
  (`files/docs`), update downloads and temporary files. Built, no backup or restore run yet.
- Old document copies (next version): copies untouched for 30 days are deleted at start-up on a
  worker thread, except open ones and unplaced Save As files. Unit-tested on Linux; not yet run
  on a device.

## Not built here

- Windows and macOS: code paths untouched, no toolchains in this environment.
- The ArtCraft AI app (Tauri + React), cloud sync (0.3.0, `docs/SYNC.md`), live sync (phase C)
  and the model hub (phase D).

## Known gaps

| Area                     | Gap                                                                                                  | Plan                                                       |
|--------------------------|------------------------------------------------------------------------------------------------------|------------------------------------------------------------|
| Phones (compact width)   | Desktop ribbons (Soratra, Tabilao, Seho) are clipped on the right in portrait                         | a compact ribbon/toolbar per app; landscape works better meanwhile |
| APK size (arm64)         | Sary 23.2, Taratasy 17.6, Soratra 17.1, Seho 12.2, Tabilao 8.5, Mahay HUB 4.8 MB                         | optional OCR/JS features off on Android, `cargo bloat`     |
| Phone: File screens      | Soratra's File screen and Seho's Home are the desktop screens: the left menu is 435 px (Soratra) and 350 px (Seho) wide on a 411 dp screen, the content is cut; Soratra's page opens at 100%, wider than the phone | adapt the backstage views and fit the page to the width in Compact |
| Touch: finger drag       | seen on the phone emulator: in **Tabilao** a swipe over the grid selects cells (D1:D20) instead of scrolling; the other apps are not looked at yet (a static survey finds tooltips used as the only label in every app: they do not show on touch) | one rule per app: finger = scroll or pan, long press/stylus/mouse = select; audit all 14 on the emulator (`TODO.md`) |
| Phone: touch targets     | the ribbon's small buttons are about 22 dp high (bars, tabs and menus are 44–48 dp); only Soratra and Tabilao were looked at in portrait and landscape; Medium (600–839 dp) only headless | stack fewer small buttons per column in Compact; a tablet in portrait for Medium |
| Phone: other apps        | audited on the phone emulator (411 dp), 2026-10-20 (`docs/img/phone-audit-desktop-layouts.png`): **Sary** desktop layout, three panel columns leave ~100 px for the canvas, menu bar cut off; **Tsilo** and **Pejy** desktop toolbars and panels, the welcome text wraps one letter per line; **Hazavana** library and panels squeezed, buttons overlap; **Feo** transport and mixer cut off; **Endrika** starts on a half-adapted home (panels at the right edge). **Hetsika, Tselatra, Drafitra**: built, not looked at yet. **Taratasy** (Home scrolls by finger) and **Seho** (editor) are fine. Five Taratasy tests (About dialog, Ukrainian) already failed before this change | per `TODO.md`: Sary next, then the others |
| Dialogs and keyboard     | a dialog doesn't move above the soft keyboard                                                        | apply the keyboard inset to egui windows                   |
| Folders                  | folder pickers (batch export, OCR to a folder, pinned folders) answer "cancelled" on Android         | `ACTION_OPEN_DOCUMENT_TREE` support in `mahay-android`     |
| Printing                 | no printing on Android                                                                               | Android print framework via the shell                      |
| Clipboard                | text only on Android (no images or rich text)                                                         | `ClipData` with HTML/URI                                   |
| Stylus                   | finger and stylus pressure built, never tried with a real stylus on Android                          | test on a tablet with a pen                                 |
| Translations             | community labels in other languages were filled from related strings ("GitHub" for Discord)          | review French and Malagasy wording                          |
| Upstream tests           | translation tests green; some UI tests still assert upstream links, social accounts and app order    | fix per app, then run per crate in CI                       |

## Next steps

1. Run the arm64 APKs on a real Android phone and tablet; fix what breaks.
2. Compact (phone) layouts: Soratra, Tabilao, Seho and Taratasy done; the other nine apps next.
3. Shrink Taratasy and Soratra.
4. Import the next apps (Tsilo, Endrika, Pejy).
