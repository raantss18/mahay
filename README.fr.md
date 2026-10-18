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

> **État : alpha (0.1).** Les apps fonctionnent, mais il reste des angles vifs, surtout sur
> téléphone. Voir [docs/STATUS.md](docs/STATUS.md) : ce qui marche, ce qui manque, ce qui a été testé.

## Les apps

<p align="center"><img src="docs/img/icons.png" alt="La famille d'icônes Mahay" width="760"></p>

| App          | Rôle                                            | Linux | Android |
|--------------|-------------------------------------------------|-------|---------|
| **Trano**    | l'accueil : liste et ouvre les apps             | oui   | oui     |
| **Sary**     | retouche d'images (calques, PSD, RAW)           | oui   | alpha   |
| **Taratasy** | PDF : lire, annoter, remplir, signer, organiser | oui   | alpha   |
| **Soratra**  | traitement de texte (DOCX, ODT)                 | oui   | alpha   |
| **Tabilao**  | tableur (XLSX, ODS, CSV)                        | oui   | alpha   |
| **Seho**     | présentations (PPTX)                            | oui   | alpha   |

Huit autres arrivent : Tsilo (vectoriel), Endrika (design d'interfaces), Pejy (mise en page),
Hazavana (photothèque), Feo (audio), Hetsika (vidéo), Tselatra (animation), Drafitra (CAO).

## Installer

**Android 8.0 ou plus récent.** Téléchargez l'APK depuis la
[dernière version](https://github.com/raantss18/mahay/releases/latest) : `arm64-v8a` pour les
téléphones et tablettes, `x86_64` pour les émulateurs et Chromebooks. N'installez que les apps
dont vous avez besoin ; Trano les ouvre.

**Linux** (x86_64) : téléchargez le `.AppImage`, rendez-le exécutable, lancez-le. Des paquets
`.deb` sont aussi fournis.

## Léger par conception

- Un APK par app : vous ne téléchargez que ce que vous utilisez (de 5 à 23 Mo : Trano 4,8, Tabilao 8,5, Seho 12, Soratra 17, Taratasy 18, Sary 23 Mo en arm64).
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
