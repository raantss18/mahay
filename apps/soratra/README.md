<p align="center">
  <a href="https://github.com/raantss18/mahay">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="docs/brand/mahay-logo-white.svg">
      <img alt="Mahay" src="docs/brand/mahay-logo.svg" width="200">
    </picture>
  </a>
</p>

<h1 align="center">Soratra</h1>

<p align="center">
  <b>Writing and document design; an open-source, clean-room reimplementation of Microsoft Word, rebuilt in pure Rust.</b>
</p>

<p align="center">
  A fast, open-source word processor with the Word workflow you already know: the ribbon, styles,
  tables, track changes, references and mail merge. It reads and writes .docx, runs natively on
  macOS, Windows, Linux and BSD, and in the browser via WebAssembly.<br>
  <i>By the Mahay team.</i>
</p>

<p align="center">
  <img alt="Written in Rust" src="https://img.shields.io/badge/written%20in-Rust-2b47b5?style=flat-square&logo=rust&logoColor=white">
  <img alt="Runs on macOS, Windows, Linux, BSD and the web" src="https://img.shields.io/badge/runs%20on-macOS%20%C2%B7%20Windows%20%C2%B7%20Linux%20%C2%B7%20BSD%20%C2%B7%20Web-3b5bdb?style=flat-square">
  <img alt="License: MIT OR Apache-2.0" src="https://img.shields.io/badge/license-MIT%20%2F%20Apache--2.0-2b47b5?style=flat-square">
  <img alt="Agent-drivable over MCP" src="https://img.shields.io/badge/agents-MCP%20%C2%B7%20CLI-3b5bdb?style=flat-square">
</p>

<p align="center">
  <a href="https://github.com/raantss18/mahay/discussions"><img alt="Join the Mahay community on Discord" src="https://img.shields.io/badge/Join%20us%20on%20Discord-5865F2?style=for-the-badge&logo=discord&logoColor=white" height="40"></a>
</p>

<p align="center">
  <a href="https://github.com/raantss18/mahay"><b>Soratra on mahay.com</b></a> ·
  <a href="https://github.com/raantss18/mahay">Mahay</a> ·
  <a href="https://github.com/raantss18/mahay">All Mahay apps</a>
</p>

<br>

<p align="center">
  <img src="docs/images/hero.png" alt="Soratra with the Home tab of the ribbon open over a two-page document titled The Open Studio Handbook. The Navigation pane on the left lists the document's headings, the Styles gallery shows live previews of Normal, Heading 1, Title and Subtitle, and a word in the first paragraph is selected." width="100%">
  <br><sub><b>The Open Studio Handbook</b>, Soratra's built-in sample: the ribbon, live Styles gallery, rulers and the Navigation pane.</sub>
</p>

> [!NOTE]
> **Mahay is a community of artists from all walks of life.** Painters, photographers,
> filmmakers, illustrators, designers, animators, hobbyists, and people who picked up a pencil
> last week. If you make things, you're one of us. **[Come say hi on Discord](https://github.com/raantss18/mahay/discussions).**

<p align="center">
  <a href="#a-tour">A tour</a> ·
  <a href="#why-soratra">Why Soratra</a> ·
  <a href="#what-works-today">What works today</a> ·
  <a href="#quick-start">Quick start</a> ·
  <a href="#for-agents-cli-and-mcp">For agents</a> ·
  <a href="#architecture">Architecture</a> ·
  <a href="#roadmap">Roadmap</a> ·
  <a href="#downloads">Downloads</a> ·
  <a href="#the-crafting-apps">The Mahay apps</a> ·
  <a href="#license-and-credits">License and credits</a>
</p>

## A tour

Every screenshot below is Soratra itself, rendered offscreen by its own UI test harness
(`cargo run -p soratra-ui-egui --example ui_shot`).

<table>
<tr>
<td width="50%" valign="top"><img src="docs/images/review.png" alt="The Review tab with Track Changes on: the word forty is inserted in magenta and thirty struck through; three commented phrases are shaded and joined by dashed leader lines to comment balloons in a grey markup area to the right of the page" width="100%"><p align="center"><sub><b>Review.</b> Track changes, comment balloons in the margin, accept and reject, spelling and grammar as you type.</sub></p></td>
<td width="50%" valign="top"><img src="docs/images/references.png" alt="The References tab with two pages side by side: a table of contents with dotted leaders and page numbers on page one, and a styled table, numbered list and hyperlink on page two" width="100%"><p align="center"><sub><b>References.</b> Tables of contents, footnotes, citations in APA, MLA, Chicago or IEEE, index and captions.</sub></p></td>
</tr>
<tr>
<td width="50%" valign="top"><img src="docs/images/design.png" alt="The Design tab showing style-set previews; the document is set in a serif theme with plum headings underlined by thin rules and a pale diagonal DRAFT watermark behind the text" width="100%"><p align="center"><sub><b>Design.</b> Themes, style sets, paragraph spacing, watermarks, page colour and borders.</sub></p></td>
<td width="50%" valign="top"><img src="docs/images/dark.png" alt="Soratra in dark mode with the Insert tab open and formatting marks shown: pilcrows at paragraph ends and dots for spaces" width="100%"><p align="center"><sub><b>Dark mode</b> with formatting marks, and the Insert tab: tables, pictures, shapes, links, headers, footers, fields and symbols.</sub></p></td>
</tr>
<tr>
<td colspan="2"><img src="docs/images/layout.png" alt="The Layout tab at 80% zoom: the first paragraph opens with a three-line drop cap E, a dark blue circle sits beside the second section with its paragraph wrapping around it and hyphenating art-ist at the line end, every line is numbered in the left margin and a thin blue border surrounds the page" width="100%"><p align="center"><sub><b>Layout.</b> Drop caps, text wrapping around pictures and shapes, automatic hyphenation, line numbers and page borders.</sub></p></td>
</tr>
<tr>
<td colspan="2"><img src="docs/images/backstage.png" alt="The File tab start page with a Good morning greeting, thumbnails of the Blank document, Studio handbook, Letter, Résumé and Report templates, and a list of recent documents" width="100%"><p align="center"><sub><b>File.</b> Start from a template, open recent documents, edit properties, export to PDF and other formats.</sub></p></td>
</tr>
</table>

## Why Soratra

- **Familiar.** Word's ribbon tabs, groups, shortcuts and behaviour: Enter continues a list,
  Tab demotes it, Ctrl/⌘+B bolds the word under the caret, the Styles gallery previews styles live,
  F4 repeats, F7 checks spelling, F8 extends the selection.
- **Your files.** Opens and saves .docx (OOXML), and also .odt, .rtf, .html, .md, .tex (LaTeX), .txt; opens Word 97-2003 .doc; exports PDF
  with real, selectable text, links and bookmarks.
- **Fast.** Paragraph layout is cached, so typing in a 188-page document re-lays it out in about
  1.4 ms; pages render on demand.
- **Everywhere.** One Rust codebase for macOS, Windows, Linux, BSD and the web. No Electron, no
  Tauri: native [egui](https://github.com/emilk/egui) on the GPU.
- **Built for agents.** Every action is a command with an id. The same 422 commands drive the
  ribbon, keyboard shortcuts, the command search, a command-line tool, a JSON control channel and
  an MCP server.
- **Private.** Spelling, grammar and everything else work offline.
- **Open.** MIT OR Apache-2.0. Clean-room: built from public specifications and observation, with
  every asset original or openly licensed.

## What works today

| Area | Highlights |
|---|---|
| **Writing** | Fast typing with IME, smart quotes, AutoCorrect, list autoformat (`* `, `1. `), dashes; word, sentence, paragraph selection; drag-select; clipboard with formatting; undo/redo; find and replace with regex |
| **Formatting** | Fonts, sizes, bold/italic/underline styles, strike, sub/superscript, caps, highlight, colours, character spacing, Format Painter, Change Case, Clear Formatting |
| **Paragraphs** | Alignment, indents (draggable on the ruler), spacing, line spacing, tabs with leaders, borders, shading, keep with next, widow/orphan control |
| **Styles** | Built-in style set, live gallery, Styles pane, create/modify/update styles, style sets, themes |
| **Lists** | Bullets, numbering, multilevel, restart, set value, custom formats |
| **Tables** | Insert by grid, merge/split, styles with banded rows, borders, shading, header rows repeated across pages, rows that split across pages, sort, formulas, text ↔ table |
| **Pages** | Margins, orientation, size, columns, page/column/section breaks, headers and footers (first page, odd/even), page numbers, watermark, page borders, line numbers, vertical alignment, drop caps, automatic hyphenation |
| **Objects** | Pictures (resize, crop, recolour, brightness/contrast, transparency, background removal, picture styles, rotate), shapes, text boxes, floating position with text wrapping (square, top and bottom, behind or in front of text) |
| **References** | Table of contents, footnotes and endnotes, citations and bibliography (APA, MLA, Chicago, IEEE), captions, table of figures, cross-references, index, table of authorities |
| **Review** | Spelling and grammar with suggestions (English and Polish, by each run's language), thesaurus, word count, comments in margin balloons or a pane, track changes, accept/reject, compare documents, restrict editing, accessibility checker, document inspector |
| **Mailings** | Mail merge from CSV, merge fields, address block, greeting line, rules, preview, finish to a document; envelopes and labels |
| **View** | Print layout, web layout, draft, read mode, focus, zoom, one/multiple pages, page width, Navigation pane, rulers, gridlines, dark mode; interface in English, 简体中文, 繁體中文, 日本語, Português (Brasil), Español, Українська, Српски / Srpski, Eesti, Türkçe, Русский, Italiano, Deutsch, Polski, Čeština, Magyar, Français, Български, Tiếng Việt, 한국어 or Svenska (follows the system language by default) |
| **Files** | .docx read/write (opens in Word), Word 97-2003 .doc import, PDF export, .odt, .rtf, .html, .md, .tex, .txt import/export, page images |

The honest picture, area by area, is in [ROADMAP.md](ROADMAP.md), the
[parity assessment](docs/target-app-parity.md) and the generated
[feature checklist](docs/parity-checklist.md).

## Quick start

```sh
git clone https://github.com/raantss18/mahay
cd soratra
cargo run --release -p soratra -- --sample        # the desktop app with the sample document
cargo run --release -p soratra -- report.docx     # open a document
```

Command line:

```sh
soratra-cli convert report.docx report.pdf        # docx, pdf, odt, rtf, html, md, tex, txt, png
soratra-cli text report.docx                      # plain text
soratra-cli inspect report.docx                   # structure as JSON
soratra-cli run --template sample \
  --cmd 'select.text={"text":"Membership"}' --cmd format.bold --save out.docx
```

Web: `cd apps/soratra-web && trunk serve`, then open <http://127.0.0.1:8771/?sample>.

## For agents: CLI and MCP

Soratra was designed to be driven by people *and* by AI agents.

```sh
claude mcp add soratra -- soratra-cli mcp                  # headless documents
# confine the agent to folders: soratra-cli mcp --automation-read-root DIR --automation-write-root DIR
soratra --control 7981 &                                     # or drive the running app…
claude mcp add soratra-app -- soratra-cli mcp --connect 127.0.0.1:7981
```

Tools include `list_commands`, `execute`, `batch`, `type_text`, `select_text`, `inspect_document`,
`render_page`, `save_document`, and, with a running app, `screenshot`, `click` and `key`. Agents
can check their work through `inspect_document` without screenshots. See [docs/mcp.md](docs/mcp.md)
and the [control protocol](docs/control-protocol.md). Macros record any sequence of commands and
play it back (`tools.recordMacro`, `tools.macros`).

## Logs

The desktop app writes its log records to standard error and to `logs/soratra.log` next to its
preferences: on Linux `$XDG_CONFIG_HOME/soratra/logs/` (by default `~/.config/soratra/logs/`),
on macOS `~/Library/Application Support/Soratra/logs/`, on Windows `%APPDATA%\Soratra\logs\`.
A start from a desktop menu or the Dock has no terminal, so this file is what to attach to a bug
report: unreadable parts of a .docx, pictures or fonts a PDF export had to leave out and panics the
command guard recovered from land there. Each launch moves the previous log to `soratra.1.log`
(and that one to `soratra.2.log`), so the log of a run that crashed survives the next start; a
log stops growing at 16 MiB. `--version` writes no file, and runs with `SORATRA_NO_PREFS` (agents'
test runs) log to standard error only. Document text is never logged; a panic message leaves out
the text it quotes.

By default Soratra's own crates log at `info` and everything else at `warn`. `RUST_LOG` replaces
that with env_logger-style directives, for example `RUST_LOG=debug`,
`RUST_LOG=warn,soratra_docx=trace` or `RUST_LOG=info,wgpu_core=warn`; a directive ending in `*`
covers every target starting with it (`soratra*=debug`). The logger is
`apps/soratra/src/logging.rs`.

## Architecture

| Layer | Crate | Job |
|---|---|---|
| L0 | `soratra-geom` | units and measurements |
| L1 | `soratra-doc`, `soratra-fonts`, `soratra-proof` | document model and editing; fonts and shaping; spelling, grammar, hyphenation |
| L2 | `soratra-layout`, `soratra-docx`, `soratra-formats` | line breaking, pagination, tables, notes, hit testing; OOXML; ODT/RTF/HTML/Markdown/TXT |
| L3 | `soratra-render`, `soratra-pdf` | rasteriser (vello_cpu); PDF ([mahay-pdf](docs/shared-pdf.md), Krilla) |
| L4 | `soratra-engine` | session, undo, 422 commands, Word feature catalog |
| L5 | `soratra-mcp` | MCP server |
| L6 | `soratra-ui-egui` | the Word-style front end (swappable) |
| apps | `soratra`, `soratra-cli`, `soratra-web` | desktop, command line, browser |

`cargo xtask ci` runs formatting, clippy, ~800 tests, the asset-attribution check, the layering
check and the wasm build. Contributor and agent instructions: [AGENTS.md](AGENTS.md).

## Roadmap

Soratra is in **alpha**. It covers 88% of Word's ribbon features with commands today; counting
depth, file fidelity, stability and the rest, we estimate it is about 60% of the way to replacing
all of Word for real work, about 55% for a typical professional's weekly work and about 63% for
everyday essentials. Beta needs testing against real-world .docx files, keeping charts and SmartArt
through a round trip, pagination closer to Word's, a stability sweep and native printing. Details
and estimates: [ROADMAP.md](ROADMAP.md) and [where we fall short](docs/gaps.md).

## Downloads

**Download Soratra** from GitHub: the [latest release](https://github.com/raantss18/mahay/releases/latest) has every build listed below, and [all releases](https://github.com/raantss18/mahay/releases) has earlier versions and their notes. `<ver>` in the file names is the version number, and `SHA256SUMS.txt` lists a checksum for every file.

### Windows

| Build | Installer | Portable |
|---|---|---|
| x64 (64-bit Intel/AMD) | `soratra-<ver>-windows-x64.msi` | `soratra-<ver>-windows-x64-portable.zip` |
| arm64 (Snapdragon and other ARM PCs) | `soratra-<ver>-windows-arm64.msi` | `soratra-<ver>-windows-arm64-portable.zip` |
| x86 (32-bit) | `soratra-<ver>-windows-x86.msi` | `soratra-<ver>-windows-x86-portable.zip` |

Installers and executables are code-signed.

**If the app doesn't open on Windows:** the desktop app initializes only DirectX 12 by default.
Letting wgpu also create an OpenGL instance can crash some graphics drivers (AMD's
`atio6axx.dll`) before the window appears, so the app would flash in Task Manager and quit.
`WGPU_BACKEND` overrides the default for troubleshooting (for example `dx12` or `vulkan`). In
PowerShell, from the folder containing the executable:

```powershell
$env:WGPU_BACKEND = "vulkan"
& .\soratra.exe
Remove-Item Env:WGPU_BACKEND                     # restore the default for later launches
```

An explicit `gl` override can bring the driver crash back on affected systems. The macOS, Linux
and web backend defaults are unchanged.

The **32-bit** build uses OpenGL even when a DirectX 12 graphics card is present. On 32-bit
Windows, creating a DirectX 12 graphics pipeline fails immediately and the app exits before the
window appears. `WGPU_BACKEND=vulkan` (or `dx12`) still overrides that.

**If the app quits right after its window appears with "Parent device is lost" in the log:**
Soratra compiles DirectX 12 shaders with FXC, the compiler built into Windows, so it needs no
`dxcompiler.dll`/`dxil.dll` and ignores any copy another program put on `PATH` (#603).
`WGPU_DX12_COMPILER` (`fxc`, `dxc`, `auto`) overrides this; with `dxc`, a matching
`dxcompiler.dll` *and* `dxil.dll` must sit next to `soratra.exe`. `WGPU_BACKEND = "vulkan"`
also avoids DirectX 12 altogether.

**If the app is slow on an older Windows PC:** when DirectX 12 finds no graphics card with a
driver for it (for example Intel 4th-generation graphics on Windows 11), it only offers Microsoft's
software renderer, which draws every frame on the processor. In that case Soratra uses OpenGL
instead, without setting anything. The log file records the choice (lines starting with
`graphics:`). `WGPU_BACKEND` still overrides it, e.g. `$env:WGPU_BACKEND = "gl"` or `"dx12"`.

### macOS

| Build | File | Notes |
|---|---|---|
| App, universal (Apple silicon + Intel) | `soratra-<ver>-macos-universal.dmg` | Signed and notarized |
| Command-line tool, universal | `soratra-cli-<ver>-macos-universal.zip` | Signed and notarized |

### Linux

| Format | x86_64 | aarch64 (ARM64) | Notes |
|---|---|---|---|
| AppImage | `soratra-<ver>-linux-x86_64.AppImage` | `soratra-<ver>-linux-aarch64.AppImage` | Runs anywhere; updates itself with [AppImageUpdate](https://github.com/AppImageCommunity/AppImageUpdate) (`.zsync` files) |
| Flatpak | `soratra-<ver>-linux-x86_64.flatpak` | `soratra-<ver>-linux-aarch64.flatpak` | Sandboxed; `flatpak install --user <file>` |
| Debian/Ubuntu | `soratra-<ver>-linux-x86_64.deb` | `soratra-<ver>-linux-aarch64.deb` | |
| Fedora/RHEL/openSUSE | `soratra-<ver>-linux-x86_64.rpm` | `soratra-<ver>-linux-aarch64.rpm` | |
| Tarball | `soratra-<ver>-linux-x86_64.tar.gz` | `soratra-<ver>-linux-aarch64.tar.gz` | Unpack anywhere |

RISC-V (riscv64): `soratra-<ver>-linux-riscv64.tar.gz`, a tarball cross-compiled for glibc 2.39+
(Ubuntu 24.04, Debian 13 and newer).

**Wayland:** on a Wayland desktop the app runs through XWayland when it is available, because
winit (the windowing library) supports dragging files into the window only on X11. To use the
native Wayland backend instead (for example for sharper text with fractional scaling, at the cost
of file drag-and-drop), start it with `SORATRA_WAYLAND=1`:

```sh
SORATRA_WAYLAND=1 soratra
```

**Arch Linux (AUR).** Community members maintain three AUR packages; the project doesn't publish
to the AUR itself. Install one of them:
| Package | What it installs | Choose it when |
|---|---|---|
| [`soratra-bin`](https://aur.archlinux.org/packages/soratra-bin) | The release tarball above, prebuilt (x86_64, aarch64) | You want the latest release without compiling (quickest) |
| [`soratra`](https://aur.archlinux.org/packages/soratra) | The latest release, built from source with `cargo` | You prefer to build releases yourself |
| [`soratra-git`](https://aur.archlinux.org/packages/soratra-git) | The current `main` branch, built from source | You want unreleased changes and accept an occasional broken build |
For example `yay -S soratra-bin` or `paru -S soratra-bin`.

#### Community packages

These packages are maintained by community members, not by the Soratra team.

On Gentoo, the [::snakebyte overlay](https://github.com/switch87/snakebyte-overlay) packages the
Linux release as `app-office/soratra-bin` (prebuilt) and `app-office/soratra` (built from source):

```sh
eselect repository add snakebyte git https://github.com/switch87/snakebyte-overlay.git
emaint sync -r snakebyte
echo 'app-office/soratra-bin ~amd64' >> /etc/portage/package.accept_keywords/soratra
emerge --ask app-office/soratra-bin
```

### FreeBSD

| Build | File |
|---|---|
| x86_64 | `soratra-<ver>-freebsd-x86_64.tar.gz` |

### Web (WebAssembly)

| Build | File | Notes |
|---|---|---|
| Static site | `soratra-web-<ver>.zip` | Runs in a modern browser; host it on any static server |

## The Mahay apps

Soratra is one of the **Mahay apps**: free, open-source creative tools from the
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
| <img src="https://raw.githubusercontent.com/raantss18/soratra/main/assets/app-icon/hicolor/64x64/apps/org.mahay.soratra.png" alt="" width="32" height="32"> | **Soratra** | **Writing and document design · you are here** | [GitHub](https://github.com/raantss18/mahay) | [Website](https://github.com/raantss18/mahay) |

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
  <a href="https://github.com/raantss18/mahay">Soratra</a>
</p>

## License and credits

Soratra is dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
Copyright (c) 2026 Mahay Team and the Soratra contributors. Required notices are in [NOTICE](NOTICE).

Bundled fonts, icons, images and other assets keep their own open licenses; each one is listed
with its author, source and license in [ATTRIBUTION.md](ATTRIBUTION.md).

The English spelling dictionary and hyphenation come from Grady Ward's public-domain Moby
Hyphenator II word list. Polish spelling reads the Hunspell `pl_PL` dictionary installed on the system (not bundled), as
do Russian, Portuguese, Czech, Italian, Croatian and Turkish spelling (how to install them:
[docs/spelling-dictionaries.md](docs/spelling-dictionaries.md)), and
Polish hyphenation the TeX patterns by Hanna Kołodziejska, Bogusław Jackowski and Marek Ryćko (MIT
option). The sample documents and templates are original text written for Soratra.

The Mahay name, wordmark and logos in [`docs/brand/`](docs/brand/) are trademarks of the
Mahay Team and are not covered by this license. They may be used only unmodified, and only as
part of this repository and Soratra, under [`docs/brand/LICENSE-brand.txt`](docs/brand/LICENSE-brand.txt).
Forks and modified versions must remove them.

<sub>Microsoft and Microsoft Word are trademarks of the Microsoft group of companies. Adobe, Photoshop, Illustrator, Premiere Pro, Lightroom, Acrobat, After Effects and InDesign are trademarks or registered trademarks of Adobe Inc. in the United States and/or other countries. Soratra is an independent, open-source project and is not affiliated with, sponsored by or endorsed by Microsoft Corporation or Adobe Inc.; these names are used only to describe the workflows it is compatible with.</sub>

<p align="center">
  <a href="https://github.com/raantss18/mahay"><img alt="Mahay" src="docs/brand/mahay-mark.svg" width="28"></a><br>
  <sub>Made by the <a href="https://github.com/raantss18/mahay">Mahay</a> team and community.</sub>
</p>
