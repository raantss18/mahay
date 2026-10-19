#!/usr/bin/env bash
# Publish a staged release (dist/release, from stage-release.sh or the CI) as GitHub's "latest".
#
#   scripts/publish-release.sh VERSION [--dry-run]
#
# Runs every check first, then (without --dry-run): pushes main, tags vVERSION, pushes the tag and
# creates the release with the files and the release page. Refuses to publish when:
#   - the working tree has uncommitted changes, or HEAD is not on main;
#   - a signing key, keystore or account secret is tracked by git;
#   - VERSION or the CHANGELOG do not say this version;
#   - the tag or the release already exists;
#   - the release folder is incomplete, its checksums or signature do not verify, or an APK is not
#     signed with Mahay's release key (scripts/finalize-release.sh --require-all).
# Publish as "latest", never as a pre-release: `releases/latest`, which the apps ask, skips them.
set -euo pipefail
cd "$(dirname "$0")/.."
version=${1:?usage: publish-release.sh VERSION [--dry-run]}
dry=0
[ "${2:-}" = "--dry-run" ] && dry=1
tag=v$version
repo=raantss18/mahay
fail() { echo "refused: $*" >&2; exit 1; }

[ -z "$(git status --porcelain)" ] || fail "the working tree has uncommitted changes"
[ "$(git branch --show-current)" = main ] || fail "HEAD is not on main"
tracked=$(git ls-files | grep -E '(^|/)(release-keys/|accounts\.env|signing\.env|keystore\.properties)|\.jks$|update-signing\.pem$' || true)
[ -z "$tracked" ] || fail "secret files are tracked by git: $tracked"
[ "$(cat VERSION)" = "$version" ] || fail "VERSION says $(cat VERSION), not $version"
grep -q "^## \[$version\]" CHANGELOG.md || fail "CHANGELOG.md has no section for $version"
git rev-parse -q --verify "refs/tags/$tag" >/dev/null && fail "tag $tag already exists locally"
git ls-remote --exit-code --tags origin "$tag" >/dev/null 2>&1 && fail "tag $tag already exists on GitHub"
gh release view "$tag" --repo "$repo" >/dev/null 2>&1 && fail "release $tag already exists"

# The folder, its checksums and signature, the APK signing key, the release page.
UPDATE_SIGNING_KEY_FILE=${UPDATE_SIGNING_KEY_FILE:-release-keys/update-signing.pem}
export UPDATE_SIGNING_KEY_FILE REQUIRE_APKSIGNER=1
scripts/finalize-release.sh "$version" --require-all >/dev/null
(cd dist/release && sha256sum -c --quiet SHA256SUMS.txt) || fail "checksums do not verify"
files=$(ls dist/release | wc -l)
echo "checks passed: $files files for $tag, signed, complete"
if [ $dry = 1 ]; then
  echo "dry run: would push main, tag $tag, push the tag and create the release as latest with $files files"
  exit 0
fi

git push origin main
git tag -a "$tag" -m "Mahay $version"
git push origin "$tag"
gh release create "$tag" --repo "$repo" --latest --title "Mahay $version" --notes-file dist/release-notes.md dist/release/*
gh release view "$tag" --repo "$repo" --json isDraft,isPrerelease,assets \
  -q '"published: draft=\(.isDraft) prerelease=\(.isPrerelease) files=\(.assets|length)"'
latest=$(curl -sI "https://github.com/$repo/releases/latest" | tr -d '\r' | sed -n 's/^[Ll]ocation: .*\/tag\///p')
[ "$latest" = "$tag" ] && echo "latest release is $tag" || echo "warning: latest points to '$latest'" >&2
