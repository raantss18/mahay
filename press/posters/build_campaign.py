#!/usr/bin/env python3
"""Mahay campaign posters (1080 x 1440, fr / en / mg), Tontolo's way: one idea per poster, short
lines to read at a glance, the app's icon, a QR code to the latest release, and the caption to
post with it.

    python3 -I press/posters/build_campaign.py 2026-10-18

Input:  press/campaigns/<date>/campaign.json
Output: press/campaigns/<date>/<id>-<lang>.png, captions.md, overview-<lang>.png
Rules (docs/CAMPAIGNS.md): no personal names or data, no emoji, every poster in the three
languages, the QR code goes to the latest release (where Mahay HUB is the file to download).
Needs rsvg-convert, qrencode and ImageMagick.
"""
import html
import json
import os
import subprocess
import sys
import tempfile

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
LATEST = "https://github.com/raantss18/mahay/releases/latest"
W, H = 1080, 1440
INK = "#1F2328"
PAPER = "#F7F5F2"
MUTED = "#5B6470"
FONT = "Inter, 'DejaVu Sans', sans-serif"
LABELS = {
    "fr": {"get": "Installez Mahay HUB", "scan": "Scannez pour télécharger", "free": "Gratuit · Hors ligne · Android et Linux"},
    "en": {"get": "Install Mahay HUB", "scan": "Scan to download", "free": "Free · Offline · Android and Linux"},
    "mg": {"get": "Apetraho i Mahay HUB", "scan": "Scannez raha haka", "free": "Maimaim-poana · Tsy mila aterineto · Android sy Linux"},
}
ACCENT = {
    "hub": "#E4572E", "mahay": "#E4572E", "sary": "#2E86DE", "taratasy": "#D7263D", "soratra": "#3F51B5", "tabilao": "#1B998B",
    "seho": "#F28F3B", "tsilo": "#8E44AD", "endrika": "#0097A7", "pejy": "#795548", "hazavana": "#E09F00", "feo": "#D81B60",
    "hetsika": "#37474F", "tselatra": "#7CB342", "drafitra": "#0D47A1",
}


def check(text, where):
    if any(ord(c) >= 0x1F000 or 0x2600 <= ord(c) < 0x2800 for c in text):
        raise SystemExit(f"{where}: no emoji on posters")


def icon_body(app):
    """The app's mark (brand/svg/<app>.svg) without its <svg> wrapper."""
    svg = open(os.path.join(ROOT, "brand", "svg", f"{app}.svg"), encoding="utf-8").read()
    return svg.split("\n", 1)[1].rsplit("</svg>", 1)[0]


def qr_svg():
    out = subprocess.run(["qrencode", "-t", "SVG", "-m", "1", "-s", "10", "-o", "-", LATEST], check=True, capture_output=True, text=True).stdout
    return out[out.index("<svg"):]


def lines(items, x, y, size, color, weight="400", gap=1.25):
    out = []
    for i, t in enumerate(items):
        out.append(f'<text x="{x}" y="{y + i * size * gap:.0f}" font-family="{FONT}" font-size="{size}" '
                   f'font-weight="{weight}" fill="{color}">{html.escape(t)}</text>')
    return "\n".join(out)


def poster(p, lang, qr):
    t = p[lang]
    check(json.dumps(t, ensure_ascii=False), f"{p['id']}-{lang}")
    app = p.get("app", "hub")
    accent = ACCENT.get(app, "#E4572E")
    lab = LABELS[lang]
    title_size = 84 if max(len(s) for s in t["title"]) <= 18 else 68
    body = [
        f'<rect width="{W}" height="{H}" fill="{PAPER}"/>',
        f'<rect width="{W}" height="18" fill="{accent}"/>',
        f'<g transform="translate(72,96) scale(1.5)">{icon_body(app)}</g>',
        lines([t["kicker"]], 72, 330, 34, accent, "700"),
        lines(t["title"], 72, 430, title_size, INK, "800", 1.12),
        lines(t["sub"], 72, 430 + len(t["title"]) * title_size * 1.12 + 40, 38, MUTED, "400", 1.35),
    ]
    y = 980
    for point in t.get("points", []):
        body.append(f'<circle cx="86" cy="{y - 12}" r="9" fill="{accent}"/>')
        body.append(lines([point], 112, y, 34, INK, "500"))
        y += 58
    body += [
        f'<rect x="0" y="{H - 250}" width="{W}" height="250" fill="{INK}"/>',
        f'<g transform="translate({W - 72 - 200},{H - 225}) scale({200 / 330:.4f})"><rect width="330" height="330" fill="#FFFFFF"/>{qr}</g>',
        lines([lab["get"]], 72, H - 160, 46, "#FFFFFF", "800"),
        lines([lab["scan"]], 72, H - 108, 30, "#E6E6E6", "400"),
        lines([lab["free"]], 72, H - 62, 26, "#B8BEC6", "400"),
    ]
    return f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}">\n' + "\n".join(body) + "\n</svg>\n"


def main():
    date = sys.argv[1]
    folder = os.path.join(ROOT, "press", "campaigns", date)
    campaign = json.load(open(os.path.join(folder, "campaign.json"), encoding="utf-8"))
    qr = qr_svg()
    # Size the QR code's own viewBox to 330 units so the scale above holds.
    import re
    head_end = qr.index(">")
    head = re.sub(r'\s(width|height)="[^"]*"', "", qr[:head_end])
    qr = head.replace("<svg", '<svg width="330" height="330"', 1) + qr[head_end:]
    captions = [f"# Campagne {date}: {campaign['theme']}", ""]
    made = {"fr": [], "en": [], "mg": []}
    with tempfile.TemporaryDirectory() as tmp:
        for p in campaign["posters"]:
            for lang in ("fr", "en", "mg"):
                svg = os.path.join(tmp, f"{p['id']}-{lang}.svg")
                with open(svg, "w", encoding="utf-8") as f:
                    f.write(poster(p, lang, qr))
                png = os.path.join(folder, f"{p['id']}-{lang}.png")
                subprocess.run(["rsvg-convert", "-w", str(W), "-h", str(H), svg, "-o", png], check=True)
                made[lang].append(png)
                captions.append(f"## {p['id']} ({lang})\n\n{p[lang]['caption']}\n\n{LATEST}\n")
    with open(os.path.join(folder, "captions.md"), "w", encoding="utf-8") as f:
        f.write("\n".join(captions))
    for lang, pngs in made.items():
        subprocess.run(["magick", "montage", *pngs, "-tile", "5x", "-geometry", "270x360+8+8", "-background", "#DDDDDD",
                        os.path.join(folder, f"overview-{lang}.png")], check=True)
    print(f"{sum(len(v) for v in made.values())} posters in {folder}")


if __name__ == "__main__":
    main()
