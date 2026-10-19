<p align="center">
  <img src="docs/img/mahay-logo.svg" alt="Mahay" width="360">
</p>

> **Language / Langue / Fiteny:** **English** · [Français](README.fr.md) · [Malagasy](README.mg.md)

**Mahay** (Malagasy: *skilled, able*) is a free, local-first suite of creative and office apps
for **Linux** and **Android phones and tablets**. Each app is native Rust: no Electron, no
WebView, no account and no telemetry. Your files stay on your device.

Mahay is a fork of the [ArtCraft](https://github.com/storytold) crafting apps, renamed and
ported to Android. See [NOTICE](NOTICE).

> **Status: alpha (0.2).** The apps work, but expect rough edges, especially on phones. See
> [docs/STATUS.md](docs/STATUS.md) for what works, what doesn't and what was tested.

## The apps

<p align="center"><img src="docs/img/icons.png" alt="The Mahay icon family" width="760"></p>

| App          | What it does                                    | Linux | Android |
|--------------|-------------------------------------------------|-------|---------|
| **Mahay HUB** | the hub: installs, opens and updates every app  | yes   | yes     |
| **Sary**     | image editor (layers, PSD, raw)                 | yes   | alpha   |
| **Taratasy** | PDF: read, annotate, fill, sign, organise       | yes   | alpha   |
| **Soratra**  | word processor (DOCX, ODT)                      | yes   | alpha   |
| **Tabilao**  | spreadsheet (XLSX, ODS, CSV)                    | yes   | alpha   |
| **Seho**     | presentations (PPTX)                            | yes   | alpha   |
| **Tsilo**    | vector illustration                             | yes   | alpha   |
| **Endrika**  | interface design and prototyping                | yes   | alpha   |
| **Pejy**     | page layout and publishing                      | yes   | alpha   |
| **Hazavana** | photo library and raw development               | yes   | alpha   |
| **Feo**      | recording, editing and mixing audio             | yes   | alpha   |
| **Hetsika**  | video editing                                   | yes   | alpha   |
| **Tselatra** | motion graphics and visual effects              | yes   | alpha   |
| **Drafitra** | technical drawing (CAD)                         | yes   | alpha   |


## Install: just Mahay HUB

Download **Mahay HUB** from the [latest release](https://github.com/raantss18/mahay/releases/latest):
`hub-…-android-arm64-v8a.apk` on Android 8+ (phones and tablets), `hub-…-linux-x86_64.AppImage`
on Linux. **Mahay HUB installs, opens and updates every other app**, straight from the release.
Every app also updates itself (checked daily, signed files, you confirm the install).

## Works with your files

Every app opens its own kind of file from anywhere on the device: tap a spreadsheet in Files,
Drive or an e-mail and choose **Tabilao**; a picture opens in **Sary** (or is imported into
**Hazavana**), a PDF in **Taratasy**, a document in **Soratra**, slides in **Seho**, an SVG in
**Tsilo**, audio in **Feo**, a video in **Hetsika**, a DXF/DWG drawing in **Drafitra**. "Share"
works too. Saving writes back to the file you opened.

## Always up to date

Mahay updates itself, like Tontolo: each app (and Mahay HUB for all of them) checks the latest
release when it starts and once a day, downloads it in the background, checks its signature
(Ed25519) and SHA-256, then offers **Install**. On Android you confirm with one tap; on Linux the
AppImage replaces itself and restarts. You can turn it off in **About**.

**On Android, Mahay HUB looks after the other apps.** Install Mahay HUB first, then your apps
from it: on Android 12+ it updates them for you, at start, every day and in the background,
without asking you to confirm each one. An app you installed some other way joins after one
confirmed update. Limits Android imposes: **the first install of each app always needs a tap**
(Mahay HUB's **Install all** queues them, one confirmation each); below Android 12 every update is
confirmed (the hub's notification says how many are waiting); an app that is open when it is
updated restarts, so the background update leaves open apps for later. Background updates use
Wi-Fi only, unless you switch on **Also on mobile data** in the hub's settings. Every app also
works and updates alone, without the hub.

## Light by design

- One APK per app: you only download what you use (from about 5 MB for Mahay HUB to about 25 MB for the largest apps).
- Native code with whole-program optimisation, no browser engine inside.
- Works fully offline. Documents open from, and save back to, wherever you keep them (Files,
  Drive, a USB stick) through Android's own document picker.

## Build from source

```bash
# Linux desktop app (from its folder)
cd apps/tabilao && cargo run --release -p tabilao
```

```bash
# Android APKs (needs the Android SDK + NDK 28, rustup with the Android targets, cargo-ndk)
scripts/android-build.sh --abi arm64-v8a tabilao
```

Details: [docs/BUILDING.md](docs/BUILDING.md). Design decisions: [docs/DECISIONS.md](docs/DECISIONS.md).
Porting notes: [docs/PORTING_AUDIT.md](docs/PORTING_AUDIT.md).

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option
(`SPDX: MIT OR Apache-2.0`). Upstream copyright notices are kept in every imported tree.
Third-party licenses: [THIRD_PARTY_LICENSES.md](THIRD_PARTY_LICENSES.md).
