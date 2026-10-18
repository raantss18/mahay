#!/usr/bin/env python3
"""One-off rename: the hub "Trano" becomes "Mahay HUB" (id `hub`, `org.mahay.hub`).

In Rust, identifiers become `hub`/`Hub`/`HUB`, while text inside string literals and comments
becomes "Mahay HUB" (shown to people) or `hub` (ids, file names). Other text files: "Mahay HUB"
and `hub`. File and directory names: `trano` -> `hub`.
"""
import os
import re

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SKIP = {".git", "target", "release-keys", "dist", ".gradle", "build", "node_modules"}

STRING = re.compile(r'"(?:\\.|[^"\\])*"')


def prose(t):
    t = t.replace("Mahay Trano", "Mahay HUB").replace("TRANO", "HUB")
    t = re.sub(r"\bTrano\b", "Mahay HUB", t)
    t = re.sub(r"(?<![A-Za-z])trano(?![a-z])", "hub", t)
    # Inside identifiers spelled out in text (TranoApp, trano_core).
    t = re.sub(r"(?<![A-Za-z])Trano(?=[A-Z_])", "Hub", t)
    t = re.sub(r"(?<![A-Za-z])trano(?=[_-])", "hub", t)
    return t.replace("Mahay Mahay HUB", "Mahay HUB")


def code(t):
    t = re.sub(r"(?<![A-Za-z])TRANO", "HUB", t)
    t = re.sub(r"(?<![A-Za-z])Trano", "Hub", t)
    return re.sub(r"(?<![A-Za-z])trano", "hub", t)


def rust_line(line):
    # Split off a line comment (not inside a string).
    out, i = [], 0
    for m in STRING.finditer(line):
        before = line[i:m.start()]
        if "//" in before:
            k = before.index("//")
            out.append(code(before[:k]) + prose(before[k:] + line[m.start():]))
            return "".join(out)
        out.append(code(before))
        out.append(prose(m.group(0)))
        i = m.end()
    rest = line[i:]
    if "//" in rest:
        k = rest.index("//")
        out.append(code(rest[:k]) + prose(rest[k:]))
    else:
        out.append(code(rest))
    return "".join(out)


def main():
    changed = 0
    for root, dirs, files in os.walk(ROOT, topdown=True):
        dirs[:] = [d for d in dirs if d not in SKIP]
        for f in files:
            p = os.path.join(root, f)
            if os.path.islink(p) or f == "rename-hub.py":
                continue
            try:
                b = open(p, "rb").read()
                if b"\x00" in b[:8192]:
                    continue
                s = b.decode("utf-8")
            except (OSError, UnicodeDecodeError):
                continue
            if "rano" not in s and "RANO" not in s:
                continue
            if f.endswith(".rs"):
                n = "\n".join(rust_line(line) for line in s.split("\n"))
            elif f in ("Cargo.toml", "Cargo.lock") or f.endswith((".gradle", ".java")):
                n = code(s).replace('"Mahay Hub"', '"Mahay HUB"')
                n = re.sub(r'label: "Mahay"', 'label: "Mahay HUB"', n)
            else:
                n = prose(s)
            if n != s:
                open(p, "w", encoding="utf-8").write(n)
                changed += 1
    for root, dirs, files in os.walk(ROOT, topdown=False):
        if any(x in root.split(os.sep) for x in SKIP):
            continue
        for name in files + dirs:
            if "trano" in name or "Trano" in name:
                new = name.replace("trano", "hub").replace("Trano", "Hub")
                os.rename(os.path.join(root, name), os.path.join(root, new))
    print("files changed", changed)


if __name__ == "__main__":
    main()
