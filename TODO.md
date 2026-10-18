# TODO

Living list of what's left. Status of what's done: [docs/STATUS.md](docs/STATUS.md). Order of
the larger phases: the spec's phases B → G (Android core apps, sync, model hub, office, 3D,
polish).

## 0.2.0 — Centralisation et mises à jour (principes de Tontolo)

- [ ] **Une seule version pour toute la suite** (comme Tontolo) : chaque app, le hub, les APK et
      les AppImage portent la version Mahay (`VERSION`) ; noms de fichiers stables, jamais changés :
      `<app>-<v>-android-<abi>.apk`, `<app>-<v>-linux-<arch>.AppImage`, `SHA256SUMS`, `SHA256SUMS.sig`.
- [ ] **Releases signées** : `SHA256SUMS` signé en Ed25519 (`SHA256SUMS.sig`), clé privée dans
      `release-keys/` (ignoré par git), clé publique embarquée dans les apps ; la release échoue
      sans signature.
- [ ] **`mahay-update`** (crate partagé, testé) : dernière release via l'API GitHub, repli sur la
      redirection `releases/latest` (limite de 60 requêtes/h) ; https + hôtes GitHub uniquement ;
      SHA-256 + signature obligatoires ; jamais de rétrogradation ; notes cumulées du CHANGELOG
      depuis la version installée ; une seule mise à jour gardée en cache.
- [ ] **Trano télécharge et installe directement** : « Télécharger » récupère l'APK vérifié et
      ouvre l'installeur Android, sans passer par la page GitHub ; barre de progression.
- [ ] **Trano met tout à jour** : version installée de chaque app (PackageManager) comparée à la
      dernière release ; « Mettre à jour » par app et « Tout mettre à jour ».
- [ ] **Chaque app se met à jour seule** : vérification au démarrage puis toutes les 24 h
      (désactivable), téléchargement en arrière-plan, bandeau « Version x.y prête — Installer »
      avec les nouveautés ; installation par l'installeur système (un appui, Android interdit
      l'installation silencieuse).
- [ ] **Linux** : remplacement atomique de l'AppImage puis redémarrage (comme Tontolo) ; paquets
      .deb : on signale seulement.
- [ ] **« À propos » façon Tontolo** dans Trano et dans chaque app : icône, nom et version, slogan,
      lien GitHub, QR code de la dernière version, carte de l'auteur, attributions des
      bibliothèques, licences ; **aucune mention d'ArtCraft dans l'interface** (les mentions de
      copyright exigées par la licence MIT restent dans `LICENSE-*`/`NOTICE`).
- [ ] Permissions Android ajoutées et justifiées : `INTERNET` (mises à jour),
      `REQUEST_INSTALL_PACKAGES` (installer les apps et leurs mises à jour) ; audit mis à jour.
- [ ] **Trano est le seul fichier à télécharger** : la page de chaque release (générée par
      `scripts/release-notes.py`) commence, comme Tontolo, par « ⬇️ Télécharger » avec Trano en
      gras (APK Android arm64 = le bon choix dans 95 % des cas, AppImage Linux), puis
      « Déjà installé ? Mahay se met à jour tout seul », les nouveautés, et en bas, repliés, les
      fichiers des autres apps que Trano télécharge lui-même, `SHA256SUMS` et sa signature.
- [ ] README (fr/en/mg) : « Installez Trano, il installe le reste ».
- [ ] `docs/RELEASING.md` : procédure de release (version, signature, vérification).

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

- [ ] Import Tsilo (VectorCraft), Endrika (FigCraft), Pejy (DesignCraft) with
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
