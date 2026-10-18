# Changelog

All notable changes to Mahay. Format: [Keep a Changelog](https://keepachangelog.com/);
versions follow [SemVer](https://semver.org/).

## [0.1.0] — 2026-10-18 (alpha)

First release of Mahay, a fork of the ArtCraft crafting apps.

### Added

- **Android (phone and tablet, Android 8+)** for Sary, Taratasy, Soratra, Tabilao and Seho, and
  a new Android hub, Trano. One APK per app (arm64-v8a, x86_64).
- Android glue (`mahay-android`): system document picker with write-back to the original
  document, Save As to any location, soft keyboard with full IME text (accents, suggestions),
  Android clipboard, edge-to-edge layout that avoids the system bars and the keyboard, bar
  colours that follow the app theme, opening documents shared from other apps.
- Android backends for `rfd` (file dialogs) and `arboard` (clipboard), vendored in
  `libs/core/vendor`.
- The Mahay icon family and wordmark (`docs/BRAND.md`), generated from one script.
- Trilingual README (English, French, Malagasy); Trano's Android hub in the three languages.
- CI for Linux and Android; tagged releases build signed APKs, AppImages and .deb packages.

### Changed

- Every app renamed: PhotoCraft → Sary, PdfCraft → Taratasy, WordCraft → Soratra,
  GridCraft → Tabilao, DeckCraft → Seho, the launcher → Trano. Ids are `org.mahay.<app>`.
- Trano reads GitHub releases of `raantss18/mahay` directly.

### Removed

- Upstream branding (names, logos, app icons, screenshots), social links, Discord links and the
  upstream release-feed server.

### Known gaps

See [docs/STATUS.md](docs/STATUS.md).

Upstream commits this release is based on: [docs/APPS_INVENTORY.md](docs/APPS_INVENTORY.md).
