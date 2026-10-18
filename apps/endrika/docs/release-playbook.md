# Release playbook

The release playbook shared by every crafting app lives in craftrules:
[`../craftrules/release/playbook.md`](../../../craftrules/release/playbook.md)
(`storytold/craftrules`, checked out next to `craft-apps/`; the repository is not public). It is
the canonical copy; don't duplicate it here. Cutting a release step by step:
`craftrules/release/cut-a-release.md`.

FigCraft copies PhotoCraft's reference implementation (`.github/workflows/release.yml`,
`packaging/`, the `xtask version` command) with FigCraft's names. FigCraft-specific details are in
[releasing.md](releasing.md).
