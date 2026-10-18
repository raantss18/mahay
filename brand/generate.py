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
    # Trano, the hub: a house holding a 2x2 grid of apps.
    "trano": ("#E4572E", [
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


def main():
    for app in APPS:
        write(os.path.join(ROOT, "brand/svg", f"{app}.svg"), svg(app))
        write(os.path.join(ROOT, "brand/svg", f"{app}-mono.svg"), svg(app, mono=True))
        if app == "mahay":
            continue
        res = os.path.join(ROOT, "android/app/src", app, "res")
        color = APPS[app][0]
        write(os.path.join(res, "drawable/ic_launcher_foreground.xml"), vector(app))
        write(os.path.join(res, "mipmap-anydpi-v26/ic_launcher.xml"), ADAPTIVE)
        write(
            os.path.join(res, "values/ic_launcher.xml"),
            f'<?xml version="1.0" encoding="utf-8"?>\n<resources>\n    <color name="ic_launcher_background">#FF{color[1:]}</color>\n</resources>\n',
        )
        app_dir = os.path.join(ROOT, "apps", app)
        if os.path.isdir(app_dir):
            write(os.path.join(app_dir, "assets/mahay-icon.svg"), svg(app))
    print("icons written for", ", ".join(APPS))


if __name__ == "__main__":
    main()
