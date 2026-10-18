# Changelog

All notable changes to Mahay. Format: [Keep a Changelog](https://keepachangelog.com/);
versions follow [SemVer](https://semver.org/).

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
