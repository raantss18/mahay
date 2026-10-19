<p align="center">
  <img src="docs/img/mahay-logo.svg" alt="Mahay" width="360">
</p>

<h3 align="center">Rindranasa 14 hamoronana sy hiasana amin'ny birao. Malalaka, teratany, manokana.<br>Fakana iray ihany. Linux, finday sy takelaka Android.</h3>

<p align="center">
  <a href="https://github.com/raantss18/mahay/releases/latest"><img alt="Dikan-teny farany" src="https://img.shields.io/github/v/release/raantss18/mahay?label=dikan-teny&color=3b5bdb"></a>
  <img alt="Toerana: alpha" src="https://img.shields.io/badge/toerana-alpha-e8590c">
  <img alt="Android · Linux" src="https://img.shields.io/badge/Android%20%C2%B7%20Linux-2b8a3e">
  <img alt="Rust" src="https://img.shields.io/badge/Rust-b7410e">
  <a href="LICENSE-MIT"><img alt="MIT | Apache-2.0" src="https://img.shields.io/badge/MIT%20%7C%20Apache--2.0-495057"></a>
</p>

<p align="center">
  <a href="README.md">English</a> · <a href="README.fr.md">Français</a> · <b>Malagasy</b>
  &nbsp;|&nbsp;
  <a href="https://github.com/raantss18/mahay/releases/latest">Alaivo</a> ·
  <a href="#ny-tena-ilaina">Ny tena ilaina</a> ·
  <a href="#ireo-rindranasa">Ireo rindranasa</a> ·
  <a href="docs/STATUS.md">Toerana</a>
</p>

---

**Mahay** dia andiana rindranasa **14** hanoratana, hikajiana, hampisehoana, hanaovana sary,
hanamboarana sary, feo ary horonantsary, handaminana pejy ary hamolavolana endrika. Samy nosoratana
tamin'ny **Rust** avokoa izy ireo: tsy misy Electron, tsy misy WebView, tsy mila kaonty, tsy misy
fanaraha-maso. Mijanona ao amin'ny fitaovanao ny rakitrao, mandeha tsy mila aterineto izy rehetra,
ary maivana ny rindranasa tsirairay ka azo apetraka tsirairay.

Mahay dia fork an'ireo rindranasa [ArtCraft](https://github.com/storytold), novana anarana ary
nampandehanina tamin'ny **finday sy takelaka Android** ary **Linux**. Jereo ny [NOTICE](NOTICE).

> **Toerana: alpha (0.2).** Mandeha ireo rindranasa, saingy mbola misy tsy fahatanterahana,
> indrindra amin'ny finday. Jereo ny [docs/STATUS.md](docs/STATUS.md).

## Ireo rindranasa

<p align="center"><img src="docs/img/icons.png" alt="Ny sarin'ireo rindranasa Mahay" width="760"></p>

| Rindranasa | Asany | Linux | Android |
|------------|-------|:-----:|:-------:|
| **Mahay HUB** | fandraisana: mametraka, manokatra, manavao | ✅ | ✅ |
| **Soratra** | fanoratana lahatsoratra (DOCX, ODT) | ✅ | alpha |
| **Tabilao** | tabilao kajy (XLSX, ODS, CSV) | ✅ | alpha |
| **Seho** | fampisehoana (PPTX) | ✅ | alpha |
| **Taratasy** | PDF: vakiana, asiana fanamarihana, fenoina, soniavina | ✅ | alpha |
| **Sary** | fanamboarana sary (sosona, PSD, RAW) | ✅ | alpha |
| **Hazavana** | tahirin-tsary sy RAW | ✅ | alpha |
| **Tsilo** | sary vektoraly | ✅ | alpha |
| **Pejy** | fandaminana pejy | ✅ | alpha |
| **Endrika** | famolavolana endrika | ✅ | alpha |
| **Drafitra** | sary teknika (CAO) | ✅ | alpha |
| **Feo** | fandraisam-peo sy fampifangaroana | ✅ | alpha |
| **Hetsika** | fanamboarana horonantsary | ✅ | alpha |
| **Tselatra** | sary mihetsika sy effets | ✅ | alpha |

<table align="center">
  <tr>
    <td align="center"><img src="docs/img/phone-soratra-portrait.png" alt="Soratra amin'ny finday" width="250"><br><sub><b>Soratra</b></sub></td>
    <td align="center"><img src="docs/img/phone-tabilao-portrait.png" alt="Tabilao amin'ny finday" width="250"><br><sub><b>Tabilao</b></sub></td>
    <td align="center"><img src="docs/img/phone-taratasy-portrait.png" alt="Taratasy amin'ny finday" width="250"><br><sub><b>Taratasy</b></sub></td>
  </tr>
</table>

## Fametrahana

Alaivo ihany i **Mahay HUB** ao amin'ny [dikan-teny farany](https://github.com/raantss18/mahay/releases/latest)
(`hub-…-android-arm64-v8a.apk` ho an'ny Android, `hub-…-linux-x86_64.AppImage` ho an'ny Linux).
**Mahay HUB no mametraka, manokatra ary manavao ny rindranasa hafa rehetra.** Manavao ny tenany koa
ny rindranasa tsirairay.

## Ny tena ilaina

| | **Android** | **Linux** |
|---|---|---|
| **Rafitra** | Android 8.0 (API 26) na vaovao kokoa, finday na takelaka | Linux 64 bit (x86_64) misy glibc 2.39 na vaovao kokoa: Ubuntu 24.04, Debian 13, Fedora 40, Arch |
| **Mpikajy** | ARM 64 bit (`arm64-v8a`); `x86_64` ho an'ny emulateur sy Chromebook | x86_64 |
| **Sary** | GPU misy OpenGL ES (Vulkan raha misy) | Mpamily GPU misy Vulkan na OpenGL |
| **Toerana** | 5 Mo eo ho eo ho an'i Mahay HUB, hatramin'ny 25 Mo eo ho eo isaky ny rindranasa | mitovy |
| **Aterineto** | Tsy ilaina afa-tsy amin'ny fakana an'i Mahay HUB sy ny fanavaozana | mitovy |
| **Hafa** | Avelao hametraka rindranasa tsy fantatra i Mahay HUB | Mila FUSE 2 (`libfuse2`) ny AppImage |

Jereo ny [docs/STATUS.md](docs/STATUS.md) hahalalana izay voazaha-toetra tokoa.

## Manokatra ny rakitrao

Samy manokatra ny karazana rakitra azy avy ny rindranasa: tsindrio ny tabilao ao amin'ny
Fichiers na Drive dia safidio i **Tabilao**; ny sary ao amin'ny **Sary**, ny PDF ao amin'ny
**Taratasy**, ny antontan-taratasy ao amin'ny **Soratra**, ny fampisehoana ao amin'ny **Seho**,
ny feo ao amin'ny **Feo**, ny horonantsary ao amin'ny **Hetsika**.

## Vaovao foana

Manavao ny tenany i Mahay: mijery ny dikan-teny farany isan'andro ny rindranasa tsirairay (sy i
Mahay HUB ho an'ny rehetra), maka azy, manamarina ny sonia (Ed25519), dia manolotra **Apetraho**.

**Amin'ny Android, i Mahay HUB no mikarakara ny rindranasa hafa.** Apetraho aloha i Mahay HUB,
avy eo ny rindranasanao avy aminy: manomboka amin'ny Android 12, izy no mavao azy ireo, rehefa
misokatra, isan'andro ary ao ambadika, tsy mangataka fanamafisana isaky ny rindranasa. Ny
fametrahana voalohany isaky ny rindranasa dia mila tsindrona foana (ny **Apetraho daholo** dia
mandahatra azy ireo, fanamafisana iray isaky ny rindranasa); alohan'ny Android 12 dia
ekena ny fanavaozana rehetra; ny rindranasa misokatra dia avela hatrany aorian'ny fanavaozana
ao ambadika. Ny Wi-Fi ihany no ampiasaina ao ambadika, raha tsy mampiasa ny **Amin'ny data finday
koa** ao amin'ny fanamboarana. Mandeha sy mavao ho azy koa ny rindranasa tsirairay raha tsy
misy ny hub.

## Fahazoan-dalana

[MIT](LICENSE-MIT) na [Apache-2.0](LICENSE-APACHE), araka ny safidinao. Voatahiry ny fanamarihana
copyright an'ny upstream.
