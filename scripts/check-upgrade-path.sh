#!/usr/bin/env bash
# Before publishing: check that apps of earlier releases update to the staged release
# (dist/release), skipping versions, on a device or an emulator.
#
#   scripts/check-upgrade-path.sh --serial emulator-5558 [--from "v0.2.1 v0.2.2"] [--apps "hub tabilao soratra"]
#
# For each earlier release it installs that release's x86_64 APKs, then installs the staged ones
# over them with `adb install -r` (what Android does for an update: same signing key, higher
# versionCode, a manifest the installer accepts), in two orders: the hub last, then the hub first
# (the 0.2.3 apps use a permission only the hub declares). Catches what an old app cannot survive.
#
# It UNINSTALLS the listed apps on the device first, which also resets the hub's "install unknown
# apps" setting: use a scratch emulator (for example the Android 10 one), never your own device.
set -euo pipefail
cd "$(dirname "$0")/.."
serial=""; from=""; apps="hub tabilao soratra"
while [ $# -gt 0 ]; do
  case "$1" in
    --serial) serial=$2; shift ;;
    --from) from=$2; shift ;;
    --apps) apps=$2; shift ;;
    *) echo "unknown option $1" >&2; exit 2 ;;
  esac
  shift
done
[ -n "$serial" ] || { echo "give --serial of a scratch emulator (this uninstalls $apps there)" >&2; exit 2; }
adb=$(command -v adb || echo "$HOME/Android/Sdk/platform-tools/adb")
new=$(cat VERSION)
[ -d dist/release ] || { echo "no dist/release: stage the release first" >&2; exit 1; }
if [ -z "$from" ]; then
  from=$(gh release list --repo raantss18/mahay --limit 4 --json tagName -q '.[].tagName' | grep -v "^v$new$" | head -3 | tr '\n' ' ')
fi
work=$(mktemp -d); trap 'rm -rf "$work"' EXIT
fail=0
step() { # label, command...
  local label=$1; shift
  if out=$("$@" 2>&1) && echo "$out" | grep -q Success; then echo "   ok   $label"; else echo "   FAIL $label: $(echo "$out" | tail -1)"; fail=1; fi
}
for tag in $from; do
  old=${tag#v}
  for a in $apps; do
    gh release download "$tag" --repo raantss18/mahay --dir "$work" -p "$a-$old-android-x86_64.apk" --clobber >/dev/null
  done
  for order in "hub-last" "hub-first"; do
    echo "== $tag -> $new, $order, on $serial"
    for a in $apps; do "$adb" -s "$serial" uninstall "org.mahay.$a" >/dev/null 2>&1 || true; done
    for a in $apps; do step "install $a $old" "$adb" -s "$serial" install "$work/$a-$old-android-x86_64.apk"; done
    if [ "$order" = hub-last ]; then seq_apps="$(echo "$apps" | tr ' ' '\n' | grep -v '^hub$' | tr '\n' ' ') hub"; else seq_apps="hub $(echo "$apps" | tr ' ' '\n' | grep -v '^hub$' | tr '\n' ' ')"; fi
    for a in $seq_apps; do
      [ -n "$a" ] || continue
      step "update $a to $new" "$adb" -s "$serial" install -r "dist/release/$a-$new-android-x86_64.apk"
      got=$("$adb" -s "$serial" shell dumpsys package "org.mahay.$a" | sed -n 's/.*versionName=//p' | head -1 | tr -d '\r')
      [ "$got" = "$new" ] || { echo "   FAIL $a reports $got, not $new"; fail=1; }
    done
  done
done
[ $fail = 0 ] && echo "every upgrade path works" || { echo "an upgrade path is broken" >&2; exit 1; }
