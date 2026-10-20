# New apps: a LaTeX editor (15th) and a notebook (16th)

Status: **agreed in principle** (2026-10-19): the owner chose the names, one release (0.3.0), packs
of 50 to 100 MB, a first TeXnika with the real engine and the instant math layer (LyX-style blocks
come after), Python first for Naoty. The engine was open until the spikes report. Nothing here is built except the throwaway M0
spike (`experiments/tex-m0/`, measured 2026-10-20: **Tectonic works, with conditions**, see "M0 spike
results"); every number marked "to measure" still comes from the spike K0 or later work, never from this page.

> A LaTeX editor as complete as possible but light: LyX/BaKoMa-style, with TeXstudio's speed,
> packages preinstalled (mathematics, bibliography, figures…), beautiful, for Android and Linux.
> Then a Jupyter-lab-like notebook on the same principle.

## Principles (the same as the 14 apps)

- Native Rust, egui/wgpu, no WebView; one version for the suite; Android phones and tablets and
  Linux; offline first; MIT OR Apache-2.0 (D1), upstream notices kept.
- **Light by construction:** a small app, with what is heavy (TeX packages, Python) in optional
  **packs** the user installs once, signed and verified like updates (D15).
- Phone and tablet layouts follow the window width (D23); installed and updated through Mahay HUB
  (D21), names in Malagasy.
- Names, chosen by the owner: **TeXnika** for the LaTeX editor, **Naoty** for the notebook
  (`A_VALIDER.md`: meanings to confirm).

---

## App 15 — TeXnika, the LaTeX editor

### What it must feel like

| Model | What TeXnika takes from it |
|---|---|
| **TeXstudio** | a fast source editor: highlighting, completion of commands, labels and citations, snippets, symbol palette, structure tree, projects, error list, SyncTeX |
| **LyX** | structure-first editing: sections, lists, theorems, figures and tables as blocks you can fold, move and restyle without reading the markup |
| **BaKoMa TeX** | the typeset result is always in front of you, updating as you type |

Two layers make that possible without a slow compile on every key:

1. **Instant layer.** Formulas, figures and tables are typeset in the editor itself by a fast
   Rust math layout (inline "insets" that show the result while the cursor is elsewhere, the
   source when it is inside). Mahay already has one: `apps/soratra/crates/layout/src/math.rs`
   lays out equations (UnicodeMath/OMML), with `math_symbols.rs`; the plan is to move the layout
   core into a shared crate and add a LaTeX-syntax front end.
2. **Exact layer.** The real TeX engine builds the PDF in the background after a short pause,
   exactly as LaTeX would; the PDF viewer (Taratasy's renderer, `taratasy-render`/hayro) shows
   it beside the source, and SyncTeX links click and cursor both ways.

### The engine: the decision everything depends on

| Option | Real LaTeX? | Light? | Android | Verdict |
|---|---|---|---|---|
| **Tectonic** (XeTeX, embeddable Rust crate, MIT) | yes: most packages | engine a few MB; packages fetched from a bundle on demand | a Tectonic-based F-Droid app (TeXslate) compiles on device, so it is possible; its core is mostly C: **M0 built it with the NDK for x86_64 and arm64 and compiled on the emulator** (conditions below) | **chosen** (D25), conditions in "M0 spike results" |
| WebAssembly TeX engines (SwiftLaTeX-style) in a Rust wasm runtime | yes | runtime + engine heavy | slow in an interpreter | fallback if Tectonic cannot be built |
| A TeX engine written in Rust | in theory | yes | yes | no complete one exists (found 2026-10-19); years of work, not a plan |
| Typst | no (another language) | very | yes | not LaTeX; could be an extra export, never the engine |

What Tectonic does **not** do, and the plan for each: only XeTeX (no pdfTeX/LuaTeX: fine for
almost all documents, not for `\write18`/`minted`/LuaTeX-only packages); `biber` (Perl) is not
part of it, so BibTeX/natbib work out of the box and **biblatex needs a replacement backend**
(a small Rust BibLaTeX processor is on the list, flagged as risk); its default bundle is
downloaded from the internet, so TeXnika ships a **curated offline bundle** instead.
`texlab` (the usual LaTeX language server) is GPL-3.0 and cannot be embedded: editor
intelligence is written in Rust on a permissive parser (`tree-sitter-latex` licence to check,
otherwise our own lexer, which LaTeX's syntax makes small).

### M0 spike results (2026-10-20, `experiments/tex-m0/`, recipe in its README)

**Verdict.** Linux: **go**. Android: **go with conditions**: the engine builds for `x86_64` and
`aarch64`, and compiled documents (a) to (e) plus seven more on the x86_64 emulator, with
byte-identical PDFs to Linux, offline, from a directory bundle. Not proven: speed and memory on a real
arm64 device, and four things that must be solved before a release (below). biblatex with `biber`: **no-go**
on both, a replacement backend is needed (as planned).

**Setup.** `tectonic` 0.17.0 and `tectonic_bundles` 0.4.2, both **MIT**. `tectonic_bundles::dir::DirBundle`
reads a flat local directory (plus a `SHA256SUM` file for the format-cache key). The directory bundle was
made by compiling the samples once with the default online bundle through a recording wrapper that copies
every file the engine asked for (610 files, 24.9 MB raw). Every figure below is from **offline** runs
(Linux: `unshare -rn`, no network interface; Android: the executable and the bundle are pushed to `/data/local/tmp`, and the program contains no network code).
"Cold" = empty format cache (the engine builds `latex.fmt` from the bundle, about 2 s); "warm" = format cached;
peak RAM = `VmHWM` of the process. Linux: this machine under load from other builds (16 cores, so
only the order of magnitude counts); Android: `emulator-5554` (x86_64, API 36, hardware-accelerated by the
host CPU, so it says **nothing** about the speed of an arm64 tablet).

| Document | OK | Linux cold / warm (s) | Linux peak RAM cold / warm (MB) | Android x86_64 cold / warm (s) | Android peak RAM cold / warm (MB) | PDF (KB) | SyncTeX (KB) |
|---|---|---|---|---|---|---|---|
| (a) article: ams*, mathtools, graphicx, hyperref | yes | 2.8 / 0.7 | 222 / 174 | 4.8 / 1.1 | 216 / 161 | 41 | 3.5 |
| (b) TikZ + pgfplots | yes | 3.0 / 1.1 | 223 / 177 | 4.3 / 1.8 | 216 / 164 | 8 | 2.2 |
| (c) BibTeX + natbib | yes | 2.2 / 0.4 | 223 / 170 | 3.5 / 0.8 | 216 / 159 | 13 | 1.2 |
| (d) beamer | yes | 3.1 / 1.7 | 223 / 176 | 5.4 / 2.5 | 216 / 164 | 25 | 6.3 |
| (e) fontspec, Latin Modern | yes | 2.5 / 0.7 | 220 / 173 | 3.9 / 1.1 | 216 / 163 | 11 | 0.6 |
| (f) biblatex (backend=biber) | NO | – | – | – | – | – | – |
| extra: physics, siunitx, tcolorbox, listings, algorithm2e | yes | 3.1 / 1.1 | 220 / 177 | 4.3 / 1.8 | 216 / 163 | 18 | 3.3 |
| extra: subcaption, float, wrapfig, caption | yes | 2.4 / 0.4 | 220 / 173 | 3.4 / 0.7 | 216 / 158 | 13 | 1.3 |
| extra: standalone class + tikz | yes | 2.7 / 0.6 | 220 / 172 | 3.8 / 1.0 | 216 / 160 | 1 | 1.3 |
| extra: babel (French) + csquotes | yes | 2.4 / 0.7 | 218 / 172 | 3.6 / 0.8 | 216 / 158 | 4 | 0.7 |
| extra: algorithmicx | yes | 2.6 / 0.5 | 220 / 172 | 3.6 / 0.8 | 216 / 158 | 9 | 0.9 |
| extra: unicode-math + Latin Modern Math | yes | 4.0 / 1.9 | 218 / 184 | 5.1 / 2.2 | 216 / 174 | 7 | 1.0 |
| extra: TeX Gyre fonts | yes | 4.2 / 1.0 | 216 / 172 | 3.8 / 1.1 | 216 / 163 | 8 | 0.6 |

Reading it: a warm rebuild of an ordinary article takes **0.4 to 1.1 s** on Linux and 0.7 to 1.8 s on the
emulator (beamer and `unicode-math` about 2 s), inside the "about two seconds" target; cold, 2 to 5 s; peak
memory **about 220 MB cold, 160 to 185 MB warm** on both, above the idle target of 150 MB (a compile worker that
exits after each build, or a memory limit on the format, would be needed on phones). SyncTeX files are produced
(`--synctex`, gzip, used by the editor's click-to-source); they hold absolute input paths. PDFs from the emulator
are byte-identical to Linux's for (a), (b), (d), (e). Biblatex (f): see below.

**What does not work or needs care (all observed):**

1. **biblatex / biber.** The bundle's biblatex is 3.17 (control-file 3.8). With a system `biber` 2.22 on the
   path, Tectonic runs it and it refuses (`Found biblatex control file version 3.8, expected version 3.11`);
   with no `biber` (Android, and a Linux without Perl) the run fails with `No such file or directory (os error 2)`.
   `natbib` + BibTeX works. So TeXnika needs its own Rust BibLaTeX processor, as the plan already said.
2. **Fonts by name.** `\setmainfont{Latin Modern Roman}` fails (`The font ... cannot be found`): family names
   only reach fontconfig's *system* fonts; bundle fonts load by **file name** (`\setmainfont{lmroman10-regular.otf}[...]`,
   which works and is what sample (e) uses). The editor should insert file-name font setups, or TeXnika must
   expose the bundle fonts to the name lookup (a custom font manager in the layout crate).
3. **Android build conditions** (below): ICU, converters, system fonts, Graphite.
4. Cosmetic: with no build date the engine dates documents 1970-01-01 (set `build_date` from the clock);
   `Missing character ... nullfont` warnings appear during the first pass of `beamer` (the final PDF text is right);
   the format needs a writable cache directory.

**Engine size** (executable of the spike, stripped, `opt-level = "s"`, thin LTO, no network code):

| Target | Executable | zstd -19 | xz -9e | Links |
|---|---|---|---|---|
| Android `aarch64` | 9.13 MB | 2.47 MB | 2.10 MB | `libicu.so` (system), libz, libc |
| Android `x86_64` | 9.41 MB | 2.61 MB | 2.37 MB | same |
| Linux `x86_64` | 8.17 MB | n/a | n/a | system freetype, fontconfig, graphite2, ICU, libpng (dynamic, not counted) |

That is the XeTeX + xdvipdfmx + BibTeX engines with freetype, harfbuzz and libpng statically linked on Android,
well inside "under about 30 MB for the app"; an arm64 APK would grow by about 2.5 MB compressed.

**Android obstacles and how they were solved** (all in `experiments/tex-m0/`, `build-android.sh`,
`tools/android-deps.sh`, `tools/android-deps/stubs/`):

| Obstacle | Solution | Left to do for the product |
|---|---|---|
| The `tectonic_bridge_*` crates find freetype, fontconfig, graphite2, ICU and libpng only through `pkg-config`; nothing is vendored. `harfbuzz` is vendored. | cross-built freetype (CMake with the NDK toolchain file, from the `freetype-sys` crate's sources) and libpng (the `libpng-sys` crate's sources) as static libraries, and wrote `.pc` files; `PKG_CONFIG_ALLOW_CROSS=1`, `PKG_CONFIG_LIBDIR` | replace by a build crate in the repo (no `pkg-config`) |
| No fontconfig on Android | a 15-function C stub that reports "no system fonts" plus an empty `fontconfig/fontconfig.h` (xetex only includes it) | a real font provider for Android's `/system/fonts` if documents need system fonts by name |
| No graphite2 (no crate bundles its source) | stub headers and functions: Graphite fonts are unsupported; OpenType/AAT shaping through harfbuzz works | accept, or vendor graphite2 later |
| ICU: the NDK exposes ICU (`libicu.so`) **only from API 31** (Android 12), and without the converter API (`ucnv_*`) | built for API 31 (the C code refuses to compile for 26: "unavailable: introduced in Android 31"), a `ucnv` stub that fails legacy `\XeTeXinputencoding` (UTF-8 works) | **Mahay's minSdk is 26**: either build ICU4C into the app (a slimmed data file, about 2 to 3 MB, not tried: its source is not a crate and the spike downloads only crates and the TeX bundle), or replace the few ICU calls (bidi, break iterator, converters) by Rust, or raise minSdk for this app. Decision for M3 |
| `libc++_shared.so` missing at run time | `CXXSTDLIB=c++_static` and `RUSTFLAGS=-C link-arg=-lc++abi` | none |
| `rustc` of the distribution has no Android std | rustup's `cargo` first in `PATH` (as `scripts/android-build.sh` does); `ANDROID_HOME` forced to `$HOME/Android/Sdk` | none |

No `cc` flag problems beyond the above; the C of xetex, xdvipdfmx, bibtex and harfbuzz compiled unchanged on both
ABIs (warnings only). The arm64 binary was **built, not run** (no device).

**Bundle size** (files the samples opened, split by first pack that needs them; zstd -19 / xz -9e of a tar of the
directory):

| Pack | files | raw MB | zstd MB | xz MB | Contents measured |
|---|---|---|---|---|---|
| Core | 343 | 16.59 | 3.26 | 3.00 | kernel + format sources, `article`, `ams*`, `mathtools`, `graphicx`, `hyperref`, `fontspec`, `unicode-math`, hyphenation patterns, Latin Modern (+ Math), CM Type 1 |
| Mathematics | 107 | 2.50 | 0.61 | 0.59 | `physics`, `siunitx`, `cancel`, `bm`, `esint`, `thmtools`, `tcolorbox`, `algorithm2e`, `algorithmicx`, `listings` |
| Bibliography | 23 | 1.14 | 0.18 | 0.18 | `natbib`, `biblatex` (no backend), `csquotes`, `babel` French |
| Figures | 67 | 2.75 | 0.48 | 0.46 | `tikz`/`pgf`, `pgfplots`, `subcaption`, `float`, `wrapfig`, `caption`, `standalone` |
| Slides | 64 | 0.80 | 0.21 | 0.20 | `beamer` with the Madrid theme |
| Fonts | 6 | 1.14 | 0.40 | 0.36 | TeX Gyre Pagella and Heros (what sample m loaded) |
| **All** | **610** | **24.92** | **5.15** | **4.80** | one archive: 5.07 MB zstd, 4.70 MB xz |

**These are floors, not pack sizes.** They hold only the files these 13 documents opened (no other `beamer`
theme, TikZ library, language, font or package), so the real packs are larger; the decision of "50 to 100 MB" is
comfortable (compressed 5 MB for what was touched). The biggest single files: `pdftex.map` 4.4 MB, `UnicodeData.txt`
1.9 MB, `expl3-code.tex` 1.2 MB, `latinmodern-math.otf` 0.7 MB. The full default Tectonic bundle was **not**
downloaded (it is large), so the size of "all of TeX Live" is not measured here.

**Licences** (`experiments/tex-m0/results/licences.md`, read from the files' own headers): nothing
non-redistributable was found. LaTeX packages are LPPL 1.3c; Latin Modern and TeX Gyre are the GUST Font
License; Computer Modern Type 1 is SIL OFL; Unicode data is the Unicode licence. Flags: **`pgfplots` is
GPL-3.0-or-later** and a few **hyphenation patterns are GPL** (`hyph-as, bn, ga, gu, hi, kn, cs, hu, id, lv, mk,
sk`): they stay in separate data packs with their licence text, or are left out; `float`, `preview`
(TeX Live: GPL-3.0+) and `algorithmicx` carry only a "see the source" header and must be confirmed before a pack
is published. The Tectonic crates and their bridge crates are MIT.

### Packs ("preinstalled packages")

Installed from inside TeXnika (or Mahay HUB), signed, offline afterwards. Sizes: the measured floors of the M0 spike are in "M0 spike results" (5 MB compressed for the files 13 test documents used; the full packs are not measured).

| Pack | Content |
|---|---|
| Core | LaTeX kernel, article/report/book/letter, `amsmath`, `amssymb`, `amsthm`, `mathtools`, `graphicx`, `xcolor`, `geometry`, `hyperref`, `babel`, `fontspec`, Latin Modern and a math font |
| Mathematics | `physics`, `siunitx`, `cancel`, `bm`, `esint`, `thmtools`, `tcolorbox`, `algorithm2e`, `algorithmicx`, `listings` |
| Bibliography | `natbib`, `biblatex` + backend, `csquotes`, common styles |
| Figures | `tikz`/`pgf`, `pgfplots`, `subcaption`, `float`, `wrapfig`, `caption`, `standalone` |
| Slides | `beamer` and themes |
| Fonts and languages | extra text and math fonts, hyphenation patterns |

Only redistributable packages go in a pack; each pack carries a licence manifest (most CTAN
packages are LPPL), checked like the other third-party notices.

### Editing features (TeXstudio class)

Highlighting and bracket/environment matching, auto-closing `\begin…\end`, completion from the
loaded packages' command lists plus the project's labels and the `.bib` entries, snippets,
symbol palette and matrix/table/figure wizards, outline and structure view, multi-file
projects (`\input`, `\include`, master file), find/replace across the project, spelling (reuse
`soratra-proof`), word count, log parser with jump-to-error, a bibliography manager (BibTeX
entries, import; Soratra's `zotero` crate is a starting point), TikZ/pgfplots templates.

### Android specifics

A key row above the soft keyboard (`\ { } $ ^ _ & ~ %`), a bottom-sheet symbol palette, source
and PDF side by side on tablets and a switch on phones (D23), the compile on a worker thread with
a battery-aware pause, files through "Open with" (`.tex`, `.bib`, `.sty`, `.cls`, D22).

### Shared building blocks (they also serve the notebook)

| Crate | What | From |
|---|---|---|
| `mahay-editor` | code editor widget: rope buffer, undo tree, virtual scrolling, highlighting, IME and touch | new (egui's `TextEdit` is too slow for long sources) |
| `mahay-math` | TeX-style math layout and drawing | Soratra's layout core |
| `mahay-pdfview` | PDF page viewer and SyncTeX | Taratasy's renderer |
| `mahay-pack` | signed, resumable content packs | the update system (`mahay-update`) |

### Lightness budget (targets, to be measured, never promised before)

App download under about 30 MB for arm64 without packs; a cold start under a second on a
mid-range tablet; the instant layer under one frame; a warm rebuild of an ordinary article in
about two seconds or less; idle memory under about 150 MB with a 100-page project.

### Milestones

| | What | Gate |
|---|---|---|
| **M0 spike (done 2026-10-20: go with conditions)** | Tectonic builds for Linux and for Android (arm64 and x86_64); compiles an article with `amsmath`, `tikz` and BibTeX **offline** from a curated mini-bundle; measure size and time | decides the engine (D25) |
| M1 | editor and PDF side by side on Linux: highlight, compile on a pause, errors, SyncTeX | usable for a real document |
| M2 | TeXstudio-class features: completion, outline, projects, bibliography, palette, snippets, spelling | |
| M3 | Android: touch UI, key row, compact layout, packs by download | on a real tablet and phone |
| M4 | LyX/BaKoMa class: inline math/figure insets, structure ("document") view | |
| M5 | all packs, biblatex backend, fr/en/mg, accessibility, docs | |

---

## App 16 — Naoty, the notebook

A native Jupyter client in the same spirit: light, beautiful, offline, Android and Linux.

- **Files:** `.ipynb` read and written without loss (nbformat 4.x; the Rust `nbformat` crate or our
  own).
- **Cells:** code, Markdown (with LaTeX math through `mahay-math`) and raw; outputs as text,
  ANSI-coloured errors, PNG/SVG plots, tables and a small, **script-free** HTML subset.
- **Kernels:** the Jupyter messaging protocol natively in Rust over ZeroMQ, with the
  `jupyter-zmq-client` + `jupyter-protocol` crates of the runtimed project (BSD-3; `runtimelib` is
  now only a deprecated re-export of `jupyter-zmq-client`) on the pure-Rust `zeromq` crate (MIT):
  **no C library for ZeroMQ** (K0: the only C in the tree is `ring`'s HMAC, which builds with the
  NDK). Three kinds: a **local kernel** (Linux: the system's `ipykernel` and any installed
  kernelspec; Android: a Python pack), a **remote kernel** (a Jupyter server on the user's PC or a
  server, the light option for a tablet), and a small **built-in** one (JavaScript through Boa,
  already in Taratasy).
- **Python on Android (measured, K0):** CPython 3.13 builds for Android with its own
  `Android/android.py` (NDK 28.2, `x86_64` and `aarch64`) and runs on the emulator. The scientific
  stack is the weak point, not the kernel: **PyPI has no numpy, matplotlib or Pillow wheel for
  Android**; the only source found is the Chaquopy index (`cp313`, arm64 and x86_64: numpy 1.26.2,
  matplotlib 3.8.4, Pillow 11, with 2017–2018 OpenBLAS, libgfortran 4.9 and FreeType 2.9 inside).
  `sympy` is pure Python. **`pyzmq` is not a blocker after all**: PyPI ships Android wheels of
  `pyzmq` 27.2 (cp313 and cp314), and the stock `ipykernel` runs on the emulator. Two kernels
  work there: the stock `ipykernel` (10 MB more, 2.2 s to start) and our small Rust kernel that
  embeds CPython (1.6 MB, 0.07 s to start).
- **Pack size (measured, zstd -19, stripped, bytecode included; arm64 / x86_64):** CPython +
  stdlib 9.2 / 9.4 MB, numpy with BLAS and runtime libraries 6.1 / 8.0 MB, matplotlib with
  Pillow, fontTools etc. 9.7 MB, sympy with mpmath 8.3 MB, Rust kernel 0.6 MB. **Core + numpy
  15.5 / 17.6 MB; everything (plots and sympy) 33.7 / 35.9 MB** to download, 146 / 156 MB
  installed. The stock `ipykernel` stack would add 10.2 MB (50 MB installed). Well inside the
  50–100 MB the owner accepts.
- **Speed (measured):** Rust client to our Rust kernel on Linux: kernel ready in 0.16–0.22 s,
  `print(1+1)` result in under 1 ms. Same on the x86_64 emulator (KVM on the desktop CPU; a real
  arm64 tablet will be slower, not measured): ready in 0.07 s, first matplotlib figure 0.86 s,
  first `numpy` + `sympy` cell 0.55 s, with precompiled bytecode (without it 5 s for each of those
  cells: ship the `.pyc`). Stock `ipykernel`: ready in 2.2–2.7 s (Linux and emulator).
- **Safety:** a notebook never runs by itself; opening one from elsewhere shows outputs
  without executing anything, as Jupyter's "trust" model does; HTML outputs never run scripts.
- **K0 verdict (2026-10-20): go on Linux and on Android, with conditions** (details below, recipe
  in `experiments/naoty-k0/README.md`, decision D26).
- **Milestones:** K0 spike (done, see below), K1 notebook UI on Linux with a local Python kernel, K2 Android with a
  remote kernel, K3 the Python pack on Android, K4 other kernels (Julia, Lean 4, Rust/evcxr).

### K0 results (2026-10-20, `experiments/naoty-k0`)

| # | Question | Result |
|---|---|---|
| 1 | Rust client ↔ Linux `ipykernel` | **Works.** `jupyter-zmq-client` 1.0 + `jupyter-protocol` 2.0: `kernel_info`, `execute_request` for `print(1+1)`, `1+1`, a matplotlib plot, `1/0` and a numpy + sympy cell: `stream`, `execute_result`, `display_data` (`image/png`, inline backend), `error`, `execute_reply`, `shutdown_reply`, all received. |
| 1 | `.ipynb` with `nbformat` 3.0 | **Lossless for 4.5** (JSON-equal on a notebook with outputs, PNG, attachments, tags and custom metadata, 30 KB). Reads 4.0–4.4 and v3 into separate types but **writes only 4.5**: saving an old notebook means upgrading it (our own step). |
| 2 | Rust kernel with embedded CPython, no `pyzmq` | **Works** (`kernel/`, about 300 lines + 60 of Python): `kernel_info_request`, `execute_request` (live stdout/stderr, last expression, errors with traceback, matplotlib figures as PNG), `shutdown_request`, heartbeat. Not done: interrupt, completion, inspection, stdin, comms. Driven by the same client on Linux and Android. |
| 3 | CPython on Android | **Works.** CPython 3.13.16 from source with `Android/android.py` (x86_64 and aarch64, about 7 minutes each on 4 jobs). On the emulator: `print(1+1)`, the stdlib (ssl, sqlite3, zlib, bz2, lzma, ctypes…), then numpy 1.26.2, matplotlib 3.8.4 (Agg, PNG out), sympy 1.14 (`integrate`, `factor`, `solve`). Not built: `readline`, `curses`, `_multiprocessing`, `_uuid`, `grp`, `_tkinter`. The aarch64 build and its Rust kernel are built and checked as ELF, **not run** (no arm64 device; the emulator's translation layer did not find `libpython` through `LD_LIBRARY_PATH`). |
| 3 | Scientific wheels | PyPI: none for numpy, matplotlib, Pillow (3.13 and 3.14). **Chaquopy index** (`chaquo.com/pypi-13.1`): numpy 1.26.2, matplotlib 3.8.4, Pillow 11.0, kiwisolver, contourpy for `cp313` only (no 3.14), plus `chaquopy-openblas` 0.2.20, `-libgfortran` 4.9, `-freetype` 2.9.1, `-libjpeg`; the NDK provides `libc++_shared`. BeeWare's index has iOS builds only. Nothing newer (numpy 2.x) was found for Android. |
| 3 | `pyzmq` | **Not a blocker.** `pyzmq` 27.2.0 Android wheels (cp313/cp314, both ABIs) are on PyPI; stock `ipykernel` 7.4 + IPython 8.37 ran all five cells on the emulator, including the inline PNG (IPython 9 breaks matplotlib 3.8: `backend2gui`). |
| 4 | Sizes | table below |
| 4 | Times | table below |
| 5 | Remote kernel | **Works.** `jupyter-websocket-client` 2.0 (BSD-3) against `jupyter server` with a token: REST `POST /api/kernels` (the crate does not do REST: we used `reqwest`, 84 ms), WebSocket `v1.kernel.websocket.jupyter.org`, the same five cells, 0.63 s to ready. Only `http://` tested; the crate carries the token in the URL and, for `https://`, needs root certificates supplied by us (`tokio-rustls-manual-roots`). |

Sizes (stripped, `zstd -19`, bytecode included, tests removed; `arm64` / `x86_64`):

| Component | Installed | Download (zstd) |
|---|---|---|
| CPython `libpython3.13.so` + ssl + sqlite + launcher | 11.6 / 11.6 MB | 3.5 / 3.7 MB |
| stdlib (with `lib-dynload`, `.pyc`) | 23.8 / 23.7 MB | 5.7 / 5.7 MB |
| numpy 1.26.2 (`.pyc` included) | 12.8 / 17.8 MB | 2.6 / 3.5 MB |
| matplotlib 3.8.4 | 21.0 / 21.0 MB | 5.4 / 5.4 MB |
| matplotlib dependencies (Pillow, kiwisolver, contourpy, fontTools, dateutil…) | 22.4 / 22.4 MB | 4.3 / 4.3 MB |
| sympy 1.14 + mpmath | 40.5 / 40.5 MB | 8.3 / 8.3 MB |
| native libraries (OpenBLAS, libgfortran, libc++, FreeType, libjpeg) | 12.3 / 17.1 MB | 3.5 / 4.5 MB |
| Rust kernel (release, stripped) | 1.5 / 1.6 MB | 0.6 / 0.6 MB |
| Rust client with `nbformat`, WebSocket, `reqwest` (release) | 2.5 / 2.7 MB | 1.0 / 1.0 MB |
| **Pack "Core + numpy"** (runtime, stdlib, numpy, libs, kernel) | 60.7 / 70.4 MB | **15.5 / 17.6 MB** |
| **Pack everything** (+ matplotlib, dependencies, sympy) | 145.8 / 155.7 MB | **33.7 / 35.9 MB** |
| Stock `ipykernel` stack instead of our kernel (IPython, jedi, prompt_toolkit, tornado, pyzmq…) | 49.9 MB | 10.2 MB |

Times (release builds; the desktop was busy with other builds, so Linux numbers are ± 30 %;
emulator = Android 16 x86_64 with KVM, 4 cores, the host's CPU, **not** a phone):

| Measure | Linux | Emulator |
|---|---|---|
| Our Rust kernel: spawn → `kernel_info_reply` | 0.16–0.22 s | 0.07 s |
| Stock `ipykernel`: spawn → `kernel_info_reply` | 2.6–2.7 s | 2.2–2.3 s |
| `print(1+1)` → `stream` (either kernel) | under 1 ms (Rust), 5–15 ms (`ipykernel`) | 1 ms (Rust), 8–11 ms (`ipykernel`) |
| First matplotlib figure → `display_data` | 3–4 s (cold, Linux venv) | 0.86 s (Rust kernel), 0.75 s (`ipykernel`), with `.pyc`; 5 s without |
| First `import numpy, sympy` + `integrate` | 3–5 s | 0.55 s (Rust kernel), 0.66 s (`ipykernel`), with `.pyc`; 5–8 s without |

Trap found: a ZeroMQ socket connected *before* the kernel has bound its ports pays the `zeromq`
crate's reconnect backoff (about 1.1 s); wait for the port, then connect (2–5 ms).

Risks and blockers still open:
1. **Wheels are the real dependency.** Chaquopy's `cp313` wheels are old (numpy 1.26 is not
   officially 3.13; OpenBLAS from 2017, FreeType 2.9.1, libgfortran 4.9), pin us to Python 3.13,
   and come from one third-party index. Fallbacks: build them ourselves with Chaquopy's or
   BeeWare's recipes (days of work per package), or ship numpy without BLAS and plots as a later
   pack. Their licences are the upstream ones (BSD-3, PSF, MIT-CMU, MIT); `libgfortran` is
   GPL-3 with the GCC runtime exception, FreeType is FTL or GPL-2: add them to the About page.
2. **Executing the pack on a real device.** From Android's rules (not tested here: the spike ran
   as the `adb shell` user): an app targeting API 29+ cannot `exec` a file it downloaded into its
   own data directory, but can `dlopen` a `.so` from there. So the kernel launcher must ship in
   the APK (as `lib…so` in `jniLibs`) and load `libpython3.13.so` from the pack, or run in-process.
   To test with a real APK at K3, with the pack on arm64.
3. **arm64 never ran**, and the emulator is a desktop CPU: real phone timings are unknown.
4. **Licences of the Rust side:** `jupyter-zmq-client`, `jupyter-protocol`, `nbformat`,
   `jupyter-websocket-client` BSD-3-Clause; `zeromq` MIT; `pyo3` Apache-2.0 or MIT; of 159 crates
   in the tree one is MPL-2.0 (`option-ext`, via `dirs`) and none is GPL.

---

## Order of work (decided)

One release holds everything: **0.3.0 = account and sync (D20) + TeXnika + Naoty**, after the
comfort release 0.2.3 (Mahay-managed updates, phone layouts, Open with).

| Step | Content |
|---|---|
| 0.2.2 | being released (Android keyboard fix, security, animated PDFs in Taratasy) |
| 0.2.3 | comfort for Android: hub-managed updates, phone layouts for the first apps, Open with |
| spikes | M0 (can Tectonic run on Android?) and K0 (Python and a Jupyter client on Android) decide the engines before any app code |
| 0.3.0 | sync and account, TeXnika M1–M3, Naoty K1–K3 (Python, local and remote kernels) |
| 0.3.x | TeXnika M4–M5 (LyX-style blocks, all packs, biblatex), Naoty K4 (other kernels) |

Decided answers: packs of 50–100 MB are fine ("Core + Mathematics"); a first TeXnika with the
real engine and the instant math layer is enough; Naoty starts with Python (numpy, sympy,
matplotlib), then other languages.
