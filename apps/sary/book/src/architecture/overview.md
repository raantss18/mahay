# Architecture overview

Sary separates the document engine from presentation and transport. The desktop UI, CLI, JSON control channel, and MCP server reach editing behavior through the command registry rather than maintaining separate implementations.

```text
desktop UI       CLI       JSON control       MCP
    |             |              |              |
    +-------------+--------------+--------------+
                          |
                   command registry
                          |
                    engine Session
                          |
       document model + operations + compositors
                          |
             PSD / raster / .sary I/O
```

The enforced dependency direction runs from format-independent foundation crates through the document and operation layers to composition, I/O, the engine, and finally UI/automation. `cargo xtask layers` checks this rule.

The complete design narrative and dependency diagram remain in [`docs/architecture.md`](https://github.com/raantss18/mahay/blob/main/docs/architecture.md). That document contains historical design material as well as implemented architecture; confirm behavior in current source when security or compatibility depends on it.

## Core properties

- `sary-doc` holds the pure-data document model.
- `sary-engine::Session` owns open documents, active state, history, and command dispatch.
- `sary-compose` is the CPU reference compositor; `sary-gpu` accelerates supported interactive paths.
- `sary-io` maps between documents and PSD or flat formats.
- `sary-format` reads and writes the native `.sary` bundle.
- `sary-ui-egui` is a thin shell over engine commands.
- `sary-automation` exposes the headless engine and the live-GUI bridge.

See [Crates and dependency layers](crates.md) for the complete workspace map.
