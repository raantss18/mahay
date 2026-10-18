<p align="center">
  <img src="docs/img/mahay-logo.svg" alt="Mahay" width="360">
</p>

> **Language / Langue / Fiteny:** **English** · [Français](README.fr.md) · [Malagasy](README.mg.md)

**Mahay** (Malagasy: *skilled, able*) is a free, local-first suite of creative and office apps
for **Linux** and **Android phones and tablets**. Each app is native Rust: no Electron, no
WebView, no account and no telemetry. Your files stay on your device.

Mahay is a fork of the [ArtCraft](https://github.com/storytold) crafting apps, renamed and
ported to Android. See [NOTICE](NOTICE).

> **Status: alpha (0.1).** The apps work, but expect rough edges, especially on phones. See
> [docs/STATUS.md](docs/STATUS.md) for what works, what doesn't and what was tested.

## The apps

<p align="center"><img src="docs/img/icons.png" alt="The Mahay icon family" width="760"></p>

| App          | What it does                                    | Linux | Android |
|--------------|-------------------------------------------------|-------|---------|
| **Mahay HUB**    | the hub: opens and lists the apps               | yes   | yes     |
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

Download **Mahay HUB**, the Mahay hub, from the [latest release](https://github.com/raantss18/mahay/releases/latest):
`hub-…-android-arm64-v8a.apk` on Android 8+ (phones and tablets), `hub-…-linux-x86_64.AppImage`
on Linux. **Mahay HUB installs, opens and updates every other app**, straight from the release.
Every app also updates itself (checked daily, signed files, you confirm the install).

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
