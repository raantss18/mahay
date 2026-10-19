# Sync

Decision: [D20](DECISIONS.md). Status (see [TODO](../TODO.md)):

- **Built and tested offline** in `libs/core/crates/sync` (`mahay-sync`): encryption, the check
  block, the manifest, the `Backend` trait with an in-memory backend and a WebDAV/Nextcloud
  backend, and the engine (journal, conflicts, sync bin). `cargo test -p mahay-sync` runs 37
  tests, including a full sync through a local WebDAV server.
- **Not built yet:** the Supabase account and the `sync_items` table, the Google Drive backend
  (both need accounts and OAuth clients the project owner creates), secure storage of the key
  and passwords, background runs, and the hub screens. No app uses `mahay-sync` yet, and it has
  not been tried against a real Nextcloud server.

## Accounts

- The account is optional. Every app works without it, as today.
- Mahay HUB → "Continue with Google" or "Continue with GitHub". This is Supabase Auth with the
  PKCE flow, as in Tontolo.
- Redirects:
  - Android: the app comes back through `org.mahay.hub://login-callback/`.
  - Linux: a loopback listener on `http://localhost:<port>/`.
- The session (access and refresh tokens) is kept in secure storage: Android Keystore on Android,
  the libsecret / Secret Service on Linux. It is never written to a plain file.
- Configuration is passed at build time (`MAHAY_SUPABASE_URL`, `MAHAY_SUPABASE_KEY`, the public
  anon key) and described in `docs/BUILDING.md`. A build without them hides the account.

## Where files go

| Backend | What it stores | Space | How it is chosen |
|---|---|---|---|
| Google Drive (`drive.file` scope) | documents, in a `Mahay/` folder | the user's own quota (15 GB free) | default after "Continue with Google"; available to GitHub users too |
| Nextcloud / WebDAV | documents, in `Mahay/` | the user's server or provider | URL + app password |
| Supabase table `sync_items` | small state: settings, recent files, installed apps, the passphrase check | a few KB per user | always, once signed in |

- Drive access uses its own Google OAuth client (installed-app flow with PKCE, refresh token in
  secure storage). It is separate from the Supabase sign-in, so a GitHub user can link a Drive
  too.
- `drive.file` is a non-sensitive scope: only brand verification is needed, with no security
  assessment.
- Every backend implements the same `Backend` trait: `put`, `get`, `list` and `delete` of opaque
  blobs in a flat `Mahay/` folder.
- WebDAV (`mahay_sync::WebDav`): the user's WebDAV root (for Nextcloud
  `https://<host>/remote.php/dav/files/<user>/`), a user name and an app password sent with HTTP
  Basic. `https://` only (plain `http://` only for `localhost`), redirects are not followed, and
  `Mahay/` is created with `MKCOL` on the first write. It uses `ureq` with rustls and `ring`,
  which Mahay HUB already ships for updates: no new crate.

## Encryption

- **Sync passphrase:** chosen once, never sent anywhere. A recovery note is shown at setup.
- **Key:** PBKDF2-HMAC-SHA256 with 310,000 iterations and a 16-byte random salt.
- **Check block** (`meta/check`, as in Tontolo): the JSON
  `{"v":1,"salt":<base64>,"iter":310000,"check":<base64 of nonce ‖ AES-GCM("mahay-sync-check-v1")>}`
  with the associated data `meta|check`. A wrong passphrase is refused before anything is
  decrypted. It holds no secret, so it is stored as `check.json` on the backend too: a
  Nextcloud user with no account can still sync. With an account it also goes to `sync_items`
  under `meta/check` (not built yet).
- **Each file:** AES-256-GCM in 1 MiB chunks, each chunk with its own nonce. The associated data
  is `mahay-v1|<file id>|<chunk index>|<last chunk flag>`, which stops chunks being reordered or
  the file being cut short.
- **Blob format:** `MHY1`, then for each chunk `nonce (12) ‖ ciphertext ‖ tag (16)`. An empty
  file is one empty last chunk. The file id in the associated data is the blob's random id.
- **Names and folders are encrypted too.** The provider sees a flat `Mahay/` folder with
  `check.json`, `manifest.mhy` and random ids like `<32 hex>.mhy`. A new version of a file gets
  a new id; the old blob is deleted once the manifest no longer points to it.
- **Manifest** (`manifest.mhy`, same chunk format with the file id `manifest`): an encrypted JSON
  `{"v":1,"files":{<path>:{id,size,mtime,sha256,version,device,deleted?}}}`. Paths are relative
  with `/`; on reading, a path with `..`, `.`, a hidden part, `\` or `:` is ignored.
- **Vectors:** the unit tests check the RFC 7914 PBKDF2 vectors and a check block and chunk made
  independently with Python's `cryptography`.

## Engine (`libs/core/crates/sync`, `mahay-sync`)

- **What it watches:** the Mahay documents folder on each device (Android: `files/` plus the
  documents opened through the system picker that the user chose to sync).
- **When it runs:** a scan on start, then a check every few minutes while the hub is open. On
  Android, a WorkManager job runs when the network is available.
- **Offline:** the journal (`journal.json` in the engine's private state folder) keeps, for each
  path, the last synced manifest entry. A sync compares three states: the local file, the
  journal and the remote manifest. Changes made offline are simply found at the next scan.
- **Each step is saved as it ends**, so a network cut anywhere leaves a state the next run
  finishes from: a blob sent before the cut is reused (`pending` in the journal), a manifest
  write whose answer was lost is recognised, blobs left unused are deleted later (`unused`).
  The tests cut the network at every step of an upload and of a download.
- **Two devices writing the manifest at once:** the engine reads it again just before writing
  and only applies a change if that path's version is still the one it planned on; a change
  that was overwritten anyway is published again on the next run (its version is higher than
  the remote's).
- **Conflicts:** if both sides changed the same file, the remote copy is kept as
  `name (conflit <device> <date>).ext` (the date is the remote copy's, UTC, `YYYY-MM-DD HH-MM`,
  without `:` so the name works everywhere) and the local one is published. Same content on
  both sides is not a conflict. An edit wins over a delete. Nothing is overwritten silently.
- **Deletes:** a deleted file stays in the manifest marked `deleted` with its blob, in the sync
  bin, for 30 days (`Engine::bin`, `Engine::restore`); after that the blob is removed. Another
  device deletes its copy only if it did not change it.
- **Skipped:** hidden files and folders (names starting with `.`, which includes Mahay's
  temporary `.mahay_tmp` files), symbolic links, non-UTF-8 names and names with `:`.
- **Unchanged files are not hashed again:** same size and same modification time as at the last
  sync, once that time is more than two seconds old (file systems keep coarse times).
- **Limits today:** a file is read whole into memory to be encrypted (streaming is in the TODO);
  a blob missing on the backend is reported (`Report::missing`) and that file is skipped.
- **Apps:** they see ordinary local files and need no change. The hub shows the sync state:
  account, backend, last sync, conflicts.

## Later (phase C)

Live collaboration on the LAN or through a relay (CRDT with Automerge or Loro, Noise, QR
pairing) adds to this. It does not replace it.
