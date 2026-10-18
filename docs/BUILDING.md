# Building Mahay

Each app under `apps/` is its own Cargo workspace (with its own `Cargo.lock`), exactly as
upstream builds it. `libs/core` holds the shared crates and the vendored third-party crates
Mahay patches (`rfd`, `arboard` with Android backends; `krilla`, `hayro`).

## Linux desktop

Needs Rust 1.95+ (`rustup` or your distribution's), a C compiler, and the usual X11/Wayland
development headers (upstream lists them in each app's `docs/`).

```bash
cd apps/tabilao
cargo run --release -p tabilao            # the app
cargo run --release -p tabilao-cli -- --help
```

Packages (AppImage, .deb, .rpm, tar.gz) come from upstream's scripts, unchanged apart from the
names:

```bash
apps/tabilao/packaging/linux/package.sh --formats "appimage deb"
```

Build one crate at a time (`-p`) on a laptop: a full `--workspace` build of the larger apps
(Sary, Taratasy) uses a lot of memory.

## Android (phone and tablet)

Needs:

- the Android SDK with platform 36 and NDK `28.2.13676358` (`sdkmanager "ndk;28.2.13676358"`);
- `rustup` with `aarch64-linux-android` and `x86_64-linux-android`;
- `cargo install cargo-ndk`;
- JDK 17+.

```bash
scripts/android-build.sh --abi arm64-v8a tabilao      # one app, phones/tablets
scripts/android-build.sh --abi x86_64 taratasy        # for the emulator
scripts/android-build.sh --aab                        # every app, both ABIs, plus AABs
```

The script finds an SDK that has the NDK (a stale `ANDROID_HOME` is skipped), builds
`apps/<app>/apps/<app>-android` with the `android` profile (fat LTO, stripped), copies the
library into `android/app/src/<app>/jniLibs`, runs Gradle for that app's flavour, and copies
the APKs to `dist/android/`.

### How an app runs on Android

```
MahayActivity (Java, GameActivity)  ── JNI ──  mahay-android (Rust)
        │  pickers, intents, clipboard,            │  init, poll(), save_target(),
        │  insets, launching apps                  │  EguiBridge (keyboard, insets)
        ▼                                          ▼
  android-activity ── winit ── eframe ── <app>-ui-egui (the same UI as on desktop)
```

### Signing

Release APKs are signed when a keystore is available, in `android/keystore.properties`
(git-ignored):

```properties
storeFile=/home/you/.mahay/release.jks
storePassword=…
keyAlias=mahay
keyPassword=…
```

or from the environment (CI): `MAHAY_KEYSTORE`, `MAHAY_KEYSTORE_PASSWORD`, `MAHAY_KEY_ALIAS`,
`MAHAY_KEY_PASSWORD`. Without one, release builds use the debug key. Keep the keystore
**outside** the repository and back it up: Android refuses updates signed with another key.

### Emulator

```bash
avdmanager create avd -n mahay_tablet -k "system-images;android-36;google_apis;x86_64" -d pixel_tablet
emulator -avd mahay_tablet
adb install -r dist/android/app-tabilao-x86_64-release.apk
```

## Icons

```bash
python3 -I brand/generate.py && python3 -I brand/apply.py
```
