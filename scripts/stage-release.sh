#!/usr/bin/env bash
# Gather a release's files into dist/release/: the APKs under their release names, each app's
# Linux AppImage (+ .zsync) and .deb at the app's current version, and SHA256SUMS.txt.
#
#   scripts/stage-release.sh 0.1.0
set -euo pipefail
cd "$(dirname "$0")/.."
version=${1:?usage: stage-release.sh VERSION}
mkdir -p dist/release
find dist/release -maxdepth 1 -type f -delete
scripts/release-names.sh "$version" >/dev/null
for app in trano sary taratasy soratra tabilao seho; do
  v=$(awk '/^\[workspace.package\]/{p=1;next} /^\[/{p=0} p && $1=="version"{gsub(/[" ]/,"",$3); print $3; exit}' "apps/$app/Cargo.toml")
  for f in "apps/$app/dist/release/$app-$v-linux-x86_64".{AppImage,AppImage.zsync,deb}; do
    [ -e "$f" ] && cp "$f" dist/release/ || echo "missing: $f" >&2
  done
done
(cd dist/release && sha256sum -- *.apk *.AppImage *.deb > SHA256SUMS.txt)
ls -la dist/release
