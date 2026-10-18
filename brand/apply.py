#!/usr/bin/env python3
"""Replace upstream brand artwork in the imported app trees with the Mahay icon family.

Upstream ships its own app icons (engravings), the ArtCraft logo and mark (trademarks, not
open source) and screenshots of its apps. None of that may stay in Mahay. This script:

  - re-renders every file under apps/<app>/assets/app-icon/ (PNG at the file's own size, SVG,
    ICO, ICNS) from brand/svg/<app>.svg, and rewrites that folder's LICENSE.txt and README.md;
  - replaces apps/*/docs/brand/ (upstream marks) with the Mahay mark, and its license;
  - re-renders the launcher's per-app icons (apps/trano/assets/app-icons/<id>.png);
  - deletes upstream screenshots (docs/images, docs/screenshots) and the launcher's
    social-network logos.

Needs rsvg-convert, ImageMagick and qrencode. Run after brand/generate.py:
    python3 -I brand/apply.py
"""
import os
import shutil
import struct
import subprocess
import tempfile

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SVG = os.path.join(ROOT, "brand", "svg")
APPS = ["trano", "sary", "taratasy", "soratra", "tabilao", "seho", "tsilo", "endrika", "pejy", "hazavana", "feo", "hetsika", "tselatra", "drafitra"]

ICON_LICENSE = """{name} app icon (every file in this directory).

Original artwork for Mahay, generated from brand/svg/{id}.svg by brand/generate.py and
brand/apply.py. No third-party material. Licensed like the rest of Mahay, under either of

  * Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
  * MIT license (https://opensource.org/licenses/MIT)

at your option. See docs/BRAND.md for how to use the Mahay marks.
"""

ICON_README = """# {name} app icon

The {name} mark of the Mahay icon family: a white glyph on the app's accent colour, drawn on
the shared 108-unit grid with one 6-unit line weight. See docs/BRAND.md.

Every file here is generated. Edit brand/generate.py, then run:

    python3 -I brand/generate.py && python3 -I brand/apply.py
"""

BRAND_LICENSE = """Mahay marks: license

The Mahay name, mark and app icons in this directory are original Mahay artwork, licensed under
MIT OR Apache-2.0 like the code. Use them to refer to Mahay. Don't use them in a way that
suggests the Mahay project made or endorses something it didn't. See docs/BRAND.md.

The upstream ArtCraft marks that used to be here were removed: they are trademarks of the
ArtCraft Team and are not part of Mahay.
"""


def render_png(svg, size, out):
    subprocess.run(["rsvg-convert", "-w", str(size), "-h", str(size), svg, "-o", out], check=True)


def png_size(path):
    with open(path, "rb") as f:
        head = f.read(24)
    if head[:8] != b"\x89PNG\r\n\x1a\n":
        return None
    w, h = struct.unpack(">II", head[16:24])
    return w, h


def write_icns(svg, out):
    """An ICNS made of PNG entries (ic07..ic10), which macOS 10.7+ reads."""
    entries = b""
    with tempfile.TemporaryDirectory() as tmp:
        for kind, size in [(b"ic07", 128), (b"ic08", 256), (b"ic09", 512), (b"ic10", 1024)]:
            p = os.path.join(tmp, f"{size}.png")
            render_png(svg, size, p)
            data = open(p, "rb").read()
            entries += kind + struct.pack(">I", len(data) + 8) + data
    with open(out, "wb") as f:
        f.write(b"icns" + struct.pack(">I", len(entries) + 8) + entries)


def write_ico(svg, out):
    with tempfile.TemporaryDirectory() as tmp:
        pngs = []
        for size in [16, 24, 32, 48, 64, 128, 256]:
            p = os.path.join(tmp, f"{size}.png")
            render_png(svg, size, p)
            pngs.append(p)
        subprocess.run(["magick", *pngs, out], check=True)


def brand_source(name):
    """The Mahay file standing in for an upstream brand file of this name."""
    stem = os.path.splitext(name)[0]
    for key in ("logo-white", "logo", "mark-black"):
        if key in stem:
            return os.path.join(SVG, f"mahay-{key}.svg")
    return os.path.join(SVG, "mahay.svg")


def render_any(svg, path, size):
    """Render keeping the file's own width and height (a wordmark isn't square)."""
    w, h = size
    subprocess.run(["rsvg-convert", "-w", str(w), "-h", str(h), "--keep-aspect-ratio", svg, "-o", path], check=True)


def replace_icon_dir(app, folder, brand=False):
    for dirpath, _, files in os.walk(folder):
        for name in files:
            svg = brand_source(name) if brand else os.path.join(SVG, f"{app}.svg")
            path = os.path.join(dirpath, name)
            ext = os.path.splitext(name)[1].lower()
            if ext == ".png":
                size = png_size(path)
                if size:
                    render_any(svg, path, size)
            elif ext == ".svg":
                shutil.copyfile(svg, path)
            elif ext == ".ico":
                write_ico(svg, path)
            elif ext == ".icns":
                write_icns(svg, path)


def main():
    names = {"trano": "Mahay Trano", "sary": "Sary", "taratasy": "Taratasy", "soratra": "Soratra", "tabilao": "Tabilao", "seho": "Seho",
             "tsilo": "Tsilo", "endrika": "Endrika", "pejy": "Pejy", "hazavana": "Hazavana", "feo": "Feo", "hetsika": "Hetsika",
             "tselatra": "Tselatra", "drafitra": "Drafitra"}
    for app in APPS:
        base = os.path.join(ROOT, "apps", app)
        icon_dir = os.path.join(base, "assets", "app-icon")
        if os.path.isdir(icon_dir):
            replace_icon_dir(app, icon_dir)
            with open(os.path.join(icon_dir, "LICENSE.txt"), "w") as f:
                f.write(ICON_LICENSE.format(name=names[app], id=app))
            with open(os.path.join(icon_dir, "README.md"), "w") as f:
                f.write(ICON_README.format(name=names[app]))
        brand_dir = os.path.join(base, "docs", "brand")
        if os.path.isdir(brand_dir):
            replace_icon_dir("mahay", brand_dir, brand=True)
            for name in os.listdir(brand_dir):
                if name.startswith("LICENSE"):
                    with open(os.path.join(brand_dir, name), "w") as f:
                        f.write(BRAND_LICENSE)
        for shots in ("docs/images", "docs/screenshots"):
            shutil.rmtree(os.path.join(base, shots), ignore_errors=True)
    launcher_icons = os.path.join(ROOT, "apps", "trano", "assets", "app-icons")
    for name in os.listdir(launcher_icons):
        app, ext = os.path.splitext(name)
        svg = os.path.join(SVG, f"{app}.svg")
        if ext == ".png" and os.path.exists(svg):
            size = png_size(os.path.join(launcher_icons, name))
            render_png(svg, size[0] if size else 256, os.path.join(launcher_icons, name))
    # Upstream's social accounts aren't Mahay's (catalog::SOCIAL is empty).
    shutil.rmtree(os.path.join(ROOT, "apps", "trano", "assets", "social"), ignore_errors=True)
    # The About page's icons (raw 96x96 RGBA, no image decoder needed) and the QR code to the
    # latest release (a 0/1 module grid drawn as vector squares).
    about = os.path.join(ROOT, "libs", "core", "crates", "about", "assets")
    os.makedirs(about, exist_ok=True)
    with tempfile.TemporaryDirectory() as tmp:
        for app in ["mahay"] + APPS:
            png = os.path.join(tmp, f"{app}.png")
            render_png(os.path.join(SVG, f"{app}.svg"), 96, png)
            subprocess.run(["magick", png, "-depth", "8", f"rgba:{os.path.join(about, app + '.rgba')}"], check=True)
    qr = subprocess.run(["qrencode", "-t", "ASCII", "-m", "0", "https://github.com/raantss18/mahay/releases/latest"],
                        check=True, capture_output=True, text=True).stdout
    rows = [line for line in qr.split("\n") if line]
    grid = ["".join("1" if line[i:i + 2] == "##" else "0" for i in range(0, len(line), 2)) for line in rows]
    width = max(map(len, grid))
    with open(os.path.join(about, "qr-latest.txt"), "w") as f:
        f.write("\n".join(r.ljust(width, "0") for r in grid) + "\n")
    print("brand applied")


if __name__ == "__main__":
    main()
