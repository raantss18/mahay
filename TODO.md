# TODO

Living list of what's left. Status of what's done: [docs/STATUS.md](docs/STATUS.md). Order of
the larger phases: the spec's phases B → G (Android core apps, sync, model hub, office, 3D,
polish).

## Now (0.1.x)

- [ ] Test the arm64-v8a APKs on a real Android phone and a real tablet (Vulkan GPU, real
      keyboard, stylus, rotation, split screen, foldable if available).
- [ ] Phone (compact) layout: collapse the ribbon of Soratra, Tabilao and Seho into a scrollable
      strip or a "⋯" sheet under 600 dp; check Taratasy's tool panel and Sary's panels.
- [ ] Touch pressure: a finger draws with near-zero pressure in Sary; use 1.0 for fingers and
      the real pressure for a stylus (`MotionEvent.getToolType`).
- [ ] Soft keyboard: move dialogs above the keyboard (egui windows ignore the insets).
- [ ] Clean up `files/docs/` copies of documents that were closed long ago (storage growth).
- [ ] Folder picker (`ACTION_OPEN_DOCUMENT_TREE`) for Taratasy's batch actions and pinned folders.
- [ ] Android printing through the system print framework.
- [ ] Rich clipboard on Android (HTML, images).
- [ ] VIEW / SEND intent filters per app (open a PDF from Files straight into Taratasy), with the
      `content://`-only rule kept.
- [ ] Shrink Taratasy (25 MB) and Soratra (22 MB): optional OCR/JS features on Android, and see
      what `cargo bloat` reports.
- [ ] Translations of the strings Mahay changed (Discussions instead of Discord) in every catalogue;
      add a Malagasy catalogue to each app.
- [ ] Run the upstream test suites per crate in CI after the rename.
- [ ] Set the signing secrets in GitHub and let the Release workflow build 0.1.1.
- [ ] Linux arm64 (aarch64) builds and Flatpak bundles in the release.

## Security

- [ ] Fix or pin around the advisories in `docs/SECURITY_AUDIT.md`, re-run `cargo deny` and
      `cargo audit` in CI on every push (job added).
- [ ] Fuzz the Android document path handling (`mahay-android`) like upstream fuzzes its parsers.
- [ ] Review `allowBackup`: backups include the local copies of documents.

## Next apps (phase B continued)

- [ ] Import Tsipika (VectorCraft), Endrika (FigCraft), Pejy (DesignCraft) with
      `scripts/rebrand.py`; add their Android crates.
- [ ] Then Hazavana, Feo, Hetsika, Tselatra; then Drafitra.

## Later phases

- [ ] C — sync: CRDT (Automerge/Loro evaluation), pairing by QR code, E2E encryption (Noise),
      LAN discovery, self-hostable relay, selective sync, `docs/SYNC.md` with the threat model.
- [ ] D — model hub: BYOK keys in Android Keystore / libsecret, provider trait, versioned model
      catalogue, job queue that survives restarts, offline mode.
- [ ] E — office interop (ODF/OOXML round-trips across apps, shared asset library).
- [ ] F — 3D and editing (glTF/OBJ import, scene editor) and the ArtCraft AI app.
- [ ] G — polish: accessibility pass (TalkBack), app store listings (F-Droid metadata), website.
