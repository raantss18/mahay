#!/usr/bin/env python3
"""Check on a device that each Mahay app is offered in "Open with" for its own files (D22).

Reads `android/open-with-matrix.json` (written by scripts/android-associations.py) and asks the
system's package manager, with `cmd package query-activities`, which apps answer a VIEW of:

  - a document of each declared MIME type, with no extension in its address (a MediaStore address);
  - a file of each extension, reported as `application/octet-stream`, with no type, with a
    several-dot name (`a.b.c.ext`) and in capitals;
  - files that are not Mahay's (`.zip`, `.apk`, `.exe`): no Mahay app may claim them.

Exits 1 when an expected app is missing or an unexpected one answers. Needs the apps installed
from a build that has the current manifests (apps that are not installed are skipped).

    python3 -I scripts/android-open-with-test.py [--serial emulator-5554] [--apps sary,soratra]
"""
import argparse
import json
import os
import shutil
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
BASE = "content://com.android.externalstorage.documents/document/primary%3ADownload%2F"
MEDIA = "content://media/external/file/42"
CONCRETE = {"image/*": "image/png", "audio/*": "audio/mpeg", "video/*": "video/mp4"}
OCTET = "application/octet-stream"


def find_adb():
    for candidate in (shutil.which("adb"), os.path.expanduser("~/Android/Sdk/platform-tools/adb"),
                      os.path.join(os.environ.get("ANDROID_SDK_ROOT", ""), "platform-tools", "adb")):
        if candidate and os.path.exists(candidate):
            return candidate
    sys.exit("adb not found (ANDROID_HOME may point to a stale path: use ~/Android/Sdk)")


def build_cases(matrix, apps):
    """(label, action, uri, type, expected package or None, must-not-match-any-Mahay-app)."""
    cases = []
    for app in apps:
        entry = matrix[app]
        pkg = entry["package"]
        for t in entry["types"]:
            cases.append((f"{app}: type {t}", "VIEW", MEDIA, CONCRETE.get(t, t), pkg, False))
        for ext in entry["extensions"]:
            cases.append((f"{app}: .{ext} as octet-stream", "VIEW", f"{BASE}test.{ext}", OCTET, pkg, False))
            cases.append((f"{app}: .{ext} edited", "EDIT", f"{BASE}test.{ext}", OCTET, pkg, False))
            cases.append((f"{app}: .{ext} with no type", "VIEW", f"{BASE}test.{ext}", None, pkg, False))
            cases.append((f"{app}: .{ext} several dots", "VIEW", f"{BASE}a.b.c.{ext}", OCTET, pkg, False))
            cases.append((f"{app}: .{ext.upper()} in capitals", "VIEW", f"{BASE}TEST.{ext.upper()}", OCTET, pkg, False))
    for ext in ("zip", "apk", "exe", "unknownext"):
        cases.append((f"nobody: .{ext}", "VIEW", f"{BASE}test.{ext}", OCTET, None, True))
    return cases


def run(adb, serial, cases):
    lines = []
    for i, (_, action, uri, mime, _, _) in enumerate(cases):
        t = f" -t '{mime}'" if mime else ""
        lines.append(f"echo '@@CASE {i}'")
        lines.append(f"cmd package query-activities --brief -a android.intent.action.{action} "
                     f"-c android.intent.category.DEFAULT -d '{uri}'{t}")
    out = subprocess.run([adb, "-s", serial, "shell"], input="\n".join(lines) + "\n", capture_output=True, text=True, timeout=1800).stdout
    results = {}
    current = None
    for line in out.splitlines():
        if line.startswith("@@CASE "):
            current = int(line.split()[1])
            results[current] = set()
        elif current is not None and "org.mahay." in line:
            for token in line.replace("/", " ").split():
                if token.startswith("org.mahay.") and token.count(".") == 2:
                    results[current].add(token)
    return results


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--serial", default=os.environ.get("ANDROID_SERIAL", "emulator-5554"))
    ap.add_argument("--apps", help="comma-separated app ids (default: every installed app)")
    a = ap.parse_args()
    adb = find_adb()
    matrix = json.load(open(os.path.join(ROOT, "android", "open-with-matrix.json"), encoding="utf-8"))
    installed = subprocess.run([adb, "-s", a.serial, "shell", "pm", "list", "packages", "org.mahay."], capture_output=True, text=True).stdout
    chosen = [x.strip() for x in a.apps.split(",")] if a.apps else list(matrix)
    apps = []
    for app in chosen:
        if app not in matrix:
            sys.exit(f"unknown app {app}")
        if f"package:{matrix[app]['package']}" in installed.split():
            apps.append(app)
        else:
            print(f"skipped (not installed): {app}")
    if not apps:
        sys.exit("no Mahay app installed on the device")
    cases = build_cases(matrix, apps)
    results = run(adb, a.serial, cases)
    failures = []
    for i, (label, _, uri, mime, pkg, nobody) in enumerate(cases):
        got = results.get(i, set())
        if nobody and got:
            failures.append(f"{label}: claimed by {', '.join(sorted(got))}")
        elif pkg and pkg not in got:
            failures.append(f"{label}: {pkg} is not offered (offered: {', '.join(sorted(got)) or 'nobody'})")
    print(f"{len(cases)} cases, {len(apps)} apps ({', '.join(apps)}), {len(failures)} failures")
    for f in failures[:40]:
        print("  FAIL", f)
    if len(failures) > 40:
        print(f"  ... and {len(failures) - 40} more")
    sys.exit(1 if failures else 0)


if __name__ == "__main__":
    main()
