#!/usr/bin/env bash
# Regenerate the derived app icons from assets/app-icon/drafitra-1024.png (macOS: sips, iconutil).
# The outputs are committed, so builds and packaging never need these tools.
#
#   packaging/icons.sh
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIR="$ROOT/assets/app-icon"
SRC="$DIR/drafitra-1024.png"
ID="org.mahay.drafitra"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT
command -v sips >/dev/null || { echo "error: sips not found (macOS only)" >&2; exit 1; }

resize() { sips -s format png -z "$2" "$2" "$1" --out "$3" >/dev/null; }

for s in 16 24 32 48 64 128 256 512; do
  mkdir -p "$DIR/hicolor/${s}x${s}/apps"
  resize "$SRC" "$s" "$DIR/hicolor/${s}x${s}/apps/$ID.png"
done
resize "$SRC" 64 "$DIR/drafitra-64.png"
resize "$SRC" 256 "$DIR/drafitra-256.png"

ICO_PNGS=()
for s in 16 20 24 32 40 48 64 128 256; do
  resize "$SRC" "$s" "$TMP/ico-$s.png"
  ICO_PNGS+=("$TMP/ico-$s.png")
done
(cd "$ROOT" && cargo run -q -p xtask -- ico "$DIR/drafitra.ico" "${ICO_PNGS[@]}")

# macOS icons sit on Apple's grid: the macOS-margin master is drafitra-macos-512.png.
SET="$TMP/drafitra.iconset"
mkdir -p "$SET"
for s in 16 32 128 256; do
  resize "$DIR/drafitra-macos-512.png" "$s" "$SET/icon_${s}x${s}.png"
  resize "$DIR/drafitra-macos-512.png" $((s * 2)) "$SET/icon_${s}x${s}@2x.png"
done
resize "$DIR/drafitra-macos-512.png" 512 "$SET/icon_512x512.png"
iconutil -c icns -o "$DIR/drafitra.icns" "$SET"
echo "icons written to $DIR"
