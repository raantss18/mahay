#!/usr/bin/env python3
"""Generate the Mahay icon family from one shape grid.

Every mark is drawn on a 108x108 grid (Android's adaptive-icon canvas). The glyph stays
inside the 66x66 safe zone (21..87) so any launcher mask keeps it whole. Each app is a white
glyph on its accent colour; the monochrome variant is the glyph alone.

Outputs:
  brand/svg/<app>.svg             full-colour master (rounded tile)
  brand/svg/<app>-mono.svg        monochrome glyph (currentColor)
  android/app/src/<flavour>/res/  adaptive icon: vector foreground, colour background,
                                  monochrome layer (Android 13 themed icons)
  apps/<app>/assets/mahay-icon.svg
Run: python3 -I brand/generate.py
"""
import os

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

# Stroke-based glyphs: (kind, data). "s" = stroked path, "f" = filled path.
# Stroke width is 6 units everywhere: the one line weight of the family.
W = 6
APPS = {
    # Master mark: two peaks (the island's highlands) and the rising sun.
    "mahay": ("#E4572E", [
        ("s", "M27,76 L44,44 L54,60 L64,44 L81,76"),
        ("f", "M54,25 a7,7 0 1,0 0.01,0 Z"),
    ]),
    # Mahay HUB, the hub: a house holding a 2x2 grid of apps.
    "hub": ("#E4572E", [
        ("s", "M28,50 L54,28 L80,50 L80,80 L28,80 Z"),
        ("f", "M41,55 h10 v9 h-10 Z M57,55 h10 v9 h-10 Z M41,68 h10 v8 h-10 Z M57,68 h10 v8 h-10 Z"),
    ]),
    # Sary, images: a frame, a peak and the sun.
    "sary": ("#2E86DE", [
        ("s", "M27,31 H81 V77 H27 Z"),
        ("s", "M30,74 L48,52 L60,66 L67,58 L78,72"),
        ("f", "M66,40 a5,5 0 1,0 0.01,0 Z"),
    ]),
    # Taratasy, PDF: a page with a folded corner and a stamp line.
    "taratasy": ("#D7263D", [
        ("s", "M33,25 H63 L77,39 V83 H33 Z"),
        ("s", "M63,25 V39 H77"),
        ("s", "M42,56 H68 M42,68 H60"),
    ]),
    # Soratra, writing: lines of text and the cursor.
    "soratra": ("#3F51B5", [
        ("s", "M27,34 H81 M27,48 H81 M27,62 H62 M27,76 H50"),
        ("s", "M72,57 V81"),
    ]),
    # Tabilao, spreadsheet: a 3x3 grid with the selected cell filled.
    "tabilao": ("#1B998B", [
        ("s", "M27,27 H81 V81 H27 Z M27,45 H81 M27,63 H81 M45,27 V81 M63,27 V81"),
        ("f", "M45,45 H63 V63 H45 Z"),
    ]),
    # Seho, presentations: a screen on a stand with a play mark.
    "seho": ("#F28F3B", [
        ("s", "M25,29 H83 V69 H25 Z"),
        ("s", "M54,69 V82 M42,82 H66"),
        ("f", "M48,39 L64,49 L48,59 Z"),
    ]),
    # Later apps (no Android build in release 1 yet), same grid and line weight.
    # Tsilo, vector illustration: a curve between two anchors, with a handle.
    "tsilo": ("#8E44AD", [
        ("s", "M30,76 C36,40 72,40 78,76"),
        ("s", "M54,46 L72,30"),
        ("f", "M25,71 h10 v10 h-10 Z M73,71 h10 v10 h-10 Z M68,26 h8 v8 h-8 Z"),
    ]),
    # Endrika, interface design: a screen holding a header and two blocks.
    "endrika": ("#0097A7", [
        ("s", "M33,24 H75 V84 H33 Z"),
        ("f", "M40,32 H68 V40 H40 Z M40,48 H52 V76 H40 Z M57,48 H68 V60 H57 Z"),
    ]),
    # Pejy, page layout: a page set in two columns under a title.
    "pejy": ("#795548", [
        ("s", "M30,24 H78 V84 H30 Z"),
        ("f", "M38,32 H70 V40 H38 Z"),
        ("s", "M39,52 H49 M39,62 H49 M39,72 H49 M59,52 H69 M59,62 H69 M59,72 H69"),
    ]),
    # Hazavana, photo library and raw: the sun's disc and rays.
    "hazavana": ("#E09F00", [
        ("f", "M54,40 a14,14 0 1,0 0.01,0 Z"),
        ("s", "M54,24 V30 M54,78 V84 M24,54 H30 M78,54 H84 M33,33 L37,37 M71,71 L75,75 M33,75 L37,71 M71,37 L75,33"),
    ]),
    # Feo, audio: a waveform.
    "feo": ("#D81B60", [
        ("s", "M28,48 V60 M38,38 V70 M48,28 V80 M58,42 V66 M68,32 V76 M78,46 V62"),
    ]),
    # Hetsika, video: a strip of film.
    "hetsika": ("#37474F", [
        ("s", "M30,26 H78 V82 H30 Z M41,26 V82 M67,26 V82"),
        ("f", "M33,32 h5 v6 h-5 Z M33,51 h5 v6 h-5 Z M33,70 h5 v6 h-5 Z M70,32 h5 v6 h-5 Z M70,51 h5 v6 h-5 Z M70,70 h5 v6 h-5 Z"),
    ]),
    # Tselatra, motion graphics and effects: a lightning bolt.
    "tselatra": ("#7CB342", [
        ("f", "M60,22 L34,60 H52 L46,86 L74,46 H56 Z"),
    ]),
    # Drafitra, CAD: a set square and a dimension line.
    "drafitra": ("#0D47A1", [
        ("s", "M30,78 V30 L78,78 Z"),
        ("s", "M40,66 V56 L50,66 Z"),
        ("s", "M30,86 H78"),
    ]),
}


def svg(app, mono=False):
    color, parts = APPS[app]
    ink = "currentColor" if mono else "#FFFFFF"
    body = []
    if not mono:
        body.append(f'<rect x="4" y="4" width="100" height="100" rx="24" fill="{color}"/>')
    for kind, d in parts:
        if kind == "s":
            body.append(f'<path d="{d}" fill="none" stroke="{ink}" stroke-width="{W}" stroke-linecap="round" stroke-linejoin="round"/>')
        else:
            body.append(f'<path d="{d}" fill="{ink}"/>')
    inner = "\n  ".join(body)
    return f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 108 108" width="108" height="108">\n  {inner}\n</svg>\n'


def vector(app):
    _, parts = APPS[app]
    body = []
    for kind, d in parts:
        if kind == "s":
            body.append(
                f'    <path android:pathData="{d}" android:strokeColor="#FFFFFFFF" android:strokeWidth="{W}" '
                'android:strokeLineCap="round" android:strokeLineJoin="round" android:fillColor="#00000000" />'
            )
        else:
            body.append(f'    <path android:pathData="{d}" android:fillColor="#FFFFFFFF" />')
    inner = "\n".join(body)
    return (
        '<?xml version="1.0" encoding="utf-8"?>\n'
        '<vector xmlns:android="http://schemas.android.com/apk/res/android"\n'
        '    android:width="108dp" android:height="108dp"\n'
        '    android:viewportWidth="108" android:viewportHeight="108">\n'
        f"{inner}\n"
        "</vector>\n"
    )


ADAPTIVE = """<?xml version="1.0" encoding="utf-8"?>
<adaptive-icon xmlns:android="http://schemas.android.com/apk/res/android">
    <background android:drawable="@color/ic_launcher_background" />
    <foreground android:drawable="@drawable/ic_launcher_foreground" />
    <monochrome android:drawable="@drawable/ic_launcher_foreground" />
</adaptive-icon>
"""


def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        f.write(text)


# Apps with an Android flavour (android/app/build.gradle).
ANDROID = {"hub", "sary", "taratasy", "soratra", "tabilao", "seho", "tsilo", "endrika", "pejy", "hazavana", "feo", "hetsika", "tselatra", "drafitra"}


# The wordmark: the mark, then MAHAY drawn with the same line weight (no font needed).
LETTERS = "M130,78 V30 L152,60 L174,30 V78 M190,78 L210,30 L230,78 M197,62 H223 M247,30 V78 M281,30 V78 M247,54 H281 M298,78 L318,30 L338,78 M305,62 H331 M352,30 L371,56 L390,30 M371,56 V78"


def wordmark(ink):
    tile = svg("mahay").split("\n", 1)[1].rsplit("</svg>", 1)[0]
    return (
        '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 400 108" width="400" height="108">\n'
        f"  {tile.strip()}\n"
        f'  <path d="{LETTERS}" fill="none" stroke="{ink}" stroke-width="9" stroke-linecap="round" stroke-linejoin="round"/>\n'
        "</svg>\n"
    )


def main():
    write(os.path.join(ROOT, "brand/svg/mahay-logo.svg"), wordmark("#1F2328"))
    write(os.path.join(ROOT, "brand/svg/mahay-logo-white.svg"), wordmark("#FFFFFF"))
    write(os.path.join(ROOT, "brand/svg/mahay-mark-black.svg"), svg("mahay", mono=True).replace("currentColor", "#1F2328"))
    for app in APPS:
        write(os.path.join(ROOT, "brand/svg", f"{app}.svg"), svg(app))
        write(os.path.join(ROOT, "brand/svg", f"{app}-mono.svg"), svg(app, mono=True))
        if app == "mahay" or app not in ANDROID:
            continue
        res = os.path.join(ROOT, "android/app/src", app, "res")
        color = APPS[app][0]
        write(os.path.join(res, "drawable/ic_launcher_foreground.xml"), vector(app))
        write(os.path.join(res, "mipmap-anydpi-v26/ic_launcher.xml"), ADAPTIVE)
        write(
            os.path.join(res, "values/ic_launcher.xml"),
            f'<?xml version="1.0" encoding="utf-8"?>\n<resources>\n    <color name="ic_launcher_background">#FF{color[1:]}</color>\n</resources>\n',
        )
    print("icons written for", ", ".join(APPS))


if __name__ == "__main__":
    main()
