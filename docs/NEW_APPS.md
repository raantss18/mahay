# New apps: a LaTeX editor (15th) and a notebook (16th)

Status: **agreed in principle** (2026-10-19): the owner chose the names, one release (0.3.0), packs
of 50 to 100 MB, a first TeXnika with the real engine and the instant math layer (LyX-style blocks
come after), Python first for Naoty. The engine itself stays open until the spikes report. Nothing here is built;
every number marked "to measure" comes from the spike in M0/K0, never from this page.

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
| **Tectonic** (XeTeX, embeddable Rust crate, MIT) | yes: most packages | engine a few MB; packages fetched from a bundle on demand | a Tectonic-based F-Droid app (TeXslate) compiles on device, so it is possible; its core is mostly C, so the NDK build of its C libraries must be proven | **recommended**, behind the M0 spike |
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

### Packs ("preinstalled packages")

Installed from inside TeXnika (or Mahay HUB), signed, offline afterwards. Sizes: **to measure**.

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
| **M0 spike** | Tectonic builds for Linux and for Android (arm64 and x86_64); compiles an article with `amsmath`, `tikz` and BibTeX **offline** from a curated mini-bundle; measure size and time | decides the engine (D25) |
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
