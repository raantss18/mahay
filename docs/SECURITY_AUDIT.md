# Security audit: releases 0.1.0 and 0.2.0

Date: 2026-10-18. Scope: the code Mahay adds (Android glue, Java shell, vendored backends, build
and release scripts), the dependency trees of all seven Cargo workspaces, and the built APKs.
Upstream code is covered by upstream's own reviews (each app's `SECURITY.md` and `AGENTS.md`
"never panic" rules); this audit didn't re-review it line by line.

## 1. Dependency vulnerabilities (RustSec)

Tool: `cargo audit` (RustSec advisory database, fetched 2026-10-18) on every `Cargo.lock`.

| Advisory          | Crate (version)        | Where                     | Status                                                                                                                         |
|-------------------|------------------------|---------------------------|--------------------------------------------------------------------------------------------------------------------------------|
| RUSTSEC-2026-0346 | rustls 0.23.45         | libs/core, Mahay HUB, Sary    | **fixed**: updated to 0.23.46; Mahay HUB and Sary rebuilt for the release                                                          |
| RUSTSEC-2026-0347 | rustls 0.23.45         | libs/core, Mahay HUB, Sary    | **fixed** (same update)                                                                                                        |
| RUSTSEC-2026-0348 | rustls 0.23.45         | libs/core, Mahay HUB, Sary    | **fixed** (same update)                                                                                                        |
| RUSTSEC-2026-0349 | rustls 0.23.45         | libs/core, Mahay HUB, Sary    | **fixed** (same update)                                                                                                        |
| RUSTSEC-2026-0367 | zip 3.0.0 / 8.6.0      | Soratra, Tabilao, Seho, Mahay HUB | **not reachable**: the advisories are in `ZipArchive::extract` / `ZipStreamReader::extract`; no Mahay or upstream code calls them (`grep -rn '\.extract(' apps libs` finds no zip use); entries are read one by one into memory. Fix needs zip 9 (API break): tracked in TODO |
| RUSTSEC-2026-0368 | zip                    | same                      | not reachable (same reason)                                                                                                    |
| RUSTSEC-2026-0369 | zip                    | same                      | not reachable (same reason)                                                                                                    |
| RUSTSEC-2023-0071 | rsa 0.10.0-rc          | Taratasy (signatures)     | **accepted, no fix upstream**: the Marvin attack needs an attacker who can time many decryptions on demand. Taratasy uses RSA keys locally to sign PDFs the user chose to sign; no service exposes decryption. Revisit when `rsa` ships constant-time decryption |

### 0.2.0: the eight new apps and Mahay HUB

| Advisory                         | Crate (version)          | Where                      | Status                                                                                      |
|----------------------------------|--------------------------|----------------------------|---------------------------------------------------------------------------------------------|
| RUSTSEC-2026-0346..0349          | rustls 0.23.45           | Endrika, Hazavana, Hetsika | **fixed**: 0.23.46                                                                          |
| RUSTSEC-2026-0049, -0098, -0099, -0104 | rustls-webpki 0.102.8 | Hazavana (model downloads), Hetsika (speech models) | **fixed**: the abandoned `rustls-rustcrypto` alpha provider that pulled it was replaced by rustls's `ring` provider |
| RUSTSEC-2026-0194, -0195         | quick-xml 0.38.4         | Hazavana (XMP, lens data)  | **fixed** (after 0.2.0): quick-xml 0.41.0. Hazavana's readers didn't reach either path (plain `Reader`, own duplicate check), but its own namespace resolution was quadratic too (one tag with 100 000 `xmlns:` took 33 s to read, 91 s to merge): now capped at 256 declarations in scope (`TooComplex`); test `crates/meta/tests/hostile_xml.rs` |
| RUSTSEC-2026-0367..0369          | zip 2.4 / 3.0 / 7.2 / 8.6 | Endrika, Pejy, Hazavana, Hetsika, Mahay HUB | not reachable (no `extract()` call), as above |
| RUSTSEC-2023-0071                | rsa 0.9                  | Hazavana, Hetsika          | accepted, as above (no decryption oracle)                                                   |

Warnings (no known exploit): unmaintained `proc-macro-error`, `paste`, `rustybuzz`, `ttf-parser`,
`zbus_names`, `zvariant*` (Linux D-Bus only); unsound `glib 0.18` (Mahay HUB's Linux tray only);
yanked `yoke-derive 0.8.3` (Sary). None of these is linked into the Android builds except the
font crates, which parse only fonts bundled with the apps.

CI now runs `cargo audit` on every workspace (`.github/workflows/ci.yml`, job "Security audit").

## 2. Android attack surface (checked on the built APKs with `aapt2 dump`)

- **Permissions (since 0.2.0):** `INTERNET`, only for automatic updates (GitHub releases of
  `raantss18/mahay`, https, GitHub hosts only), and `REQUEST_INSTALL_PACKAGES`, to hand a verified
  APK to Android's installer (every install is confirmed by the user; Android also asks once to
  allow installs from the app). AndroidX's own `DYNAMIC_RECEIVER_NOT_EXPORTED_PERMISSION`
  (signature-level). No storage, camera, location or contacts permission.
- **Update chain:** `SHA256SUMS.txt` is signed with Ed25519 (private key kept off the
  repository in `release-keys/`); the apps embed the public key and refuse a missing or bad
  signature, a file missing from the list, a hash mismatch, a non-GitHub or non-https URL and any
  version not newer than the installed one. The installer only accepts `.apk` files inside the
  app's own storage (canonical path check), and the install-status receiver is not exported.
- **Exported components:** `MahayActivity` (launcher, required); AndroidX `ProfileInstallReceiver`
  (guarded by `android.permission.DUMP`, system only). AndroidX's startup provider is not
  exported.
- **Intents into the exported activity** (`onNewIntent`/launch intent): only `content://` URIs
  are read. A `file://` URI (for example one pointing into the app's private storage, sent by
  another app) is ignored. **Found and fixed during this audit** (`MahayActivity.isContentUri`).
- **Write-back:** `writeBack` only copies files from the app's own `files/docs/` folder
  (canonical path check, so `..` can't escape) and only to `content://` URIs the user granted
  through the system picker.
- **Display names** from providers are reduced to their last path segment; `.`/`..`/empty
  become `document`, so a malicious provider can't write outside the per-document folder.
- **Links:** `openUrl` opens only `http`, `https` and `mailto`.
- **Backups:** `allowBackup=true` for settings, but the backup rules (`res/xml/
  data_extraction_rules.xml` for Android 12+, `backup_rules.xml` before) leave out the local
  copies of opened documents (`files/docs/`), update downloads and their state
  (`files/updates/`, `files/update.json`) and temporary files (`files/cache/`). The originals
  stay where the user keeps them; a copy in a backup could leak a private document.
- **Old copies:** copies in `files/docs/` untouched for 30 days are deleted at start-up, except
  open ones and Save As files never placed outside the app (kept with a `<n>.unsaved` marker).
- **Debuggable:** no (release builds), R8 minification on.

## 3. Native code added by Mahay

`unsafe` in Mahay's own code is limited to:

| Place                                    | Why                                                                                   |
|------------------------------------------|---------------------------------------------------------------------------------------|
| `#[unsafe(no_mangle)] android_main` ×6   | the entry symbol android-activity calls                                               |
| `#[unsafe(no_mangle)]` JNI callbacks ×2  | `nativeOnOpened`, `nativeOnExported`                                                  |
| `JavaVM::from_raw`, `JObject::from_raw`  | pointers owned by android-activity, valid for the app's lifetime                     |
| `std::env::set_var` in `init`            | runs first in `android_main`, before Rust starts other threads                        |

JNI calls never panic across the boundary: every error is logged, a pending Java exception is
described and cleared, and the call returns `None`. Answers from Java go through channels; the
app thread never blocks on Java (no deadlock with the activity lifecycle).

## 4. Secrets and supply chain

- The release keystore lives in `~/.mahay/` (mode 600), outside the repository;
  `android/keystore.properties` is a git-ignored symlink. `git grep` finds no private key,
  password or token in the repository (the `.p12`/`.pem` files are upstream's public test
  fixtures for signature verification).
- Build tools fetched for packaging come from their official GitHub releases: nfpm (checksum
  verified against `checksums.txt`), appimagetool (Sary's script verifies the pinned 1.9.1 by
  SHA-256).
- Vendored crates (`rfd`, `arboard`, `krilla`, `hayro`) are copies of the published versions
  plus the Android backends; their diffs are in `libs/core/vendor/*/src/**/android.rs`.

## 5. What this audit did not cover

- A penetration test of upstream's parsers (PDF, DOCX, XLSX, PPTX, PSD, raw). Upstream fuzzes
  them (`cargo xtask fuzz`); Mahay doesn't change them.
- Windows and macOS builds.
- A real-device test of the SAF permission flows across providers (Drive, USB).
