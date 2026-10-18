<p align="center">
  <a href="https://github.com/raantss18/mahay">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="docs/brand/mahay-logo-white.svg">
      <img alt="Mahay" src="docs/brand/mahay-logo.svg" width="200">
    </picture>
  </a>
</p>

<h1 align="center">Mahay Trano</h1>

<p align="center">
  <b>One place to install, update and open every app in the Mahay App Suite, rebuilt in pure Rust.</b><br>
  Sary, Hazavana, Hetsika, Pejy, Tsipika, Taratasy, Tselatra, Soratra,<br>
  Seho, Tabilao, Drafitra, Feo and Endrika: the right build for your computer, kept up to date.
</p>

<p align="center">
  <img alt="100% Rust" src="https://img.shields.io/badge/100%25-Rust-b7410e?style=flat-square&logo=rust">
  <img alt="macOS · Windows · Linux · FreeBSD · Web" src="https://img.shields.io/badge/macOS%20%C2%B7%20Windows%20%C2%B7%20Linux%20%C2%B7%20FreeBSD%20%C2%B7%20Web-native-26282e?style=flat-square">
  <img alt="License: MIT OR Apache-2.0" src="https://img.shields.io/badge/license-MIT%20%2F%20Apache--2.0-3a3a3a?style=flat-square">
  <img alt="Status: early alpha" src="https://img.shields.io/badge/status-early%20alpha-d69e2e?style=flat-square">
</p>

<p align="center">
  <a href="https://github.com/raantss18/mahay/discussions"><img alt="Join the Mahay community on Discord" src="https://img.shields.io/badge/Join%20us%20on%20Discord-5865F2?style=for-the-badge&logo=discord&logoColor=white" height="40"></a>
</p>

<p align="center">
  <a href="https://github.com/raantss18/mahay/releases/latest"><b>Download Mahay Trano</b></a> ·
  <a href="https://github.com/raantss18/mahay">Mahay</a> ·
  <a href="https://github.com/raantss18/mahay">The Mahay App Suite</a>
</p>

> [!NOTE]
> **Mahay is a community of artists from all walks of life.** Painters, photographers,
> filmmakers, illustrators, designers, animators, hobbyists, and people who picked up a pencil
> last week. If you make things, you're one of us. **[Come say hi on Discord](https://github.com/raantss18/mahay/discussions).**

<p align="center">
  <a href="#what-it-does">What it does</a> ·
  <a href="#features">Features</a> ·
  <a href="#downloads">Downloads</a> ·
  <a href="#build-from-source">Build from source</a> ·
  <a href="#the-mahay-app-suite">Mahay App Suite</a> ·
  <a href="https://github.com/raantss18/mahay/discussions">Discord</a>
</p>

<p align="center">
  <img src="docs/screenshots/trano-grid.png" alt="Mahay Trano on macOS in the grid view (dark theme): a card for each Mahay App with its icon, status, version, release date and an Install or Open button" width="900">
</p>
<p align="center">
  <img src="docs/screenshots/trano-table.png" alt="The same apps in the table view (light theme), with columns for each app's status, installed and latest versions, and release date" width="900"><br>
  <sub>Switch between the grid and a sortable table.</sub>
</p>

## What it does

The Mahay apps each ship their own builds for every platform. Mahay Trano saves you from
hunting through a dozen release pages: it lists every app in the suite, shows which ones you have
and which version, installs the right build for your operating system and processor, and tells
you when an update is out. It is a small native app that can sit in your menu bar or system tray.

## Features

<table>
  <tr>
    <td width="33%" valign="top">
      <h3>📦 Install in one click</h3>
      Picks the right download for your OS and CPU (DMG, MSI, AppImage, .deb, .rpm and more), checks it against the release's SHA-256 checksums and installs it.
    </td>
    <td width="33%" valign="top">
      <h3>🔄 Stay up to date</h3>
      Checks each app's GitHub releases and updates the ones you have. A notification tells you when there's something new.
    </td>
    <td width="33%" valign="top">
      <h3>🧭 Always at hand</h3>
      Opens any Mahay app from one window, or from the tray and menu-bar icon. Native egui, no Electron, no web view.
    </td>
  </tr>
</table>

The web version shows the same catalog in the browser, with the latest download for each app.

## Interface languages

The interface is in English, Brazilian Portuguese (Português (Brasil)), Czech (Čeština), French
(Français), German (Deutsch), Hungarian (Magyar), Japanese (日本語), Polish (Polski), Russian
(Русский), Simplified Chinese (简体中文) or Spanish (Español). **Auto**, the default, follows the
system or browser language; **Settings → Language** picks one and switches at once, and the choice
is kept between runs. Czech locales (`cs-CZ`, `cs_CZ.UTF-8`…) select Czech, German ones (`de-DE`,
`de-AT`, `de-CH`, `de_DE.UTF-8`…) German, French ones (`fr-FR`, `fr-CA`…) French, Hungarian ones
(`hu`, `hu-HU`, `hu_HU.UTF-8`…) Hungarian, Japanese ones (`ja`, `ja-JP`, `ja_JP.UTF-8`…) Japanese,
Polish ones (`pl`, `pl-PL`, `pl_PL.UTF-8`) Polish, Spanish ones (`es-ES`, `es-MX`, `es-419`…)
Spanish, and Portuguese ones (`pt-BR`, and `pt-PT` too, for now) Brazilian Portuguese. Simplified
Chinese locales (`zh-CN`, `zh-SG`, `zh-Hans`, and POSIX forms such as `zh_CN.UTF-8`) select
Chinese; Traditional Chinese ones don't. To try a language without changing the setting, start the
desktop app with `TRANO_LOCALE=pt-BR`, or add `?lang=ru` to the web address.

The CJK languages are drawn with the system's own CJK font (Hiragino Sans or PingFang on macOS,
Yu Gothic, Meiryo or Microsoft YaHei on Windows, Noto Sans CJK, IPA, Takao or WenQuanYi on Linux and
FreeBSD). Where the system has no font for one, and in the web version, the interface stays in
English and Settings says why. Product names and release notes stay as published; text without a
translation falls back to English.

## Where apps are installed

Mahay Trano installs each app where your system normally keeps applications, using the
app's official release package, so an app installed by the launcher is the same as one you
installed by hand (and the launcher finds apps you installed yourself):

| System | Where | How |
|---|---|---|
| macOS | `/Applications/<App>.app` | Copied from the signed, notarized release DMG |
| Windows | `C:\Program Files\<App>` | The release MSI, through Windows Installer (with Start menu entry and Apps & features entry) |
| Linux | `/usr/bin`, `/usr/share/applications` | The release `.deb` (apt) or `.rpm` (dnf, zypper), with your system's password prompt. Without those, a Flatpak, or an AppImage in `~/Applications` |
| Linux, inside the launcher's Flatpak, with no Flatpak on the host | `~/.var/app/org.mahay.trano/data/trano/apps/<app>` | The release tarball, unpacked into the launcher's own folder and run inside the same sandbox, with its settings in a `home/` folder beside it. Nothing is installed on the host, and each app's configuration stays in the sandbox |
| FreeBSD | `/usr/local` | The release tree, with a password prompt (polkit); `~/.local` when polkit isn't installed |

Mahay Trano keeps itself current too: when a new version is out, **Update and restart**
downloads it, verifies it, puts it where this copy is installed (the `.app`, the Windows Installer
package or portable folder, the AppImage, or the `.deb`/`.rpm`) and reopens it.

Updates use the same format an app was installed with. Every download is checked against the
release's published SHA-256 before it is installed, and a running app is never replaced.

The launcher writes its log to `logs/trano.log` in its data folder
(`~/.local/share/trano` on Linux, `~/Library/Application Support/Mahay Trano`
on macOS, `%LOCALAPPDATA%\Mahay Trano` on Windows, `MahayLauncherData` beside the
portable zip's executable) and keeps the previous two runs as `trano.1.log` and
`trano.2.log`. `RUST_LOG` changes what it records (`debug`, `info,zbus=warn`, …).

## Downloads

Download the build for your computer from the [latest release](https://github.com/raantss18/mahay/releases/latest), or browse [all releases](https://github.com/raantss18/mahay/releases). Every release has a `SHA256SUMS.txt` to check the files against. `<ver>` is the version number, for example `0.1.0`.

**Windows** (10 and 11)

| File | What it is |
|---|---|
| `trano-<ver>-windows-x64.msi` | Installer for most PCs (Intel and AMD, 64-bit). Installs for the current user, no admin rights needed |
| `trano-<ver>-windows-arm64.msi` | Installer for Windows on Arm (Snapdragon and other Arm PCs) |
| `trano-<ver>-windows-x86.msi` | Installer for 32-bit Windows |
| `trano-<ver>-windows-{x64,arm64,x86}-portable.zip` | No install: unzip and run `trano.exe`. Keeps its settings in the folder |

**macOS** (11 Big Sur or later)

| File | What it is |
|---|---|
| `trano-<ver>-macos-universal.dmg` | Universal app for Apple silicon and Intel Macs, signed and notarized. Drag Mahay Trano to Applications |

**Linux** (x86_64 and aarch64; glibc 2.35 or newer, e.g. Ubuntu 22.04+, Debian 12+, Fedora 36+)

| File | What it is |
|---|---|
| `trano-<ver>-linux-{x86_64,aarch64}.AppImage` | Runs on almost any distro without installing; `chmod +x` and run. The `.AppImage.zsync` beside it lets AppImageUpdate fetch only what changed |
| `trano-<ver>-linux-{x86_64,aarch64}.deb` | Debian, Ubuntu, Mint, Pop!_OS: `sudo apt install ./trano-…deb` |
| `trano-<ver>-linux-{x86_64,aarch64}.rpm` | Fedora, openSUSE, RHEL: `sudo dnf install ./trano-…rpm` |
| `trano-<ver>-linux-{x86_64,aarch64}.flatpak` | Flatpak bundle: `flatpak install --user trano-…flatpak`. Inside the sandbox the launcher installs apps as Flatpaks on the host through `flatpak-spawn --host`; where the host has no Flatpak it falls back to each app's release tarball, unpacked and run inside the sandbox with the app's settings kept in it. Either way it can only reach the host as far as that permission allows; the AppImage, .deb and .rpm builds are the recommended way to run the launcher on Linux |
| `trano-<ver>-linux-{x86_64,aarch64}.tar.gz` | Plain tree (`bin/`, `share/`) for everyone else and for scripts |

RISC-V (riscv64): `trano-<ver>-linux-riscv64.{deb,rpm,tar.gz}`, built against Ubuntu 24.04 (needs glibc 2.39+).

The tray icon uses the StatusNotifierItem standard: KDE, Xfce, Cinnamon and most other desktops show it; on GNOME install the AppIndicator extension.

**FreeBSD** (14, x86_64)

| File | What it is |
|---|---|
| `trano-<ver>-freebsd-x86_64.tar.gz` | A tree laid out like `/usr/local` |

```sh
pkg install libxkbcommon wayland libX11 libXcursor libXrandr libXi libxcb mesa-libs vulkan-loader fontconfig freetype2
tar -xzf trano-<ver>-freebsd-x86_64.tar.gz --strip-components 1 -C /usr/local
trano
```

**Web (WebAssembly)**

| File | What it is |
|---|---|
| `trano-web-<ver>.zip` | The catalog as a static site you can host anywhere; see [`packaging/web/README.md`](packaging/web/README.md) |

## Build from source

You need a recent stable Rust (1.90 or newer). On Linux, install the windowing headers first
(Debian/Ubuntu: `sudo apt install libxkbcommon-dev libwayland-dev libx11-dev libxrandr-dev libxi-dev libgl1-mesa-dev`).

```sh
git clone https://github.com/raantss18/mahay
cd mahay-launcher
cargo run --release -p trano   # the desktop app
cargo xtask ci                             # fmt, clippy, tests and the wasm check
```

The web version builds with [trunk](https://trunkrs.dev/):

```sh
rustup target add wasm32-unknown-unknown
cd apps/trano-web && trunk serve
```

### Installing your own builds

To install and update the Mahay apps from checkouts on your machine instead of GitHub, clone them
next to this repo (`../sary`, `../soratra`, …) and run:

```sh
cargo xtask local sync      # git pull every app, build and package each new version into ../local-mirror
cargo xtask local serve     # serve the builds and a release feed on http://127.0.0.1:8787/
TRANO_DOWNLOAD_BASE=http://127.0.0.1:8787/ cargo run --release -p trano --features local-mirror
```

Run `sync` again whenever you want updates, or let the server do it: `cargo xtask local serve
--sync-every 1h` pulls and builds on that interval while it serves. An app is rebuilt when its
checkout's version isn't in the mirror yet (`--rebuild` forces it), and the launcher's next check
picks it up. Add `--every-commit` to build every new commit instead, as a dev version between
releases (`0.4.0` builds as `0.4.1-dev.<commit time>`), which the launcher offers as an update. While
`TRANO_DOWNLOAD_BASE` is set (loopback addresses only), the launcher asks only the
mirror, never GitHub. Only builds with the `local-mirror` feature read it; releases never have it. `cargo xtask local help` lists the options (`--only`, `--no-pull`, `--arch`, …).

New contributors and AI agents: start with [`AGENTS.md`](AGENTS.md). Release packaging lives in
[`packaging/`](packaging/) and [`.github/workflows/release.yml`](.github/workflows/release.yml).

> [!IMPORTANT]
> **Status:** Mahay Trano is new and in early alpha. Expect rough edges, and please file
> issues (include your OS, which app you were installing and the launcher's log file) or tell us on
> [Discord](https://github.com/raantss18/mahay/discussions).

## The Mahay App Suite

Mahay Trano installs the **Mahay App Suite**: free, open-source creative tools from the
[Mahay](https://github.com/raantss18/mahay) team, each written from scratch in Rust and each able to
stand on its own.

| | App | What it's for | Code | Learn more |
|:-:|---|---|---|---|
| <img src="https://raw.githubusercontent.com/raantss18/sary/main/assets/app-icon/hicolor/64x64/apps/org.mahay.sary.png" alt="" width="32" height="32"> | **Sary** | Image editing: layers, masks, type and real PSD files | [GitHub](https://github.com/raantss18/mahay) | [Website](https://github.com/raantss18/mahay) |
| <img src="https://raw.githubusercontent.com/raantss18/tsipika/main/assets/app-icon/hicolor/64x64/apps/org.mahay.tsipika.png" alt="" width="32" height="32"> | **Tsipika** | Vector illustration | [GitHub](https://github.com/raantss18/mahay) | [Website](https://github.com/raantss18/mahay) |
| <img src="https://raw.githubusercontent.com/raantss18/hetsika/main/assets/app-icon/hicolor/64x64/apps/org.mahay.hetsika.png" alt="" width="32" height="32"> | **Hetsika** | Video editing, color and sound | [GitHub](https://github.com/raantss18/mahay) | [Website](https://github.com/raantss18/mahay) |
| <img src="https://raw.githubusercontent.com/raantss18/hazavana/main/assets/app-icon/hicolor/64x64/apps/org.mahay.hazavana.png" alt="" width="32" height="32"> | **Hazavana** | Photo library and raw development | [GitHub](https://github.com/raantss18/mahay) | [Website](https://github.com/raantss18/mahay) |
| <img src="https://raw.githubusercontent.com/raantss18/taratasy/main/assets/app-icon/hicolor/64x64/apps/org.mahay.taratasy.png" alt="" width="32" height="32"> | **Taratasy** | Reading, organizing and protecting PDFs | [GitHub](https://github.com/raantss18/mahay) | [Website](https://github.com/raantss18/mahay) |
| <img src="https://raw.githubusercontent.com/raantss18/tselatra/main/assets/app-icon/hicolor/64x64/apps/org.mahay.tselatra.png" alt="" width="32" height="32"> | **Tselatra** | Motion graphics and visual effects | [GitHub](https://github.com/raantss18/mahay) | [Website](https://github.com/raantss18/mahay) |
| <img src="https://raw.githubusercontent.com/raantss18/pejy/main/assets/app-icon/hicolor/64x64/apps/org.mahay.pejy.png" alt="" width="32" height="32"> | **Pejy** | Page layout and publishing | [GitHub](https://github.com/raantss18/mahay) | [Website](https://github.com/raantss18/mahay) |
| <img src="https://raw.githubusercontent.com/raantss18/endrika/main/assets/app-icon/hicolor/64x64/apps/org.mahay.endrika.png" alt="" width="32" height="32"> | **Endrika** | Interface design and prototyping | [GitHub](https://github.com/raantss18/mahay) | [Website](https://github.com/raantss18/mahay) |
| <img src="https://raw.githubusercontent.com/raantss18/soratra/main/assets/app-icon/hicolor/64x64/apps/org.mahay.soratra.png" alt="" width="32" height="32"> | **Soratra** | Word processing and documents | [GitHub](https://github.com/raantss18/mahay) | [Website](https://github.com/raantss18/mahay) |
| <img src="https://raw.githubusercontent.com/raantss18/feo/main/assets/app-icon/hicolor/64x64/apps/org.mahay.feo.png" alt="" width="32" height="32"> | **Feo** | Audio recording and editing | [GitHub](https://github.com/raantss18/mahay) | [Website](https://github.com/raantss18/mahay) |
| <img src="https://raw.githubusercontent.com/raantss18/seho/main/assets/app-icon/hicolor/64x64/apps/org.mahay.seho.png" alt="" width="32" height="32"> | **Seho** | Presentations and slide decks | [GitHub](https://github.com/raantss18/mahay) | [Website](https://github.com/raantss18/mahay) |
| <img src="https://raw.githubusercontent.com/raantss18/drafitra/main/assets/app-icon/hicolor/64x64/apps/org.mahay.drafitra.png" alt="" width="32" height="32"> | **Drafitra** | Computer-aided design | [GitHub](https://github.com/raantss18/mahay) | [Website](https://github.com/raantss18/mahay) |
| <img src="https://raw.githubusercontent.com/raantss18/tabilao/main/assets/app-icon/hicolor/64x64/apps/org.mahay.tabilao.png" alt="" width="32" height="32"> | **Tabilao** | Spreadsheets and data | [GitHub](https://github.com/raantss18/mahay) | [Website](https://github.com/raantss18/mahay) |

And [**Mahay**](https://github.com/raantss18/mahay) itself, our AI image and video studio for artists who want real control.

<br>

<p align="center">
  <a href="https://github.com/raantss18/mahay/discussions"><img alt="Join the Mahay community on Discord" src="https://img.shields.io/badge/Join%20us%20on%20Discord-5865F2?style=for-the-badge&logo=discord&logoColor=white" height="40"></a>
</p>

<h3 align="center">Come make things with us</h3>

<p align="center">
  Our Discord is where artists of every kind hang out: people who paint, shoot, draw, cut film,
  set type, and people still figuring out what they like to make. Share what you're working on,
  ask for help, tell us what's broken, or tell us what you wish these tools could do.
  Whatever your medium and however long you've been at it, you're welcome here.
</p>

<p align="center">
  <a href="https://github.com/raantss18/mahay/discussions"><b>discord.gg/mahay</b></a> ·
  <a href="https://github.com/raantss18/mahay">mahay.com</a> ·
  <a href="https://github.com/raantss18/mahay">The Mahay App Suite</a> ·
  <a href="https://github.com/raantss18/mahay">Mahay Trano</a>
</p>

---

## Thanks

The community built launchers for the Mahay App Suite before we did, and Mahay Trano learned a
lot from them. Thank you to their authors:

| Project | Author | What we learned from it |
|---|---|---|
| [Mahay Suite Updater](https://github.com/Lyraedan/Mahay-Suit-Updater) | [@Lyraedan](https://github.com/Lyraedan) | Noticing apps installed or removed outside the updater with a quiet background re-scan, detecting Arch and AUR installs through pacman, and a platform picker that lists the downloads for every OS |
| [craftcenter](https://github.com/avokadosauce/craftcenter) | [@avokadosauce](https://github.com/avokadosauce) | Finding the latest release without spending API quota (the `releases/latest` redirect plus `SHA256SUMS.txt`), and verifying every file |
| [CRAFTSPACE](https://github.com/emircesur/CRAFTSPACE) | [@emircesur](https://github.com/emircesur) | Adopting apps that were already installed, falling back gracefully when GitHub rate-limits, background update notifications |
| [GetCraft](https://github.com/mbirnbach/getcraft) | [@mbirnbach](https://github.com/mbirnbach) | Picking the right download for each OS and CPU, recognising Windows Installer and macOS installs, a native Rust/egui launcher |
| [MahayHub](https://github.com/MahayHubLauncher/MahayHub) | [MahayHubLauncher](https://github.com/MahayHubLauncher) | Strict checks on where downloads come from, clear rate-limit messages, never replacing an app while it runs |
| [mahay-suite](https://github.com/erikerikerik/mahay-suite) | [@erikerikerik](https://github.com/erikerikerik) | ETag-cached release checks and installs that recover cleanly from interruptions |
| [Mahay Apps Launcher & Installer](https://github.com/ZifuM/Mahay-apps-launcher-installer) | [@ZifuM](https://github.com/ZifuM) | Release-page fallbacks and starting AppImage apps with a clean environment |
| [Mahay Trano](https://github.com/EricVogt93/trano) | [@EricVogt93](https://github.com/EricVogt93) | Desktop integration for AppImages and a careful self-update design |
| [LandingCraft](https://github.com/rallegretti/LandingCraft) | [@rallegretti](https://github.com/rallegretti) | Working with the Windows Installer directly and offering stable releases only |
| [Mahay Apps Manager](https://github.com/CryptoKey98/mahay-apps-manager) | [@CryptoKey98](https://github.com/CryptoKey98) | Scheduled background update checks and per-app launch options |
| [Mahay Hub Launcher](https://github.com/tinyplayerss/mahay-hub-launcher) | [@tinyplayerss](https://github.com/tinyplayerss) | A friendly, one-window hub for the whole suite |
| [MahayHub](https://github.com/MegaTheLEGEND/MahayHub) | [@MegaTheLEGEND](https://github.com/MegaTheLEGEND) | Serving the apps' web builds from a self-hosted launcher |

## License and credits

Mahay Trano is dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
Copyright (c) 2026 Mahay Team and the Mahay Trano contributors. Required notices are in [NOTICE](NOTICE).

Bundled fonts, icons, images and other assets keep their own open licenses; each one is listed
with its author, source and license in [ATTRIBUTION.md](ATTRIBUTION.md).

The app icons of the Mahay apps (`assets/app-icons/`) are each app's own icon, copied unmodified
from its repository and licensed like that app (MIT OR Apache-2.0).

**The Mahay name, logo and "A" mark are not licensed under MIT or Apache-2.0.** They (everything
in [`docs/brand/`](docs/brand/), plus the app icon in [`assets/app-icon/`](assets/app-icon/), the
logo in the launcher's header and the "A" tray icon)
are trademarks of the Mahay Team and are not covered by this license. They may be used only unmodified, and only as
part of this repository and Mahay Trano, under [`docs/brand/LICENSE-brand.txt`](docs/brand/LICENSE-brand.txt).
Forks and modified versions must remove them.

<p align="center">
  <a href="https://github.com/raantss18/mahay"><img alt="Mahay" src="docs/brand/mahay-mark.svg" width="28"></a><br>
  <sub>Made by the <a href="https://github.com/raantss18/mahay">Mahay</a> team and community.</sub>
</p>
