<p align="center">
  <img src="docs/img/mahay-logo.svg" alt="Mahay" width="360">
</p>

> **Langue / Language / Fiteny :** [English](README.md) · **Français** · [Malagasy](README.mg.md)

**Mahay** (en malgache : *compétent, qui sait faire*) est une suite libre d'applications de
création et de bureautique pour **Linux** et les **téléphones et tablettes Android**. Chaque app
est en Rust natif : pas d'Electron, pas de WebView, pas de compte, pas de télémétrie. Vos
fichiers restent sur votre appareil.

Mahay est un fork des applications [ArtCraft](https://github.com/storytold), renommées et portées
sur Android. Voir [NOTICE](NOTICE).

> **État : alpha (0.2).** Les apps fonctionnent, mais il reste des angles vifs, surtout sur
> téléphone. Voir [docs/STATUS.md](docs/STATUS.md) : ce qui marche, ce qui manque, ce qui a été testé.

## Les apps

<p align="center"><img src="docs/img/icons.png" alt="La famille d'icônes Mahay" width="760"></p>

| App          | Rôle                                            | Linux | Android |
|--------------|-------------------------------------------------|-------|---------|
| **Mahay HUB** | l'accueil : installe, ouvre et met à jour les apps | oui   | oui     |
| **Sary**     | retouche d'images (calques, PSD, RAW)           | oui   | alpha   |
| **Taratasy** | PDF : lire, annoter, remplir, signer, organiser | oui   | alpha   |
| **Soratra**  | traitement de texte (DOCX, ODT)                 | oui   | alpha   |
| **Tabilao**  | tableur (XLSX, ODS, CSV)                        | oui   | alpha   |
| **Seho**     | présentations (PPTX)                            | oui   | alpha   |
| **Tsilo**    | illustration vectorielle                        | oui   | alpha   |
| **Endrika**  | design d'interfaces et prototypes               | oui   | alpha   |
| **Pejy**     | mise en page et publication                     | oui   | alpha   |
| **Hazavana** | photothèque et développement RAW                | oui   | alpha   |
| **Feo**      | enregistrement, montage et mixage audio         | oui   | alpha   |
| **Hetsika**  | montage vidéo                                   | oui   | alpha   |
| **Tselatra** | animation et effets visuels                     | oui   | alpha   |
| **Drafitra** | dessin technique (CAO)                          | oui   | alpha   |


## Installer : seulement Mahay HUB

Téléchargez **Mahay HUB**, l'accueil de Mahay, depuis la [dernière version](https://github.com/raantss18/mahay/releases/latest) :
`hub-…-android-arm64-v8a.apk` sur Android 8+ (téléphones et tablettes), `hub-…-linux-x86_64.AppImage`
sous Linux. **Mahay HUB installe, ouvre et met à jour toutes les autres apps**, directement.
Chaque app se met aussi à jour toute seule (vérification quotidienne, fichiers signés, vous confirmez l'installation).

## Ouvre vos fichiers

Chaque app ouvre son type de fichier depuis n'importe où sur l'appareil : touchez un tableur
dans Fichiers, Drive ou un e-mail et choisissez **Tabilao** ; une image s'ouvre dans **Sary** (ou
s'importe dans **Hazavana**), un PDF dans **Taratasy**, un document dans **Soratra**, des diapos
dans **Seho**, un SVG dans **Tsilo**, de l'audio dans **Feo**, une vidéo dans **Hetsika**, un
plan DXF/DWG dans **Drafitra**. « Partager » marche aussi. L'enregistrement réécrit le fichier
ouvert.

## Toujours à jour

Mahay se met à jour tout seul, comme Tontolo : chaque app (et Mahay HUB pour toutes) vérifie la
dernière version au démarrage puis chaque jour, la télécharge en arrière-plan, contrôle sa
signature (Ed25519) et son SHA-256, puis propose **Installer**. Sur Android, un appui suffit ;
sous Linux, l'AppImage se remplace et redémarre. Désactivable dans **À propos**.

## Léger par conception

- Un APK par app : vous ne téléchargez que ce que vous utilisez (d'environ 5 Mo pour Mahay HUB à environ 25 Mo pour les plus grosses apps).
- Code natif optimisé de bout en bout, aucun moteur de navigateur embarqué.
- Fonctionne entièrement hors ligne. Les documents s'ouvrent depuis, et s'enregistrent vers,
  l'endroit où vous les rangez (Fichiers, Drive, clé USB) via le sélecteur de documents d'Android.

## Compiler

```bash
# App Linux (depuis son dossier)
cd apps/tabilao && cargo run --release -p tabilao
```

```bash
# APK Android (SDK Android + NDK 28, rustup avec les cibles Android, cargo-ndk)
scripts/android-build.sh --abi arm64-v8a tabilao
```

Détails : [docs/BUILDING.md](docs/BUILDING.md). Décisions : [docs/DECISIONS.md](docs/DECISIONS.md).

## Licence

Double licence [MIT](LICENSE-MIT) ou [Apache-2.0](LICENSE-APACHE), au choix
(`SPDX : MIT OR Apache-2.0`). Les mentions de copyright de l'upstream sont conservées.
Licences tierces : [THIRD_PARTY_LICENSES.md](THIRD_PARTY_LICENSES.md).
