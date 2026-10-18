# Sync (design, not built yet)

Decision: [D20](DECISIONS.md). Status: planned after 0.2.0 (see [TODO](../TODO.md)).

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

## Encryption

- **Sync passphrase:** chosen once, never sent anywhere. A recovery note is shown at setup.
- **Key:** PBKDF2-HMAC-SHA256 with 310,000 iterations and a random salt. The salt and a check
  block are stored in `sync_items` under `meta/check`, as in Tontolo.
- **Each file:** AES-256-GCM in 1 MiB chunks, each chunk with its own nonce. The associated data
  is `mahay-v1|<file id>|<chunk index>|<last chunk flag>`, which stops chunks being reordered or
  the file being cut short.
- **Names and folders are encrypted too.** The provider sees random ids like `Mahay/<id>.mhy`
  plus an encrypted manifest.
- **Manifest:** an encrypted JSON listing path, size, mtime, SHA-256 of the plaintext, version
  and device.

## Engine (`libs/core/crates/sync`, `mahay-sync`)

- **What it watches:** the Mahay documents folder on each device (Android: `files/` plus the
  documents opened through the system picker that the user chose to sync).
- **When it runs:** a scan on start, then a check every few minutes while the hub is open. On
  Android, a WorkManager job runs when the network is available.
- **Offline:** changes wait in a local journal and are uploaded when the network comes back.
- **Conflicts:** if both sides changed the same file, the remote copy is kept as
  `name (conflit <device> <date>).ext`. Nothing is overwritten silently.
- **Deletes:** a deleted file goes to a sync bin for 30 days and is then removed from the remote.
- **Apps:** they see ordinary local files and need no change. The hub shows the sync state:
  account, backend, last sync, conflicts.

## Later (phase C)

Live collaboration on the LAN or through a relay (CRDT with Automerge or Loro, Noise, QR
pairing) adds to this. It does not replace it.
