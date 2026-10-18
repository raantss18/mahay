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

### Signing and secrets

All keys live in **`release-keys/`** at the root of the checkout. The folder is listed in
`.gitignore`: it stays on the maintainer's machine and is never pushed. Back it up (an encrypted
USB key, a password manager). Losing it means Android refuses updates of installed apps.

| File                               | What it is                                                      |
|------------------------------------|-----------------------------------------------------------------|
| `release-keys/mahay.jks`           | Android signing keystore (PKCS12, RSA 4096, alias `mahay`)      |
| `release-keys/signing.env`         | `MAHAY_KEYSTORE`, `MAHAY_KEYSTORE_PASSWORD`, `MAHAY_KEY_ALIAS`, `MAHAY_KEY_PASSWORD` |
| `release-keys/keystore.properties` | the same for Gradle (`android/keystore.properties` links to it)  |
| `release-keys/update-signing.pem`  | Ed25519 private key that signs `SHA256SUMS` → `SHA256SUMS.sig`   |
| `release-keys/SECRETS.md`          | the values below, filled in, and the certificate fingerprints   |
| `release-keys/set-github-secrets.sh` | uploads the five secrets to GitHub Actions                    |

The public half of the update key is committed: `.github/update-signing.pub`, and embedded in
the apps (`libs/core/crates/update/src/key.rs`). Apps refuse any update whose `SHA256SUMS` isn't
signed by it.

GitHub Actions secrets (Settings ▸ Secrets and variables ▸ Actions), set with
`release-keys/set-github-secrets.sh`:

| Secret                    | Content                                       |
|---------------------------|-----------------------------------------------|
| `MAHAY_KEYSTORE_B64`      | `base64 -w0 release-keys/mahay.jks`           |
| `MAHAY_KEYSTORE_PASSWORD` | keystore password (`release-keys/signing.env`)|
| `MAHAY_KEY_ALIAS`         | `mahay`                                       |
| `MAHAY_KEY_PASSWORD`      | key password (`release-keys/signing.env`)     |
| `UPDATE_SIGNING_KEY`      | `release-keys/update-signing.pem` (PEM text)  |

Local builds read `android/keystore.properties`; without it, release APKs get the debug key
(and installed apps then refuse them as updates).

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
