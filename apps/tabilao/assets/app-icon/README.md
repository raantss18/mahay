# Tabilao app icon (PLACEHOLDER)

**This icon is temporary.** Every Crafting App's icon is an engraved creature portrait that the
product owner makes (mahay-rules `standards/icon-design.md`); Tabilao's hasn't been made yet.
Until it is, this placeholder stands in: a simple ledger page with a header row, a row-number
column, a selected cell with its fill handle and a double rule under a total, on a full-bleed
Tabilao-green tile. It is original geometry written by hand for this repository, deliberately
not modelled on any vendor's spreadsheet logo (no letters, no "X", no overlapping panels).

When the owner's art arrives, replace `tabilao.svg`, run `packaging/icons.sh`, update this
README (provenance, creature) and the rows in `ATTRIBUTION.md`, and drop the word placeholder.

## Palette

Exactly three colours:

| Colour | Hex | Used for |
|---|---|---|
| Ink | `#0b0b0c` | Line work and contour |
| Paper | `#efe9dc` | The page |
| Tabilao green (app colour, **proposed**) | `#1f9d55` | The full-bleed field and the header cells |

The app colour is a proposal pending the owner's approval: Tabilao green `#1f9d55`, with ink
(darker) variant `#147a40` for text and accents on light backgrounds. It differs from
Pejy's `#7bb51c`.

## Geometry

A 512 × 512 viewBox, clipped to a rounded square with `rx=112`. Windows and Linux use the full
tile. The macOS icons (`.icns`, `tabilao-macos-512.png`) sit on Apple's grid: an 824/1024
body with a transparent margin.

## Files

| File | Use |
|---|---|
| `tabilao.svg` | Canonical artwork; also `hicolor/scalable` |
| `tabilao-1024.png` | 1024 px render of the full tile |
| `tabilao-macos-512.png` | Runtime Dock icon on macOS (for `apps/tabilao` to embed) |
| `tabilao.icns` | macOS bundle icon (`CFBundleIconFile`) |
| `tabilao.ico` | Windows exe icon, 16–256 px (embedded by `apps/tabilao/build.rs`) |
| `hicolor/<size>/apps/ai.storyteller.tabilao.png` | Linux icon theme, 16–512 px; the 256 px one is also the runtime window icon on Windows and Linux |
| `hicolor/scalable/apps/ai.storyteller.tabilao.svg` | Linux scalable icon |
| `LICENSE.txt` | MIT OR Apache-2.0 |

The app ID is `ai.storyteller.tabilao`. It's used for the hicolor icon names, the Wayland app
ID and `packaging/linux/ai.storyteller.tabilao.desktop`.

## Regenerate

Edit `tabilao.svg`, then run `packaging/icons.sh`. It needs `resvg`, and `iconutil` for the
`.icns` (macOS only). It packs the `.ico` with `cargo xtask ico`.
