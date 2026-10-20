#!/bin/bash
# stage-python.sh <host-triple> <outdir>: minimal stripped CPython runtime tree from android.py's prefix
set -euo pipefail
H=$1; OUT=$2
P=$(dirname "$0")/build/cpython/cross-build/$H/prefix
STRIP=$HOME/Android/Sdk/ndk/28.2.13676358/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-strip
rm -rf "$OUT"; mkdir -p "$OUT/bin" "$OUT/lib"
cp "$P/bin/python3.13" "$OUT/bin/"
cp "$P"/lib/libpython3.13.so "$OUT/lib/"
cp "$P"/lib/libcrypto_python.so "$P"/lib/libssl_python.so "$P"/lib/libsqlite3_python.so "$OUT/lib/"
cp -r "$P/lib/python3.13" "$OUT/lib/"
L=$OUT/lib/python3.13
rm -rf "$L"/test "$L"/idlelib "$L"/tkinter "$L"/turtledemo "$L"/turtle.py "$L"/ensurepip "$L"/lib2to3 \
       "$L"/config-3.13-* "$L"/site-packages "$L"/pydoc_data "$L"/unittest/test "$L"/distutils
find "$OUT" -name __pycache__ -type d -prune -exec rm -rf {} +
find "$OUT" -name 'test' -path '*/python3.13/*' -type d -prune -exec rm -rf {} + 2>/dev/null || true
find "$OUT" \( -name '*.so' -o -name 'python3.13' \) -type f -exec "$STRIP" --strip-unneeded {} + 2>/dev/null || true
mkdir -p "$L/site-packages"
