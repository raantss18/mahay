#!/bin/bash
# stage-site.sh <x86_64|arm64> <outdir>: site-packages with the Android wheels (Chaquopy index), pruned
set -euo pipefail
A=$1; OUT=$2; W=$(dirname "$0")/work/wh
STRIP=$HOME/Android/Sdk/ndk/28.2.13676358/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-strip
rm -rf "$OUT"; mkdir -p "$OUT"
for w in "$W/$A"/*.whl "$W"/pure/*.whl; do python3 -I -m zipfile -e "$w" "$OUT" >/dev/null; done
# fontTools: drop its optional compiled accelerators (built for Linux); pure-Python fallbacks are used
find "$OUT/fontTools" -name '*.so' -delete
# fontTools: only what matplotlib needs is kept big; drop nothing else
find "$OUT" -type d \( -name tests -o -name test -o -name testing -o -name __pycache__ \) -prune -exec rm -rf {} +
rm -rf "$OUT"/*.dist-info "$OUT"/numpy/f2py "$OUT"/numpy/doc "$OUT"/numpy/typing "$OUT"/numpy/distutils "$OUT"/numpy/_pyinstaller \
       "$OUT"/matplotlib/backends/web_backend "$OUT"/matplotlib/mpl-data/sample_data "$OUT"/matplotlib/mpl-data/images \
       "$OUT"/sympy/benchmarks "$OUT"/fontTools/ttLib/removeOverlaps.py "$OUT"/PIL/*.pyi "$OUT"/pip
find "$OUT" -name '*.so' -exec "$STRIP" --strip-unneeded {} + 2>/dev/null || true
find "$OUT" -name '*.pyi' -delete

# Native libraries the wheels link against (Chaquopy's openblas/freetype/libjpeg wheels + the NDK's libc++)
LIBS=$OUT-libs; rm -rf "$LIBS" "$LIBS.tmp"; mkdir -p "$LIBS" "$LIBS.tmp"
for w in "$W"/libs-$A/*.whl; do python3 -I -m zipfile -e "$w" "$LIBS.tmp" >/dev/null; done
cp "$LIBS.tmp"/chaquopy/lib/*.so* "$LIBS"/
TRIPLE=$([ "$A" = x86_64 ] && echo x86_64-linux-android || echo aarch64-linux-android)
cp "$HOME/Android/Sdk/ndk/28.2.13676358/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/$TRIPLE/libc++_shared.so" "$LIBS"/
rm -rf "$LIBS.tmp"
find "$LIBS" -name "*.so*" -exec "$STRIP" --strip-unneeded {} + 2>/dev/null || true
