#!/usr/bin/env python3
"""Rename upstream "…Craft" branding to Mahay names inside imported trees.

Usage: python3 -I scripts/rebrand.py <dir>...

Legal files (LICENSE-*, NOTICE, licenses/, ATTRIBUTION*, contributors/) are left
untouched so upstream copyright lines stay intact. Links to upstream issues and
pull requests are preserved verbatim (provenance), everything else is renamed.
File and directory names are renamed too.
"""
import os
import re
import sys

# Upstream name -> Mahay name (lowercase); case variants are derived.
APPS = {
    "photocraft": "sary",
    "pdfcraft": "taratasy",
    "wordcraft": "soratra",
    "gridcraft": "tabilao",
    "deckcraft": "seho",
    "vectorcraft": "tsipika",
    "figcraft": "endrika",
    "designcraft": "pejy",
    "lightcraft": "hazavana",
    "soundcraft": "feo",
    "filmcraft": "hetsika",
    "effectcraft": "tselatra",
    "cadcraft": "drafitra",
    "anicraft": "sarimiaina",
}
DISPLAY = {
    "PhotoCraft": "Sary", "PdfCraft": "Taratasy", "WordCraft": "Soratra",
    "GridCraft": "Tabilao", "DeckCraft": "Seho", "VectorCraft": "Tsipika",
    "FigCraft": "Endrika", "DesignCraft": "Pejy", "LightCraft": "Hazavana",
    "SoundCraft": "Feo", "FilmCraft": "Hetsika", "EffectCraft": "Tselatra",
    "CADCraft": "Drafitra", "CadCraft": "Drafitra", "AniCraft": "Sarimiaina",
    "ArtCraft Launcher": "Mahay Trano", "ArtCraft": "Mahay",
}
SKIP_NAMES = re.compile(r"^(LICENSE.*|NOTICE.*|ATTRIBUTION.*|COPYRIGHT.*)$", re.I)
SKIP_DIRS = {"licenses", "contributors", ".git", "target"}
UPSTREAM_REF = re.compile(r"https?://github\.com/storytold/[\w.-]+/(?:issues|pull|discussions)/\d+[^\s)\]\"'>`]*")
OUR_REPO = "https://github.com/raantss18/mahay"


def build_rules():
    rules = []
    for up, ours in DISPLAY.items():
        rules.append((re.escape(up), ours))
    rules += [
        (r"artcraft[-_]launcher", "trano"),
        (r"ARTCRAFT_LAUNCHER", "TRANO"),
        (r"https?://(?:app\.)?getartcraft\.com[^\s)\]\"'>`]*", OUR_REPO),
        (r"getartcraft", "mahay"),
        (r"https?://github\.com/storytold/craft-([\w.-]+)", OUR_REPO),
        (r"https?://github\.com/storytold/artcraft[\w.-]*", OUR_REPO),
        (r"https?://github\.com/storytold/(\w+?)craft\b", OUR_REPO),
        (r"storytold", "raantss18"),
    ]
    for up, ours in APPS.items():
        rules.append((up.upper(), ours.upper()))
        rules.append((up.capitalize(), ours.capitalize()))
        rules.append((up, ours))
    rules += [
        (r"ARTCRAFT", "MAHAY"), (r"Artcraft", "Mahay"), (r"artcraft", "mahay"),
        # Shared libraries: craft-fonts, craft_quit, CRAFT_FONTS_DIR, ...
        (r"\bcraft([-_])", r"mahay\1"),
        (r"\bCRAFT_", "MAHAY_"),
        (r"\bCraft apps\b", "Mahay apps"),
        (r"\bCrafting [Aa]pps\b", "Mahay apps"),
        (r"\bcraftrules\b", "mahay-rules"),
        (r"\bpcraft\b", "sary"),
        (r"\bPhoto Craft\b", "Sary"),
        (r"\bCraft\b", "Mahay"),
    ]
    return [(re.compile(p), r) for p, r in rules]


RULES = build_rules()


def rename_text(text):
    kept = []

    def stash(m):
        kept.append(m.group(0))
        return f"\x00{len(kept) - 1}\x00"

    text = UPSTREAM_REF.sub(stash, text)
    for pat, rep in RULES:
        text = pat.sub(rep, text)
    return re.sub(r"\x00(\d+)\x00", lambda m: kept[int(m.group(1))], text)


def is_text(path):
    with open(path, "rb") as f:
        chunk = f.read(8192)
    if b"\x00" in chunk:
        return False
    try:
        chunk.decode("utf-8")
    except UnicodeDecodeError:
        return False
    return True


def process(root):
    changed = 0
    for dirpath, dirnames, filenames in os.walk(root, topdown=True):
        dirnames[:] = [d for d in dirnames if d not in SKIP_DIRS]
        for name in filenames:
            if SKIP_NAMES.match(name):
                continue
            path = os.path.join(dirpath, name)
            if os.path.islink(path) or not is_text(path):
                continue
            with open(path, encoding="utf-8") as f:
                old = f.read()
            new = rename_text(old)
            if new != old:
                with open(path, "w", encoding="utf-8") as f:
                    f.write(new)
                changed += 1
    # Rename paths bottom-up so parents move last.
    for dirpath, dirnames, filenames in os.walk(root, topdown=False):
        if any(part in SKIP_DIRS for part in dirpath.split(os.sep)):
            continue
        for name in filenames + dirnames:
            if SKIP_NAMES.match(name):
                continue
            new = rename_text(name)
            if new != name:
                os.rename(os.path.join(dirpath, name), os.path.join(dirpath, new))
    return changed


if __name__ == "__main__":
    for d in sys.argv[1:]:
        print(d, process(d), "files changed")
