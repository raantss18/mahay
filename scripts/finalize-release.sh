#!/usr/bin/env bash
# Finish a release folder (dist/release): checksums, signature, checks and the release page.
# Used by stage-release.sh (local) and by the Release workflow (CI), so both publish the same way.
#
#   scripts/finalize-release.sh VERSION [--require-all]
#
# Expects dist/release/ to hold the files named <app>-<version>-... (APKs, AppImages and their
# .zsync, .debs). Then:
#   - writes SHA256SUMS.txt and signs it (Ed25519, SHA256SUMS.txt.sig): the apps refuse an update
#     without a valid signature, so a release without one would never install;
#   - checks the signature with .github/update-signing.pub;
#   - checks every APK is signed with Mahay's release key (.github/apk-signing-cert.sha256): Android
#     refuses an update signed with another key, so such a release would break every install;
#   - with --require-all, checks the 14 apps are all there;
#   - writes dist/release-notes.md.
# The signing key is read from $UPDATE_SIGNING_KEY_FILE (default release-keys/update-signing.pem).
set -euo pipefail
cd "$(dirname "$0")/.."
version=${1:?usage: finalize-release.sh VERSION [--require-all]}
require_all=0
[ "${2:-}" = "--require-all" ] && require_all=1
apps=(hub sary taratasy soratra tabilao seho tsilo endrika pejy hazavana feo hetsika tselatra drafitra)
dir=dist/release
[ -d "$dir" ] || { echo "no $dir: stage the release first" >&2; exit 1; }

if [ $require_all = 1 ]; then
  missing=0
  for app in "${apps[@]}"; do
    for f in "$app-$version-android-arm64-v8a.apk" "$app-$version-android-x86_64.apk" \
             "$app-$version-linux-x86_64.AppImage" "$app-$version-linux-x86_64.deb"; do
      [ -e "$dir/$f" ] || { echo "missing: $f" >&2; missing=1; }
    done
  done
  [ $missing = 0 ] || { echo "the release is incomplete" >&2; exit 1; }
fi

(
  cd "$dir"
  shopt -s nullglob
  files=(*.apk *.aab *.AppImage *.deb)
  [ ${#files[@]} -gt 0 ] || { echo "no release files in $dir" >&2; exit 1; }
  sha256sum -- "${files[@]}" > SHA256SUMS.txt
)

key=${UPDATE_SIGNING_KEY_FILE:-release-keys/update-signing.pem}
[ -f "$key" ] || { echo "no update-signing key at $key: the release can't be signed" >&2; exit 1; }
openssl pkeyutl -sign -inkey "$key" -rawin -in "$dir/SHA256SUMS.txt" -out "$dir/SHA256SUMS.txt.sig"
openssl pkeyutl -verify -pubin -inkey .github/update-signing.pub -rawin -in "$dir/SHA256SUMS.txt" -sigfile "$dir/SHA256SUMS.txt.sig"

expected=$(tr -d ' \n' < .github/apk-signing-cert.sha256)
apksigner=""
for sdk in "${ANDROID_HOME:-}" "${ANDROID_SDK_ROOT:-}" "$HOME/Android/Sdk"; do
  [ -n "$sdk" ] && [ -d "$sdk/build-tools" ] && apksigner=$(ls -d "$sdk"/build-tools/*/apksigner 2>/dev/null | sort -V | tail -1) && [ -n "$apksigner" ] && break
done
if [ -n "$apksigner" ]; then
  bad=0
  for apk in "$dir"/*.apk; do
    [ -e "$apk" ] || continue
    got=$("$apksigner" verify --print-certs "$apk" | sed -n 's/.*certificate SHA-256 digest: //p' | head -1)
    [ "$got" = "$expected" ] || { echo "wrong signing key: $(basename "$apk") ($got)" >&2; bad=1; }
  done
  [ $bad = 0 ] || { echo "an APK is not signed with Mahay's release key: Android would refuse the update" >&2; exit 1; }
  echo "all APKs are signed with the release key"
elif [ "${REQUIRE_APKSIGNER:-0}" = 1 ]; then
  echo "apksigner not found: can't check the APK signing key" >&2; exit 1
else
  echo "warning: apksigner not found, the APK signing key was not checked" >&2
fi

python3 -I scripts/release-notes.py "$version" > dist/release-notes.md
ls -la "$dir"
