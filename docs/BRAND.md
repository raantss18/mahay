# Mahay brand

## Name

**Mahay** (Malagasy: *skilled, able, knows how*). Each app has a one-word Malagasy name
(`docs/DECISIONS.md` D2); its full name is "Mahay <App>" where context needs it.

## The icon family

![The family](img/icons.png)

One design language for the suite:

- **Grid:** 108 × 108 units (Android's adaptive-icon canvas). Glyphs stay inside the 66-unit
  safe zone (21–87), so any launcher mask keeps them whole.
- **Line:** one weight, 6 units, round caps and joins. Fills only for small accents (a dot, a
  selected cell, a play mark).
- **Colour:** a white glyph on one accent colour per app; a rounded tile (radius 24) for
  full-colour use. The monochrome variant is the glyph alone (Android 13 themed icons, tray).
- **No text, no equations, no detail** that disappears at 16 px.

| Mark       | Accent    | Glyph                                   |
|------------|-----------|-----------------------------------------|
| Mahay      | `#E4572E` | two peaks and the rising sun            |
| Trano      | `#E4572E` | a house holding a 2×2 grid of apps      |
| Sary       | `#2E86DE` | a frame, a peak and the sun             |
| Taratasy   | `#D7263D` | a page with a folded corner             |
| Soratra    | `#3F51B5` | lines of text and the cursor            |
| Tabilao    | `#1B998B` | a 3×3 grid with the selected cell       |
| Seho       | `#F28F3B` | a screen on a stand, play mark          |
| Tsipika    | `#8E44AD` | a curve between two anchors             |
| Endrika    | `#0097A7` | a screen with a layout                  |
| Pejy       | `#795548` | a page in two columns                   |
| Hazavana   | `#E09F00` | the sun                                 |
| Feo        | `#D81B60` | a waveform                              |
| Hetsika    | `#37474F` | a film strip                            |
| Tselatra   | `#7CB342` | a lightning bolt                        |
| Drafitra   | `#0D47A1` | a set square and a dimension line       |

The wordmark is the Mahay tile followed by MAHAY drawn with the same strokes (no font).

## Files

Everything is generated; nothing is hand-exported:

```bash
python3 -I brand/generate.py   # brand/svg/*, Android adaptive icons (vector, no PNGs)
python3 -I brand/apply.py      # every app's assets/app-icon (PNG sizes, SVG, ICO, ICNS),
                               # docs/brand, the launcher's app icons
```

## License and use

The marks are original Mahay artwork under MIT OR Apache-2.0, like the code. Use them to refer
to Mahay. Don't use them in a way that suggests the project made or endorses something it
didn't.

The upstream ArtCraft name, logos and app icons are trademarks of the ArtCraft Team. They were
removed from Mahay and must not come back. The only mention left is the attribution "Forked
from ArtCraft".
