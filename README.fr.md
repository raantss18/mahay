<p align="center">
  <img src="docs/img/mahay-logo.svg" alt="Mahay" width="360">
</p>

<h3 align="center">Quatorze applis de création et de bureautique. Libres, natives, privées.<br>Un seul téléchargement. Linux, téléphones et tablettes Android.</h3>

<p align="center">
  <a href="https://github.com/raantss18/mahay/releases/latest"><img alt="Dernière version" src="https://img.shields.io/github/v/release/raantss18/mahay?label=version&color=3b5bdb"></a>
  <img alt="État : alpha" src="https://img.shields.io/badge/%C3%A9tat-alpha-e8590c">
  <img alt="Plateformes : Android, Linux" src="https://img.shields.io/badge/plateformes-Android%20%C2%B7%20Linux-2b8a3e">
  <img alt="Écrit en Rust" src="https://img.shields.io/badge/%C3%A9crit%20en-Rust-b7410e">
  <a href="LICENSE-MIT"><img alt="Licence : MIT ou Apache-2.0" src="https://img.shields.io/badge/licence-MIT%20%7C%20Apache--2.0-495057"></a>
</p>

<p align="center">
  <a href="README.md">English</a> · <b>Français</b> · <a href="README.mg.md">Malagasy</a>
  &nbsp;|&nbsp;
  <a href="https://github.com/raantss18/mahay/releases/latest">Télécharger</a> ·
  <a href="#configuration-requise">Configuration requise</a> ·
  <a href="#les-apps">Les apps</a> ·
  <a href="docs/STATUS.md">État</a> ·
  <a href="CHANGELOG.md">Journal des changements</a>
</p>

---

**Mahay** (en malgache : *compétent, qui sait faire*) est une suite de **14 apps** pour écrire,
calculer, présenter, dessiner, retoucher photos, audio et vidéo, mettre en page et concevoir des
interfaces. Chaque app est en **Rust natif** : pas d'Electron, pas de WebView, pas de compte, pas
de télémétrie. Vos fichiers restent sur votre appareil, tout marche hors ligne, et chaque app est
assez légère pour s'installer seule.

Mahay est un fork des applications [ArtCraft](https://github.com/storytold), renommées et portées
sur les **téléphones et tablettes Android** et sur **Linux**. Voir [NOTICE](NOTICE).

> **État : alpha (0.2).** Les apps fonctionnent et sont testées sous Android et Linux, mais il
> reste des angles vifs, surtout sur téléphone. [docs/STATUS.md](docs/STATUS.md) dit ce qui marche,
> ce qui manque et ce qui a été testé sur quel appareil.

## Pourquoi Mahay

|   |   |
|---|---|
| 🔒 **Privé par conception** | Pas de compte, pas de télémétrie, pas de cloud obligatoire. Les documents s'ouvrent depuis, et s'enregistrent vers, l'endroit où vous les rangez. |
| ⚡ **Natif et léger** | Un APK par app, d'environ 5 Mo (Mahay HUB) à environ 25 Mo. Aucun moteur de navigateur embarqué. |
| 📴 **Hors ligne d'abord** | Tout marche sans connexion. Le réseau ne sert qu'à chercher les mises à jour. |
| 🔄 **Toujours à jour** | Mises à jour signées (Ed25519 + SHA-256), vérifiées chaque jour, installées d'un appui. |
| 📱 **Pensé pour le tactile** | La disposition suit la largeur de la fenêtre, sans réglage : téléphone, tablette, écran partagé, bureau. |
| 🧰 **Ouvre vos fichiers** | DOCX, XLSX, PPTX, PDF, PSD, SVG, DXF et plus, depuis Fichiers, Drive ou un e-mail. |

## Les apps

<p align="center"><img src="docs/img/icons.png" alt="La famille d'icônes Mahay" width="760"></p>

| App | Rôle | Linux | Android |
|-----|------|:-----:|:-------:|
| **Mahay HUB** | l'accueil : installe, ouvre et met à jour les apps | ✅ | ✅ |
| **Soratra** | traitement de texte (DOCX, ODT) | ✅ | alpha |
| **Tabilao** | tableur (XLSX, ODS, CSV) | ✅ | alpha |
| **Seho** | présentations (PPTX) | ✅ | alpha |
| **Taratasy** | PDF : lire, annoter, remplir, signer, organiser | ✅ | alpha |
| **Sary** | retouche d'images (calques, PSD, RAW) | ✅ | alpha |
| **Hazavana** | photothèque et développement RAW | ✅ | alpha |
| **Tsilo** | illustration vectorielle | ✅ | alpha |
| **Pejy** | mise en page et publication | ✅ | alpha |
| **Endrika** | design d'interfaces et prototypes | ✅ | alpha |
| **Drafitra** | dessin technique (CAO) | ✅ | alpha |
| **Feo** | enregistrement, montage et mixage audio | ✅ | alpha |
| **Hetsika** | montage vidéo | ✅ | alpha |
| **Tselatra** | animation et effets visuels | ✅ | alpha |

### Sur téléphone

Les apps à ruban passent en disposition tactile sur les fenêtres étroites : onglets et barres de
commandes défilants, panneaux latéraux en tiroirs, rien ne se superpose ni ne sort de l'écran. Elle
est choisie d'après la seule largeur de la fenêtre : tourner le téléphone ou ouvrir l'écran
partagé la change tout seul. Soratra, Tabilao, Seho et Taratasy l'ont déjà (en développement pour
0.2.3) ; les autres apps suivent.

<table align="center">
  <tr>
    <td align="center"><img src="docs/img/phone-soratra-portrait.png" alt="Soratra sur téléphone" width="250"><br><sub><b>Soratra</b> : traitement de texte</sub></td>
    <td align="center"><img src="docs/img/phone-tabilao-portrait.png" alt="Tabilao sur téléphone" width="250"><br><sub><b>Tabilao</b> : tableur</sub></td>
    <td align="center"><img src="docs/img/phone-taratasy-portrait.png" alt="Taratasy sur téléphone" width="250"><br><sub><b>Taratasy</b> : PDF</sub></td>
  </tr>
</table>

## Installer : seulement Mahay HUB

1. Téléchargez **Mahay HUB** depuis la [dernière version](https://github.com/raantss18/mahay/releases/latest) :
   `hub-…-android-arm64-v8a.apk` pour les téléphones et tablettes Android (le bon fichier pour
   presque tous les appareils), `hub-…-linux-x86_64.AppImage` pour Linux.
2. Ouvrez-le. Sous Android, autorisez Mahay HUB à installer des apps quand Android le demande.
3. Touchez **Installer** sur les apps voulues. **Mahay HUB installe, ouvre et met à jour toutes
   les autres apps**, directement depuis la version publiée. Chaque app se met aussi à jour seule.

Android peut afficher une alerte « Play Protect » pour les apps installées hors du Play Store :
choisissez *Analyser* ou *Installer quand même*. Chaque fichier est signé et son empreinte figure
dans `SHA256SUMS.txt`.

## Configuration requise

| | **Android** | **Linux** |
|---|---|---|
| **Système** | Android 8.0 (API 26) ou plus récent, téléphone ou tablette | Distribution 64 bits (x86_64) avec glibc 2.39 ou plus récente : Ubuntu 24.04, Debian 13, Fedora 40, Arch et plus récentes |
| **Processeur** | ARM 64 bits (`arm64-v8a`), la plupart des téléphones et tablettes depuis 2017. Les versions `x86_64` sont pour les émulateurs, Chromebooks et Android sur PC | x86_64 |
| **Graphismes** | Un GPU avec OpenGL ES (Vulkan si disponible) | Un pilote GPU avec Vulkan ou OpenGL |
| **Stockage** | Environ 5 Mo pour Mahay HUB, jusqu'à environ 25 Mo par app, plus vos documents | À peu près pareil par app (AppImage) |
| **Mémoire** | Pas encore mesurée : plus c'est mieux pour Sary, Hazavana, Hetsika et Tselatra (grandes images et vidéo) | idem |
| **Réseau** | Inutile. Seulement pour télécharger Mahay HUB et les mises à jour | idem |
| **Autre** | Autoriser « installer des apps inconnues » pour Mahay HUB | L'AppImage demande FUSE 2 (`libfuse2`), ou se lance avec `--appimage-extract-and-run` |

Testé jusqu'ici : Android 16 (API 36) sur émulateurs x86_64 (un téléphone 1080×2400 et une tablette
2560×1600) et Linux x86_64. Les APK arm64 sont construits et signés mais n'ont pas encore tourné
sur un vrai appareil : [docs/STATUS.md](docs/STATUS.md) dit exactement ce qui a été vérifié.

## Ouvre vos fichiers

Chaque app ouvre son type de fichier depuis n'importe où sur l'appareil : touchez un tableur
dans Fichiers, Drive ou un e-mail et choisissez **Tabilao** ; une image s'ouvre dans **Sary** (ou
s'importe dans **Hazavana**), un PDF dans **Taratasy**, un document dans **Soratra**, des diapos
dans **Seho**, un SVG dans **Tsilo**, de l'audio dans **Feo**, une vidéo dans **Hetsika**, un
plan DXF/DWG dans **Drafitra**. « Partager » marche aussi. L'enregistrement réécrit le fichier
ouvert.

## Toujours à jour

Mahay se met à jour tout seul : chaque app (et Mahay HUB pour toutes) vérifie la dernière version
au démarrage puis chaque jour, la télécharge en arrière-plan, contrôle sa signature (Ed25519) et
son SHA-256, puis propose **Installer**. Sur Android, un appui suffit ; sous Linux, l'AppImage se
remplace et redémarre. Désactivable dans **À propos**.

## Compiler

```bash
# App Linux (depuis son dossier)
cd apps/tabilao && cargo run --release -p tabilao
```

```bash
# APK Android (SDK Android + NDK 28, rustup avec les cibles Android, cargo-ndk)
scripts/android-build.sh --abi arm64-v8a tabilao
```

Chaque app est son propre workspace Cargo sous `apps/` ; le code partagé est dans `libs/core`.
Détails : [docs/BUILDING.md](docs/BUILDING.md) · décisions : [docs/DECISIONS.md](docs/DECISIONS.md) ·
portage : [docs/PORTING_AUDIT.md](docs/PORTING_AUDIT.md) · feuille de route : [TODO.md](TODO.md).

## Sécurité

Une faille trouvée ? Signalez-la en privé, comme décrit dans [SECURITY.md](SECURITY.md). Les notes
d'audit sont dans [docs/SECURITY_AUDIT.md](docs/SECURITY_AUDIT.md).

## Licence

Double licence [MIT](LICENSE-MIT) ou [Apache-2.0](LICENSE-APACHE), au choix
(`SPDX : MIT OR Apache-2.0`). Les mentions de copyright de l'upstream sont conservées dans chaque
arborescence importée. Licences tierces : [THIRD_PARTY_LICENSES.md](THIRD_PARTY_LICENSES.md).
