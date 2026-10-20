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
# write the CHANGELOG section "## [0.3.0] — date", commit everything
scripts/android-build.sh --abi all                 # 14 apps × 2 ABIs, signed (release-keys/)
scripts/linux-build.sh                             # 14 apps: AppImage and .deb
scripts/stage-release.sh 0.3.0                     # gathers the files, then finalize-release.sh
scripts/publish-release.sh 0.3.0 --dry-run         # every check, publishes nothing
scripts/publish-release.sh 0.3.0                   # pushes main, tags v0.3.0, creates the release
```

`scripts/finalize-release.sh` (run by `stage-release.sh` and by the CI) writes `SHA256SUMS.txt`,
signs it (`release-keys/update-signing.pem`, checked with `.github/update-signing.pub`), refuses an
APK that is not signed with Mahay's release key (`.github/apk-signing-cert.sha256`: Android would
refuse the update), and with `--require-all` refuses a release with a missing app.

`publish-release.sh` refuses to publish when the tree is not clean or not on `main`, when a key or
secret file is tracked by git, when `VERSION` or the CHANGELOG do not say this version, when the tag or
release already exists, or when `finalize-release.sh --require-all` or the checksums fail. Then it
pushes, tags, creates the release as **latest** and checks that `releases/latest` points to it.

## Updates that skip versions

A user may jump from any old version to the newest one, so every release is checked against the old
ones, before and after publishing:

```bash
scripts/stage-release.sh 0.3.0
scripts/check-upgrade-path.sh --serial emulator-5558   # BEFORE: old APKs, then the staged ones over them
scripts/publish-release.sh 0.3.0
scripts/check-updater-replay.sh                        # AFTER: the old updaters against the live release
```

- `check-upgrade-path.sh` installs the APKs of the last three releases on a **scratch** emulator
  (it uninstalls the apps there, which also resets the hub's "install unknown apps" setting), then
  installs the staged APKs over them with `adb install -r`, in two orders (hub last, hub first). It
  catches a changed signing key, a lower `versionCode` or a manifest the installer refuses.
- `check-updater-replay.sh` builds the `mahay-update` crate of each old tag and runs it as an installed
  app of that version would: find the latest release (GitHub's API, then the redirect fallback used
  when the API is rate-limited), verify the signed checksums, download an APK. It catches a change in
  file names, the signature or the release layout that an old updater cannot follow.

Publish as **latest**, never as a pre-release: `releases/latest` (what the apps ask) skips
pre-releases.

## The release page

`scripts/release-notes.py` writes it, Tontolo-style: **Mahay HUB first** (Android arm64 = the right
choice in 95 % of cases; AppImage for Linux), the system requirements (kept in step with the README), the note that Mahay updates itself, the new
features from the CHANGELOG, then the other apps' files folded away (Mahay HUB downloads them).

## From CI (without this computer)

Actions ▸ **Release** ▸ Run workflow. The secrets are set (`release-keys/set-github-secrets.sh`,
`docs/BUILDING.md`). Inputs: `ref` (the tag `v0.3.0`, or `main` for a trial), `apps` (`all`, or a list such
as `tabilao` for a quick trial) and `dry_run` (**true by default**: everything is built and checked,
nothing is published).

1. **Trial first:** `ref=main`, `apps=tabilao`, `dry_run=true`: a few minutes; checks the toolchain, the
   signing key and the checksum signature.
2. **Full dry run:** `ref=main`, `apps=all`, `dry_run=true`: compare file names and sizes with a local
   build.
3. **Release:** tag and push `v0.3.0` (`git tag -a v0.3.0 && git push origin v0.3.0`), then `ref=v0.3.0`,
   `apps=all`, `dry_run=false`. The workflow refuses to publish unless `ref` is the tag of `VERSION`, all
   14 apps are built and the CHANGELOG has the section.

The CI signs APKs with the same key and `SHA256SUMS.txt` with the same Ed25519 key as a local release, so
installed apps update from either.
