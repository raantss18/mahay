#!/bin/bash
# build-kernel-android.sh <x86_64|aarch64>: cross-compile naoty-kernel against the android.py CPython prefix
set -euo pipefail
export PATH=$HOME/.cargo/bin:$PATH
cd "$(dirname "$0")"
ARCH=$1; T=$ARCH-linux-android; API=24
export ANDROID_HOME=$HOME/Android/Sdk ANDROID_SDK_ROOT=$HOME/Android/Sdk
TC=$ANDROID_HOME/ndk/28.2.13676358/toolchains/llvm/prebuilt/linux-x86_64/bin
U=$(echo $T | tr 'a-z-' 'A-Z_')
L=$(echo $T | tr "-" "_"); export "CC_$L=$TC/$T$API-clang" "AR_$L=$TC/llvm-ar" "CARGO_TARGET_${U}_LINKER=$TC/$T$API-clang"
export PYO3_CROSS_LIB_DIR=$PWD/build/cpython/cross-build/$T/prefix/lib PYO3_CROSS_PYTHON_VERSION=3.13
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=$PWD/target
cargo build --release -j 4 -p naoty-kernel -p naoty-k0-client --target $T
ls -la target/$T/release/naoty-kernel target/$T/release/naoty-k0-client
