# Changelog

All notable changes to Mahay. Format: [Keep a Changelog](https://keepachangelog.com/);
versions follow [SemVer](https://semver.org/).

## [Unreleased]

### Added

- **Phone and tablet layouts that follow the window width, with no setting** (D23). Compact
  (< 600 dp), medium (600–839) and expanded (≥ 840, the desktop layout, unchanged) are worked out
  every frame, so rotation, split screen and floating windows change the layout by themselves.
  Done in **Soratra, Tabilao, Seho and Taratasy**: touch-sized title bar, scrolling tab strip with
  the rarely used buttons in one menu, scrolling ribbon, status bar and mode bar, side panels and
  the slide list as full-window drawers, full-screen dialogs in Soratra. The ribbons no longer
  overlap or leave the screen. The other apps follow. Shared piece: `mahay_ui::adaptive`
  (`FormFactor`, a scrolling command bar, a layout audit used by one test per app at 360, 400,
  600 and 840 points).

### Changed

- **Seho**: the ribbon scrolls instead of cutting off groups that do not fit the window.

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
