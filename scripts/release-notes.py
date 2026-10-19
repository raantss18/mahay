#!/usr/bin/env python3
"""Write the GitHub release page for a version, Tontolo-style: Mahay HUB first (the one file people
download by hand; it installs and updates every other app), what's new from CHANGELOG.md, then
the other files Mahay HUB fetches by itself, folded away.

Usage: python3 -I scripts/release-notes.py 0.2.0 > dist/release/NOTES.md
Reads the sizes from dist/release/.
"""
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
REPO = "raantss18/mahay"
APPS = ["sary", "taratasy", "soratra", "tabilao", "seho", "tsilo", "endrika", "pejy", "hazavana", "feo", "hetsika", "tselatra", "drafitra"]
NAMES = {"sary": "Sary", "taratasy": "Taratasy", "soratra": "Soratra", "tabilao": "Tabilao", "seho": "Seho", "tsilo": "Tsilo",
         "endrika": "Endrika", "pejy": "Pejy", "hazavana": "Hazavana", "feo": "Feo", "hetsika": "Hetsika", "tselatra": "Tselatra",
         "drafitra": "Drafitra"}


def size(name):
    p = os.path.join(ROOT, "dist", "release", name)
    if not os.path.exists(p):
        return ""
    n = os.path.getsize(p)
    return f"{n / 1_000_000:.0f} Mo" if n >= 1_000_000 else f"{n / 1000:.0f} Ko"


def link(v, name, bold=False):
    url = f"https://github.com/{REPO}/releases/download/v{v}/{name}"
    text = f"**{name}**" if bold else name
    return f"[{text}]({url})"


def changelog_section(v):
    text = open(os.path.join(ROOT, "CHANGELOG.md"), encoding="utf-8").read()
    m = re.search(rf"^## \[{re.escape(v)}\][^\n]*\n(.*?)(?=^## \[|\Z)", text, re.S | re.M)
    return m.group(1).strip() if m else "_Voir CHANGELOG.md._"


def main():
    v = sys.argv[1]
    out = []
    out.append(f"# Mahay {v}")
    out.append("")
    out.append("## ⬇️ Télécharger : installez seulement Mahay HUB")
    out.append("")
    out.append("**Mahay HUB** est l'accueil de Mahay : il installe, ouvre et met à jour toutes les autres apps, "
               "directement, sans repasser par cette page.")
    out.append("")
    out.append("| Fichier | Pour qui | Taille |")
    out.append("|---|---|---|")
    apk = f"hub-{v}-android-arm64-v8a.apk"
    out.append(f"| {link(v, apk, True)} | 📱 **Android : le bon choix dans 95 % des cas** (téléphones et tablettes, Android 8 et plus) | {size(apk)} |")
    apk86 = f"hub-{v}-android-x86_64.apk"
    out.append(f"| {link(v, apk86)} | 💻 Émulateurs, Chromebooks, Android sur PC | {size(apk86)} |")
    img = f"hub-{v}-linux-x86_64.AppImage"
    out.append(f"| {link(v, img, True)} | 🐧 **Linux** (Ubuntu, Debian, Fedora, Arch…) : `chmod +x` puis double-clic | {size(img)} |")
    out.append(f"| {link(v, 'SHA256SUMS.txt')} · {link(v, 'SHA256SUMS.txt.sig')} | 🔒 Empreintes signées (Ed25519) | |")
    out.append("")
    out.append("> Déjà installé ? **Mahay se met à jour tout seul**, dans Mahay HUB et dans chaque app "
               "(vérification au démarrage puis chaque jour, fichiers signés et vérifiés). "
               "Alerte « Play Protect » : normale hors Play Store, touchez « Analyser ».")
    out.append(">")
    out.append("> 🇬🇧 Download **Mahay HUB** only: it installs and updates every other app. "
               "🇲🇬 **Mahay HUB** ihany no alaina: izy no mametraka sy manavao ny rindranasa hafa rehetra.")
    out.append("")
    out.append("## 🖥️ Configuration requise · System requirements")
    out.append("")
    out.append("| | 📱 Android | 🐧 Linux |")
    out.append("|---|---|---|")
    out.append("| Système · System | Android 8.0 (API 26) ou plus récent · or newer, téléphone ou tablette | x86_64, glibc 2.39 ou plus · or newer (Ubuntu 24.04, Debian 13, Fedora 40, Arch) |")
    out.append("| Processeur · Processor | ARM 64 bits (`arm64-v8a`) ; `x86_64` pour émulateurs et Chromebooks | x86_64 |")
    out.append("| Graphismes · Graphics | GPU avec OpenGL ES (Vulkan si disponible) | pilote GPU Vulkan ou OpenGL |")
    out.append("| Stockage · Storage | ≈ 5 Mo Mahay HUB, jusqu'à ≈ 25 Mo par app | idem (AppImage) |")
    out.append("| Réseau · Network | inutile, sauf téléchargement et mises à jour · only for downloads and updates | idem |")
    out.append("| Autre · Other | autoriser « installer des apps inconnues » pour Mahay HUB | AppImage : FUSE 2 (`libfuse2`) ou `--appimage-extract-and-run` |")
    out.append("")
    out.append("Testé · Tested: Android 16 (API 36), émulateurs x86_64 (téléphone 1080×2400, tablette 2560×1600), Linux x86_64. "
               "Les APK arm64 ne sont pas encore essayés sur un vrai appareil · arm64 APKs not yet run on a real device. "
               "Détails · Details: [docs/STATUS.md](https://github.com/" + REPO + "/blob/main/docs/STATUS.md).")
    out.append("")
    out.append(f"## ✨ Nouveautés de la version {v}")
    out.append("")
    out.append(changelog_section(v))
    out.append("")
    out.append("<details>")
    out.append("<summary>📦 Fichiers des autres apps (Mahay HUB les télécharge pour vous)</summary>")
    out.append("")
    out.append("| App | Android arm64 | Android x86_64 | Linux |")
    out.append("|---|---|---|---|")
    for a in APPS:
        cells = []
        for name in (f"{a}-{v}-android-arm64-v8a.apk", f"{a}-{v}-android-x86_64.apk", f"{a}-{v}-linux-x86_64.AppImage"):
            cells.append(f"{link(v, name)} ({size(name)})" if size(name) else "—")
        out.append(f"| {NAMES[a]} | " + " | ".join(cells) + " |")
    out.append("")
    out.append("</details>")
    print("\n".join(out))


if __name__ == "__main__":
    main()
