<p align="center">
  <a href="https://github.com/raantss18/mahay">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="docs/brand/mahay-logo-white.svg">
      <img alt="Mahay" src="docs/brand/mahay-logo.svg" width="200">
    </picture>
  </a>
</p>

<h1 align="center">Seho</h1>

<p align="center">
  <b>Presentations and slide shows; an open-source, clean-room reimplementation of Microsoft PowerPoint, rebuilt in pure Rust.</b>
</p>

<p align="center">
  Build decks with themes, layouts, shapes, pictures, tables, charts, transitions and animations,
  open and save PowerPoint files, and present them full screen. Native on macOS, Windows, Linux
  and FreeBSD, and in the browser. Every button is also a command that the CLI and AI agents
  (through MCP) can drive.
</p>

<p align="center">
  <img alt="Status: early development" src="https://img.shields.io/badge/status-early%20development-f26b1d?style=flat-square">
  <img alt="License: MIT OR Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue?style=flat-square">
  <img alt="Written in Rust" src="https://img.shields.io/badge/written%20in-Rust-b7410e?style=flat-square">
  <img alt="Platforms" src="https://img.shields.io/badge/platforms-macOS%20%C2%B7%20Windows%20%C2%B7%20Linux%20%C2%B7%20BSD%20%C2%B7%20Web-555?style=flat-square">
</p>

<p align="center">
  <a href="https://github.com/raantss18/mahay/discussions"><img alt="Join the Mahay community on Discord" src="https://img.shields.io/badge/Join%20us%20on%20Discord-5865F2?style=for-the-badge&logo=discord&logoColor=white" height="40"></a>
</p>

<p align="center">
  <a href="https://github.com/raantss18/mahay"><b>Seho on mahay.com</b></a> ·
  <a href="https://github.com/raantss18/mahay">Mahay</a> ·
  <a href="https://github.com/raantss18/mahay">All Mahay apps</a>
</p>

> [!NOTE]
> **Mahay is a community of artists from all walks of life.** Painters, photographers,
> filmmakers, illustrators, designers, animators, hobbyists, and people who picked up a pencil
> last week. If you make things, you're one of us. **[Come say hi on Discord](https://github.com/raantss18/mahay/discussions).**

<p align="center">
  <img alt="Seho editing a slide: ribbon, slide thumbnails with sections, a process diagram on the canvas, notes and status bar" src="docs/images/editor.png" width="900">
</p>

<p align="center">
  <a href="#what-works-today">What works</a> ·
  <a href="#getting-started">Getting started</a> ·
  <a href="#agents-cli-and-mcp">Agents, CLI and MCP</a> ·
  <a href="#roadmap">Roadmap</a> ·
  <a href="#downloads">Downloads</a> ·
  <a href="#the-crafting-apps">The Mahay apps</a> ·
  <a href="#license-and-credits">License</a>
</p>

## What works today

Seho is in alpha, heading for beta (see the [roadmap](ROADMAP.md)).
Today it can:

- **Make decks**: slide masters and eleven layouts with placeholders that inherit position and
  formatting, eight original themes with colour and font schemes, sections, notes, comments,
  headers and footers, custom slide sizes.
- **Draw**: 150+ preset shapes with adjust handles, connectors that stay glued to shapes, freeform,
  curve and scribble tools, merge shapes (union, combine, fragment, intersect, subtract), gradient,
  picture and pattern fills, outlines with dashes and arrowheads, shadows, glow, soft edges and
  reflections.
- **Write**: in-place text editing with full font and paragraph formatting, bullets and numbering,
  columns, autofit, vertical text, fields and hyperlinks.
- **Add content**: pictures with crop and corrections, tables with styles, charts, SmartArt basics,
  audio and video (MP3, AAC, FLAC, ALAC, Ogg, Opus, WAV, AIFF; H.264, HEVC, VP9, AV1).
- **Present**: transitions (including Morph), entrance/emphasis/exit and motion-path animations,
  by-paragraph builds, full-screen slide show, presenter view, pen, rehearse timings, media playback.
- **Exchange files**: open and save PowerPoint `.pptx`, export PDF (slides, notes pages, handouts
  with selectable text), PNG/JPEG, an animated GIF of the deck and outlines. Unsaved work is kept by AutoRecover.
- **Automate**: 200+ commands with undo, all scriptable from the CLI, a JSON control channel and an
  MCP server for AI agents.

<table>
  <tr>
    <td width="50%"><img alt="A selected shape with the Shape Format tab" src="docs/images/shape-format.png"><br><sub>Shape Format contextual tab and selection handles</sub></td>
    <td width="50%"><img alt="The Format Shape pane editing a gradient fill" src="docs/images/format-pane.png"><br><sub>Format Shape pane: gradient type, direction and stops</sub></td>
  </tr>
  <tr>
    <td width="50%"><img alt="Slide Sorter view with transitions under each slide" src="docs/images/slide-sorter.png"><br><sub>Slide Sorter with each slide's transition</sub></td>
    <td width="50%"><img alt="Seho in dark mode showing a chart slide" src="docs/images/dark-mode.png"><br><sub>Dark mode, with a native chart</sub></td>
  </tr>
</table>

<p align="center">
  <img alt="All eight slides of the Northern Lights sample deck" src="docs/images/sample-deck.png" width="900"><br>
  <sub>The built-in sample deck, rendered by <code>seho-cli render --sample --all</code></sub>
</p>

## Getting started

PDF export uses the shared `mahay-pdf` writer with import features disabled. See
[`crates/pdf/README.md`](crates/pdf/README.md) for the dependency and validation requirements.

```sh
git clone https://github.com/raantss18/mahay
cd seho
cargo run --release -p seho-cli -- render --sample --all --scale 1 out/
```

On Linux, the build also needs the ALSA headers for slide-show sound: `alsa-lib-devel` on Fedora,
`libasound2-dev` on Debian and Ubuntu, `alsa-lib` on Arch. Without them it stops at `alsa-sys`.

Fonts come from [mahay-fonts](https://github.com/raantss18/mahay): clone it next to this repo
(`../mahay-fonts`) and local builds pick it up automatically; without it Seho uses your system
fonts.

Each [GitHub release](https://github.com/raantss18/mahay/releases) has ready-made builds, on Linux as
an AppImage, a `.deb`, an `.rpm` and a tarball. On Gentoo, the community [::snakebyte
overlay](https://github.com/switch87/snakebyte-overlay) packages the Linux release as
`app-office/seho-bin` (not maintained by the Seho team):

```sh
eselect repository add snakebyte git https://github.com/switch87/snakebyte-overlay.git
emaint sync -r snakebyte
echo 'app-office/seho-bin ~amd64' >> /etc/portage/package.accept_keywords/seho
emerge --ask app-office/seho-bin
```

### Logs

The desktop app writes its `log` records to standard error and to `logs/seho.log` in the
settings directory, beside `ui.json` and `prefs.json` (Linux `~/.config/seho/logs/`, or
`$XDG_CONFIG_HOME/seho/logs/`; macOS `~/Library/Application Support/Seho/logs/`; Windows
`%APPDATA%\Seho\logs\`). A start from a desktop menu or the Dock has no terminal, so this file
is what to attach to a bug report: the crash guard's panic report, a failed crash-recovery save,
PowerPoint parts skipped on import and audio output problems land there. Each launch moves the
previous log to `seho.1.log` (and that one to `seho.2.log`), so the log of a run that
crashed survives the next start. The file stops growing at 16 MiB. `--version` writes no file.

| Variable | Effect |
|---|---|
| `RUST_LOG` | Log levels for standard error and the log file. Default: `info` for Seho's own crates, `warn` for everything else. env_logger-style directives replace that, e.g. `RUST_LOG=debug`, `RUST_LOG=warn,seho_pptx=trace` or `RUST_LOG=info,wgpu_core=warn`; a directive ending in `*` covers every target starting with it (`seho*=debug`). |

The logger is `apps/seho/src/logging.rs`; the realtime audio callback does not log. The web
app logs to the browser console instead.

## Agents, CLI and MCP

Every action in Seho is a command with a stable id, so people, scripts and AI agents use the
same verbs:

```sh
seho-cli commands format.          # list commands
seho-cli run --sample --cmd 'slide.new={"layout":"titleOnly","title":"Hello"}' --save hello.seho
seho-cli mcp                       # MCP server over stdio (headless)
seho-cli mcp --connect 7990        # drive the running app (seho --control 7990)
seho-cli mcp --automation-read-root DIR --automation-write-root DIR   # confine the headless agent to folders
```

## Roadmap

Seho is in **alpha**. It covers **84% of PowerPoint's features by breadth** (92% of the core
ones, measured) and is about **55% of the way to replacing PowerPoint for real work** (estimated).
See [ROADMAP.md](ROADMAP.md) for the stage, numbers by dimension and what's next,
[docs/target-app-parity.md](docs/target-app-parity.md) for the full assessment,
[docs/gaps.md](docs/gaps.md) for every known shortfall and
[docs/parity-checklist.md](docs/parity-checklist.md) for the feature-by-feature checklist.

## Downloads

**Download Seho** from GitHub: the [latest release](https://github.com/raantss18/mahay/releases/latest) has every build listed below, and [all releases](https://github.com/raantss18/mahay/releases) has earlier versions and their notes. `<ver>` in the file names is the version number, and `SHA256SUMS.txt` lists a checksum for every file.

### Windows

| Build | Installer | Portable |
|---|---|---|
| x64 (64-bit Intel/AMD) | `seho-<ver>-windows-x64.msi` | `seho-<ver>-windows-x64-portable.zip` |
| arm64 (Snapdragon and other ARM PCs) | `seho-<ver>-windows-arm64.msi` | `seho-<ver>-windows-arm64-portable.zip` |
| x86 (32-bit) | `seho-<ver>-windows-x86.msi` | `seho-<ver>-windows-x86-portable.zip` |

Installers and executables are code-signed.

**If the app doesn't open on Windows:** the desktop app initializes only DirectX 12 by default.
Letting wgpu also create an OpenGL instance can crash some graphics drivers (AMD's
`atio6axx.dll`) before the window appears, so the app would flash in Task Manager and quit.
`WGPU_BACKEND` overrides the default for troubleshooting (for example `dx12` or `vulkan`). In
PowerShell, from the folder containing the executable:

```powershell
$env:WGPU_BACKEND = "vulkan"
& .\seho.exe
Remove-Item Env:WGPU_BACKEND                     # restore the default for later launches
```

An explicit `gl` override can bring the driver crash back on affected systems. The macOS, Linux
and web backend defaults are unchanged.

DirectX 12 shaders are compiled with FXC, which is part of Windows, so no `dxcompiler.dll` or
`dxil.dll` is needed ("Parent device is lost" at start, #153). `$env:WGPU_DX12_COMPILER = "dxc"`
(or `"auto"`) selects wgpu's DXC handling again for troubleshooting.

### macOS

| Build | File | Notes |
|---|---|---|
| App, universal (Apple silicon + Intel) | `seho-<ver>-macos-universal.dmg` | Signed and notarized |
| Command-line tool, universal | `seho-cli-<ver>-macos-universal.zip` | Signed and notarized |

### Linux

| Format | x86_64 | aarch64 (ARM64) | Notes |
|---|---|---|---|
| AppImage | `seho-<ver>-linux-x86_64.AppImage` | `seho-<ver>-linux-aarch64.AppImage` | Runs anywhere; updates itself with [AppImageUpdate](https://github.com/AppImageCommunity/AppImageUpdate) (`.zsync` files) |
| Flatpak | `seho-<ver>-linux-x86_64.flatpak` | `seho-<ver>-linux-aarch64.flatpak` | Sandboxed; `flatpak install --user <file>` |
| Debian/Ubuntu | `seho-<ver>-linux-x86_64.deb` | `seho-<ver>-linux-aarch64.deb` | |
| Fedora/RHEL/openSUSE | `seho-<ver>-linux-x86_64.rpm` | `seho-<ver>-linux-aarch64.rpm` | |
| Tarball | `seho-<ver>-linux-x86_64.tar.gz` | `seho-<ver>-linux-aarch64.tar.gz` | Unpack anywhere |

RISC-V 64 (riscv64): `seho-<ver>-linux-riscv64.tar.gz`, a tarball only (cross-compiled; needs glibc 2.39+, e.g. Ubuntu 24.04+).

### FreeBSD

| Build | File |
|---|---|
| x86_64 | `seho-<ver>-freebsd-x86_64.tar.gz` |

### Web (WebAssembly)

| Build | File | Notes |
|---|---|---|
| Static site | `seho-web-<ver>.zip` | Runs in a modern browser; host it on any static server |

## The Mahay apps

Seho is one of the **Mahay apps**: free, open-source creative tools from the
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
| <img src="https://raw.githubusercontent.com/raantss18/seho/main/assets/app-icon/hicolor/64x64/apps/org.mahay.seho.png" alt="" width="32" height="32"> | **Seho** | **Presentations and slide shows · you are here** | [GitHub](https://github.com/raantss18/mahay) | [Website](https://github.com/raantss18/mahay) |

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
  <a href="https://github.com/raantss18/mahay/discussions"><b>github.com/raantss18/mahay/discussions</b></a> ·
  <a href="https://github.com/raantss18/mahay">mahay.com</a> ·
  <a href="https://github.com/raantss18/mahay">The Mahay apps</a> ·
  <a href="https://github.com/raantss18/mahay">Seho</a>
</p>

## License and credits

Seho is dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
Copyright (c) 2026 Mahay Team and the Seho contributors. Required notices are in [NOTICE](NOTICE).

Bundled fonts, icons, images and other assets keep their own open licenses; each one is listed
with its author, source and license in [ATTRIBUTION.md](ATTRIBUTION.md).

Seho's themes, layouts, shape geometry, icons and sample decks are original work, drawn or
generated in code; no Microsoft artwork, fonts, themes or templates are used.

The Mahay name, wordmark and logos in [`docs/brand/`](docs/brand/) are trademarks of the
Mahay Team and are not covered by this license. They may be used only unmodified, and only as
part of this repository and Seho, under [`docs/brand/LICENSE-brand.txt`](docs/brand/LICENSE-brand.txt).
Forks and modified versions must remove them.

<sub>Microsoft and PowerPoint are trademarks of the Microsoft group of companies. Adobe, Photoshop, Illustrator, Premiere Pro, Lightroom, Acrobat, After Effects and InDesign are trademarks or registered trademarks of Adobe Inc. in the United States and/or other countries. Seho is an independent, open-source project and is not affiliated with, sponsored by or endorsed by Microsoft Corporation or Adobe Inc.; these names are used only to describe the workflows it is compatible with.</sub>

<p align="center">
  <a href="https://github.com/raantss18/mahay"><img alt="Mahay" src="docs/brand/mahay-mark.svg" width="28"></a><br>
  <sub>Made by the <a href="https://github.com/raantss18/mahay">Mahay</a> team and community.</sub>
</p>
