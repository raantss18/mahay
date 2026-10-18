#!/usr/bin/env bash
# Build and package every app for Linux x86_64 (AppImage + .deb) with upstream's packaging
# scripts, at the suite version. Tools: nfpm and appimagetool on PATH (or APPIMAGETOOL), or let
# each script fetch its pinned, checksummed appimagetool.
#
#   scripts/linux-build.sh [app...]          (default: all 14)
set -euo pipefail
cd "$(dirname "$0")/.."
apps=("$@")
[ ${#apps[@]} -gt 0 ] || apps=(hub sary taratasy soratra tabilao seho tsilo endrika pejy hazavana feo hetsika tselatra drafitra)
export APPIMAGELAUNCHER_DISABLE=1 GITHUB_REPOSITORY=raantss18/mahay
for app in "${apps[@]}"; do
  echo "==> $app: build"
  pkgs=(-p "$app")
  # Some packages ship the command-line tool too (and check its --version at the end).
  [ -d "apps/$app/apps/$app-cli" ] && pkgs+=(-p "$app-cli")
  (cd "apps/$app" && cargo build --release --locked "${pkgs[@]}" -j "${JOBS:-8}")
  echo "==> $app: package"
  (cd "apps/$app" && packaging/linux/package.sh --skip-build --formats "appimage deb" 2>&1 | grep -E '^wrote|created package|error|✘|E: ' || true)
done
