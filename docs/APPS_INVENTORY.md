# Apps inventory

Upstream: <https://github.com/storytold>. Repos were listed with `gh repo list storytold`
on 2026-10-18. Licenses come from each repo's `LICENSE-*` files and `Cargo.toml`, not from
the website. All the app repos ship `LICENSE-MIT` + `LICENSE-APACHE`, and the workspace
`license` field reads `MIT OR Apache-2.0`.

| #  | Upstream repo         | Mahay name | Path in Mahay   | License            | Upstream commit                            | Date       | Since        |
|----|-----------------------|------------|-----------------|--------------------|--------------------------------------------|------------|--------------|
| –  | craft-libs            | (core)     | `libs/core`     | MIT OR Apache-2.0  | `82a24b006eefa036b43c09a67fb9a3fbd17ffe63` | 2026-10-17 | 0.1.0        |
| –  | craft-launcher        | Trano      | `apps/trano`    | MIT OR Apache-2.0  | `ad7f24a912d6f6cd586f5306d85ab22ef9bf2cbe` | 2026-10-18 | 0.1.0        |
| 1  | photocraft            | Sary       | `apps/sary`     | MIT OR Apache-2.0  | `8d2e615ab93d424c4ec25ab15b4edb5368d1f9a6` | 2026-10-18 | 0.1.0        |
| 5  | pdfcraft              | Taratasy   | `apps/taratasy` | MIT OR Apache-2.0  | `50f261d70c8e199600a82dbaa6912c6fbc4b4612` | 2026-10-18 | 0.1.0        |
| 9  | wordcraft             | Soratra    | `apps/soratra`  | MIT OR Apache-2.0  | `4a251d4b9c724a5bd3dd75f7c91c6aae77c6650e` | 2026-10-18 | 0.1.0        |
| 13 | gridcraft             | Tabilao    | `apps/tabilao`  | MIT OR Apache-2.0  | `c30fa3f1cf324603f3bfc0171689b1d4083b40e5` | 2026-10-17 | 0.1.0        |
| 11 | deckcraft             | Seho       | `apps/seho`     | MIT OR Apache-2.0  | `22db9261f9d1a214c296fd390173130c45d76d66` | 2026-10-17 | 0.1.0        |
| 2  | vectorcraft           | Tsilo      | `apps/tsilo`     | MIT OR Apache-2.0  | `73f921fa0a96eeebeba48ca0bf7c2b59ffce4ac9` | 2026-10-18 | 0.2.0        |
| 8  | figcraft              | Endrika    | `apps/endrika`   | MIT OR Apache-2.0  | `d8ff1441b1cd47cb45f1e58b030409ef6f1e24bf` | 2026-10-18 | 0.2.0        |
| 7  | designcraft           | Pejy       | `apps/pejy`      | MIT OR Apache-2.0  | `7141a68f1850255e079e3089c448c499cd45e896` | 2026-10-18 | 0.2.0        |
| 4  | lightcraft            | Hazavana   | `apps/hazavana`  | MIT OR Apache-2.0  | `31a677fe3dbf4bc1042f8f6346e83b0afd1b04e8` | 2026-10-18 | 0.2.0        |
| 10 | soundcraft            | Feo        | `apps/feo`       | MIT OR Apache-2.0  | `31f038939715e055dc24d09d613e206d397d5fd4` | 2026-10-18 | 0.2.0        |
| 3  | filmcraft             | Hetsika    | `apps/hetsika`   | MIT OR Apache-2.0  | `48088e3a8e50bd4a7a71d1ebdfde0835b01a3051` | 2026-10-18 | 0.2.0        |
| 6  | effectcraft           | Tselatra   | `apps/tselatra`  | MIT OR Apache-2.0  | `22072d9f25846dbf8a3bf10105fdb51c3c0db4c1` | 2026-10-18 | 0.2.0        |
| 12 | cadcraft              | Drafitra   | `apps/drafitra`  | MIT OR Apache-2.0  | `3370a91fe96d0f8752622d041b7fed199a2bbac0` | 2026-10-18 | 0.2.0        |

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
