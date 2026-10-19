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
- **Kernels:** the Jupyter messaging protocol natively in Rust over ZeroMQ (the `runtimelib` /
  `jupyter-protocol` crates of the runtimed project are used by Zed and Deno, and need no C
  library; licences to check). Three kinds: a **local kernel** (Linux: the system's `ipykernel`
  and any installed kernelspec; Android: a Python pack), a **remote kernel** (a Jupyter server on
  the user's PC or a server, the light option for a tablet), and a small **built-in** one
  (JavaScript through Boa, already in Taratasy).
- **Python on Android:** CPython supports Android officially from 3.13 (PEP 738, to confirm);
  the real work is science wheels (`numpy`, `matplotlib`, `sympy`) and a kernel without `pyzmq`
  (a C extension): the likely answer is our own small Rust kernel that embeds CPython and speaks
  the protocol itself. A Python pack with the scientific stack is large (to measure, likely
  100 MB or more): optional, never in the app.
- **Safety:** a notebook never runs by itself; opening one from elsewhere shows outputs
  without executing anything, as Jupyter's "trust" model does; HTML outputs never run scripts.
- **Milestones:** K0 spike (CPython and a kernel on Android, a Rust Jupyter client talking to
  a Linux `ipykernel`), K1 notebook UI on Linux with a local Python kernel, K2 Android with a
  remote kernel, K3 the Python pack on Android, K4 other kernels (Julia, Lean 4, Rust/evcxr).

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
