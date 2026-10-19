# TODO

La liste vivante de ce qui reste. Ce qui est fait et vérifié : [docs/STATUS.md](docs/STATUS.md).
Légende : `[x]` fait · `[ ]` à faire. Les grandes phases du cahier des charges (B → G) sont en bas.

---

## 0.2.0 — Mahay HUB, mises à jour automatiques, 14 apps (en cours)

### Mahay HUB (l'accueil, seul fichier à télécharger)
- [x] Trano renommé **Mahay HUB** partout (id `hub`, `org.mahay.hub`, fichiers `hub-<v>-…`).
- [x] Télécharger une app installe directement l'APK vérifié (sans passer par GitHub).
- [x] « Mettre à jour » par app et « Tout mettre à jour ».
- [x] Liste des 13 apps, versions installées (PackageManager), page « À propos ».
- [ ] Tester sur l'émulateur : installer une app depuis la release 0.2.0, puis une mise à jour 0.2.1.
- [ ] Mahay HUB sur Linux (lanceur hérité de l'upstream) : vérifier qu'il installe les AppImage 0.2.0.

### Mises à jour automatiques (principes de Tontolo)
- [x] Une seule version pour toute la suite (`VERSION`, `scripts/bump-version.py`).
- [x] `mahay-update` : API GitHub + repli `releases/latest`, noms stables, SHA-256, signature
      Ed25519 obligatoire, jamais de rétrogradation, notes depuis la version installée.
- [x] Bandeau « mise à jour prête — Installer » dans chaque app (Android et Linux).
- [x] Linux : remplacement atomique de l'AppImage puis redémarrage ; .deb : simple signalement.
- [x] `SHA256SUMS.txt` signé par `scripts/stage-release.sh` ; test qui prouve que les apps
      acceptent ces signatures.
- [x] Clés dans `release-keys/` (local, ignoré par git) ; secrets décrits dans `docs/BUILDING.md`.
- [ ] Mettre les secrets dans GitHub (`release-keys/set-github-secrets.sh`) pour que la CI
      publie les releases suivantes.

### « À propos » façon Tontolo
- [x] Page commune `mahay-about` (version, mises à jour, QR code, auteur, licences) en fr/en/mg.
- [x] Branchée dans Sary, Taratasy, Soratra, Tabilao, Seho et Mahay HUB ; aucune mention de
      l'upstream dans l'interface.
- [x] La brancher dans les « À propos » des 8 nouvelles apps (Tsilo, Endrika, Pejy, Hazavana,
      Feo, Hetsika, Tselatra, Drafitra).

### Associations de fichiers (« Ouvrir avec »)
- [x] Android : filtres VIEW / EDIT / SEND par app, générés depuis les types MIME déclarés pour
      Linux (Tabilao = tableurs, Sary = images, Taratasy = PDF, Soratra = textes, Seho =
      présentations, Tsilo = SVG/EPS, Hazavana = photos et RAW, Feo = audio, Hetsika = vidéo,
      Drafitra = DXF/DWG, Pejy = IDML, Endrika = ses fichiers et images).
- [x] Chaque app ouvre le fichier reçu (au lancement et quand elle tourne déjà).
- [ ] Linux : vérifier les `.desktop` / `mime.xml` des 14 apps (Tselatra n'en déclare aucun).

### Campagnes de pub (comme Tontolo)
- [x] `press/` : gabarit d'affiche 1080 × 1440 fr/en/mg, QR code vers la dernière version,
      `build_campaign.py`, `docs/CAMPAIGNS.md` (une campagne par jour, liée aux mises à jour).
- [x] Première campagne : « Installez Mahay HUB, il installe le reste » + une affiche par app.
- [ ] Affiche de version 0.2.0 jointe à la release ; légendes pour les réseaux.

### Les 13 apps sur Android
- [x] Sary, Taratasy, Soratra, Tabilao, Seho (0.1.0).
- [x] Tsilo, Endrika, Pejy, Hazavana, Feo, Hetsika, Tselatra, Drafitra : importées, renommées,
      compilent pour Android (0.2.0).
- [x] Les 8 nouvelles démarrent sur l'émulateur tablette (x86_64, sans plantage).
- [ ] Les essayer sur l'émulateur téléphone, et ouvrir un fichier reçu (« Ouvrir avec ») dans chacune.
- [ ] Tselatra : passer aux crates média de Hetsika au lieu de FilmCraft épinglé (D17).

### Nettoyage hérité de l'upstream
- [x] Plus de serveur ArtCraft : Endrika ne se connecte plus à `collab.craftapps.dev` par défaut.
- [x] Liens morts (`raantss18/<app>`, `…/apps/<app>`) remplacés par le dépôt `raantss18/mahay`.
- [x] Libellés « Discord » et « mahay.com » remplacés (Discussions, GitHub, Mahay HUB) ; catalogues réalignés.
- [ ] Tests upstream encore rouges après le renommage (liens, réseaux sociaux retirés, tris) : à
      remettre au vert app par app (voir Linux et CI).
- [ ] Traductions françaises et malgaches des libellés communautaires (« Rejoindre les discussions »).

### Publication 0.2.0
- [ ] APK signés (14 × 2 ABI) et AppImage/.deb des 14 apps.
- [ ] Page de release avec Mahay HUB en premier (`scripts/release-notes.py`), publiée en « latest ».
- [ ] STATUS, CHANGELOG, audit de sécurité à jour.

---

## 0.3.0 — Compte et synchronisation en ligne (après la 0.2.0, [D20](docs/DECISIONS.md), [SYNC](docs/SYNC.md))

### Compte (comme Tontolo)
- [ ] Projet Supabase : fournisseurs Google et GitHub, redirections `org.mahay.hub://login-callback/`
      et boucle locale Linux ; clés dans `release-keys/` et `docs/BUILDING.md`.
- [ ] `mahay-account` : « Continuer avec Google / GitHub » (PKCE), jetons dans le stockage sécurisé
      (Android Keystore, libsecret), déconnexion, suppression des données.
- [ ] Écran Compte dans Mahay HUB (fr/en/mg), « Bonjour » + prénom.

### Stockage et chiffrement
- [x] Phrase secrète de synchronisation : PBKDF2 310 000 itérations, bloc de vérification `meta/check`
      (`mahay-sync`, copie `check.json` sur le stockage).
- [ ] Copie du bloc `meta/check` dans `sync_items` (attend le projet Supabase).
- [x] Chiffrement AES-256-GCM par blocs de 1 Mio ; noms et dossiers chiffrés ; manifeste chiffré.
- [ ] Google Drive (`drive.file`, dossier `Mahay/`) : client OAuth Google, vérification de marque.
- [x] Nextcloud / WebDAV : URL + mot de passe d'application (backend `WebDav`, testé contre un
      serveur local ; pas encore essayé sur un vrai Nextcloud).
- [ ] Écran Nextcloud dans Mahay HUB ; mot de passe d'application dans le stockage sécurisé.
- [ ] Table `sync_items` (réglages, fichiers récents, apps installées) avec règles RLS par utilisateur.

### Moteur `mahay-sync`
- [x] Analyse, journal hors ligne, envoi et réception (`Engine::sync`).
- [ ] Synchro en arrière-plan : minuterie dans Mahay HUB, WorkManager sur Android.
- [ ] Gros fichiers en flux (aujourd'hui un fichier entier est en mémoire pendant l'envoi).
- [x] Conflits : garder les deux versions ; corbeille de 30 jours (avec restauration).
- [ ] État dans Mahay HUB : compte, stockage, dernière synchro, conflits.
- [x] Tests : chiffrement (vecteurs), conflits, coupure réseau.
- [ ] Audit de sécurité dédié de `mahay-sync`.

---

## Android : qualité

- [ ] Tester les APK arm64 sur un vrai téléphone et une vraie tablette (GPU Vulkan, clavier,
      stylet, rotation, écran partagé, pliable).
- [ ] Mode compact (téléphone) : rubans de Soratra, Tabilao, Seho repliés sous 600 dp ;
      panneaux de Taratasy, Sary et des nouvelles apps.
- [ ] Pression tactile : 1.0 pour un doigt, vraie pression pour un stylet (`getToolType`).
- [ ] Dialogues au-dessus du clavier virtuel (les fenêtres egui ignorent les marges).
- [ ] Nettoyer les copies de documents anciennes dans `files/docs/`.
- [ ] Sélecteur de dossier (`ACTION_OPEN_DOCUMENT_TREE`) : Taratasy, Hazavana, Feo, Hetsika.
- [ ] Impression Android ; presse-papiers riche (HTML, images).
- [ ] Alléger Sary, Taratasy, Soratra (fonctions OCR/JS optionnelles, `cargo bloat`).
- [ ] Traductions : chaînes changées par Mahay ; catalogue malgache dans chaque app.

## Linux et CI

- [ ] Tests upstream crate par crate dans la CI après le renommage.
- [ ] Builds Linux arm64 (aarch64) et paquets Flatpak dans la release.

## Sécurité

- [x] `cargo audit` sur les workspaces dans la CI ; rustls corrigé ; zip/rsa documentés.
- [x] Shell Android : URI `content://` seulement, réécriture limitée au dossier de l'app.
- [x] Audit des 8 nouvelles apps : rustls 0.23.46, fournisseur TLS `ring` (rustls-webpki corrigé).
- [ ] Hazavana : passer quick-xml à 0.41 (RUSTSEC-2026-0194/0195, déni de service sur XML piégé).
- [ ] Fuzz du chemin des documents Android (`mahay-android`).
- [ ] Revoir `allowBackup` (les sauvegardes incluent les copies de documents).

## Phases suivantes du cahier des charges

- [ ] C — synchronisation en direct (en plus de la 0.3.0) : CRDT (Automerge/Loro), appairage par QR code, chiffrement de bout
      en bout (Noise), découverte LAN, relais auto-hébergeable, `docs/SYNC.md`.
- [ ] D — hub de modèles IA : clés BYOK (Android Keystore / libsecret), catalogue versionné,
      file de tâches persistante, mode hors ligne.
- [ ] E — interopérabilité bureautique (ODF/OOXML entre apps, bibliothèque d'éléments partagée).
- [ ] F — 3D et montage (glTF/OBJ, éditeur de scène) et l'app IA d'ArtCraft.
- [ ] G — finition : accessibilité (TalkBack), fiches F-Droid, site web.
