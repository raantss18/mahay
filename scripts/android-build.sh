#!/usr/bin/env bash
# Build Mahay Android apps: the Rust library with cargo-ndk, then the APK/AAB with Gradle.
#
#   scripts/android-build.sh [--debug] [--abi arm64-v8a|x86_64|all] [--aab] app...
#   apps: trano sary taratasy soratra tabilao seho (default: all)
#
# Needs ANDROID_HOME (or ~/Android/Sdk), the NDK named in android/app/build.gradle, rustup with
# the Android targets, and cargo-ndk. Outputs land in dist/android/.
set -euo pipefail
cd "$(dirname "$0")/.."
ROOT=$PWD
NDK_VERSION=$(sed -n 's/.*ndkVersion "\(.*\)"/\1/p' android/app/build.gradle)
# The first SDK that has the NDK we need (a stale ANDROID_HOME is common).
for sdk in "${ANDROID_HOME:-}" "${ANDROID_SDK_ROOT:-}" "$HOME/Android/Sdk"; do
  if [ -n "$sdk" ] && [ -d "$sdk/ndk/$NDK_VERSION" ]; then export ANDROID_HOME=$sdk; break; fi
done
export ANDROID_NDK_HOME=$ANDROID_HOME/ndk/$NDK_VERSION
[ -d "$ANDROID_NDK_HOME" ] || { echo "NDK $NDK_VERSION not found (set ANDROID_HOME)" >&2; exit 1; }
echo "sdk.dir=$ANDROID_HOME" > android/local.properties
# rustup's cargo (with the Android targets) before a distribution's cargo.
[ -x "$HOME/.cargo/bin/cargo" ] && export PATH="$HOME/.cargo/bin:$PATH"

profile=android; gradle_type=Release; abis=(arm64-v8a x86_64); aab=0; apps=()
while [ $# -gt 0 ]; do
  case "$1" in
    --debug) profile=dev; gradle_type=Debug ;;
    --abi) shift; [ "$1" = all ] || abis=("$1") ;;
    --aab) aab=1 ;;
    *) apps+=("$1") ;;
  esac
  shift
done
[ ${#apps[@]} -gt 0 ] || apps=(trano sary taratasy soratra tabilao seho)

ndk_args=()
for a in "${abis[@]}"; do ndk_args+=(-t "$a"); done
mkdir -p dist/android
for app in "${apps[@]}"; do
  echo "==> $app: Rust ($profile, ${abis[*]})"
  jni="$ROOT/android/app/src/$app/jniLibs"
  rm -rf "$jni"
  (cd "apps/$app" && cargo ndk "${ndk_args[@]}" -P 26 -o "$jni" \
     build -p "$app-android" --profile "$profile" --target-dir "$ROOT/target/android")
  # cargo-ndk also copies dylibs some dependencies build on the side (boa, ocrs, rten): they are
  # already linked into the app's library, so only lib<app>_android.so ships.
  find "$jni" -name '*.so' ! -name "lib${app}_android.so" -delete
  flavour="$(tr '[:lower:]' '[:upper:]' <<< "${app:0:1}")${app:1}"
  echo "==> $app: Gradle"
  (cd android && ./gradlew --quiet "assemble$flavour$gradle_type")
  [ $aab = 1 ] && (cd android && ./gradlew --quiet "bundle$flavour$gradle_type")
  lower=$(tr '[:upper:]' '[:lower:]' <<< "$gradle_type")
  find "android/app/build/outputs/apk/$app/$lower" -name '*.apk' -exec cp {} dist/android/ \;
  [ $aab = 1 ] && find "android/app/build/outputs/bundle/$app$gradle_type" -name '*.aab' -exec cp {} dist/android/ \; || true
done
ls -la dist/android
