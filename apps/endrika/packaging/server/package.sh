#!/usr/bin/env bash
# Build and package the collaboration server for this machine (see docs/server.md):
#
#   $DIST/endrika-server-<version>-<os>-<arch>.tar.gz
#
# One directory with the endrika-server binary, the licences, docs/server.md and the deployment
# files (install.sh, systemd units, Caddyfile, nginx). A headless binary: no desktop files, no fonts.
#
# Usage: packaging/server/package.sh [--skip-build]
set -euo pipefail
# shellcheck source=../env.sh
. "$(dirname "${BASH_SOURCE[0]}")/../env.sh"
HERE="$ROOT/packaging/server"

SKIP_BUILD=0
case "${1:-}" in
  --skip-build) SKIP_BUILD=1 ;;
  "") ;;
  -h | --help) sed -n '2,9p' "$0"; exit 0 ;;
  *) echo "unknown argument: $1" >&2; exit 2 ;;
esac

OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"
case "$ARCH" in
  arm64) ARCH=aarch64 ;;
  amd64) ARCH=x86_64 ;;
esac
NAME="endrika-server-$VERSION-$OS-$ARCH"
echo "==> endrika-server $VERSION for $OS $ARCH"

if [ "$SKIP_BUILD" = 0 ]; then
  (cd "$ROOT" && cargo build --release --locked -p endrika-server)
fi
WORK="$CARGO_TARGET_DIR/server-package"
STAGE="$WORK/$NAME"
rm -rf "$WORK"
mkdir -p "$STAGE"
cp "$CARGO_TARGET_DIR/release/endrika-server" "$STAGE/"
strip "$STAGE/endrika-server" 2>/dev/null || true
for f in LICENSE-MIT LICENSE-APACHE NOTICE docs/server.md; do cp "$ROOT/$f" "$STAGE/"; done
cp "$HERE/endrika-server.service" "$HERE/endrika-backup.service" "$HERE/endrika-backup.timer" "$HERE/Caddyfile" "$HERE/nginx.conf" "$STAGE/"
install -m 755 "$HERE/install.sh" "$STAGE/"

# Smoke test: the binary runs and reports the version being shipped.
"$STAGE/endrika-server" --version | grep -F "$VERSION"

tar -C "$WORK" -czf "$DIST/$NAME.tar.gz" "$NAME"
echo "==> $DIST/$NAME.tar.gz"
