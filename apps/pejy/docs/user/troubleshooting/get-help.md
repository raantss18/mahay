---
title: Report a problem
description: What to include and where to post when Pejy misbehaves or crashes.
order: 20
verified: 0.7.1
---

Post in the [issue tracker](https://github.com/raantss18/mahay/issues) or in [Discord](https://github.com/raantss18/mahay/discussions), and attach the log file. The log is the most useful thing you can send after a crash.

## What to include

- Your Pejy version and how you installed it (installer, portable, AppImage and so on).
- Your operating system and graphics card.
- What you did, what you expected and what happened.
- The log file `pejy.log`.

## Where the log is

Pejy writes `logs/pejy.log` in its settings folder, beside `ui.json` and `prefs.json`.

| System  | Log folder                                                              |
|---------|-------------------------------------------------------------------------|
| Windows | `%APPDATA%\Pejy\logs\`                                           |
| macOS   | `~/Library/Application Support/Pejy/logs/`                       |
| Linux   | `~/.config/pejy/logs/` (or `$XDG_CONFIG_HOME/pejy/logs/`) |

Each start moves the previous log to `pejy.1.log`, and that one to `pejy.2.log`. The log from a run that crashed survives the next start, so attach `pejy.1.log` if you have restarted since. The file stops growing at 16 MiB.

## More detail in the log

Set the environment variable `RUST_LOG` to `debug` before starting Pejy to log more. The default is `info` for Pejy's own parts and `warn` for everything else.
