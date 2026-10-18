# Release playbook

The release playbook shared by every crafting app lives in mahay-rules:
[`../mahay-rules/release/playbook.md`](../../../mahay-rules/release/playbook.md)
(`raantss18/mahay-rules`, checked out next to `mahay-apps/`; the repository is not public). It is
the canonical copy; don't duplicate it here. Cutting a release step by step:
`mahay-rules/release/cut-a-release.md`.

Endrika copies Sary's reference implementation (`.github/workflows/release.yml`,
`packaging/`, the `xtask version` command) with Endrika's names. Endrika-specific details are in
[releasing.md](releasing.md).
