# Security policy

## Reporting a vulnerability

Please report security problems privately through GitHub's
[private vulnerability reporting](https://github.com/raantss18/mahay/security/advisories/new),
not in a public issue. Say which app and version, and how to reproduce it (a small sample file
helps). You'll get an answer within a week.

Problems that come from the upstream ArtCraft apps can also be reported upstream; Mahay will
take the fix.

## Supported versions

Only the latest release gets fixes while Mahay is in alpha (0.x).

## What we check

Every push runs `cargo audit` on all workspaces. The last full review is in
[docs/SECURITY_AUDIT.md](docs/SECURITY_AUDIT.md).
