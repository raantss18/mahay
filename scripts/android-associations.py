#!/usr/bin/env python3
"""Android file associations: each app shows up in "Open with" and "Share" for its own files.

Three kinds of intent filter per app, for VIEW and EDIT (D22):

1. **By MIME type**, for `content://` documents: the types come from each app's Linux desktop
   entry (`MimeType=` in `apps/<app>/packaging/linux/*.desktop`), plus the generic types Android's
   file providers actually report (a provider labels a CSV `text/comma-separated-values`, a photo
   `image/*`...).
2. **By file extension, with a generic type** (`*/*`, which also covers `application/octet-stream`):
   many file managers report no useful type for a file the system does not know (IDML, DXF, RAW
   photos, `.tex`...). The extensions come from the system's shared-mime-info (the globs of the
   types in 1) and from the app's own `*.mime.xml`.
3. **By file extension, with no type**, for providers that give none.

Extension filters need one path pattern per case and per number of dots in the file name (Android's
`pathPattern` has no real wildcard for a dotted name), so each extension is written as
`.*\\.ext`, `.*\\..*\\.ext`... with `[eE][xX][tT]` for both cases.

"Share" (SEND) uses the MIME types only. Only `content://` documents are accepted at run time
(MahayActivity.isContentUri): no `file://` filter is declared, so an app is never offered a file
it would refuse.

Writes `android/app/src/<app>/AndroidManifest.xml` (merged into the shared manifest by Gradle) and
`android/open-with-matrix.json`, which `scripts/android-open-with-test.py` checks on a device.

    python3 -I scripts/android-associations.py [--out DIR]     # DIR: write there, touch nothing else
"""
import argparse
import glob
import json
import os
import re
import xml.etree.ElementTree as ET

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
APPS = ["sary", "taratasy", "soratra", "tabilao", "seho", "tsilo", "endrika", "pejy", "hazavana", "feo", "hetsika", "tselatra", "drafitra"]
GLOBS2 = "/usr/share/mime/globs2"

# What Android providers send for these files, beyond the desktop's exact types.
EXTRA = {
    "sary": ["image/*"],
    "taratasy": ["application/pdf"],
    "soratra": ["application/msword", "application/vnd.oasis.opendocument.text", "application/rtf", "text/rtf", "text/plain", "text/markdown"],
    "tabilao": ["text/csv", "text/comma-separated-values", "text/tab-separated-values", "application/vnd.ms-excel",
                "application/vnd.oasis.opendocument.spreadsheet"],
    "seho": ["application/vnd.ms-powerpoint", "application/vnd.oasis.opendocument.presentation",
             "application/vnd.openxmlformats-officedocument.presentationml.slideshow"],
    "tsilo": ["image/svg+xml", "application/postscript", "application/illustrator", "image/x-eps"],
    "endrika": [],
    "pejy": ["application/vnd.adobe.indesign-idml-package"],
    "hazavana": ["image/*"],
    "feo": ["audio/*", "audio/midi", "audio/x-midi"],
    "hetsika": ["video/*"],
    "tselatra": [],
    "drafitra": ["image/vnd.dxf", "image/vnd.dwg", "application/dxf", "application/x-autocad"],
}

# Extensions to add by hand: formats the system's MIME database does not know under the type the
# app declares, or that file managers hand out with no type.
EXTRA_EXT = {
    "soratra": ["tex", "md", "markdown", "txt", "rtf", "odt", "doc", "docx", "docm", "dotx"],
    "tabilao": ["csv", "tsv", "xlsx", "xlsm", "xltx", "xls", "ods"],
    "seho": ["pptx", "ppsx", "potx", "ppt", "odp"],
    "taratasy": ["pdf"],
    "tsilo": ["svg", "ai", "eps"],
    "pejy": ["idml"],
    "drafitra": ["dxf", "dwg"],
    "hazavana": ["dng", "cr2", "cr3", "nef", "arw", "orf", "rw2", "raf", "pef", "srw", "heic", "avif"],
    "feo": ["wav", "flac", "mp3", "ogg", "opus", "m4a", "aac", "aiff", "mid", "midi"],
    "hetsika": ["mp4", "mov", "mkv", "avi", "webm", "m4v"],
    "sary": ["psd", "png", "jpg", "jpeg", "webp", "tif", "tiff", "bmp", "gif"],
}

# Extensions an app must never claim, whatever its MIME types say: containers and data formats
# that would put it in the list for files that are not its documents.
NEVER = {"xml", "json", "zip", "gz", "bz2", "xz", "7z", "tar", "bin", "dat", "db", "log", "apk", "exe", "html", "htm", "css", "js"}
# Extensions the system database attaches to the app's types that would only add noise: another
# kind of file (`.ts` is TypeScript as much as video), generic names (`.raw`, `.asc`), obscure
# camera and disc-image containers, and legacy Excel macro or presentation formats Mahay cannot open.
NEVER |= {"ts", "mp2", "moov", "mpl", "mpls", "cpi", "clpi", "bdm", "bdmv", "avf", "lrf", "qtvr", "fcproj", "asc", "ins", "raw",
          "xla", "xlc", "xld", "xll", "xlm", "xlw", "ppz", "dib", "pic", "vda", "vst", "tpic", "icb", "xyze", "rgbe"}

# Types Android never reports for a document (URL schemes) are left out.
SKIP_PREFIXES = ("x-scheme-handler/",)
SIMPLE_EXT = re.compile(r"^[A-Za-z0-9]{1,8}$")
DOT_LEVELS = 4  # `.*\.ext` up to `.*\..*\..*\..*\.ext`


def desktop_types(app):
    types = []
    for path in glob.glob(os.path.join(ROOT, "apps", app, "packaging", "linux", "*.desktop")):
        for line in open(path, encoding="utf-8"):
            if line.startswith("MimeType="):
                types += [t for t in line[len("MimeType="):].strip().split(";") if t]
    return types


def system_globs():
    """mime type -> set of simple extensions, from shared-mime-info (empty when it is missing)."""
    out = {}
    if not os.path.exists(GLOBS2):
        return out
    for line in open(GLOBS2, encoding="utf-8"):
        if line.startswith("#"):
            continue
        parts = line.rstrip("\n").split(":")
        if len(parts) >= 3 and parts[2].startswith("*."):
            ext = parts[2][2:]
            if SIMPLE_EXT.match(ext):
                out.setdefault(parts[1], set()).add(ext.lower())
    return out


def own_globs(app):
    """The extensions an app's own `*.mime.xml` defines."""
    exts = set()
    for path in glob.glob(os.path.join(ROOT, "apps", app, "packaging", "linux", "*.mime.xml")):
        for node in ET.parse(path).getroot().iter():
            if node.tag.endswith("glob"):
                pat = node.get("pattern", "")
                if pat.startswith("*.") and SIMPLE_EXT.match(pat[2:]):
                    exts.add(pat[2:].lower())
    return exts


def extensions(app, types, globs):
    exts = set(EXTRA_EXT.get(app, [])) | own_globs(app)
    for t in types:
        exts |= globs.get(t, set())
    return sorted(e for e in exts if e not in NEVER)


def case_insensitive(ext):
    return "".join(f"[{c.lower()}{c.upper()}]" if c.isalpha() else c for c in ext)


def path_patterns(exts):
    patterns = []
    for ext in exts:
        for dots in range(DOT_LEVELS):
            patterns.append(".*" + "".join("\\\\..*" for _ in range(dots)) + "\\\\." + case_insensitive(ext))
    return patterns


def manifest(types, exts):
    mime = "\n".join(f'                <data android:mimeType="{t}" />' for t in types)
    out = f'''<?xml version="1.0" encoding="utf-8"?>
<!-- Generated by scripts/android-associations.py from the app's desktop entry: edit that script,
     not this file. "Open with" (VIEW, EDIT) and "Share" (SEND) for the app's own documents. -->
<manifest xmlns:android="http://schemas.android.com/apk/res/android">
    <application>
        <activity android:name="org.mahay.shell.MahayActivity" android:exported="true">
            <intent-filter>
                <action android:name="android.intent.action.VIEW" />
                <action android:name="android.intent.action.EDIT" />
                <category android:name="android.intent.category.DEFAULT" />
                <data android:scheme="content" />
{mime}
            </intent-filter>
'''
    if exts:
        paths = "\n".join(f'                <data android:pathPattern="{p}" />' for p in path_patterns(exts))
        for label, typed in (("by extension, generic type", True), ("by extension, no type", False)):
            kind = '                <data android:mimeType="*/*" />\n' if typed else ""
            out += f'''            <!-- {label}: {", ".join(exts)} -->
            <intent-filter>
                <action android:name="android.intent.action.VIEW" />
                <action android:name="android.intent.action.EDIT" />
                <category android:name="android.intent.category.DEFAULT" />
                <data android:scheme="content" android:host="*" />
{kind}{paths}
            </intent-filter>
'''
    out += f'''            <intent-filter>
                <action android:name="android.intent.action.SEND" />
                <category android:name="android.intent.category.DEFAULT" />
{mime}
            </intent-filter>
        </activity>
    </application>
</manifest>
'''
    return out


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--out", help="write the manifests and the matrix under this folder instead of the tree")
    out_root = ap.parse_args().out
    base = out_root or ROOT
    globs = system_globs()
    if not globs:
        print(f"warning: {GLOBS2} not found: extensions come from EXTRA_EXT and the apps' own mime.xml only")
    matrix = {}
    for app in APPS:
        types = []
        for t in desktop_types(app) + EXTRA.get(app, []):
            if t.startswith(SKIP_PREFIXES) or t in types:
                continue
            types.append(t)
        exts = extensions(app, types, globs)
        path = os.path.join(base, "android", "app", "src", app, "AndroidManifest.xml")
        if not types:
            if os.path.exists(path):
                os.remove(path)
            print(f"{app}: no document types")
            continue
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "w", encoding="utf-8") as f:
            f.write(manifest(types, exts))
        matrix[app] = {"package": f"org.mahay.{app}", "types": types, "extensions": exts}
        print(f"{app}: {len(types)} types, {len(exts)} extensions")
    with open(os.path.join(base, "android", "open-with-matrix.json"), "w", encoding="utf-8") as f:
        json.dump(matrix, f, indent=1, sort_keys=True)
        f.write("\n")


if __name__ == "__main__":
    main()
