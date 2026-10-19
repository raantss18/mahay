# Following ArtCraft's repositories

Mahay's apps come from the repositories of github.com/storytold ("ArtCraft"), imported at the
revisions in [APPS_INVENTORY.md](APPS_INVENTORY.md). Upstream moves fast (on 2026-10-19, one day
after the import, Sary's source repository was 238 commits ahead), and Mahay carries its own
changes on top: the Malagasy names, the Android layer, the shared About page and updater, the
removed backends and links, the vendored crates. Adopting upstream work therefore has two rules:
**tell the user what matters**, and **never replace what Mahay changed**.

## Watching

`scripts/upstream-watch.py` compares each imported revision with the upstream default branch
(GitHub's compare API, public data only) and sorts the new commits: security, crash, dependency,
feature, fix, other. It flags what needs a decision:

| Flag | Why it matters |
|---|---|
| security or crash fix | small, valuable, usually safe to adopt first |
| a licence file changed | read it before adopting anything from that repository |
| a new network endpoint in source files | ArtCraft's own backend, telemetry, a new community link or service: Mahay removed these (D18, PORTING_AUDIT item 17) |

```bash
python3 -I scripts/upstream-watch.py --out upstream.md      # all repositories, ~40 s
python3 -I scripts/upstream-watch.py --only sary,pdfcraft   # some of them
python3 -I scripts/upstream-watch.py --selftest
```

`.github/workflows/upstream-watch.yml` runs it every Monday and, when something is flagged, opens
(or updates) one issue labelled `upstream` in raantss18/mahay: GitHub then notifies the owner.
Nothing is ever merged automatically.

## Adopting a change, without breaking the apps

Pick **one app** and **one revision** (a commit, never a moving branch).

1. **Small fixes first (cherry-pick).** For a security or crash fix: take that commit's diff from
   upstream, run it through the same renaming as the import (`scripts/rebrand.py`), apply it with
   `git apply --3way` in a branch, build and test the touched crates. No other upstream change
   comes along.
2. **A whole range (sync).** Take three snapshots: upstream at the recorded revision (base),
   upstream at the chosen revision (theirs), and Mahay's `apps/<app>` (ours). Rename base and
   theirs with `scripts/rebrand.py`, then merge file by file with a three-way merge
   (`git merge-file`): a file upstream changed and Mahay didn't is taken; a file both changed is
   merged, and conflicts are resolved by hand, keeping Mahay's side for the Android layer, About
   page, updater, links and every file listed under "never adopt" below.
3. **Prove it.** `cargo test -p <touched crates>`, the Android build of that app, a start and a
   quick use on the tablet emulator, a look at the Linux build. Taratasy's own rules apply
   (`apps/taratasy/AGENTS.md`).
4. **Record it.** Update the revision in APPS_INVENTORY.md only once the merged tree builds and
   passes, add a CHANGELOG line ("Upstream sync: <app> to <revision>") and release as a normal
   version.

Never adopt: a backend or API client for ArtCraft's services, telemetry, the upstream update
feed, social or community links to accounts that are not Mahay's, anything that changes a
licence or adds a font or asset without an entry in the app's `ATTRIBUTION.toml`.

`scripts/upstream-sync.py` (planned, TODO) automates the snapshots, the renaming and the
three-way merge for steps 1 and 2.
