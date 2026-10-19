# Releasing Mahay

One version for the whole suite (`VERSION`). File names never change: installed apps find their
updates by them.

| File                                   | What                                                   |
|----------------------------------------|--------------------------------------------------------|
| `<app>-<v>-android-arm64-v8a.apk`      | phones and tablets                                     |
| `<app>-<v>-android-x86_64.apk`         | emulators, Chromebooks                                 |
| `<app>-<v>-linux-x86_64.AppImage`      | Linux (self-updating)                                  |
| `<app>-<v>-linux-x86_64.AppImage.zsync`| AppImage delta updates                                 |
| `<app>-<v>-linux-x86_64.deb`           | Debian/Ubuntu package (updates are only reported)      |
| `SHA256SUMS.txt`                       | checksums of every file                                |
| `SHA256SUMS.txt.sig`                   | Ed25519 signature of `SHA256SUMS.txt` (mandatory)      |

## Steps (local, with the keys in `release-keys/`)

```bash
python3 -I scripts/bump-version.py 0.3.0          # every app, VERSION
# write the CHANGELOG section "## [0.3.0] — date"
scripts/android-build.sh --abi all                 # 14 apps × 2 ABIs, signed (release-keys/)
for a in hub sary taratasy soratra tabilao seho tsilo endrika pejy hazavana feo hetsika tselatra drafitra; do
  (cd apps/$a && cargo build --release --locked -p $a && APPIMAGELAUNCHER_DISABLE=1 packaging/linux/package.sh --skip-build --formats "appimage deb")
done
scripts/stage-release.sh 0.3.0                     # renames, SHA256SUMS.txt, signature, notes
git tag -a v0.3.0 -m "Mahay 0.3.0" && git push origin main v0.3.0
gh release create v0.3.0 --latest --title "Mahay 0.3.0" --notes-file dist/release-notes.md dist/release/*
```

`stage-release.sh` refuses to finish without `release-keys/update-signing.pem`, and checks the
signature with `.github/update-signing.pub` before anything is published.

Publish as **latest**, never as a pre-release: `releases/latest` (what the apps ask) skips
pre-releases.

## The release page

`scripts/release-notes.py` writes it, Tontolo-style: **Mahay HUB first** (Android arm64 = the right
choice in 95 % of cases; AppImage for Linux), the system requirements (kept in step with the README), the note that Mahay updates itself, the new
features from the CHANGELOG, then the other apps' files folded away (Mahay HUB downloads them).

## From CI

Actions ▸ Release ▸ Run workflow does the same once the secrets are set
(`release-keys/set-github-secrets.sh`, see `docs/BUILDING.md`).
