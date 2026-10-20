#!/usr/bin/env bash
# After a release is published: replay the updater of earlier releases against it.
#
#   scripts/check-updater-replay.sh [tag...]       (default: the three releases before the latest)
#
# For each old tag, builds that tag's `mahay-update` crate in a scratch folder and runs it as an
# installed app of that version would: find the latest release (API, then the redirect fallback),
# verify the signed checksums, download an APK. This is what a user jumping from an old version to
# the new one runs; it catches changes to file names, the signature or the release layout that an
# old updater cannot follow. Needs the network and cargo; touches nothing in the repository.
set -euo pipefail
cd "$(dirname "$0")/.."
tags=("$@")
if [ ${#tags[@]} -eq 0 ]; then
  mapfile -t tags < <(gh release list --repo raantss18/mahay --limit 4 --json tagName -q '.[].tagName' | tail -n +2)
fi
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
fail=0
for tag in "${tags[@]}"; do
  version=${tag#v}
  echo "== an app of $tag updating to the latest release"
  rm -rf "$work/src"; mkdir -p "$work/src"
  git archive "$tag" libs/core/crates/update | tar -x -C "$work/src"
  echo "$version" > "$work/src/VERSION"
  crate="$work/src/libs/core/crates/update"
  python3 -I - "$crate/Cargo.toml" "$version" <<'PY'
import re, sys
p, version = sys.argv[1:3]
s = open(p).read()
s = re.sub(r'^rust-version.*\n', '', s, flags=re.M)
s = s.replace("version.workspace = true", f'version = "{version}"').replace("edition.workspace = true", 'edition = "2024"')
s = s.replace("license.workspace = true", 'license = "MIT"').replace("repository.workspace = true", "")
s = s.replace("[lints]\nworkspace = true\n", "")
open(p, "w").write(s)
PY
  mkdir -p "$crate/examples"
  cp scripts/updater-replay/replay.rs "$crate/examples/replay.rs"
  if (cd "$crate" && CARGO_INCREMENTAL=0 CARGO_TARGET_DIR="$work/target" cargo run -q --example replay -- "$version"); then
    echo "   $tag: OK"
  else
    echo "   $tag: FAILED" >&2; fail=1
  fi
done
[ $fail = 0 ] && echo "all earlier updaters can reach the latest release" || { echo "an earlier updater cannot update to the latest release" >&2; exit 1; }
