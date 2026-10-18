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
| launcher    | **Trano**      | house, home          | hub / launcher             |
| PhotoCraft  | **Sary**       | picture              | image editor               |
| PdfCraft    | **Taratasy**   | paper, document      | PDF workbench              |
| WordCraft   | **Soratra**    | writing              | word processor             |
| GridCraft   | **Tabilao**    | table                | spreadsheet                |
| DeckCraft   | **Seho**       | show, display        | presentations              |
| VectorCraft | **Tsipika**    | line                 | vector illustration        |
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

- **Decision:** release 1 ships Trano + Sary, Taratasy, Soratra, Tabilao, Seho on Linux and
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
  launching sibling apps from Trano. Java only, no Kotlin, no AndroidX, to keep APKs small.
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
