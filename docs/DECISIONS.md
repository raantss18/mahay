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

## D20 — Accounts and cloud sync: Supabase sign-in, the user's own storage, end-to-end encryption

- **Decision:** Mahay HUB gets an optional account, "Continue with Google / GitHub", through
  Supabase Auth with PKCE as in Tontolo. Tokens are kept in the system's secure storage (Android
  Keystore, libsecret).
  - **Storage is the user's, never ours.** Documents go to the user's own **Google Drive**, in a
    "Mahay" folder, under the `drive.file` scope: Mahay sees only the files it created.
  - **Nextcloud / WebDAV** is an alternative that any user can pick by entering a URL.
  - The small state (settings, recent files, installed apps) goes to a Supabase table, as
    Tontolo's `sync_items`.
  - **Everything is encrypted on the device** before upload: AES-256-GCM with a key derived from a
    sync passphrase (PBKDF2-HMAC-SHA256), with the item's kind and id as associated data.
  - Local-first: apps keep working on local files; the hub syncs in the background when the
    network is there.
- **Alternatives:**
  - **A hosted Nextcloud with automatic accounts:** we would pay for disks that grow with every
    user (creative files are large), run backups, and answer for other people's files.
  - **Supabase Storage:** 1 GB for the whole project on the free tier.
  - **Peer-to-peer only (phase C, CRDT + Noise):** stays planned for live collaboration, but does
    not give a copy reachable when the other devices are off.
- **Why:** free for the user and for the project, scales without a server bill, and the provider
  (Google, the Nextcloud host, Supabase) only ever sees encrypted blobs. Same principles as
  Tontolo (user's choice, 2026-10-18). Protocol: `docs/SYNC.md`.

## D21 — Android backups and old document copies

- **Decision:** Android backups keep settings but leave out `files/docs/` (document copies),
  `files/updates/`, `files/update.json` and `files/cache/`. Copies in `files/docs/` untouched
  for 30 days are deleted at start-up on a worker thread; a copy open in the process, tracked
  by `mahay-android`, or marked `<n>.unsaved` (a Save As never placed outside the app, the only
  copy) is kept.
- **Alternatives:** `allowBackup="false"` (simplest, but loses settings on a new phone); keep
  copies forever (the folder only grows); delete on close (apps don't report closing a document).
- **Why:** the originals live in the user's storage, so a copy is only a cache: backing it up
  doubles private data in the user's cloud, and keeping it forever wastes space.

## D21 — Mahay HUB manages the other apps

- **Decision:** the apps Mahay HUB installs are its sub-apps. Android asks for no confirmation
  when the **installer of record** updates an app (Android 12+, `USER_ACTION_NOT_REQUIRED`), so:
  1. every install session asks for no confirmation when it can; the hub is the installer of
     record of what it installs, and an app installed some other way joins after one confirmed
     update;
  2. a sub-app hands "Install update" to the hub when the hub is installed, through an explicit
     intent to a hub component guarded by a signature-level permission (only Mahay-signed apps
     can use it); without the hub it updates itself as before and Android asks to allow "install
     unknown apps" for it, so every app also works alone;
  3. the hub updates all its apps by itself ("Update automatically", on by default): at start,
     then every day, and in the background through a scheduled job that loads the same native
     updater (so the Ed25519 check of `SHA256SUMS.txt` stays in one place); a notification says
     so when Android still needs a tap.
- **Limits Android imposes:** the first install of each app always needs the user's tap; below
  Android 12 each update is confirmed; an app that is open while updated is restarted.
- **Design details:** an app managed by the hub only *reports* a newer version (`State::Managed`)
  and downloads nothing itself, so mobile data is spent once; the hub's headless updater
  (`mahay_update::managed`) takes only a package name, matched exactly against the 13 sub-apps,
  and honours the user's settings (automatic on by default, unmetered networks only unless
  "Also on mobile data"; a request from an app's own bar may use any network). Java hashes the
  APK while copying it into the install session, so what Rust verified is what installs.
- **Measured (2026-10-19, Android 16 tablet emulator, x86_64, hub 0.2.2 as installer of record
  of Feo 0.2.0 → 0.2.1, real release downloaded from GitHub):**
  1. `setRequireUserAction(USER_ACTION_NOT_REQUIRED)` alone is **not enough**: with the hub as
     installer of record (`dumpsys package` → `installerPackageName=org.mahay.hub`) and "install
     unknown apps" allowed, Android still answered `STATUS_PENDING_USER_ACTION` (the session
     showed `requireUserAction=NOT_REQUIRED`). The installer must also hold the normal
     permission **`android.permission.UPDATE_PACKAGES_WITHOUT_USER_ACTION`** (declared in the
     hub's manifest, granted at install).
  2. With it, from the hub's "Update all": **no dialog**, Feo went 0.2.0 → 0.2.1 in a few seconds
     and the hub stayed on screen.
  3. With it, from Feo's own update bar (Feo on screen): "Install" → "Updating through Mahay HUB…"
     → the hub job downloaded and verified the file and committed the session → **no dialog**,
     Feo was replaced (and closed) by Android. Android therefore does **not** defer the update
     of an app that is on screen: that is why background runs skip apps that say they are open
     (`APP_STATE`), and only an update the user asked for in that app is allowed to replace it.
  4. An app installed by hand (`adb install`, installer `null`): the hub's update showed Android's
     dialog once ("Do you want to update this app?"); after it, `installerPackageName` was
     `org.mahay.hub`, so later updates were silent. This is "joins hub management after one
     confirmed update".
  5. A sub-app installed **after** the hub with `uses-permission org.mahay.permission.MANAGE`
     holds it (`granted=true`); the hub's service, `UpdateJobService` and result receiver are
     refused to `adb shell` (SECURITY_AUDIT).
  6. Foreground guard: with Feo 0.2.0 on screen (it had told the hub through `APP_STATE`), the
     hub's job (`cmd jobscheduler run -f`, the gate of one run per hour having passed) answered
     "ok, 0 ready" and left Feo at 0.2.0 although 0.2.1 was out.
  7. The daily job (`JobScheduler`, periodic, persisted) ran at once when scheduled
     (constraints met) and went through the same Rust updater: "update run: ok, 0 ready" when
     everything was current.
  Not measured: the silent-update throttle Android may apply to repeated updates of one app (the
  platform has a configurable one; the first updates here went through minutes apart), Android
  below 12 (each update then asks, the notification says so), the notification on a real device,
  and an app installed before the hub that was never granted the permission (it falls back to
  updating itself: `startService` throws `SecurityException`).
- **Alternatives:** one APK holding all 14 apps (more than 200 MB of native code, and no app
  could be installed alone); `sharedUserId` (deprecated, impossible on installed apps); Play's
  dynamic features (no Play Store); privileged or root installers (not for a sideloaded app).
- **Why:** it removes the repeated confirmations the user complained about without redoing any
  app, keeps the signed-update rules (D15) and keeps every app installable alone.

## D22 — "Open with" on Android

- **Decision:** each app's intent filters match its documents by MIME type **and by extension**
  (a `pathPattern` with `application/octet-stream` and `*/*`), for `content` and `file` URIs,
  because many file managers give no MIME type or a generic one. Verified by a matrix test
  (`cmd package query-activities` on the emulator) that fails if an app drops out of a list.
- **Why:** a filter on the MIME type alone resolves on the emulator but not for files a manager
  reports as `application/octet-stream` (IDML, DXF, RAW photos, …).

## D23 — Phone and tablet layouts follow the window, automatically

- **Decision:** the layout depends on the **window width in dp** (compact < 600, medium 600–839,
  expanded ≥ 840, Android's window size classes), computed every frame, never on a "phone" or
  "tablet" flag or a setting. Rotation, split screen, floating windows and foldables therefore
  change it by themselves. Expanded keeps today's desktop layout unchanged.
- **Compact rules:** nothing overlaps or leaves the screen; ribbons collapse into one scrolling
  command bar with the tabs in a menu; side panels become drawers; dialogs fill the screen; touch
  targets are at least 48 dp.
- **Why:** one rule for every app and every device shape, testable without a device (a layout
  test at 360, 400, 600 and 840 dp).

## D24 — Following ArtCraft's repositories

- **Decision:** a weekly watcher (`scripts/upstream-watch.py`, `.github/workflows/upstream-watch.yml`)
  reports what moved upstream and flags security and crash fixes, licence changes and new network
  endpoints; the owner is told through one GitHub issue. Changes are adopted per app, per revision,
  by cherry-pick or three-way merge of the *renamed* trees, and the recorded revision moves only
  after the result builds and passes (`docs/UPSTREAM.md`).
- **Alternatives:** merging upstream branches (every Mahay change would conflict on each merge);
  rebasing Mahay's changes on upstream (rewrites history, same conflicts); ignoring upstream
  (security and crash fixes would never arrive).
- **Why:** upstream moves by ~100 commits a day; only a small share matters for Mahay's users,
  and the risk is in what Mahay changed, not in what upstream added.
