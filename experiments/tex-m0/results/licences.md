# Licences of the files in the spike bundle (610 files, 6 packs)

`licence-scan.md` is the raw header scan (a grep of each file's own text); this page is the
manual reading of what it found. Nothing found is non-redistributable, but three groups need
attention before a pack is published:

| Group | Licence (from the files' own headers) | Note |
|---|---|---|
| LaTeX kernel, `article`, `ams*`, `mathtools`, `hyperref`, `graphicx`, `babel`, `fontspec`, `unicode-math`, `natbib`, `biblatex`, `csquotes`, `beamer`, `algorithm2e`, `physics`, `siunitx`, `tcolorbox`, `listings`, `caption`, `subcaption`, `wrapfig`… | LPPL 1.3c | free; a modified file must change its name; the licence text ships in the pack |
| `amsfonts.sty` and the other AMS font files | AMS licence (unlimited copying as long as the file is not modified) | redistributable unmodified |
| Latin Modern, Latin Modern Math, TeX Gyre (`lm*.otf`, `latinmodern-math.otf`, `texgyre*.otf`) | GUST Font License (LPPL-like) | free; renaming rule on modification |
| Computer Modern Type 1 (`cm*.pfb`) | SIL OFL 1.1 (AMS 2009, Reserved Font Name) | free |
| `UnicodeData.txt`, `CaseFolding.txt`, `SpecialCasing.txt` | Unicode licence | free |
| `cancel.sty`, `relsize.sty` | public domain | |
| **`pgfplots`** (28 files of the Figures pack) | **GPL-3.0-or-later** (header: "either version 3 of the License, or…") | copyleft TeX source: it is a separate data pack, shipped as source with its licence text, but it must be listed and is the one item that is not permissive |
| **Hyphenation patterns** `hyph-*.tex` (the format loads all of them) | mostly MIT/LPPL/BSD; **GPL**: `hyph-as,bn,ga,gu,hi,kn,cs,hu,id,lv,mk,sk` (headers: "licence: name: GPL") | Core only needs the languages the app offers: drop the GPL ones or ship them as a language pack |
| `float.sty`, `preview.sty`, `algorithmicx.sty`, `biblatex.def`/`blx-*` | headers say "distribute together with the .dtx", "see the source", or carry only a copyright line | the packages are free on CTAN (float: LPPL; algorithmicx: LPPL; biblatex: LPPL via `biblatex.sty`; TeX Live lists `preview` as GPL-3.0+): confirm before publishing; not verified from the files alone |

Unverified because not read in the files: the licence of every package in a pack that the
samples did not load (the Mathematics, Slides and Fonts packs hold only what the samples used).
