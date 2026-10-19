#!/bin/bash
# Build the C libraries Tectonic links against, for one Android ABI, into work/android-deps/<abi>:
#   freetype (CMake, from the freetype-sys crate's sources), libpng (the libpng-sys crate's sources),
#   stubs for fontconfig and graphite2, and pkg-config files (icu-uc points at the NDK's libicu.so,
#   which exists from API 31). Usage: tools/android-deps.sh x86_64|aarch64
set -euo pipefail
ABI=${1:?abi: x86_64 or aarch64}
HERE=$(cd "$(dirname "$0")/.." && pwd)
SDK=$HOME/Android/Sdk   # the ANDROID_HOME in the environment may be stale
NDK=$SDK/ndk/28.2.13676358
TC=$NDK/toolchains/llvm/prebuilt/linux-x86_64
API=31
CC=$TC/bin/${ABI}-linux-android$API-clang
AR=$TC/bin/llvm-ar
REG=$(echo ~/.cargo/registry/src/*/)
P=$HERE/work/android-deps/$ABI
rm -rf "$P"; mkdir -p "$P"/{include,lib/pkgconfig,build}
cd "$P/build"

# --- freetype
FT=$REG/freetype-sys-0.23.0/freetype2
cmake -G Ninja -S "$FT" -B ft -DCMAKE_TOOLCHAIN_FILE=$NDK/build/cmake/android.toolchain.cmake \
  -DANDROID_ABI=$([ "$ABI" = x86_64 ] && echo x86_64 || echo arm64-v8a) -DANDROID_PLATFORM=android-$API \
  -DCMAKE_BUILD_TYPE=Release -DBUILD_SHARED_LIBS=OFF -DCMAKE_POSITION_INDEPENDENT_CODE=ON \
  -DFT_DISABLE_ZLIB=ON -DFT_DISABLE_BZIP2=ON -DFT_DISABLE_PNG=ON -DFT_DISABLE_HARFBUZZ=ON -DFT_DISABLE_BROTLI=ON \
  -DCMAKE_INSTALL_PREFIX="$P" > ft.log 2>&1
cmake --build ft -j 4 >> ft.log 2>&1 && cmake --install ft >> ft.log 2>&1

# --- libpng (vendor/ of libpng-sys); NEON off to keep it plain C
PNG=$REG/libpng-sys-1.1.11/vendor
mkdir -p png/inc; cp "$PNG/png.h" "$PNG/pngconf.h" png/inc/; cp "$PNG/scripts/pnglibconf.h.prebuilt" png/inc/pnglibconf.h
objs=""
for f in png pngerror pngget pngmem pngpread pngread pngrio pngrtran pngrutil pngset pngtrans pngwio pngwrite pngwtran pngwutil; do
  $CC -O2 -fPIC -DPNG_ARM_NEON_OPT=0 -I png/inc -I "$PNG" -c "$PNG/$f.c" -o png/$f.o; objs="$objs png/$f.o"
done
$AR rcs "$P/lib/libpng16.a" $objs; cp png/inc/*.h "$P/include/"

# --- stubs
mkdir -p "$P/include/graphite2"; cp "$HERE/tools/android-deps/stubs/graphite2/"*.h "$P/include/graphite2/"
$CC -O2 -fPIC -I "$P/include" -c "$HERE/tools/android-deps/stubs/graphite2_stub.c" -o gr.o && $CC -O2 -fPIC -c "$HERE/tools/android-deps/stubs/graphite2_stub2.c" -o gr2.o
mkdir -p "$P/include/unicode"; cp "$HERE/tools/android-deps/stubs/unicode/ucnv.h" "$P/include/unicode/"
$CC -O2 -fPIC -I "$P/include" -c "$HERE/tools/android-deps/stubs/ucnv_stub.c" -o ucnv.o && $AR rcs "$P/lib/libgraphite2.a" gr.o gr2.o ucnv.o   # the ucnv stub rides in libgraphite2.a: the pkg-config order of a second archive is not honoured
mkdir -p "$P/include/fontconfig"; cp "$HERE/tools/android-deps/stubs/fontconfig.h" "$P/include/fontconfig/fontconfig.h"
$CC -O2 -fPIC -c "$HERE/tools/android-deps/stubs/fontconfig_stub.c" -o fc.o && $AR rcs "$P/lib/libfontconfig.a" fc.o

# --- pkg-config files
pc() { # name, version, libs, cflags
cat > "$P/lib/pkgconfig/$1.pc" <<PC
prefix=$P
Name: $1
Description: $1 for the Tectonic Android spike
Version: $2
Libs: $3
Cflags: -I$P/include $4
PC
}
pc freetype2 26.2.20 "-L$P/lib -lfreetype" "-I$P/include/freetype2"
pc libpng 1.6.50 "-L$P/lib -lpng16 -lz" ""
pc graphite2 1.3.14 "-L$P/lib -lgraphite2" ""
pc fontconfig 2.15.0 "-L$P/lib -lfontconfig" ""
pc icu-uc 72.1 "-L$NDK/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/${ABI}-linux-android/31 -licu" ""
pc icu-i18n 72.1 "-L$NDK/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/${ABI}-linux-android/31 -licu" ""
ls "$P/lib"
