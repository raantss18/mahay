#!/bin/bash
# Cross-compile the spike for Android with cargo-ndk. Usage: ./build-android.sh x86_64|aarch64
# (run tools/android-deps.sh <abi> first; it builds the C libraries and the pkg-config files)
set -euo pipefail
# rustup's cargo (with the Android std) before the distribution's, as scripts/android-build.sh does
export PATH="$HOME/.cargo/bin:$PATH"
ABI=${1:?x86_64 or aarch64}
cd "$(dirname "$0")"
export ANDROID_HOME=$HOME/Android/Sdk ANDROID_SDK_ROOT=$HOME/Android/Sdk
export ANDROID_NDK_HOME=$HOME/Android/Sdk/ndk/28.2.13676358
export ANDROID_NDK_ROOT=$ANDROID_NDK_HOME
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=$PWD/target
D=$PWD/work/android-deps/$ABI
export PKG_CONFIG_ALLOW_CROSS=1 PKG_CONFIG_PATH=$D/lib/pkgconfig PKG_CONFIG_LIBDIR=$D/lib/pkgconfig
export PKG_CONFIG_SYSROOT_DIR=
# the C++ in harfbuzz would otherwise need libc++_shared.so next to the executable
export CXXSTDLIB=c++_static RUSTFLAGS="-C link-arg=-lc++abi"
# API 31: the NDK exposes ICU (libicu.so) only from Android 12; see docs/NEW_APPS.md
cargo ndk -t $([ "$ABI" = x86_64 ] && echo x86_64 || echo arm64-v8a) -P 31 build --release --no-default-features -j 4
