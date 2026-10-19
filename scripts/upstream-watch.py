#!/usr/bin/env python3
"""Watch the upstream repositories Mahay's apps were imported from (github.com/storytold).

Reads the imported revisions from docs/APPS_INVENTORY.md, compares each with the upstream default
branch through GitHub's compare API, sorts the new commits (security, crash, dependency, feature,
fix, other) and flags what needs a human decision:

  - security or crash fixes and dependency bumps with a security flavour,
  - a changed licence file,
  - a new network endpoint in the added lines of source files (ArtCraft's own backend, telemetry,
    a new community link, a new service).

Nothing is merged or changed: the report says what moved and where; adopting a change is a
reviewed step (docs/UPSTREAM.md).

    python3 -I scripts/upstream-watch.py [--out report.md] [--json report.json]
                                         [--only sary,taratasy] [--max-commits 12]
    python3 -I scripts/upstream-watch.py --selftest

Needs the `gh` CLI (authenticated, or GH_TOKEN set) and reads public data only.
"""
import argparse
import datetime
import json
import os
import re
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
ORG = "storytold"

# "Security" alone is also a PDF feature ("Remove security") and a macOS term: only phrases that
# describe a vulnerability, and only in the commit's first line.
SECURITY = re.compile(
    r"\b(security (fix|issue|hole|advisory|hardening)|security:|vulnerab\w*|cve-\d+|rustsec|advisory|exploit\w*|"
    r"path traversal|denial.of.service|sandbox escape|zip.?slip|injection|untrusted input)\b", re.I)
CRASH = re.compile(
    r"\b(panic\w*|crash\w*|abort\w*|overflow\w*|out.of.bounds|oom|hang\w*|freez\w*|deadlock\w*|"
    r"data loss|corrupt\w*|never.crash)\b", re.I)
DEPS = re.compile(r"\b(bump|upgrade|update)\b.*\b(deps?|dependenc\w+|crates?|lock|rustls|webpki|zip|image|wgpu|winit|egui)\b", re.I)
FEATURE = re.compile(r"^(feat|add|implement|support|introduce)\b", re.I)
FIX = re.compile(r"^(fix|bug|hotfix|repair)\b|\bfix(es|ed)?\b", re.I)

# Domains that are documentation, standards or tooling: not a service the app talks to.
BENIGN_DOMAINS = (
    "github.com", "githubusercontent.com", "w3.org", "crates.io", "docs.rs", "rust-lang.org",
    "mozilla.org", "example.com", "example.org", "localhost", "wikipedia.org", "creativecommons.org",
    "apache.org", "opensource.org", "iso.org", "unicode.org", "adobe.com", "microsoft.com",
    "openxmlformats.org", "purl.org", "xmlsoft.org", "freedesktop.org", "gnu.org", "ietf.org",
    "khronos.org", "rfc-editor.org", "ecma-international.org", "schemas.openxmlformats.org",
    "docs.oasis-open.org", "oasis-open.org", "sourceforge.net", "rustsec.org", "readthedocs.io",
    # badges, CDNs for README images, code-review and quality bots: not something an app calls
    "shields.io", "jsdelivr.net", "lucide.dev", "simpleicons.org", "sonarcloud.io", "coderabbit.ai",
)
# Only source files can make an app call a service; tests, examples, docs and READMEs name hosts as data.
CODE_SUFFIXES = (".rs", ".toml", ".json", ".yml", ".yaml", ".ts", ".tsx", ".js", ".sh", ".kt", ".java", ".py")
NOT_CODE = ("/tests/", "/examples/", "/fixtures/", "/docs/", "/benches/", "/testdata/", "tests.rs", "_test.rs", "/.github/")


def is_service_code(name):
    return name.endswith(CODE_SUFFIXES) and not any(x in "/" + name for x in NOT_CODE)

URL = re.compile(r"https?://([A-Za-z0-9-]+(?:\.[A-Za-z0-9-]+)+)")

ORDER = ["security", "crash", "deps", "feature", "fix", "other"]


def classify(message, files=()):
    """The category of a commit: its message first line and body, and the files it touched."""
    first = message.splitlines()[0] if message else ""
    if SECURITY.search(first):
        return "security"
    if CRASH.search(first):
        return "crash"
    if DEPS.search(first) or (files and all(f.endswith(("Cargo.lock", "Cargo.toml")) for f in files)):
        return "deps"
    if FEATURE.search(first):
        return "feature"
    if FIX.search(first):
        return "fix"
    return "other"


def endpoints(patch):
    """Domains named in the lines a patch adds, minus documentation and standards hosts."""
    found = set()
    for line in (patch or "").splitlines():
        if line.startswith("+") and not line.startswith("+++"):
            for host in URL.findall(line):
                host = host.lower().rstrip(".")
                if not any(host == d or host.endswith("." + d) for d in BENIGN_DOMAINS):
                    found.add(host)
    return found


def watched(inventory):
    """(upstream repo, Mahay name, path, imported revision) for every imported repository."""
    out = []
    for line in inventory.splitlines():
        cells = [c.strip() for c in line.split("|")]
        if len(cells) < 8 or not cells[1:]:
            continue
        m = re.fullmatch(r"`([0-9a-f]{40})`", cells[6]) if len(cells) > 6 else None
        if m:
            out.append((cells[2].strip("`"), cells[3], cells[4].strip("`"), m.group(1)))
    # Repositories listed as "not imported" with a revision in their note, e.g. artcraft.
    for line in inventory.splitlines():
        cells = [c.strip() for c in line.split("|")]
        if len(cells) > 4 and cells[1] in ("artcraft", "craft-ai-integrations"):
            m = re.search(r"`([0-9a-f]{12,40})`", cells[3] + cells[4])
            if m:
                out.append((cells[1], "(not imported)", "-", m.group(1)))
    return out


def gh(path):
    r = subprocess.run(["gh", "api", "-H", "Accept: application/vnd.github+json", path], capture_output=True, text=True)
    if r.returncode != 0:
        raise RuntimeError((r.stderr or r.stdout).strip().splitlines()[-1] if (r.stderr or r.stdout).strip() else "gh failed")
    return json.loads(r.stdout)


def check(repo, name, path, base, max_commits):
    entry = {"repo": repo, "name": name, "path": path, "base": base, "ahead": 0, "important": [], "by_category": {},
             "endpoints": [], "licence_changed": False, "areas": {}, "error": None, "head": None, "commits": []}
    try:
        info = gh(f"repos/{ORG}/{repo}")
        branch = info["default_branch"]
        cmp = gh(f"repos/{ORG}/{repo}/compare/{base}...{branch}?per_page=100")
    except Exception as e:  # noqa: BLE001: report any failure, keep checking the others
        entry["error"] = str(e)
        return entry
    entry["head"] = cmp["commits"][-1]["sha"] if cmp.get("commits") else base
    entry["ahead"] = cmp.get("ahead_by", 0)
    files = cmp.get("files", [])
    names = [f["filename"] for f in files]
    entry["licence_changed"] = any(re.search(r"(^|/)(LICENSE|COPYING|NOTICE)", n, re.I) for n in names)
    domains = set()
    for f in files:
        if is_service_code(f["filename"]):
            domains |= endpoints(f.get("patch"))
        area = "/".join(f["filename"].split("/")[:2])
        entry["areas"][area] = entry["areas"].get(area, 0) + 1
    entry["endpoints"] = sorted(domains)
    for c in cmp.get("commits", []):
        msg = c["commit"]["message"]
        cat = classify(msg)
        entry["by_category"][cat] = entry["by_category"].get(cat, 0) + 1
        item = {"sha": c["sha"][:7], "title": msg.splitlines()[0][:110], "category": cat, "url": c["html_url"]}
        entry["commits"].append(item)
        if cat in ("security", "crash"):
            entry["important"].append(item)
    entry["important"] = entry["important"][:max_commits]
    return entry


def needs_review(e):
    return bool(e["important"] or e["licence_changed"] or e["endpoints"])


def report(entries, max_commits):
    today = datetime.date.today().isoformat()
    review = [e for e in entries if needs_review(e)]
    lines = [f"# Upstream changes to review ({today})", ""]
    lines.append(f"Compared with the revisions imported into Mahay (`docs/APPS_INVENTORY.md`). "
                 f"{len(review)} of {len(entries)} repositories need a look; the rest only moved.")
    lines += ["", "| Upstream | Mahay | New commits | Security | Crash | Dependencies | Features | Fixes | Flags |",
              "|---|---|---:|---:|---:|---:|---:|---:|---|"]
    for e in entries:
        if e["error"]:
            lines.append(f"| {e['repo']} | {e['name']} | – | | | | | | error: {e['error']} |")
            continue
        c = e["by_category"]
        flags = []
        if e["licence_changed"]:
            flags.append("licence")
        if e["endpoints"]:
            flags.append("network")
        lines.append(f"| {e['repo']} | {e['name']} | {e['ahead']} | {c.get('security', 0)} | {c.get('crash', 0)} | "
                     f"{c.get('deps', 0)} | {c.get('feature', 0)} | {c.get('fix', 0)} | {', '.join(flags)} |")
    for e in review:
        lines += ["", f"## {e['repo']} → {e['name']} (`{e['path']}`)", "",
                  f"{e['ahead']} new commits since `{e['base'][:12]}`; compare: "
                  f"https://github.com/{ORG}/{e['repo']}/compare/{e['base'][:12]}...{(e['head'] or 'HEAD')[:12]}"]
        if e["licence_changed"]:
            lines += ["", "**A licence file changed.** Read it before adopting anything from this repository."]
        if e["endpoints"]:
            lines += ["", "**New network endpoints in added lines** (check they are not a backend, telemetry or a service Mahay removed): "
                      + ", ".join(f"`{d}`" for d in e["endpoints"][:15])]
        if e["important"]:
            lines += ["", "Security and crash fixes:"]
            lines += [f"- [{i['sha']}]({i['url']}) {i['title']} ({i['category']})" for i in e["important"][:max_commits]]
        top = sorted(e["areas"].items(), key=lambda kv: -kv[1])[:6]
        if top:
            lines += ["", "Most touched: " + ", ".join(f"`{a}` ({n})" for a, n in top)]
    lines += ["", "---", "Adopting a change: `docs/UPSTREAM.md`. Nothing here was merged."]
    return "\n".join(lines) + "\n"


def selftest():
    assert classify("Fix panic when a PDF has no pages") == "crash"
    assert classify("Remove security also from files with /Encrypt inline in the trailer (fixes #2661)") == "fix"
    assert classify("Save keeps metadata") == "other"
    assert classify("Reject path traversal in archive names") == "security"
    assert is_service_code("crates/net/src/lib.rs") and not is_service_code("crates/net/tests/a.rs")
    assert not is_service_code("README.md") and not is_service_code("crates/x/src/foo_tests.rs")
    assert classify("Update rustls to 0.23.46") == "deps"
    assert classify("security: reject path traversal in zip names") == "security"
    assert classify("feat: add gradient mesh") == "feature"
    assert classify("Fix label alignment") == "fix"
    assert classify("Refactor ribbon") == "other"
    assert classify("Bump versions", ["Cargo.lock", "apps/x/Cargo.toml"]) == "deps"
    patch = "+++ b/x\n+const A: &str = \"https://api.storyteller.ai/v1\";\n+// see https://github.com/x/y and https://www.w3.org/TR\n-removed https://old.example.net/\n"
    assert endpoints(patch) == {"api.storyteller.ai"}, endpoints(patch)
    rows = ("| –  | craft-libs | (core) | `libs/core` | MIT | `" + "a" * 40 + "` | 2026-10-17 | 0.1.0 |\n"
            "| 1  | photocraft | Sary | `apps/sary` | MIT | `" + "b" * 40 + "` | 2026-10-18 | 0.1.0 |\n"
            "| artcraft | MIT | Tauri app (`d4e4ac69bbd9`); planned | later |\n")
    got = watched(rows)
    assert got[0] == ("craft-libs", "(core)", "libs/core", "a" * 40), got
    assert got[1][0] == "photocraft" and got[1][3] == "b" * 40, got
    print("selftest ok")


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--out", help="write the Markdown report here (default: standard output)")
    ap.add_argument("--json", help="write the machine-readable result here")
    ap.add_argument("--only", help="comma-separated upstream repo names or Mahay names")
    ap.add_argument("--max-commits", type=int, default=12)
    ap.add_argument("--selftest", action="store_true")
    a = ap.parse_args()
    if a.selftest:
        return selftest()
    inventory = open(os.path.join(ROOT, "docs", "APPS_INVENTORY.md"), encoding="utf-8").read()
    only = {s.strip().lower() for s in a.only.split(",")} if a.only else None
    entries = []
    for repo, name, path, base in watched(inventory):
        if only and repo.lower() not in only and name.lower() not in only:
            continue
        entries.append(check(repo, name, path, base, a.max_commits))
    text = report(entries, a.max_commits)
    if a.out:
        with open(a.out, "w", encoding="utf-8") as f:
            f.write(text)
    else:
        sys.stdout.write(text)
    if a.json:
        with open(a.json, "w", encoding="utf-8") as f:
            json.dump({"date": datetime.date.today().isoformat(), "important": any(needs_review(e) for e in entries),
                       "repos": entries}, f, indent=2)


if __name__ == "__main__":
    main()
