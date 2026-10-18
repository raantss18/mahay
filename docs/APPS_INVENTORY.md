# Apps inventory

Upstream: <https://github.com/storytold>. Repos were listed with `gh repo list storytold`
on 2026-10-18. Licenses come from each repo's `LICENSE-*` files and `Cargo.toml`, not from
the website. All the app repos ship `LICENSE-MIT` + `LICENSE-APACHE`, and the workspace
`license` field reads `MIT OR Apache-2.0`.

| #  | Upstream repo         | Mahay name | Path in Mahay   | License            | Upstream commit                            | Date       | In release 1 |
|----|-----------------------|------------|-----------------|--------------------|--------------------------------------------|------------|--------------|
| –  | craft-libs            | (core)     | `libs/core`     | MIT OR Apache-2.0  | `82a24b006eefa036b43c09a67fb9a3fbd17ffe63` | 2026-10-17 | yes          |
| –  | craft-launcher        | Trano      | `apps/trano`    | MIT OR Apache-2.0  | `ad7f24a912d6f6cd586f5306d85ab22ef9bf2cbe` | 2026-10-18 | yes          |
| 1  | photocraft            | Sary       | `apps/sary`     | MIT OR Apache-2.0  | `8d2e615ab93d424c4ec25ab15b4edb5368d1f9a6` | 2026-10-18 | yes          |
| 5  | pdfcraft              | Taratasy   | `apps/taratasy` | MIT OR Apache-2.0  | `50f261d70c8e199600a82dbaa6912c6fbc4b4612` | 2026-10-18 | yes          |
| 9  | wordcraft             | Soratra    | `apps/soratra`  | MIT OR Apache-2.0  | `4a251d4b9c724a5bd3dd75f7c91c6aae77c6650e` | 2026-10-18 | yes          |
| 13 | gridcraft             | Tabilao    | `apps/tabilao`  | MIT OR Apache-2.0  | `c30fa3f1cf324603f3bfc0171689b1d4083b40e5` | 2026-10-17 | yes          |
| 11 | deckcraft             | Seho       | `apps/seho`     | MIT OR Apache-2.0  | `22db9261f9d1a214c296fd390173130c45d76d66` | 2026-10-17 | yes          |
| 2  | vectorcraft           | Tsilo    | –               | MIT OR Apache-2.0  | `2cdb0d2d53a0ff3c4ae994bf4808ae53a5799505` | 2026-10-18 | later        |
| 8  | figcraft              | Endrika    | –               | MIT OR Apache-2.0  | `d8ff1441b1cd47cb45f1e58b030409ef6f1e24bf` | 2026-10-18 | later        |
| 7  | designcraft           | Pejy       | –               | MIT OR Apache-2.0  | `7cb8b12507def48fb9346e5004340b5e8e4cfcb7` | 2026-10-17 | later        |
| 4  | lightcraft            | Hazavana   | –               | MIT OR Apache-2.0  | `4f0b110bcbd2e350c6d97b47f1d824c2222295c4` | 2026-10-17 | later        |
| 10 | soundcraft            | Feo        | –               | MIT OR Apache-2.0  | `c1bf044a85b55e9aa083624c39fabddbb08dadeb` | 2026-10-17 | later        |
| 3  | filmcraft             | Hetsika    | –               | MIT OR Apache-2.0  | `48088e3a8e50bd4a7a71d1ebdfde0835b01a3051` | 2026-10-18 | later        |
| 6  | effectcraft           | Tselatra   | –               | MIT OR Apache-2.0  | `f36b2b3e6bd121dc52b00503cdf8d60f42ac242a` | 2026-10-17 | later        |
| 12 | cadcraft              | Drafitra   | –               | MIT OR Apache-2.0  | `b23d362f6af1dd77e319728f1d85882b098626fe` | 2026-10-17 | later        |

Other upstream repos, not imported:

| Repo                   | License (files)   | Note                                                                 |
|------------------------|-------------------|----------------------------------------------------------------------|
| artcraft               | MIT OR Apache-2.0 | Tauri + React AI app (`d4e4ac69bbd9`); planned after release 1       |
| craft-ai-integrations  | MIT OR Apache-2.0 | interim AI layer; evaluated for the model hub (phase D)              |
| craft-fonts            | MIT OR Apache-2.0 | optional `MAHAY_FONTS_DIR` input (CJK fallback); 68 MB, not vendored |
| winit                  | Apache-2.0        | upstream winit fork; Taratasy vendors its own copy                   |
| AniCraft, crafthub     | –                 | empty repos (0 KB); nothing to import                                |
| craft-common-actions   | none found        | CI actions with no license file: not used                            |

## Build status

Filled in from real runs (2026-10-18); details in `docs/STATUS.md`.

| App      | Linux x86_64 build | Android arm64 / x86_64 APK | Ran on the emulator |
|----------|--------------------|----------------------------|---------------------|
| Trano    | CI + local         | 4.8 / 5.3 MB               | tablet              |
| Sary     | CI + local         | 23.2 / 25.1 MB             | tablet              |
| Taratasy | CI + local         | 17.6 / 19.3 MB             | tablet              |
| Soratra  | CI + local         | 17.1 / 18.4 MB             | tablet              |
| Tabilao  | CI + local         | 8.5 / 9.3 MB               | tablet and phone    |
| Seho     | CI + local         | 12.2 / 13.4 MB             | tablet              |
