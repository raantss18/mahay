# Mahay Trano app icon

The app icon is the **Mahay "A" mark in blue** (mahay.com's favicon), exactly as supplied in
[`docs/brand/mahay-mark.svg`](../../docs/brand/mahay-mark.svg). Every file in this
directory is rendered from it by `packaging/icons.sh`.

**These files are an Mahay trademark. They are not open source and not covered by Mahay
Launcher's MIT or Apache-2.0 license.** Their terms are in
[`docs/brand/LICENSE-brand.txt`](../../docs/brand/LICENSE-brand.txt): use them only as part of
Mahay Trano, never redraw, recolor or crop them, and remove them from forks and modified
versions.

## Files

- `trano.svg`: a copy of `docs/brand/mahay-mark.svg`; also the hicolor scalable icon.
- `trano-1024.png`: 1024 px render, with extra transparent margin for Apple's icon grid.
- `trano.icns`: macOS bundle icon (`CFBundleIconFile`).
- `trano.ico`: Windows icon, 16–256 px, embedded in the `.exe` by
  `apps/trano/build.rs` and used by the MSI.
- `hicolor/<size>/apps/ai.storyteller.trano.png` (16–512) and
  `hicolor/scalable/apps/ai.storyteller.trano.svg`: Linux icon theme.
- The tray / menu-bar icon is the same mark, rendered into `docs/brand/mahay-mark-black-44.png`
  (macOS template, 22 pt @2x) and `docs/brand/mahay-mark-64.png` (blue; Windows, Linux, FreeBSD).

## Regenerate

Run `packaging/icons.sh` on any OS. It needs `resvg`; the `.ico` and `.icns` are packed by
`cargo xtask ico`. Never edit these files by hand: change the source only if the Mahay Team
supplies a new mark in `docs/brand/`.
