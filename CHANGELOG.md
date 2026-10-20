# Changelog

All notable changes to Mahay. Format: [Keep a Changelog](https://keepachangelog.com/);
versions follow [SemVer](https://semver.org/).

## [Unreleased]

### Added

- **Feedback** in every app's About page: a short message (problem, idea, question, thanks), an optional
  contact and the device facts shown before sending, delivered to the author's Supabase project (D28).
- **Help improve Mahay** (off by default): anonymous events (opened once a day, crash, native crash, app
  not responding, killed for memory, update failed) with the phone model and Android version. On Android 11+
  an app reads why its previous run ended; a panic is kept for the next start. This is how "Hetsika does not
  open on some phones" becomes a model, a version and a reason. Never documents, file names or text.
- `docs/FEEDBACK_SETUP.md` (the SQL to run once in Supabase), privacy policy updated (en, fr).

- **Sary: Simple workspace** (D27), the default on a phone or tablet, with a **Pro** button for the full
  workspace (and a small **Simple** button back). Six labelled tasks: **Adjust** (Light, Contrast, Color,
  Warmth sliders, Enhance, Reset), **Filters** (eight looks), **Crop** (shapes around the center, rotate,
  flip), **Draw** (brush, eraser, size, ten colors), **Text** (tap, type, size, color) and **Retouch** (spot
  heal, remove, red eye). Adjust and Filters are non-destructive: they drive four named adjustment layers
  ("Simple: Light"...), one undo step per drag, and holding the eye shows the original. The layout follows
  the window: tasks along the bottom on a narrow window, a rail on the left and the controls on the right
  on a wide one, scrolling when the keyboard takes the room. Texts in English, French and Malagasy.

### Fixed

- **Links in About did nothing on Android** (GitHub, releases, author, discussions, privacy): the Android bridge
  dropped egui's "open link" command. The activity opens them (http, https and mailto only).
- Discussions are enabled on the repository (the Discussions links answered 404).
- `apps/tsilo/crates/text/build/web_fonts.rs` was ignored by git (`build/`), so a clean checkout could not
  build Tsilo.

## [0.2.4] — 2026-10-20 (alpha)

### Fixed

- **Mahay HUB: "Install all" no longer stops after the first app.** The queue only moves on when the
  hub draws a frame, and nothing woke the hub after Android's install window closed, so the second app
  never started (on some phones it started and Android's window was then refused from the background,
  so the card stayed on "Confirm the installation" with no window). The hub is now woken as soon as it
  is back in front, keeps ticking while an install is under way, opens each confirmation window only
  from the foreground, and the card has a **"No window? Tap here to continue"** button. Found from a
  report on a real phone and reproduced on the emulator (1 of 4 confirmation windows before, 4 of 4
  after).

## [0.2.3] — 2026-10-20 (alpha)

### Added

- **Mahay HUB manages the other apps (D21).** On Android 12+ the hub, as the installer of record,
  updates the apps it installed **without asking you to confirm each one**. Mahay HUB gets
  "Update automatically" (on by default: at start, every day, and in the background when the hub
  is closed, kept across reboots), "Also on mobile data" (off by default: only Wi-Fi and other
  unmetered networks), a notification "N updates ready" when Android still needs a tap (asked
  from the settings, never at start-up), and **Install all**, a queue that installs the suite one
  app after the other. An app that is open is left alone by the background update.
- **An app hands its update to Mahay HUB** when the hub is installed and signed with the same
  certificate: "Install" in the update bar says "Updating through Mahay HUB…" and the app doesn't
  download the update a second time. Without the hub, every app still updates itself as before.
- English, French and Malagasy texts for all of it (Malagasy to be reviewed, `docs/A_VALIDER.md`).
- **"Open with" finds Mahay for more files (D22).** Each app is now also offered for its documents by
  **file extension** when a file manager reports no type or a generic one (IDML, DXF, RAW photos,
  `.tex`, ...), in addition to the MIME type. A new test (`scripts/android-open-with-test.py`) asks
  Android which apps answer each case and fails if an app drops out or claims files that are not its own.
- **Phone and tablet layouts that follow the window width, with no setting** (D23). Compact
  (< 600 dp), medium (600–839) and expanded (≥ 840, the desktop layout, unchanged) are worked out
  every frame, so rotation, split screen and floating windows change the layout by themselves.
  Done in **Soratra, Tabilao, Seho and Taratasy**: touch-sized title bar, scrolling tab strip with
  the rarely used buttons in one menu, scrolling ribbon, status bar and mode bar, side panels and
  the slide list as full-window drawers, full-screen dialogs in Soratra. The ribbons no longer
  overlap or leave the screen. The other apps follow. Shared piece: `mahay_ui::adaptive`
  (`FormFactor`, a scrolling command bar, a layout audit used by one test per app at 360, 400,
  600 and 840 points).

### Fixed

- **Android: no more crash at launch on some devices.** Asking wgpu for Vulkan and OpenGL ES together
  made the OpenGL surface fail on an Android 10 emulator (`BadAlloc`, then a panic) and the app
  closed at every start. Apps now start on OpenGL ES alone before Android 11, remember which backend
  drew a frame, try the next one after a start that did not, and show a message instead of closing
  silently when none works. One fix for all 14 apps (`mahay_android::graphics`).
- **Android: no empty bands.** The system bars were taken off twice on Android before 15 and in
  split-screen or free-form windows; the app now counts only what overlaps it, reads it again when the
  window is resized, and draws under the display cutout. Checked live on the phone emulator at 20:9,
  21:9, 22:9, 1:1 and tablet sizes.
- **Mahay HUB (Android): the grid of apps no longer runs past the right edge** on a tablet; the tiles
  share the window's width and their two buttons share the tile's.
- **Sary on a phone**: the menus scroll, the status bar is touch sized, and the panels open as a
  drawer instead of leaving ~100 px for the canvas (the toolbar is unchanged for now).
- **Soratra on a phone: a finger drag scrolls the page** (with coasting) instead of selecting text;
  a long press first still selects. A solid scroll bar a finger can grab shows on narrow windows.
  **Tabilao**: a finger drag scrolls the sheet (a tap still selects a cell; ranges by finger come later). The other apps are being checked (see `TODO.md`).

### Changed

- **Seho**: the ribbon scrolls instead of cutting off groups that do not fit the window.

### Security

- New signature-level permission `org.mahay.permission.MANAGE` guards the hub's only exported
  component; the hub's job and install receiver are not exported. Refused from `adb shell`
  (`docs/SECURITY_AUDIT.md`).

## [0.2.2] — 2026-10-19 (alpha)

### Added

- **`mahay-sync`, the core of 0.3.0's cloud sync** (not in any app yet): end-to-end encryption
  (PBKDF2-HMAC-SHA256 passphrase key with a check block, AES-256-GCM in 1 MiB chunks, encrypted
  names and manifest), a Nextcloud/WebDAV backend, and a sync engine with an offline journal,
  conflict copies `name (conflit <device> <date>).ext` and a 30-day sync bin. Sign-in (Supabase)
  and Google Drive come next. Protocol: `docs/SYNC.md`.

### Security

- **Hazavana**: quick-xml 0.41 fixes RUSTSEC-2026-0194 and RUSTSEC-2026-0195 (denial of service on
  malicious XML), and CI no longer ignores them. An XMP sidecar or embedded packet declaring
  thousands of namespaces no longer stalls reading or saving metadata for minutes: more than 256
  declarations in scope is refused as too complex (the sidecar is backed up before it's replaced).

### Changed

- **Android: a finger draws at full pressure.** Sary drew thin lines with a finger, whose
  pressure Android reports near zero; a stylus and its eraser keep their real pressure.
- **Android backups leave out document copies**, update downloads and temporary files (Android 12+
  and older); settings are still backed up.

### Fixed

- **Android: old document copies are cleaned up.** Copies in the app's storage untouched for
  30 days are deleted at start-up, in the background; open documents and Save As files not yet
  placed outside the app are kept.

## [0.2.1] — 2026-10-19 (alpha)

### Added

- **Taratasy plays animated PDFs** made with LaTeX's `animate` package (Beamer slides): a tap
  or click plays and pauses, and the control bar under the animation (play, pause, step,
  first and last frame, speed) works. Each frame is drawn once, in the background, so playing
  stays smooth on a tablet. These PDFs played nowhere but in Acrobat, because the animation is
  JavaScript; Taratasy plays them without running it.

### Fixed

- **Typing on Android shows each letter at once**, in every app. When Android opened the
  keyboard by itself (a document's field focused when it opens), the app ignored what was typed
  until the next tap, then showed it all at once. The app now follows the keyboard it is given,
  and each keyboard change wakes it immediately instead of a check every 80 ms (also kinder to
  the battery). Enter makes a new line or paragraph however the keyboard was opened.

## [0.2.0] — 2026-10-18 (alpha)

### Added

- **Eight more apps**, on Linux and Android: Tsilo (vector illustration), Endrika (interface
  design), Pejy (page layout), Hazavana (photo library and raw), Feo (audio), Hetsika (video),
  Tselatra (motion graphics) and Drafitra (CAD). Mahay now has 14 apps.
- **Mahay HUB installs everything**: "Download" fetches an app's APK straight from the release,
  checks it and opens Android's installer, without going through GitHub's page. "Update" and
  "Update all" keep every app current. Mahay HUB is the only file to download by hand.
- **Automatic updates in every app**: checked at start-up and every day, downloaded in the
  background, then a bar offers "Install" with the release notes. On Linux the AppImage replaces
  itself and restarts.
- **Signed releases**: `SHA256SUMS.txt` is signed with Mahay's Ed25519 key; apps refuse
  unsigned, altered or older files.
- **A new About page in every app**: version, updates, QR code to install Mahay on another
  device, author, licences, in English, French and Malagasy.

### Changed

- One version for the whole suite (0.2.0); release files keep stable names.
- Android permissions: `INTERNET` (updates) and `REQUEST_INSTALL_PACKAGES` (installing apps and
  updates), both explained in `docs/SECURITY_AUDIT.md`.
- Releases are published as "latest" (not pre-releases), so apps can find them.

### Fixed

- Endrika no longer connects to the upstream collaboration server: online sharing stays off
  until you enter a server of your own.
- Links that led to pages that do not exist (an app's own repository or page) now open Mahay's
  repository; "Discord" and "mahay.com" labels now say Discussions, GitHub or Mahay HUB.
- Security: rustls 0.23.46, and the new apps use rustls's `ring` provider (the vulnerable
  `rustls-webpki` 0.102 is gone).

### Removed

- Upstream names, links and contributor lists from the About pages.

## [0.1.0] — 2026-10-18 (alpha)

First release of Mahay, a fork of the ArtCraft crafting apps.

### Added

- **Android (phone and tablet, Android 8+)** for Sary, Taratasy, Soratra, Tabilao and Seho, and
  a new Android hub, Mahay HUB. One APK per app (arm64-v8a, x86_64).
- Android glue (`mahay-android`): system document picker with write-back to the original
  document, Save As to any location, soft keyboard with full IME text (accents, suggestions),
  Android clipboard, edge-to-edge layout that avoids the system bars and the keyboard, bar
  colours that follow the app theme, opening documents shared from other apps.
- Android backends for `rfd` (file dialogs) and `arboard` (clipboard), vendored in
  `libs/core/vendor`.
- The Mahay icon family and wordmark (`docs/BRAND.md`), generated from one script.
- Trilingual README (English, French, Malagasy); Mahay HUB's Android hub in the three languages.
- CI for Linux and Android; tagged releases build signed APKs, AppImages and .deb packages.

### Changed

- Every app renamed: PhotoCraft → Sary, PdfCraft → Taratasy, WordCraft → Soratra,
  GridCraft → Tabilao, DeckCraft → Seho, the launcher → Mahay HUB. Ids are `org.mahay.<app>`.
- Mahay HUB reads GitHub releases of `raantss18/mahay` directly.

### Removed

- Upstream branding (names, logos, app icons, screenshots), social links, Discord links and the
  upstream release-feed server.

### Known gaps

See [docs/STATUS.md](docs/STATUS.md).

Upstream commits this release is based on: [docs/APPS_INVENTORY.md](docs/APPS_INVENTORY.md).
