#!/usr/bin/env bash
# Copy dist/android/app-<app>-<abi>-release.apk (and the AAB) to dist/release/ under the names
# Trano and the release page use: <app>-<version>-android-<abi>.apk, <app>-<version>-android.aab.
#
#   scripts/release-names.sh 0.1.0
set -euo pipefail
cd "$(dirname "$0")/.."
version=${1:?usage: release-names.sh VERSION}
mkdir -p dist/release
for apk in dist/android/app-*-release.apk; do
  [ -e "$apk" ] || continue
  base=$(basename "$apk" -release.apk)      # app-<app>-<abi>
  rest=${base#app-}                         # <app>-<abi>
  app=${rest%%-*}
  abi=${rest#*-}
  cp "$apk" "dist/release/$app-$version-android-$abi.apk"
done
for aab in dist/android/*.aab; do
  [ -e "$aab" ] || continue
  name=$(basename "$aab" .aab)              # app-<app>-release
  app=${name#app-}; app=${app%-release}
  cp "$aab" "dist/release/$app-$version-android.aab"
done
ls -la dist/release
