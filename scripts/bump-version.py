#!/usr/bin/env python3
"""Give the whole suite one version (Tontolo's rule): VERSION, every app's
[workspace.package] version, the internal path dependencies that pin it, and Trano's catalog.

Usage: python3 -I scripts/bump-version.py 0.2.0
Then build once (cargo updates each Cargo.lock) and add a CHANGELOG section.
"""
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
APPS = ["trano", "sary", "taratasy", "soratra", "tabilao", "seho", "tsilo", "endrika", "pejy", "hazavana", "feo", "hetsika", "tselatra", "drafitra"]


def workspace_version(text):
    m = re.search(r'^\[workspace\.package\]\s*$(.*?)^\[', text, re.S | re.M)
    if not m:
        return None
    v = re.search(r'^version\s*=\s*"([^"]+)"', m.group(1), re.M)
    return v.group(1) if v else None


def bump_app(app, new):
    base = os.path.join(ROOT, "apps", app)
    top = os.path.join(base, "Cargo.toml")
    text = open(top, encoding="utf-8").read()
    old = workspace_version(text)
    if old is None:
        raise SystemExit(f"{app}: no [workspace.package] version")
    changed = 0
    for dirpath, dirnames, files in os.walk(base):
        dirnames[:] = [d for d in dirnames if d not in ("target", "vendor", ".git", "dist")]
        if "Cargo.toml" not in files:
            continue
        p = os.path.join(dirpath, "Cargo.toml")
        s = open(p, encoding="utf-8").read()
        n = s
        if p == top:
            n = re.sub(r'(^\[workspace\.package\]\s*$.*?^version\s*=\s*")[^"]+(")', rf"\g<1>{new}\g<2>", n, count=1, flags=re.S | re.M)
        # Internal crates pinned to the workspace version: `{ path = "...", version = "<old>" }`.
        n = re.sub(rf'(\bpath\s*=\s*"[^"]*"[^}}\n]*?\bversion\s*=\s*")={re.escape(old)}(")', rf"\g<1>={new}\g<2>", n)
        n = re.sub(rf'(\bpath\s*=\s*"[^"]*"[^}}\n]*?\bversion\s*=\s*"){re.escape(old)}(")', rf"\g<1>{new}\g<2>", n)
        n = re.sub(rf'(\bversion\s*=\s*"){re.escape(old)}("[^}}\n]*?\bpath\s*=)', rf"\g<1>{new}\g<2>", n)
        # Crates that spell their own version out instead of inheriting it.
        n = re.sub(rf'(^\[package\]\s*$.*?^version\s*=\s*"){re.escape(old)}(")', rf"\g<1>{new}\g<2>", n, count=1, flags=re.S | re.M)
        if n != s:
            open(p, "w", encoding="utf-8").write(n)
            changed += 1
    print(f"{app}: {old} -> {new} ({changed} manifests)")


def main():
    if len(sys.argv) != 2 or not re.fullmatch(r"\d+\.\d+\.\d+", sys.argv[1]):
        raise SystemExit(__doc__)
    new = sys.argv[1]
    with open(os.path.join(ROOT, "VERSION"), "w") as f:
        f.write(new + "\n")
    for app in APPS:
        bump_app(app, new)


if __name__ == "__main__":
    main()
