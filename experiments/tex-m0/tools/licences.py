"""Licence scan of the files in work/bundle, grouped by pack (reads each file's own header).
Run: python3 -I tools/licences.py   (from experiments/tex-m0)"""
import collections, os, re, sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
B = os.path.join(ROOT, "work", "bundle")
MARKS = [
    ("LPPL", r"LaTeX Project Public License|\blppl\b|LPPL"),
    ("GPL", r"GNU General Public License|\bGPL\b"),
    ("GFDL", r"GNU Free Documentation"),
    ("MIT", r"\bMIT License|Permission is hereby granted, free of charge"),
    ("BSD", r"Redistribution and use in source and binary forms"),
    ("OFL", r"SIL Open Font License|\bOFL\b"),
    ("GFL (GUST Font)", r"GUST Font License|\bGFL\b"),
    ("CC", r"Creative Commons"),
    ("Public domain", r"public domain"),
    ("Unicode", r"Unicode, Inc\.|unicode\.org/copyright|Unicode License"),
    ("Knuth (free, name-restricted)", r"Knuth|may be freely redistribut"),
]
rows = collections.defaultdict(lambda: collections.defaultdict(list))
for line in open(os.path.join(ROOT, "work", "membership.tsv")):
    pack, f, size = line.rstrip("\n").split("\t")
    p = os.path.join(B, f)
    raw = open(p, "rb").read()
    head = raw[:20000] if f.endswith(("tfm", "pfb", "otf", "ttf", "map", "enc")) else raw[:60000]
    # fonts: the name table / header carries the licence text
    txt = head.decode("latin-1") if not f.endswith(("otf", "ttf")) else raw.decode("latin-1")
    hit = [name for name, rx in MARKS if re.search(rx, txt, re.I if name in ("Public domain",) else 0)]
    key = " + ".join(hit) if hit else "no marker in file"
    rows[pack][key].append(f)
for pack in rows:
    print(f"## {pack}")
    for key, files in sorted(rows[pack].items(), key=lambda kv: -len(kv[1])):
        ex = ", ".join(sorted(files)[:6])
        print(f"- {key}: {len(files)} files (e.g. {ex})")
