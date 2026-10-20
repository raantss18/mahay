#!/bin/bash
# measure-sizes.sh <x86_64|arm64>: raw and zstd -19 sizes per component (bytecode precompiled, as it would ship)
set -eu
export LC_ALL=C
cd "$(dirname "$0")"
A=$1; T=$([ $A = x86_64 ] && echo x86_64-linux-android || echo aarch64-linux-android)
S=work/size-$A; rm -rf $S; mkdir -p $S/stdlib $S/site
cp -r work/pyroot-$([ $A = x86_64 ] && echo x86_64 || echo aarch64)/lib/python3.13 $S/stdlib/
cp -r work/site-$A/* $S/site/ 2>/dev/null || true
.venv/bin/python -m compileall -q -j 4 --invalidation-mode unchecked-hash $S >/dev/null 2>&1 || true
row() { # name, paths...
  local n=$1; shift
  local raw=$(du -sb "$@" | awk '{s+=$1} END{print s}')
  local z=$(tar -cf - "$@" 2>/dev/null | zstd -19 -T2 -q -c | wc -c)
  awk -v n="$n" -v r=$raw -v z=$z 'BEGIN{printf "%-60s %8.1f MB raw %7.1f MB zstd\n", n, r/1048576, z/1048576}'
}
P=work/pyroot-$([ $A = x86_64 ] && echo x86_64 || echo aarch64)
echo "== $A"
row "CPython: libpython+ssl+sqlite+exe" $P/lib/libpython3.13.so $P/lib/libcrypto_python.so $P/lib/libssl_python.so $P/lib/libsqlite3_python.so $P/bin
row "CPython: stdlib (+pyc, incl. lib-dynload)" $S/stdlib
cd $S/site
row "numpy (+pyc)"                      numpy
row "matplotlib (+pyc)"                 matplotlib mpl_toolkits pylab.py
row "matplotlib deps (pillow,kiwi,contourpy,fonttools,cycler,dateutil,pyparsing,packaging,six)" PIL kiwisolver contourpy fontTools cycler dateutil pyparsing packaging six.py
row "sympy+mpmath (+pyc)"               sympy mpmath isympy.py
cd - >/dev/null
row "native libs (openblas,gfortran,c++,freetype,jpeg)" work/site-$A-libs
row "Rust kernel (release, stripped)"   target/$T/release/naoty-kernel
row "Rust client (release, stripped)"   target/$T/release/naoty-k0-client
cp $P/lib/*.so $S/stdlib/ 2>/dev/null; cp -r $P/bin $S/bin
row "PACK numpy only (CPython+stdlib+numpy+libs+kernel)" $S/stdlib $S/bin $S/site/numpy work/site-$A-libs/libopenblas.so work/site-$A-libs/libgfortran.so.3 work/site-$A-libs/libc++_shared.so target/$T/release/naoty-kernel
row "PACK full (+matplotlib+deps+sympy)" $S/stdlib $S/bin $S/site work/site-$A-libs target/$T/release/naoty-kernel
