# Decisions

Each entry: the decision, the alternatives considered, and why. Newest last.

## D1 — Project name: Mahay

- **Decision:** the project is called **Mahay** (Malagasy: "skilled, able, knows how to do").
  Bundle/package prefix `org.mahay.*`, crates `mahay-*`, env vars `MAHAY_*`.
- **Alternatives:** Tantely (honey), Lalana (road), Vorona (bird), Tanana (hand), Asatanana
  (handicraft).
- **Why:** short, easy to say in French, English and Malagasy, and it says what the suite is
  for: making things.
- **Collision check (2026-10-18):**

  | Registry / place     | Result for `mahay`                                                     |
  |----------------------|------------------------------------------------------------------------|
  | crates.io            | free (404)                                                             |
  | npm                  | free (404)                                                             |
  | F-Droid              | no app found                                                           |
  | Flathub              | no app found                                                           |
  | GitHub               | only small personal repos (0–3 stars), none in this field              |
  | Domains              | mahay.app, mahay.org and mahay.dev are already registered by others   |
  | Trademark            | no software trademark found in a quick web search; not a legal opinion |

  Without a domain, the reverse-DNS id `org.mahay` is used only as an app-id namespace. Change
  it before publishing to a store if that ever matters.

## D2 — App names: one Malagasy word per app

| Upstream    | Mahay name     | Meaning              | Role                       |
|-------------|----------------|----------------------|----------------------------|
| launcher    | **Mahay HUB**  | the suite's hub      | hub / launcher             |
| PhotoCraft  | **Sary**       | picture              | image editor               |
| PdfCraft    | **Taratasy**   | paper, document      | PDF workbench              |
| WordCraft   | **Soratra**    | writing              | word processor             |
| GridCraft   | **Tabilao**    | table                | spreadsheet                |
| DeckCraft   | **Seho**       | show, display        | presentations              |
| VectorCraft | **Tsilo**      | lamp, torch          | vector illustration        |
| FigCraft    | **Endrika**    | shape, form          | UI design / prototyping    |
| DesignCraft | **Pejy**       | page                 | page layout                |
| LightCraft  | **Hazavana**   | light                | photo library + raw        |
| SoundCraft  | **Feo**        | voice, sound         | DAW                        |
| FilmCraft   | **Hetsika**    | movement             | video editor               |
| EffectCraft | **Tselatra**   | lightning, flash     | motion graphics / VFX      |
| CADCraft    | **Drafitra**   | plan, blueprint      | CAD / drafting             |

- **Alternatives:** keep a common suffix (as "…Craft" did).
- **Why:** the prompt forbids keeping the "…Craft" names. Single words are shorter on a phone
  home screen. "Mahay Sary" is the full name where context is needed.

## D3 — Release 1 scope: hub + five tablet apps

- **Decision:** release 1 ships Mahay HUB + Sary, Taratasy, Soratra, Tabilao, Seho on Linux and
  Android (phone + tablet). The other eight apps follow in later releases. The original
  ArtCraft AI desktop app (Tauri + React) is not in release 1.
- **Alternatives:** hub + 1–2 apps; fork-only.
- **Why:** the user chose it (2026-10-18). Those five are the most useful on a tablet.

## D4 — Monorepo of upstream snapshots (not submodules, not full history)

- **Decision:** one repo. Each upstream repo is imported as a snapshot of its tracked files at
  a recorded commit (`docs/APPS_INVENTORY.md`), then renamed by `scripts/rebrand.py` in a
  separate commit so the rename can be reviewed on its own. Each app keeps its own Cargo
  workspace and `Cargo.lock`; the shared `craft-libs` is `libs/core`, linked by path.
- **Alternatives:** git submodules (13+ remotes with someone else's names on them); subtrees
  with full history (upstream moves at ~100 commits/day, so it would be very heavy); one giant
  Cargo workspace (thousands of dependency conflicts to resolve for no user benefit).
- **Why:** one clone and one CI, a small repo, and every app still builds the way upstream
  documents it. Provenance stays exact through the recorded commits. Later upstream fixes come
  in by re-importing into a branch and replaying `rebrand.py`.

## D5 — UI stack: keep native Rust (egui/eframe + wgpu), no WebView

- **Decision:** keep the upstream native stack and add an Android entry point per app
  (`android_main` through eframe's `android-native-activity`). Rendering uses wgpu on Vulkan,
  falling back to GLES (glow) when Vulkan isn't usable.
- **Alternatives:** Tauri 2 mobile (WebView); Flutter shell around Rust (as in educonnect);
  the WASM build inside a WebView.
- **Why:** the user asked for light apps without losing speed. All five apps are already egui
  0.36 + wgpu, and eframe 0.36 supports Android. A WebView would add RAM use and input lag
  and would mean rewriting the UI. WASM stays the fallback only if native is blocked for one app.

## D6 — Android packaging: one Gradle project, one flavour per app, a thin Java shell

- **Decision:** `android/` is a Gradle project. One `app` module has a product flavour per
  Mahay app (its own applicationId `org.mahay.<app>`, label, icons and `.so`). A small Java
  `MahayActivity extends NativeActivity` handles what Rust can't reach without Java: the
  Storage Access Framework pickers (open/save documents), incoming VIEW/SEND intents, and
  launching sibling apps from Mahay HUB. Java only, no Kotlin, no AndroidX, to keep APKs small.
  The `.so` files are built by `cargo ndk`. There is one APK per app per ABI (arm64-v8a,
  x86_64), plus an AAB.
- **Alternatives:** cargo-apk (unmaintained); xbuild (no AAB signing flow we trust); one big
  APK holding every app (makes everyone download everything).
- **Why:** users install only the apps they want, and each APK only carries its own engine.
  Gradle is the standard path to signed APKs and AABs.

## D7 — Minimum Android version: 8.0 (API 26)

- **Why:** the prompt asks for Android 8+. wgpu's Vulkan path needs Vulkan 1.1 (most devices
  from API 28 onwards); older devices use the GLES 3 fallback.

## D8 — Signing

- **Decision:** a release keystore is generated locally, outside the repo
  (`~/.mahay/release.jks`, git-ignored patterns as a second guard). CI reads it from the
  `MAHAY_KEYSTORE_B64`, `MAHAY_KEYSTORE_PASSWORD`, `MAHAY_KEY_ALIAS` and
  `MAHAY_KEY_PASSWORD` secrets. Without those secrets, CI builds debug-signed APKs.
- **Why:** the user chose signed APKs. Secrets never go into git.

## D9 — GameActivity, not NativeActivity

- **Decision:** the Java shell extends `com.google.androidgamesdk.GameActivity`
  (androidx.games 4.4.0, the version android-activity 0.6 requires).
- **Alternatives:** NativeActivity (smaller, no AndroidX).
- **Why:** eframe's accessibility (accesskit, TalkBack) only builds with GameActivity, and
  NativeActivity has no input method: no accents, no suggestions, no non-Latin keyboards. For a
  word processor and a spreadsheet that isn't optional. Cost: about 1–2 MB of AndroidX per APK.
- **Gap found:** winit 0.30 drops GameActivity's text events. `EguiBridge` reads the
  GameTextInput buffer and diffs it into egui events (D-level detail in PORTING_AUDIT #3).

## D10 — Documents on Android: local copy + write-back

- **Decision:** a picked document (`ACTION_OPEN_DOCUMENT`, persistable read/write grant) is
  copied into the app's private storage; the app edits that path with its normal desktop code;
  every save is copied back to the original URI. Save As writes locally first, then asks where
  to put it (`ACTION_CREATE_DOCUMENT`) and keeps that URI for later saves.
- **Alternatives:** read/write through file descriptors from `ContentResolver` inside every
  app's I/O code (invasive in five large codebases); a private library folder only (users
  couldn't reach their files).
- **Why:** zero change to the apps' file code, works with any provider (local storage, Drive,
  USB), and a crash never leaves a half-written original (the copy-back happens after a
  complete save).

## D11 — No upstream services

- **Decision:** nothing calls upstream's servers. Mahay HUB's release feed
  (`api.storyteller.ai`) is disabled; it reads the GitHub releases of `raantss18/mahay`.
  Upstream social accounts and Discord links point to the repo and its Discussions instead.
- **Why:** the prompt forbids depending on upstream's backend, and links to accounts that
  aren't the project's would mislead users.

## D12 — Brand artwork is Mahay's own and openly licensed

- **Decision:** a new icon family (`docs/BRAND.md`) replaces every upstream icon, logo and
  screenshot. The marks are MIT OR Apache-2.0.
- **Alternatives:** keep a trademark-only license for the marks (as upstream did).
- **Why:** upstream's marks are trademarks Mahay can't use. A permissive license is simpler for
  a community project; misuse is covered by the "don't imply endorsement" note.

## D13 — Lightness measures

- **Decision:** `android` Cargo profile (fat LTO, one codegen unit, stripped), compressed
  `.so` in APKs, R8 for the Java side, one APK per app and ABI, Android adaptive icons as
  vector drawables (no PNG mipmaps), and no WebView.
- **Measured:** Tabilao x86_64 APK 11.5 MB. Taratasy 25.7 MB (it carries OCR, a JavaScript
  engine for PDF forms and signature crypto); trimming it is tracked in STATUS.

## D14 — One version for the whole suite

- **Decision:** every app, the hub, the APKs and the Linux packages carry the repository's
  `VERSION` (0.2.0 now); `scripts/bump-version.py` sets it everywhere.
- **Why:** Tontolo's rule, and what makes automatic updates simple: an app compares its own
  version with the latest release's, and Mahay HUB compares installed versions with it.

## D15 — Automatic updates, Tontolo's model

- **Decision:** `mahay-update` checks GitHub releases at start-up (at most hourly) and every 24
  hours, downloads the file for this app and device, and installs it only when the user says
  so: Android's installer (one tap; Android forbids silent installs), or the AppImage replaced
  in place then restarted. `SHA256SUMS.txt` must be signed with Mahay's Ed25519 key
  (`.github/update-signing.pub`, private half in `release-keys/`); hosts other than GitHub over
  https, a bad signature, a wrong hash or an older version are refused. File names never change.
- **Mahay HUB is the only download:** the release page puts it first; it installs every other app
  directly (no browser, no GitHub page), and "Update all" updates them.
- **Cost:** two Android permissions, `INTERNET` and `REQUEST_INSTALL_PACKAGES`, both declared
  and explained (SECURITY_AUDIT).

## D16 — Tsilo, not Tsipika

- **Decision:** the vector illustration app (upstream VectorCraft) is **Tsilo** (lamp, torch),
  at the user's request (2026-10-18).

## D17 — Tselatra keeps upstream FilmCraft's media crates

- **Decision:** Tselatra depends on FilmCraft's media crates at the commit it was written against
  (`storytold/filmcraft@2c5ba7a`), under their published names, instead of the newer copies in
  `apps/hetsika`.
- **Alternatives:** port Tselatra to Hetsika's current API (several API changes); vendor the
  old crates (a second copy of a large codec tree).
- **Why:** it builds unchanged and stays a plain source dependency (no upstream service). To be
  revisited when Tselatra moves to Hetsika's crates.

## D18 — About pages

- **Decision:** every app's About is the shared `mahay-about` page (Tontolo's layout: version,
  updates, QR code to install on another device, author, licences). No upstream names, links or
  contributor lists in the interface; the copyright notices the MIT licence requires stay in
  `LICENSE-*`/`NOTICE` and the upstream contributor files stay in the repository.

## D19 — The hub is "Mahay HUB"

- **Decision:** the hub, first called Trano, is **Mahay HUB** (id `hub`, `org.mahay.hub`,
  `hub-<v>-android-<abi>.apk`), at the user's request (2026-10-18), before 0.2.0 shipped.
- **Why:** it names what it is: the one app to download, which installs and updates the others.
  Release 0.1.0's files keep their old `trano-…` names.
