# Third-party licenses

Generated with [cargo-about](https://github.com/EmbarkStudios/cargo-about) from each Cargo
workspace (`scripts/about/`). Every dependency's license is in the accepted list of
`scripts/about/about.toml`: permissive licenses, MPL-2.0 (file-level copyleft, unmodified
crates), and free licenses for bundled data (OFL-1.1 and Ubuntu Font Licence 1.0 for fonts, the
IJG notice for JPEG code, Unicode-3.0 for character data). No GPL, LGPL or AGPL code is linked.

Regenerate:

```bash
cargo install cargo-about --features cli
for w in libs/core apps/*; do (cd $w && cargo about generate -c ../../scripts/about/about.toml ../../scripts/about/template.hbs); done
```

Fonts, icons and other assets that upstream bundles are listed in each tree's `ATTRIBUTION.md`
or `ATTRIBUTION.toml`; Mahay's own icons are in `docs/BRAND.md`.

## `libs/core`

- **MIT License** (292)
- **Unicode License v3** (19)
- **Apache License 2.0** (15)
- **BSD 3-Clause "New" or "Revised" License** (5)
- **ISC License** (5)
- **Community Data License Agreement Permissive 2.0** (1)
- **MIT No Attribution** (1)
- **Mozilla Public License 2.0** (1)
- **zlib License** (1)

### Apache License 2.0

Used by: `flagset 0.4.7`

### Apache License 2.0

Used by: `self_cell 1.3.0`

### Apache License 2.0

Used by: `unicode-general-category 1.1.0`

### Apache License 2.0

Used by: `aws-lc-sys 0.45.0`, `ryu-js 1.0.3`, `ryu 1.0.23`, `sigstore-bundle 0.6.6`, `sigstore-crypto 0.6.6`, `sigstore-merkle 0.6.6`, `sigstore-rekor 0.6.6`, `sigstore-trust-root 0.6.6`, `sigstore-tsa 0.6.6`, `sigstore-types 0.6.6`, `sigstore-verify 0.6.6`, `sync_wrapper 1.0.2`

### BSD 3-Clause "New" or "Revised" License

Used by: `encoding_rs 0.8.42`

### BSD 3-Clause "New" or "Revised" License

Used by: `winprint 0.2.1`

### BSD 3-Clause "New" or "Revised" License

Used by: `subtle 2.6.1`

### BSD 3-Clause "New" or "Revised" License

Used by: `aws-lc-sys 0.45.0`

### BSD 3-Clause "New" or "Revised" License

Used by: `encoding_rs 0.8.42`

### Community Data License Agreement Permissive 2.0

Used by: `webpki-root-certs 1.0.9`

### ISC License

Used by: `untrusted 0.7.1`, `untrusted 0.9.0`

### ISC License

Used by: `rustls-webpki 0.103.15`

### ISC License

Used by: `aws-lc-rs 1.18.1`, `aws-lc-sys 0.45.0`

### MIT License

Used by: `atomic-waker 1.1.2`

### MIT License

Used by: `sha1 0.10.7`, `sha2 0.10.9`

### MIT License

Used by: `rayon-core 1.13.0`, `rayon 1.12.0`

### MIT License

Used by: `core-foundation-sys 0.8.7`, `core-foundation 0.10.1`, `core-foundation 0.9.4`, `euclid 0.22.14`

### MIT License

Used by: `hex 0.4.3`

### MIT License

Used by: `form_urlencoded 1.2.2`

### MIT License

Used by: `idna 1.1.0`, `percent-encoding 2.3.2`, `url 2.5.8`

### MIT License

Used by: `cc 1.6.0`, `cfg-if 1.0.5`, `cmake 0.1.58`, `filetime 0.2.29`, `find-msvc-tools 0.1.14`, `jobserver 0.1.35`, `js-sys 0.3.106`, `openssl-probe 0.2.1`, `pkg-config 0.3.34`, `socket2 0.6.5`, `wasm-bindgen-futures 0.4.79`, `wasm-bindgen-macro-support 0.2.129`, `wasm-bindgen-macro 0.2.129`, `wasm-bindgen-shared 0.2.129`, `wasm-bindgen 0.2.129`, `web-sys 0.3.106`

### MIT License

Used by: `mio 1.2.4`

### MIT License

Used by: `errno 0.3.14`

### MIT License

Used by: `mime 0.3.17`

### MIT License

Used by: `base64ct 1.8.3`

### MIT License

Used by: `bitflags 2.13.2`, `log 0.4.34`, `num-traits 0.2.19`

### MIT License

Used by: `uuid 1.27.0`

### MIT License

Used by: `flate2 1.1.10`

### MIT License

Used by: `hyper 1.12.0`

### MIT License

Used by: `either 1.19.0`, `itertools 0.15.0`

### MIT License

Used by: `tempfile 3.27.0`, `xattr 1.6.1`

### MIT License

Used by: `heck 0.5.0`, `unicode-segmentation 1.13.3`

### MIT License

Used by: `jni-sys 0.3.1`, `jni-sys 0.4.1`

### MIT License

Used by: `schannel 0.1.29`

### MIT License

Used by: `httparse 1.10.1`

### MIT License

Used by: `futures-channel 0.3.34`, `futures-core 0.3.34`, `futures-io 0.3.34`, `futures-sink 0.3.34`, `futures-task 0.3.34`, `futures-util 0.3.34`

### MIT License

Used by: `hashbrown 0.17.1`

### MIT License

Used by: `serde_urlencoded 0.7.1`

### MIT License

Used by: `hyper-rustls 0.27.10`, `rustls-native-certs 0.8.4`, `rustls 0.23.45`

### MIT License

Used by: `lock_api 0.4.14`, `parking_lot 0.12.5`, `parking_lot_core 0.9.12`, `rustc_version 0.4.1`

### MIT License

Used by: `android_log-sys 0.3.2`

### MIT License

Used by: `android_logger 0.15.1`

### MIT License

Used by: `indexmap 2.14.2`

### MIT License

Used by: `equivalent 1.0.2`

### MIT License

Used by: `scopeguard 1.2.0`

### MIT License

Used by: `reqwest 0.13.5`

### MIT License

Used by: `digest 0.10.7`

### MIT License

Used by: `fnv 1.0.7`

### MIT License

Used by: `redox_syscall 0.5.18`

### MIT License

Used by: `stable_deref_trait 1.2.1`

### MIT License

Used by: `h2 0.4.20`

### MIT License

Used by: `http 1.5.0`

### MIT License

Used by: `tokio-rustls 0.26.6`

### MIT License

Used by: `core_detect 1.0.0`

### MIT License

Used by: `bytes 1.12.1`

### MIT License

Used by: `autocfg 1.5.1`

### MIT License

Used by: `fearless_simd 0.4.1`, `kurbo 0.13.1`, `peniko 0.6.1`, `polycool 0.4.0`

### MIT License

Used by: `smallvec 1.16.2`

### MIT License

Used by: `ahash 0.8.12`

### MIT License

Used by: `want 0.3.2`

### MIT License

Used by: `block-buffer 0.10.4`

### MIT License

Used by: `dirs-sys 0.5.0`, `dirs 7.0.0`

### MIT License

Used by: `signature 2.2.0`

### MIT License

Used by: `try-lock 0.2.5`

### MIT License

Used by: `rand_core 0.10.1`

### MIT License

Used by: `zeroize 1.9.1`

### MIT License

Used by: `getrandom 0.4.3`

### MIT License

Used by: `slab 0.4.12`

### MIT License

Used by: `font-types 0.12.6`, `read-fonts 0.41.0`, `skrifa 0.44.0`

### MIT License

Used by: `bumpalo 3.20.3`

### MIT License

Used by: `tracing-attributes 0.1.31`, `tracing-core 0.1.36`, `tracing 0.1.44`

### MIT License

Used by: `tower-layer 0.3.3`, `tower-service 0.3.3`, `tower 0.5.3`

### MIT License

Used by: `tower-http 0.6.11`

### MIT License

Used by: `http-body-util 0.1.5`, `http-body 1.1.0`

### MIT License

Used by: `chacha20 0.10.2`, `zeroize_derive 1.5.0`

### MIT License

Used by: `winsafe 0.0.29`

### MIT License

Used by: `iana-time-zone-haiku 0.1.2`, `iana-time-zone 0.1.65`

### MIT License

Used by: `tls_codec 0.4.2`, `tls_codec_derive 0.4.2`

### MIT License

Used by: `const-oid 0.9.6`, `der_derive 0.7.3`

### MIT License

Used by: `der 0.7.10`

### MIT License

Used by: `cpufeatures 0.2.17`

### MIT License

Used by: `cpufeatures 0.3.1`

### MIT License

Used by: `crypto-common 0.1.7`

### MIT License

Used by: `pem-rfc7468 0.7.0`, `x509-cert 0.2.5`

### MIT License

Used by: `spki 0.7.3`

### MIT License

Used by: `rustls-pki-types 1.15.1`

### MIT License

Used by: `cmpv2 0.2.0`, `crmf 0.2.0`, `x509-tsp 0.1.0`

### MIT License

Used by: `hyper-util 0.1.21`

### MIT License

Used by: `system-configuration-sys 0.6.0`, `system-configuration 0.7.0`

### MIT License

Used by: `env_filter 0.1.4`, `serde_spanned 1.1.2`, `toml_datetime 1.1.2+spec-1.1.0`, `toml_edit 0.25.17+spec-1.1.0`, `toml_parser 1.1.5+spec-1.1.0`, `toml_writer 1.1.3+spec-1.1.0`

### MIT License

Used by: `libc 0.2.190`

### MIT License

Used by: `idna_adapter 1.2.2`

### MIT License

Used by: `tar 0.4.46`

### MIT License

Used by: `arrayvec 0.7.8`

### MIT License

Used by: `synstructure 0.14.0`

### MIT License

Used by: `ipnet 2.12.2`

### MIT License

Used by: `rand 0.10.3`, `rand_core 0.6.4`

### MIT License

Used by: `nohash-hasher 0.2.0`

### MIT License

Used by: `vello_common 0.1.0`, `vello_cpu 0.1.0`

### MIT License

Used by: `zerocopy 0.8.62`

### MIT License

Used by: `encoding_rs 0.8.42`, `multiversion_no_op 1.0.0`, `utf8_iter 1.0.4`

### MIT License

Used by: `fs_extra 1.3.0`

### MIT License

Used by: `crc32fast 1.5.2`

### MIT License

Used by: `bytemuck 1.25.2`, `bytemuck_derive 1.12.1`

### MIT License

Used by: `strum 0.28.0`, `strum_macros 0.28.0`

### MIT License

Used by: `proc-macro-error-attr 1.0.4`, `proc-macro-error 1.0.4`

### MIT License

Used by: `android-properties 0.2.2`

### MIT License

Used by: `rustls-platform-verifier 0.7.1`

### MIT License

Used by: `libredox 0.1.25`

### MIT License

Used by: `craft-winprint-gdi 0.1.0`

### MIT License

Used by: `mahay-ai-core 0.1.0`, `mahay-android 0.1.0`, `mahay-atomic 0.1.0`, `mahay-cms 0.1.0`, `mahay-docpath 0.1.0`, `mahay-pdf 0.1.0`, `mahay-print 0.1.0`, `mahay-quit 0.1.0`, `mahay-settings 0.1.0`, `mahay-ui 0.1.0`, `accesskit 0.24.1`, `cesu8 1.1.0`, `chrono 0.4.45`, `cms 0.2.3`, `dispatch2 0.3.1`, `ecolor 0.36.2`, `egui 0.36.2`, `emath 0.36.2`, `epaint 0.36.2`, `fmt-derive-proc 0.1.2`, `fmt-derive 0.1.2`, `jni-macros 0.22.4`, `jni-sys-macros 0.4.1`, `jni 0.22.4`, `ndk-context 0.1.1`, `ndk-sys 0.6.0+11769913`, `ndk 0.9.0`, `objc2-app-kit 0.3.2`, `objc2-encode 4.1.0`, `objc2-foundation 0.3.2`, `objc2 0.6.5`, `profiling 1.0.18`, `r-efi 6.0.0`, `rustls-platform-verifier-android 0.2.0`, `windows-collections 0.3.2`, `windows-core 0.62.2`, `windows-future 0.3.2`, `windows-implement 0.60.2`, `windows-interface 0.59.3`, `windows-link 0.2.1`, `windows-numerics 0.3.1`, `windows-registry 0.6.1`, `windows-result 0.4.1`, `windows-strings 0.5.1`, `windows-sys 0.45.0`, `windows-sys 0.61.2`, `windows-targets 0.42.2`, `windows-threading 0.2.1`, `windows 0.62.2`, `windows_aarch64_gnullvm 0.42.2`, `windows_aarch64_msvc 0.42.2`, `windows_i686_gnu 0.42.2`, `windows_i686_msvc 0.42.2`, `windows_x86_64_gnu 0.42.2`, `windows_x86_64_gnullvm 0.42.2`, `windows_x86_64_msvc 0.42.2`

### MIT License

Used by: `tokio-util 0.7.20`, `tokio 1.53.2`

### MIT License

Used by: `simd-adler32 0.3.10`

### MIT License

Used by: `miniz_oxide 0.8.9`, `miniz_oxide 0.9.1`

### MIT License

Used by: `miniz_oxide 0.8.9`, `miniz_oxide 0.9.1`

### MIT License

Used by: `simdutf8 0.1.5`

### MIT License

Used by: `android-activity 0.6.1`, `color 0.3.3`, `linebender_resource_handle 0.1.1`

### MIT License

Used by: `serde_json_canonicalizer 0.3.2`

### MIT License

Used by: `adler2 2.0.1`, `atomic-waker 1.1.2`, `displaydoc 0.2.7`, `fastrand 2.5.0`, `itoa 1.0.18`, `linux-raw-sys 0.12.1`, `num_enum 0.7.6`, `num_enum_derive 0.7.6`, `once_cell 1.21.4`, `pin-project-lite 0.2.17`, `proc-macro-crate 3.5.0`, `proc-macro2 1.0.107`, `quote 1.0.47`, `rustix 1.1.5`, `rustversion 1.0.23`, `semver 1.0.28`, `serde 1.0.229`, `serde_core 1.0.229`, `serde_derive 1.0.229`, `serde_json 1.0.151`, `simd_cesu8 1.2.0`, `syn 1.0.109`, `syn 2.0.119`, `syn 3.0.6`, `thiserror-impl 1.0.69`, `thiserror-impl 2.0.21`, `thiserror 1.0.69`, `thiserror 2.0.21`, `unicode-ident 1.0.26`, `wasi 0.11.1+wasi-snapshot-preview1`, `zmij 1.0.23`

### MIT License

Used by: `winnow 1.0.4`

### MIT License

Used by: `android_system_properties 0.1.6`

### MIT License

Used by: `tracing-core 0.1.36`

### MIT License

Used by: `typenum 1.20.1`

### MIT License

Used by: `xml 1.4.0`

### MIT License

Used by: `base64 0.22.1`

### MIT License

Used by: `memchr 2.8.3`, `walkdir 2.5.0`

### MIT License

Used by: `combine 4.6.8`

### MIT License

Used by: `shlex 2.0.1`

### MIT License

Used by: `security-framework-sys 2.17.0`, `security-framework 3.7.0`

### MIT License

Used by: `aws-lc-sys 0.45.0`

### MIT License

Used by: `pem 3.0.6`

### MIT License

Used by: `jni 0.21.1`

### MIT License

Used by: `same-file 1.0.6`, `winapi-util 0.1.11`

### MIT License

Used by: `redox_users 0.5.3`

### MIT License

Used by: `roxmltree 0.20.0`

### MIT License

Used by: `guillotiere 0.7.0`

### MIT License

Used by: `crossbeam-deque 0.8.8`, `crossbeam-epoch 0.9.21`, `crossbeam-utils 0.8.23`

### MIT License

Used by: `base64 0.23.1`

### MIT License

Used by: `harfrust 0.12.0`

### MIT License

Used by: `version_check 0.9.5`

### MIT License

Used by: `generic-array 0.14.7`

### MIT No Attribution

Used by: `dunce 1.0.5`

### Mozilla Public License 2.0

Used by: `option-ext 0.2.0`

### Unicode License v3

Used by: `unicode-ident 1.0.26`

### Unicode License v3

Used by: `icu_collections 2.3.0`, `icu_locale_core 2.3.0`, `icu_normalizer 2.3.0`, `icu_normalizer_data 2.3.0`, `icu_properties 2.3.0`, `icu_properties_data 2.3.0`, `icu_provider 2.3.1`, `litemap 0.8.3`, `potential_utf 0.1.6`, `tinystr 0.8.4`, `writeable 0.6.4`, `yoke-derive 0.8.4`, `yoke 0.8.3`, `zerofrom-derive 0.1.8`, `zerofrom 0.1.8`, `zerotrie 0.2.5`, `zerovec-derive 0.11.6`, `zerovec 0.11.8`

### zlib License

Used by: `foldhash 0.2.0`


## `apps/hub`

- **MIT License** (514)
- **ISC License** (20)
- **Unicode License v3** (19)
- **Apache License 2.0** (17)
- **BSD 3-Clause "New" or "Revised" License** (3)
- **zlib License** (3)
- **Boost Software License 1.0** (2)
- **BSD 2-Clause "Simplified" License** (1)
- **Community Data License Agreement Permissive 2.0** (1)
- **MIT No Attribution** (1)
- **Mozilla Public License 2.0** (1)
- **SIL Open Font License 1.1** (1)
- **Ubuntu Font Licence v1.0** (1)
- **The Unlicense** (1)

### Apache License 2.0

Used by: `target-lexicon 0.12.16`

### Apache License 2.0

Used by: `ring 0.17.14`

### Apache License 2.0

Used by: `zopfli 0.8.3`

### Apache License 2.0

Used by: `moxcms 0.8.1`, `pxfm 0.1.30`

### Apache License 2.0

Used by: `codespan-reporting 0.13.1`, `self_cell 1.3.0`

### Apache License 2.0

Used by: `unicode-general-category 1.1.0`

### Apache License 2.0

Used by: `gethostname 1.1.0`, `ring 0.17.14`

### Apache License 2.0

Used by: `glutin_wgl_sys 0.6.1`

### Apache License 2.0

Used by: `dpi 0.1.2`, `winit 0.30.13`

### Apache License 2.0

Used by: `accesskit_winit 0.32.2`, `gl_generator 0.14.0`, `khronos_api 3.1.0`, `spirv 0.4.0+sdk-1.4.341.0`

### BSD 2-Clause "Simplified" License

Used by: `arrayref 0.3.9`

### BSD 3-Clause "New" or "Revised" License

Used by: `tiny-skia-path 0.11.4`, `tiny-skia 0.11.4`

### BSD 3-Clause "New" or "Revised" License

Used by: `subtle 2.6.1`

### Boost Software License 1.0

Used by: `error-code 3.4.0`

### Boost Software License 1.0

Used by: `clipboard-win 5.4.1`

### Community Data License Agreement Permissive 2.0

Used by: `webpki-root-certs 1.0.9`

### ISC License

Used by: `ring 0.17.14`

### ISC License

Used by: `ring 0.17.14`

### ISC License

Used by: `untrusted 0.9.0`

### ISC License

Used by: `ring 0.17.14`

### ISC License

Used by: `ring 0.17.14`

### ISC License

Used by: `ring 0.17.14`

### ISC License

Used by: `ring 0.17.14`

### ISC License

Used by: `ring 0.17.14`

### ISC License

Used by: `ring 0.17.14`

### ISC License

Used by: `ring 0.17.14`

### ISC License

Used by: `ring 0.17.14`

### ISC License

Used by: `ring 0.17.14`

### ISC License

Used by: `ring 0.17.14`

### ISC License

Used by: `ring 0.17.14`

### ISC License

Used by: `ring 0.17.14`

### ISC License

Used by: `ring 0.17.14`

### ISC License

Used by: `ring 0.17.14`

### ISC License

Used by: `libloading 0.7.4`, `libloading 0.8.9`

### ISC License

Used by: `rustls-webpki 0.103.15`

### MIT License

Used by: `uds_windows 1.2.1`

### MIT License

Used by: `cfg_aliases 0.2.2`

### MIT License

Used by: `atomic-waker 1.1.2`, `futures-lite 2.6.1`

### MIT License

Used by: `sha2 0.10.9`

### MIT License

Used by: `core-foundation-sys 0.8.7`, `core-foundation 0.10.1`, `core-foundation 0.9.4`, `core-graphics-types 0.1.3`, `core-graphics 0.23.2`, `euclid 0.22.14`

### MIT License

Used by: `hex 0.4.3`

### MIT License

Used by: `form_urlencoded 1.2.2`

### MIT License

Used by: `data-url 0.3.2`, `idna 1.1.0`, `percent-encoding 2.3.2`, `url 2.5.8`

### MIT License

Used by: `cc 1.6.0`, `cfg-if 1.0.5`, `filetime 0.2.29`, `find-msvc-tools 0.1.14`, `jobserver 0.1.35`, `js-sys 0.3.106`, `openssl-probe 0.2.1`, `pkg-config 0.3.34`, `scoped-tls 1.0.1`, `toml_datetime 0.6.3`, `wasm-bindgen-futures 0.4.79`, `wasm-bindgen-macro-support 0.2.129`, `wasm-bindgen-macro 0.2.129`, `wasm-bindgen-shared 0.2.129`, `wasm-bindgen 0.2.129`, `web-sys 0.3.106`

### MIT License

Used by: `errno 0.3.14`

### MIT License

Used by: `bitflags 1.3.2`, `bitflags 2.13.2`, `log 0.4.34`, `num-traits 0.2.19`

### MIT License

Used by: `uuid 1.28.0`

### MIT License

Used by: `float-cmp 0.9.0`

### MIT License

Used by: `flate2 1.1.10`

### MIT License

Used by: `either 1.19.0`, `itertools 0.15.0`

### MIT License

Used by: `plist 1.10.1`

### MIT License

Used by: `wayland-backend 0.3.17`, `wayland-client 0.31.15`, `wayland-cursor 0.31.14`, `wayland-protocols-experimental 20250721.0.1`, `wayland-protocols-misc 0.3.12`, `wayland-protocols-plasma 0.3.12`, `wayland-protocols-wlr 0.3.12`, `wayland-protocols 0.32.13`, `wayland-scanner 0.31.11`, `wayland-sys 0.31.11`

### MIT License

Used by: `winreg 0.55.0`

### MIT License

Used by: `ordered-float 5.5.0`

### MIT License

Used by: `tempfile 3.27.0`

### MIT License

Used by: `heck 0.4.1`, `heck 0.5.0`, `unicode-segmentation 1.13.3`, `unicode-width 0.2.2`

### MIT License

Used by: `quick-error 2.0.1`

### MIT License

Used by: `jni-sys 0.3.1`, `jni-sys 0.4.1`

### MIT License

Used by: `dlib 0.5.3`

### MIT License

Used by: `png 0.17.16`, `png 0.18.1`

### MIT License

Used by: `schannel 0.1.29`

### MIT License

Used by: `winapi 0.3.9`

### MIT License

Used by: `httparse 1.10.1`

### MIT License

Used by: `ash 0.38.0+1.3.281`

### MIT License

Used by: `futures-channel 0.3.34`, `futures-core 0.3.34`, `futures-executor 0.3.34`, `futures-io 0.3.34`, `futures-macro 0.3.34`, `futures-task 0.3.34`, `futures-util 0.3.34`

### MIT License

Used by: `hashbrown 0.16.1`, `hashbrown 0.17.1`

### MIT License

Used by: `utf8parse 0.2.2`

### MIT License

Used by: `rustls-native-certs 0.8.4`, `rustls 0.23.45`

### MIT License

Used by: `lock_api 0.4.14`, `parking_lot 0.12.5`, `parking_lot_core 0.9.12`, `rustc_version 0.4.1`

### MIT License

Used by: `android_log-sys 0.3.2`

### MIT License

Used by: `android_logger 0.15.1`

### MIT License

Used by: `indexmap 2.14.2`

### MIT License

Used by: `equivalent 1.0.2`

### MIT License

Used by: `scopeguard 1.2.0`

### MIT License

Used by: `digest 0.10.7`

### MIT License

Used by: `memoffset 0.9.1`

### MIT License

Used by: `enumflags2_derive 0.7.12`

### MIT License

Used by: `plain 0.2.3`

### MIT License

Used by: `keyboard-types 0.8.3`

### MIT License

Used by: `ron 0.12.2`

### MIT License

Used by: `redox_syscall 0.4.1`, `redox_syscall 0.5.18`, `redox_syscall 0.9.4`

### MIT License

Used by: `stable_deref_trait 1.2.1`

### MIT License

Used by: `foreign-types-macros 0.2.4`, `foreign-types-shared 0.3.1`, `foreign-types 0.5.0`

### MIT License

Used by: `version-compare 0.2.1`

### MIT License

Used by: `http 1.5.0`

### MIT License

Used by: `signal-hook-registry 1.4.8`

### MIT License

Used by: `enumflags2 0.7.12`

### MIT License

Used by: `bytes 1.12.1`

### MIT License

Used by: `autocfg 1.5.1`

### MIT License

Used by: `smithay-clipboard 0.7.3`

### MIT License

Used by: `fearless_simd 0.4.1`, `kurbo 0.11.3`, `kurbo 0.13.1`, `peniko 0.6.1`, `polycool 0.4.0`

### MIT License

Used by: `smallvec 1.16.2`

### MIT License

Used by: `ahash 0.8.12`

### MIT License

Used by: `smithay-client-toolkit 0.19.2`, `smithay-client-toolkit 0.20.0`

### MIT License

Used by: `calloop 0.13.0`, `calloop 0.14.5`

### MIT License

Used by: `svgtypes 0.15.3`

### MIT License

Used by: `block-buffer 0.10.4`

### MIT License

Used by: `dirs-sys 0.5.0`, `dirs 6.0.0`, `dirs 7.0.0`

### MIT License

Used by: `getrandom 0.2.17`

### MIT License

Used by: `getrandom 0.3.4`

### MIT License

Used by: `zeroize 1.9.1`

### MIT License

Used by: `getrandom 0.4.3`

### MIT License

Used by: `slab 0.4.12`

### MIT License

Used by: `cfg-expr 0.15.8`, `presser 0.3.1`

### MIT License

Used by: `font-types 0.12.6`, `read-fonts 0.41.0`, `skrifa 0.44.0`

### MIT License

Used by: `bumpalo 3.20.3`

### MIT License

Used by: `tracing-attributes 0.1.31`, `tracing-core 0.1.36`, `tracing 0.1.44`

### MIT License

Used by: `pico-args 0.5.0`

### MIT License

Used by: `downcast-rs 1.2.1`

### MIT License

Used by: `document-features 0.2.12`

### MIT License

Used by: `litrs 1.0.0`

### MIT License

Used by: `memmap2 0.9.11`

### MIT License

Used by: `pollster 0.4.0`, `pollster 1.0.1`

### MIT License

Used by: `cpufeatures 0.2.17`

### MIT License

Used by: `crypto-common 0.1.7`

### MIT License

Used by: `gpu-allocator 0.28.0`

### MIT License

Used by: `renderdoc-sys 1.1.0`

### MIT License

Used by: `atspi-common 0.13.0`, `atspi-proxies 0.13.0`, `atspi 0.29.0`

### MIT License

Used by: `strict-num 0.1.1`

### MIT License

Used by: `xkeysym 0.2.1`

### MIT License

Used by: `rustls-pki-types 1.15.1`

### MIT License

Used by: `powerfmt 0.2.1`

### MIT License

Used by: `calloop-wayland-source 0.3.0`, `calloop-wayland-source 0.4.1`

### MIT License

Used by: `cursor-icon 1.2.0`, `wayland-csd-frame 0.3.0`

### MIT License

Used by: `bit-vec 0.9.1`

### MIT License

Used by: `deranged 0.5.8`

### MIT License

Used by: `zbus 5.19.0`, `zbus_macros 5.19.0`, `zbus_names 4.3.4`, `zbus_xml 5.2.1`, `zvariant 5.15.0`, `zvariant_derive 5.15.0`

### MIT License

Used by: `bit-set 0.10.0`

### MIT License

Used by: `zcheapstr 1.1.0`

### MIT License

Used by: `anstream 1.0.0`, `anstyle-parse 1.0.0`, `anstyle-query 1.1.5`, `anstyle-wincon 3.0.11`, `anstyle 1.0.14`, `colorchoice 1.0.5`, `env_filter 0.1.4`, `env_filter 2.0.0`, `env_logger 0.11.11`, `is_terminal_polyfill 1.70.2`, `once_cell_polyfill 1.70.2`, `serde_spanned 0.6.9`, `serde_spanned 1.1.2`, `toml 0.8.2`, `toml 1.1.8+spec-1.1.0`, `toml_datetime 1.1.2+spec-1.1.0`, `toml_edit 0.19.15`, `toml_edit 0.20.2`, `toml_edit 0.25.17+spec-1.1.0`, `toml_parser 1.1.5+spec-1.1.0`

### MIT License

Used by: `num-conv 0.2.2`

### MIT License

Used by: `time-core 0.1.9`, `time 0.3.55`

### MIT License

Used by: `libc 0.2.190`

### MIT License

Used by: `idna_adapter 1.2.2`

### MIT License

Used by: `tar 0.4.46`

### MIT License

Used by: `arrayvec 0.7.8`

### MIT License

Used by: `winresource 0.1.31`

### MIT License

Used by: `synstructure 0.14.0`

### MIT License

Used by: `resvg 0.45.1`, `usvg 0.45.1`

### MIT License

Used by: `nohash-hasher 0.2.0`

### MIT License

Used by: `as-raw-xcb-connection 1.0.1`

### MIT License

Used by: `x11rb-protocol 0.13.2`, `x11rb 0.13.2`

### MIT License

Used by: `vello_common 0.1.0`, `vello_cpu 0.1.0`

### MIT License

Used by: `ureq-proto 0.6.4`

### MIT License

Used by: `zerocopy-derive 0.8.62`, `zerocopy 0.8.62`

### MIT License

Used by: `utf8_iter 1.0.4`

### MIT License

Used by: `wayland-protocols 0.32.13`

### MIT License

Used by: `fax 0.2.7`

### MIT License

Used by: `epaint_default_fonts 0.36.2`

### MIT License

Used by: `field-offset 0.3.6`

### MIT License

Used by: `notify-rust 4.18.2`

### MIT License

Used by: `static_assertions 1.1.0`

### MIT License

Used by: `libappindicator 0.9.0`

### MIT License

Used by: `tiff 0.11.3`

### MIT License

Used by: `crc32fast 1.5.2`

### MIT License

Used by: `bytemuck 1.25.2`, `bytemuck_derive 1.12.1`

### MIT License

Used by: `rgb 0.8.53`

### MIT License

Used by: `ureq 3.4.2`

### MIT License

Used by: `raw-window-handle 0.6.2`

### MIT License

Used by: `proc-macro-error-attr 1.0.4`, `proc-macro-error 1.0.4`

### MIT License

Used by: `rfd 0.17.2`

### MIT License

Used by: `cfg_aliases 0.2.2`

### MIT License

Used by: `android-properties 0.2.2`

### MIT License

Used by: `xcursor 0.3.11`

### MIT License

Used by: `sys-locale 0.3.2`

### MIT License

Used by: `rustls-platform-verifier 0.7.1`

### MIT License

Used by: `rfd 0.17.2`

### MIT License

Used by: `arboard 3.6.1`

### MIT License

Used by: `muda 0.20.0`, `tray-icon 0.25.1`

### MIT License

Used by: `libredox 0.1.25`

### MIT License

Used by: `range-alloc 0.1.5`

### MIT License

Used by: `web-time 1.1.0`

### MIT License

Used by: `task-local 0.1.1`

### MIT License

Used by: `naga-types 30.0.1`, `naga 30.0.1`, `wgpu-core-deps-apple 30.0.1`, `wgpu-core-deps-emscripten 30.0.1`, `wgpu-core-deps-wasm 30.0.1`, `wgpu-core-deps-windows-linux-android 30.0.1`, `wgpu-core 30.0.1`, `wgpu-hal 30.0.1`, `wgpu-naga-bridge 30.0.1`, `wgpu-types 30.0.1`, `wgpu 30.0.1`

### MIT License

Used by: `hub 0.5.1`, `hub-android 0.5.1`, `hub-web 0.5.1`, `hub-core 0.5.1`, `hub-native 0.5.1`, `hub-ui-egui 0.5.1`, `xtask 0.5.1`, `mahay-android 0.1.0`, `accesskit 0.24.1`, `accesskit_atspi_common 0.18.1`, `accesskit_consumer 0.35.0`, `accesskit_consumer 0.36.0`, `accesskit_consumer 0.38.0`, `accesskit_macos 0.26.3`, `accesskit_unix 0.21.1`, `accesskit_windows 0.32.1`, `block2 0.5.1`, `block2 0.6.2`, `cesu8 1.1.0`, `dispatch2 0.3.1`, `dispatch 0.2.0`, `dpi 0.1.2`, `ecolor 0.36.2`, `eframe 0.36.2`, `egui-wgpu 0.36.2`, `egui-winit 0.36.2`, `egui 0.36.2`, `egui_extras 0.36.2`, `emath 0.36.2`, `epaint 0.36.2`, `jni-macros 0.22.4`, `jni-sys-macros 0.4.1`, `jni 0.22.4`, `libappindicator-sys 0.9.0`, `libm 0.2.16`, `mac-notification-sys 0.6.15`, `ndk-context 0.1.1`, `ndk-sys 0.6.0+11769913`, `ndk 0.9.0`, `objc-sys 0.3.5`, `objc2-app-kit 0.2.2`, `objc2-app-kit 0.3.2`, `objc2-core-foundation 0.3.2`, `objc2-core-graphics 0.3.2`, `objc2-encode 4.1.0`, `objc2-foundation 0.2.2`, `objc2-foundation 0.3.2`, `objc2-metal 0.3.2`, `objc2-quartz-core 0.3.2`, `objc2-ui-kit 0.2.2`, `objc2-ui-kit 0.3.2`, `objc2 0.5.3`, `objc2 0.6.5`, `profiling 1.0.18`, `r-efi 5.3.0`, `r-efi 6.0.0`, `rustls-platform-verifier-android 0.2.0`, `tauri-winrt-notification 0.8.1`, `type-map 0.5.1`, `winapi-i686-pc-windows-gnu 0.4.0`, `winapi-x86_64-pc-windows-gnu 0.4.0`, `windows-collections 0.3.2`, `windows-core 0.62.2`, `windows-future 0.3.2`, `windows-implement 0.60.2`, `windows-interface 0.59.3`, `windows-link 0.2.1`, `windows-numerics 0.3.1`, `windows-result 0.4.1`, `windows-strings 0.5.1`, `windows-sys 0.45.0`, `windows-sys 0.52.0`, `windows-sys 0.59.0`, `windows-sys 0.61.2`, `windows-targets 0.42.2`, `windows-targets 0.52.6`, `windows-threading 0.2.1`, `windows-version 0.1.7`, `windows 0.62.2`, `windows_aarch64_gnullvm 0.42.2`, `windows_aarch64_gnullvm 0.52.6`, `windows_aarch64_msvc 0.42.2`, `windows_aarch64_msvc 0.52.6`, `windows_i686_gnu 0.42.2`, `windows_i686_gnu 0.52.6`, `windows_i686_gnullvm 0.52.6`, `windows_i686_msvc 0.42.2`, `windows_i686_msvc 0.52.6`, `windows_x86_64_gnu 0.42.2`, `windows_x86_64_gnu 0.52.6`, `windows_x86_64_gnullvm 0.42.2`, `windows_x86_64_gnullvm 0.52.6`, `windows_x86_64_msvc 0.42.2`, `windows_x86_64_msvc 0.52.6`

### MIT License

Used by: `enum-map-derive 0.17.0`, `enum-map 2.7.3`

### MIT License

Used by: `tokio 1.53.2`

### MIT License

Used by: `simd-adler32 0.3.10`

### MIT License

Used by: `zune-core 0.5.3`, `zune-jpeg 0.5.15`

### MIT License

Used by: `miniz_oxide 0.8.9`, `miniz_oxide 0.9.1`

### MIT License

Used by: `miniz_oxide 0.8.9`, `miniz_oxide 0.9.1`

### MIT License

Used by: `fdeflate 0.3.7`, `image 0.25.10`

### MIT License

Used by: `simdutf8 0.1.5`

### MIT License

Used by: `android-activity 0.6.1`, `color 0.3.3`, `half 2.7.1`, `linebender_resource_handle 0.1.1`

### MIT License

Used by: `imagesize 0.13.0`

### MIT License

Used by: `wayland-protocols-plasma 0.3.12`

### MIT License

Used by: `rustc-hash 2.1.3`

### MIT License

Used by: `adler2 2.0.1`, `async-channel 2.5.0`, `async-executor 1.14.0`, `async-io 2.6.0`, `async-lock 3.4.2`, `async-process 2.5.0`, `async-signal 0.2.14`, `async-task 4.7.1`, `async-trait 0.1.92`, `atomic-waker 1.1.2`, `blocking 1.7.0`, `concurrent-queue 2.5.0`, `displaydoc 0.2.7`, `endi 1.1.1`, `enumn 0.1.14`, `event-listener-strategy 0.5.4`, `event-listener 5.4.2`, `fastrand 2.5.0`, `futures-lite 2.6.1`, `hermit-abi 0.5.3`, `itoa 1.0.18`, `khronos-egl 6.0.0`, `linux-raw-sys 0.12.1`, `linux-raw-sys 0.4.15`, `num_enum 0.7.6`, `num_enum_derive 0.7.6`, `once_cell 1.21.4`, `ordered-stream 0.2.0`, `parking 2.2.1`, `pastey 0.2.3`, `pin-project-internal 1.1.13`, `pin-project-lite 0.2.17`, `pin-project 1.1.13`, `piper 0.2.5`, `polling 3.11.0`, `portable-atomic-util 0.2.8`, `portable-atomic 1.15.0`, `proc-macro-crate 1.3.1`, `proc-macro-crate 2.0.2`, `proc-macro-crate 3.5.0`, `proc-macro2 1.0.107`, `quote 1.0.47`, `rustc-hash 1.1.0`, `rustix 0.38.44`, `rustix 1.1.5`, `rustversion 1.0.23`, `semver 1.0.28`, `serde 1.0.229`, `serde_core 1.0.229`, `serde_derive 1.0.229`, `serde_json 1.0.151`, `serde_repr 0.1.21`, `simd_cesu8 1.2.0`, `smol_str 0.2.2`, `syn 1.0.109`, `syn 2.0.119`, `syn 3.0.6`, `system-deps 6.2.2`, `thiserror-impl 1.0.69`, `thiserror-impl 2.0.21`, `thiserror 1.0.69`, `thiserror 2.0.21`, `typed-path 0.12.3`, `typeid 1.0.3`, `unicode-ident 1.0.26`, `utf8-zero 0.8.1`, `wasi 0.11.1+wasi-snapshot-preview1`, `wasip2 1.0.4+wasi-0.2.12`, `wit-bindgen 0.57.1`, `x11-dl 2.21.0`, `x11 2.21.0`, `zbus-lockstep-macros 0.5.2`, `zbus-lockstep 0.5.2`, `zmij 1.0.23`, `zvariant_utils 4.2.0`

### MIT License

Used by: `async-recursion 1.2.0`, `glow 0.17.0`

### MIT License

Used by: `allocator-api2 0.2.21`

### MIT License

Used by: `winnow 0.5.40`, `winnow 1.0.4`

### MIT License

Used by: `raw-window-metal 1.1.0`, `siphasher 1.0.4`

### MIT License

Used by: `atk-sys 0.18.2`, `atk 0.18.2`, `cairo-rs 0.18.5`, `cairo-sys-rs 0.18.2`, `gdk-pixbuf-sys 0.18.0`, `gdk-pixbuf 0.18.5`, `gdk-sys 0.18.2`, `gdk 0.18.2`, `gio-sys 0.18.1`, `gio 0.18.4`, `glib-macros 0.18.5`, `glib-sys 0.18.1`, `glib 0.18.5`, `gobject-sys 0.18.0`, `gtk-sys 0.18.2`, `gtk3-macros 0.18.2`, `gtk 0.18.2`, `pango-sys 0.18.0`, `pango 0.18.3`

### MIT License

Used by: `android_system_properties 0.1.6`

### MIT License

Used by: `tracing-core 0.1.36`, `zip 8.6.0`

### MIT License

Used by: `typenum 1.20.1`

### MIT License

Used by: `xml-rs 0.8.29`

### MIT License

Used by: `phf 0.13.1`, `phf_generator 0.13.1`, `phf_macros 0.13.1`, `phf_shared 0.13.1`

### MIT License

Used by: `base64 0.22.1`

### MIT License

Used by: `byteorder-lite 0.1.0`, `jiff-core 0.1.1`, `jiff 0.2.38`, `memchr 2.8.3`, `walkdir 2.5.0`

### MIT License

Used by: `combine 4.6.8`

### MIT License

Used by: `libxdo-sys 0.11.0`, `libxdo 0.6.0`

### MIT License

Used by: `shlex 2.0.1`

### MIT License

Used by: `security-framework-sys 2.17.0`, `security-framework 3.7.0`

### MIT License

Used by: `orbclient 0.3.55`

### MIT License

Used by: `webbrowser 1.2.4`

### MIT License

Used by: `quick-xml 0.42.0`

### MIT License

Used by: `jni 0.21.1`

### MIT License

Used by: `same-file 1.0.6`, `winapi-util 0.1.11`

### MIT License

Used by: `redox_users 0.5.3`

### MIT License

Used by: `simplecss 0.2.2`

### MIT License

Used by: `roxmltree 0.20.0`

### MIT License

Used by: `guillotiere 0.7.0`

### MIT License

Used by: `xmlwriter 0.1.0`

### MIT License

Used by: `crossbeam-channel 0.5.17`, `crossbeam-utils 0.8.23`

### MIT License

Used by: `async-broadcast 0.7.2`

### MIT License

Used by: `xkbcommon-dl 0.4.2`

### MIT License

Used by: `base64 0.23.1`

### MIT License

Used by: `harfrust 0.12.0`

### MIT License

Used by: `weezl 0.1.12`

### MIT License

Used by: `crunchy 0.2.4`

### MIT License

Used by: `version_check 0.9.5`

### MIT License

Used by: `generic-array 0.14.7`

### MIT License

Used by: `quick-xml 0.41.0`

### MIT No Attribution

Used by: `dunce 1.0.5`

### Mozilla Public License 2.0

Used by: `option-ext 0.2.0`

### SIL Open Font License 1.1

Used by: `epaint_default_fonts 0.36.2`

### Ubuntu Font Licence v1.0

Used by: `epaint_default_fonts 0.36.2`

### Unicode License v3

Used by: `unicode-ident 1.0.26`

### Unicode License v3

Used by: `icu_collections 2.3.0`, `icu_locale_core 2.3.0`, `icu_normalizer 2.3.0`, `icu_normalizer_data 2.3.0`, `icu_properties 2.3.0`, `icu_properties_data 2.3.0`, `icu_provider 2.3.1`, `litemap 0.8.3`, `potential_utf 0.1.6`, `tinystr 0.8.4`, `writeable 0.6.4`, `yoke-derive 0.8.4`, `yoke 0.8.3`, `zerofrom-derive 0.1.8`, `zerofrom 0.1.8`, `zerotrie 0.2.5`, `zerovec-derive 0.11.6`, `zerovec 0.11.8`

### The Unlicense

Used by: `ksni 0.3.6`

### zlib License

Used by: `zlib-rs 0.6.8`

### zlib License

Used by: `slotmap 1.1.1`

### zlib License

Used by: `foldhash 0.2.0`


## `apps/sary`

- **MIT License** (687)
- **Apache License 2.0** (34)
- **Unicode License v3** (23)
- **BSD 3-Clause "New" or "Revised" License** (12)
- **ISC License** (6)
- **zlib License** (3)
- **BSD 2-Clause "Simplified" License** (2)
- **Boost Software License 1.0** (2)
- **MIT No Attribution** (2)
- **Community Data License Agreement Permissive 2.0** (1)
- **Independent JPEG Group License** (1)
- **SIL Open Font License 1.1** (1)
- **Ubuntu Font Licence v1.0** (1)

### Apache License 2.0

Used by: `clang-sys 1.9.1`, `flagset 0.4.7`

### Apache License 2.0

Used by: `target-lexicon 0.13.5`, `winx 0.36.4`

### Apache License 2.0

Used by: `moxcms 0.8.1`, `pxfm 0.1.30`

### Apache License 2.0

Used by: `xcap 0.9.8`

### Apache License 2.0

Used by: `codespan-reporting 0.13.1`, `self_cell 1.3.0`

### Apache License 2.0

Used by: `unicode-general-category 1.1.0`

### Apache License 2.0

Used by: `gethostname 1.1.0`

### Apache License 2.0

Used by: `glutin_wgl_sys 0.6.1`

### Apache License 2.0

Used by: `dpi 0.1.2`, `winit 0.30.13`

### Apache License 2.0

Used by: `accesskit_winit 0.32.2`, `aws-lc-sys 0.46.0`, `blake3 1.8.7`, `gl 0.14.0`, `gl_generator 0.14.0`, `khronos_api 3.1.0`, `rmcp-macros 3.5.0`, `rmcp 3.5.0`, `ryu-js 1.0.3`, `ryu 1.0.23`, `sigstore-bundle 0.6.6`, `sigstore-crypto 0.6.6`, `sigstore-merkle 0.6.6`, `sigstore-rekor 0.6.6`, `sigstore-trust-root 0.6.6`, `sigstore-tsa 0.6.6`, `sigstore-types 0.6.6`, `sigstore-verify 0.6.6`, `spirv 0.4.0+sdk-1.4.341.0`, `sync_wrapper 1.0.2`

### BSD 2-Clause "Simplified" License

Used by: `arrayref 0.3.9`

### BSD 2-Clause "Simplified" License

Used by: `libwayshot-xcap 0.3.3`

### BSD 3-Clause "New" or "Revised" License

Used by: `encoding_rs 0.8.42`

### BSD 3-Clause "New" or "Revised" License

Used by: `bindgen 0.72.1`

### BSD 3-Clause "New" or "Revised" License

Used by: `winprint 0.2.1`

### BSD 3-Clause "New" or "Revised" License

Used by: `tiny-skia-path 0.11.4`, `tiny-skia 0.11.4`

### BSD 3-Clause "New" or "Revised" License

Used by: `alloc-no-stdlib 2.0.4`

### BSD 3-Clause "New" or "Revised" License

Used by: `subtle 2.6.1`

### BSD 3-Clause "New" or "Revised" License

Used by: `lebe 0.5.3`

### BSD 3-Clause "New" or "Revised" License

Used by: `alloc-stdlib 0.2.4`, `aws-lc-sys 0.46.0`, `exr 1.74.2`

### BSD 3-Clause "New" or "Revised" License

Used by: `encoding_rs 0.8.42`

### Boost Software License 1.0

Used by: `error-code 3.4.0`

### Boost Software License 1.0

Used by: `clipboard-win 5.4.1`

### Community Data License Agreement Permissive 2.0

Used by: `webpki-root-certs 1.0.9`

### Independent JPEG Group License

Used by: `jpeg-encoder 0.6.1`

### ISC License

Used by: `untrusted 0.7.1`, `untrusted 0.9.0`

### ISC License

Used by: `libloading 0.8.9`

### ISC License

Used by: `rustls-webpki 0.103.15`

### ISC License

Used by: `aws-lc-rs 1.18.2`, `aws-lc-sys 0.46.0`

### MIT License

Used by: `rustybuzz 0.20.1`

### MIT License

Used by: `uds_windows 1.2.1`

### MIT License

Used by: `cfg_aliases 0.2.2`

### MIT License

Used by: `schemars 1.2.2`

### MIT License

Used by: `cexpr 0.6.0`

### MIT License

Used by: `qoi 0.4.1`

### MIT License

Used by: `xcb 1.7.1`

### MIT License

Used by: `atomic-waker 1.1.2`, `futures-lite 2.6.1`

### MIT License

Used by: `sha1 0.10.7`, `sha2 0.10.9`

### MIT License

Used by: `rayon-core 1.13.0`, `rayon 1.12.0`

### MIT License

Used by: `core-foundation-sys 0.8.7`, `core-foundation 0.10.1`, `core-foundation 0.9.4`, `core-graphics-types 0.1.3`, `core-graphics 0.23.2`, `euclid 0.22.14`

### MIT License

Used by: `xcb 1.7.1`

### MIT License

Used by: `hex 0.4.3`

### MIT License

Used by: `form_urlencoded 1.2.2`

### MIT License

Used by: `data-url 0.3.2`, `idna 1.1.0`, `percent-encoding 2.3.2`, `url 2.5.8`

### MIT License

Used by: `cc 1.5.1`, `cfg-if 1.0.5`, `cmake 0.1.58`, `filetime 0.2.29`, `find-msvc-tools 0.1.14`, `jobserver 0.1.35`, `js-sys 0.3.106`, `openssl-probe 0.2.1`, `pkg-config 0.3.34`, `scoped-tls 1.0.1`, `socket2 0.6.5`, `wasm-bindgen-futures 0.4.79`, `wasm-bindgen-macro-support 0.2.129`, `wasm-bindgen-macro 0.2.129`, `wasm-bindgen-shared 0.2.129`, `wasm-bindgen 0.2.129`, `web-sys 0.3.106`

### MIT License

Used by: `mio 1.2.3`

### MIT License

Used by: `errno 0.3.14`

### MIT License

Used by: `primal-check 0.3.4`

### MIT License

Used by: `mime 0.3.17`

### MIT License

Used by: `base64ct 1.8.3`

### MIT License

Used by: `bitflags 1.3.2`, `bitflags 2.13.2`, `glob 0.3.4`, `log 0.4.34`, `num-complex 0.4.6`, `num-integer 0.1.47`, `num-traits 0.2.19`, `regex-automata 0.4.18`, `regex-syntax 0.8.11`, `regex 1.13.1`

### MIT License

Used by: `uuid 1.26.1`

### MIT License

Used by: `nom 7.1.3`, `nom 8.0.0`

### MIT License

Used by: `float-cmp 0.9.0`

### MIT License

Used by: `flate2 1.1.10`

### MIT License

Used by: `hyper 1.12.0`

### MIT License

Used by: `either 1.18.0`, `itertools 0.10.5`, `itertools 0.15.0`, `petgraph 0.8.3`

### MIT License

Used by: `wayland-backend 0.3.17`, `wayland-client 0.31.15`, `wayland-cursor 0.31.14`, `wayland-protocols-experimental 20250721.0.1`, `wayland-protocols-misc 0.3.12`, `wayland-protocols-plasma 0.3.12`, `wayland-protocols-wlr 0.3.12`, `wayland-protocols 0.32.13`, `wayland-scanner 0.31.11`, `wayland-server 0.31.14`, `wayland-sys 0.31.11`

### MIT License

Used by: `ordered-float 5.5.0`

### MIT License

Used by: `tempfile 3.27.0`, `xattr 1.6.1`

### MIT License

Used by: `heck 0.5.0`, `unicode-bidi 0.3.18`, `unicode-properties 0.1.4`, `unicode-segmentation 1.13.3`, `unicode-width 0.2.2`

### MIT License

Used by: `rustfft 6.4.1`, `strength_reduce 0.2.4`

### MIT License

Used by: `quick-error 2.0.1`

### MIT License

Used by: `jni-sys 0.3.1`, `jni-sys 0.4.1`

### MIT License

Used by: `dlib 0.5.3`

### MIT License

Used by: `png 0.17.16`, `png 0.18.1`

### MIT License

Used by: `schannel 0.1.29`

### MIT License

Used by: `fixedbitset 0.5.7`

### MIT License

Used by: `httparse 1.10.1`

### MIT License

Used by: `ash 0.38.0+1.3.281`

### MIT License

Used by: `futures-channel 0.3.34`, `futures-core 0.3.34`, `futures-executor 0.3.34`, `futures-io 0.3.34`, `futures-macro 0.3.34`, `futures-sink 0.3.34`, `futures-task 0.3.34`, `futures-util 0.3.34`, `futures 0.3.34`

### MIT License

Used by: `hashbrown 0.15.5`, `hashbrown 0.16.1`, `hashbrown 0.17.1`

### MIT License

Used by: `serde_urlencoded 0.7.1`

### MIT License

Used by: `hyper-rustls 0.27.10`, `rustls-native-certs 0.8.4`, `rustls 0.23.45`

### MIT License

Used by: `maybe-owned 0.3.4`

### MIT License

Used by: `lock_api 0.4.14`, `parking_lot 0.12.5`, `parking_lot_core 0.9.12`, `rustc_version 0.4.1`

### MIT License

Used by: `android_log-sys 0.3.2`

### MIT License

Used by: `android_logger 0.15.1`

### MIT License

Used by: `indexmap 2.14.2`

### MIT License

Used by: `equivalent 1.0.2`

### MIT License

Used by: `scopeguard 1.2.0`

### MIT License

Used by: `reqwest 0.13.5`

### MIT License

Used by: `digest 0.10.7`

### MIT License

Used by: `fnv 1.0.7`

### MIT License

Used by: `memoffset 0.9.1`

### MIT License

Used by: `enumflags2_derive 0.7.12`

### MIT License

Used by: `plain 0.2.3`

### MIT License

Used by: `keyboard-types 0.8.3`

### MIT License

Used by: `ron 0.12.2`

### MIT License

Used by: `redox_syscall 0.4.1`, `redox_syscall 0.5.18`, `redox_syscall 0.9.4`

### MIT License

Used by: `stable_deref_trait 1.2.1`

### MIT License

Used by: `foreign-types-macros 0.2.4`, `foreign-types-shared 0.3.1`, `foreign-types 0.5.0`

### MIT License

Used by: `version-compare 0.2.1`

### MIT License

Used by: `h2 0.4.20`

### MIT License

Used by: `http 1.5.0`

### MIT License

Used by: `tokio-rustls 0.26.6`

### MIT License

Used by: `signal-hook-registry 1.4.8`

### MIT License

Used by: `core_detect 1.0.0`

### MIT License

Used by: `enumflags2 0.7.12`

### MIT License

Used by: `bytes 1.12.1`

### MIT License

Used by: `autocfg 1.5.1`

### MIT License

Used by: `smithay-clipboard 0.7.3`

### MIT License

Used by: `fearless_simd 0.4.1`, `kurbo 0.11.3`, `kurbo 0.13.1`, `peniko 0.6.1`, `polycool 0.4.0`

### MIT License

Used by: `smallvec 1.16.2`

### MIT License

Used by: `ahash 0.8.12`

### MIT License

Used by: `smithay-client-toolkit 0.19.2`, `smithay-client-toolkit 0.20.0`

### MIT License

Used by: `calloop 0.13.0`, `calloop 0.14.4`

### MIT License

Used by: `svgtypes 0.15.3`, `ttf-parser 0.25.1`

### MIT License

Used by: `want 0.3.2`

### MIT License

Used by: `block-buffer 0.10.4`

### MIT License

Used by: `signature 2.2.0`

### MIT License

Used by: `try-lock 0.2.5`

### MIT License

Used by: `getrandom 0.3.4`

### MIT License

Used by: `rand_core 0.10.1`

### MIT License

Used by: `zeroize 1.9.1`

### MIT License

Used by: `getrandom 0.4.3`

### MIT License

Used by: `slab 0.4.12`

### MIT License

Used by: `cfg-expr 0.20.10`, `presser 0.3.1`

### MIT License

Used by: `font-types 0.12.5`, `read-fonts 0.41.0`, `skrifa 0.44.0`

### MIT License

Used by: `wl-clipboard-rs 0.9.4`

### MIT License

Used by: `bumpalo 3.20.3`

### MIT License

Used by: `tracing-attributes 0.1.31`, `tracing-core 0.1.36`, `tracing 0.1.44`

### MIT License

Used by: `tower-layer 0.3.3`, `tower-service 0.3.3`, `tower 0.5.3`

### MIT License

Used by: `pico-args 0.5.0`

### MIT License

Used by: `tower-http 0.6.11`

### MIT License

Used by: `http-body-util 0.1.5`, `http-body 1.1.0`

### MIT License

Used by: `chacha20 0.10.2`, `zeroize_derive 1.5.0`

### MIT License

Used by: `winsafe 0.0.29`

### MIT License

Used by: `iana-time-zone-haiku 0.1.2`, `iana-time-zone 0.1.65`

### MIT License

Used by: `downcast-rs 1.2.1`

### MIT License

Used by: `document-features 0.2.12`

### MIT License

Used by: `litrs 1.0.0`

### MIT License

Used by: `tls_codec 0.4.2`, `tls_codec_derive 0.4.2`

### MIT License

Used by: `memmap2 0.9.11`

### MIT License

Used by: `pollster 0.4.0`, `pollster 1.0.1`

### MIT License

Used by: `const-oid 0.9.6`, `der_derive 0.7.3`

### MIT License

Used by: `der 0.7.10`

### MIT License

Used by: `cpufeatures 0.2.17`

### MIT License

Used by: `cpufeatures 0.3.1`

### MIT License

Used by: `crypto-common 0.1.7`

### MIT License

Used by: `pem-rfc7468 0.7.0`, `x509-cert 0.2.5`

### MIT License

Used by: `gpu-allocator 0.28.0`

### MIT License

Used by: `jpeg-encoder 0.6.1`

### MIT License

Used by: `spki 0.7.3`

### MIT License

Used by: `memory-stats 1.2.0`

### MIT License

Used by: `renderdoc-sys 1.1.0`

### MIT License

Used by: `qoi 0.4.1`

### MIT License

Used by: `atspi-common 0.13.0`, `atspi-proxies 0.13.0`, `atspi 0.29.0`

### MIT License

Used by: `transpose 0.2.3`

### MIT License

Used by: `strict-num 0.1.1`

### MIT License

Used by: `xkeysym 0.2.1`

### MIT License

Used by: `rustls-pki-types 1.15.1`

### MIT License

Used by: `calloop-wayland-source 0.3.0`, `calloop-wayland-source 0.4.1`

### MIT License

Used by: `cursor-icon 1.2.0`, `wayland-csd-frame 0.3.0`

### MIT License

Used by: `bit-vec 0.9.1`

### MIT License

Used by: `cmpv2 0.2.0`, `crmf 0.2.0`, `x509-tsp 0.1.0`

### MIT License

Used by: `hyper-util 0.1.21`

### MIT License

Used by: `system-configuration-sys 0.6.0`, `system-configuration 0.7.0`

### MIT License

Used by: `zbus 5.19.0`, `zbus_macros 5.19.0`, `zbus_names 4.3.4`, `zbus_xml 5.2.1`, `zvariant 5.15.0`, `zvariant_derive 5.15.0`

### MIT License

Used by: `core-graphics2 0.6.1`

### MIT License

Used by: `bit-set 0.10.0`

### MIT License

Used by: `zcheapstr 1.1.0`

### MIT License

Used by: `annotate-snippets 0.11.5`, `anstyle 1.0.14`, `env_filter 0.1.4`, `serde_spanned 1.1.1`, `toml 1.1.6+spec-1.1.0`, `toml_datetime 1.1.1+spec-1.1.0`, `toml_edit 0.25.15+spec-1.1.0`, `toml_parser 1.1.3+spec-1.1.0`, `toml_writer 1.1.2+spec-1.1.0`

### MIT License

Used by: `libc 0.2.189`

### MIT License

Used by: `idna_adapter 1.2.2`

### MIT License

Used by: `tar 0.4.46`

### MIT License

Used by: `arrayvec 0.7.8`

### MIT License

Used by: `winresource 0.1.31`

### MIT License

Used by: `synstructure 0.14.0`

### MIT License

Used by: `ipnet 2.12.2`

### MIT License

Used by: `resvg 0.45.1`, `usvg 0.45.1`

### MIT License

Used by: `rand 0.10.3`, `rand_core 0.6.4`

### MIT License

Used by: `nohash-hasher 0.2.0`

### MIT License

Used by: `as-raw-xcb-connection 1.0.1`

### MIT License

Used by: `x11rb-protocol 0.13.2`, `x11rb 0.13.2`

### MIT License

Used by: `parlance 0.1.0`, `parley 0.11.1`, `parley_data 0.11.1`

### MIT License

Used by: `vello_common 0.1.0`, `vello_cpu 0.1.0`

### MIT License

Used by: `zerocopy-derive 0.8.59`, `zerocopy 0.8.59`

### MIT License

Used by: `fontique 0.11.1`

### MIT License

Used by: `encoding_rs 0.8.42`, `multiversion_no_op 1.0.0`, `utf8_iter 1.0.4`

### MIT License

Used by: `libspa-sys 0.10.1`, `libspa 0.10.1`, `pipewire-sys 0.10.1`, `pipewire 0.10.1`

### MIT License

Used by: `wayland-protocols 0.32.13`

### MIT License

Used by: `epaint_default_fonts 0.36.2`

### MIT License

Used by: `tree_magic_mini 3.2.2`

### MIT License

Used by: `fs_extra 1.3.0`

### MIT License

Used by: `static_assertions 1.1.0`

### MIT License

Used by: `darling 0.24.1`, `darling_core 0.24.1`, `darling_macro 0.24.1`

### MIT License

Used by: `gbm 0.18.0`

### MIT License

Used by: `tiff 0.11.3`

### MIT License

Used by: `crc32fast 1.5.2`

### MIT License

Used by: `bytemuck 1.25.2`, `bytemuck_derive 1.12.1`

### MIT License

Used by: `schemars 1.2.2`, `schemars_derive 1.2.2`

### MIT License

Used by: `rgb 0.8.53`

### MIT License

Used by: `ruzstd 0.9.1`

### MIT License

Used by: `raw-window-handle 0.6.2`

### MIT License

Used by: `strum 0.28.0`, `strum_macros 0.28.0`

### MIT License

Used by: `tokio-macros 2.7.2`

### MIT License

Used by: `proc-macro-error-attr 1.0.4`, `proc-macro-error 1.0.4`

### MIT License

Used by: `rfd 0.17.2`

### MIT License

Used by: `cfg_aliases 0.2.2`

### MIT License

Used by: `android-properties 0.2.2`

### MIT License

Used by: `xcursor 0.3.11`

### MIT License

Used by: `sys-locale 0.3.2`

### MIT License

Used by: `pulp 0.22.3`

### MIT License

Used by: `rustls-platform-verifier 0.7.1`

### MIT License

Used by: `rfd 0.17.2`

### MIT License

Used by: `arboard 3.6.1`

### MIT License

Used by: `reborrow 0.5.5`

### MIT License

Used by: `muda 0.21.1`

### MIT License

Used by: `libredox 0.1.25`

### MIT License

Used by: `is-docker 0.2.0`, `is-wsl 0.4.0`

### MIT License

Used by: `range-alloc 0.1.5`

### MIT License

Used by: `web-time 1.1.0`

### MIT License

Used by: `core_maths 0.1.1`

### MIT License

Used by: `naga-types 30.0.1`, `naga 30.0.1`, `wgpu-core-deps-apple 30.0.1`, `wgpu-core-deps-emscripten 30.0.1`, `wgpu-core-deps-wasm 30.0.1`, `wgpu-core-deps-windows-linux-android 30.0.1`, `wgpu-core 30.0.1`, `wgpu-hal 30.0.1`, `wgpu-naga-bridge 30.0.1`, `wgpu-types 30.0.1`, `wgpu 30.0.1`

### MIT License

Used by: `craft-winprint-gdi 0.1.0`

### MIT License

Used by: `fmv-macos-events 0.1.0`

### MIT License

Used by: `heic-rs 0.1.4`

### MIT License

Used by: `sary 0.10.0`, `sary-android 0.10.0`, `sary-cli 0.10.0`, `sary-web 0.10.0`, `sary-web-boot 0.10.0`, `sary-affinity 0.10.0`, `sary-algo 0.10.0`, `sary-automation 0.10.0`, `sary-cms 0.10.0`, `sary-codecs 0.10.0`, `sary-color 0.10.0`, `sary-compose 0.10.0`, `sary-doc 0.10.0`, `sary-engine 0.10.0`, `sary-format 0.10.0`, `sary-geom 0.10.0`, `sary-gpu 0.10.0`, `sary-heif 0.10.0`, `sary-io 0.10.0`, `sary-ipp 0.10.0`, `sary-ops 0.10.0`, `sary-paint 0.10.0`, `sary-plugins 0.10.0`, `sary-psd 0.10.0`, `sary-raster 0.10.0`, `sary-raw 0.10.0`, `sary-tablet 0.10.0`, `sary-testkit 0.10.0`, `sary-text 0.10.0`, `sary-vector 0.10.0`, `xtask 0.10.0`, `mahay-android 0.1.0`, `mahay-docpath 0.1.0`, `mahay-print 0.1.0`, `mahay-ui 0.1.0`, `accesskit 0.24.1`, `accesskit_atspi_common 0.18.1`, `accesskit_consumer 0.35.0`, `accesskit_consumer 0.36.0`, `accesskit_consumer 0.38.0`, `accesskit_macos 0.26.3`, `accesskit_unix 0.21.1`, `accesskit_windows 0.32.1`, `block2 0.5.1`, `block2 0.6.2`, `block 0.1.6`, `brotli-decompressor 5.0.3`, `cesu8 1.1.0`, `chrono 0.4.45`, `cms 0.2.3`, `cookie-factory 0.3.3`, `dispatch2 0.3.1`, `dispatch 0.2.0`, `dpi 0.1.2`, `drm-fourcc 2.2.0`, `ecolor 0.36.2`, `eframe 0.36.2`, `egui-wgpu 0.36.2`, `egui-winit 0.36.2`, `egui 0.36.2`, `egui_extras 0.36.2`, `emath 0.36.2`, `epaint 0.36.2`, `fax 0.2.6`, `fax_derive 0.2.0`, `fmt-derive-proc 0.1.2`, `fmt-derive 0.1.2`, `gbm-sys 0.4.0`, `jni-macros 0.22.4`, `jni-sys-macros 0.4.1`, `jni 0.22.4`, `libm 0.2.16`, `ndk-context 0.1.1`, `ndk-sys 0.6.0+11769913`, `ndk 0.9.0`, `objc-sys 0.3.5`, `objc2-app-kit 0.2.2`, `objc2-app-kit 0.3.2`, `objc2-av-foundation 0.3.2`, `objc2-avf-audio 0.3.2`, `objc2-cloud-kit 0.3.2`, `objc2-core-audio-types 0.3.2`, `objc2-core-audio 0.3.2`, `objc2-core-data 0.3.2`, `objc2-core-foundation 0.3.2`, `objc2-core-graphics 0.3.2`, `objc2-core-image 0.3.2`, `objc2-core-media 0.3.2`, `objc2-core-text 0.3.2`, `objc2-core-video 0.3.2`, `objc2-encode 4.1.0`, `objc2-foundation 0.2.2`, `objc2-foundation 0.3.2`, `objc2-image-io 0.3.2`, `objc2-media-toolbox 0.3.2`, `objc2-metal 0.3.2`, `objc2-quartz-core 0.3.2`, `objc2-ui-kit 0.2.2`, `objc2-ui-kit 0.3.2`, `objc2 0.5.2`, `objc2 0.6.4`, `profiling 1.0.18`, `pulp-wasm-simd-flag 0.1.1`, `r-efi 5.3.0`, `r-efi 6.0.0`, `rustls-platform-verifier-android 0.2.0`, `type-map 0.5.1`, `wasmi 2.0.0`, `wasmi_collections 2.0.0`, `wasmi_core 2.0.0`, `wasmi_ir 2.0.0`, `wasmparser 0.228.0`, `windows-collections 0.3.2`, `windows-core 0.62.2`, `windows-future 0.3.2`, `windows-implement 0.60.2`, `windows-interface 0.59.3`, `windows-link 0.2.1`, `windows-numerics 0.3.1`, `windows-registry 0.6.1`, `windows-result 0.4.1`, `windows-strings 0.5.1`, `windows-sys 0.45.0`, `windows-sys 0.52.0`, `windows-sys 0.59.0`, `windows-sys 0.60.2`, `windows-sys 0.61.2`, `windows-targets 0.42.2`, `windows-targets 0.52.6`, `windows-targets 0.53.5`, `windows-threading 0.2.1`, `windows 0.62.2`, `windows_aarch64_gnullvm 0.42.2`, `windows_aarch64_gnullvm 0.52.6`, `windows_aarch64_gnullvm 0.53.1`, `windows_aarch64_msvc 0.42.2`, `windows_aarch64_msvc 0.52.6`, `windows_aarch64_msvc 0.53.1`, `windows_i686_gnu 0.42.2`, `windows_i686_gnu 0.52.6`, `windows_i686_gnu 0.53.1`, `windows_i686_gnullvm 0.52.6`, `windows_i686_gnullvm 0.53.1`, `windows_i686_msvc 0.42.2`, `windows_i686_msvc 0.52.6`, `windows_i686_msvc 0.53.1`, `windows_x86_64_gnu 0.42.2`, `windows_x86_64_gnu 0.52.6`, `windows_x86_64_gnu 0.53.1`, `windows_x86_64_gnullvm 0.42.2`, `windows_x86_64_gnullvm 0.52.6`, `windows_x86_64_gnullvm 0.53.1`, `windows_x86_64_msvc 0.42.2`, `windows_x86_64_msvc 0.52.6`, `windows_x86_64_msvc 0.53.1`, `zune-core 0.4.12`, `zune-inflate 0.2.54`, `zune-jpeg 0.4.21`

### MIT License

Used by: `enum-map-derive 0.17.0`, `enum-map 2.7.3`

### MIT License

Used by: `tokio-util 0.7.19`, `tokio 1.53.1`

### MIT License

Used by: `simd-adler32 0.3.10`

### MIT License

Used by: `zune-core 0.5.3`, `zune-jpeg 0.5.15`

### MIT License

Used by: `miniz_oxide 0.8.9`, `miniz_oxide 0.9.1`

### MIT License

Used by: `miniz_oxide 0.8.9`, `miniz_oxide 0.9.1`

### MIT License

Used by: `fdeflate 0.3.7`, `image-webp 0.2.4`, `image 0.25.10`

### MIT License

Used by: `simdutf8 0.1.5`

### MIT License

Used by: `android-activity 0.6.1`, `color 0.3.3`, `half 2.7.1`, `ident_case 1.0.1`, `linebender_resource_handle 0.1.1`, `widestring 1.2.1`

### MIT License

Used by: `imagesize 0.13.0`

### MIT License

Used by: `unicode-script 0.5.8`

### MIT License

Used by: `fontconfig-parser 0.5.8`

### MIT License

Used by: `serde_json_canonicalizer 0.3.2`

### MIT License

Used by: `wayland-protocols-plasma 0.3.12`

### MIT License

Used by: `async-recursion 1.1.1`, `rustc-hash 2.1.3`

### MIT License

Used by: `adler2 2.0.1`, `ambient-authority 0.0.2`, `async-channel 2.5.0`, `async-executor 1.14.0`, `async-io 2.6.0`, `async-lock 3.4.2`, `async-process 2.5.0`, `async-signal 0.2.14`, `async-task 4.7.1`, `async-trait 0.1.92`, `atomic-waker 1.1.2`, `blocking 1.7.0`, `cap-primitives 4.0.3`, `cap-std 4.0.3`, `concurrent-queue 2.5.0`, `displaydoc 0.2.7`, `drm-ffi 0.9.1`, `drm-sys 0.8.1`, `drm 0.14.1`, `drm 0.15.0`, `dyn-clone 1.0.20`, `endi 1.1.1`, `enumn 0.1.14`, `event-listener-strategy 0.5.4`, `event-listener 5.4.2`, `fastrand 2.5.0`, `fs-set-times 0.20.3`, `futures-lite 2.6.1`, `hermit-abi 0.5.3`, `io-extras 0.19.0`, `io-lifetimes 2.0.4`, `io-lifetimes 3.0.1`, `itoa 1.0.18`, `jxl-bitstream 1.1.0`, `jxl-coding 1.0.1`, `jxl-color 0.11.0`, `jxl-frame 0.13.3`, `jxl-grid 0.6.2`, `jxl-image 0.13.0`, `jxl-jbr 0.2.1`, `jxl-modular 0.11.3`, `jxl-oxide-common 1.0.0`, `jxl-oxide 0.12.6`, `jxl-render 0.12.4`, `jxl-threadpool 1.0.0`, `jxl-vardct 0.11.1`, `khronos-egl 6.0.0`, `linux-raw-sys 0.12.1`, `linux-raw-sys 0.4.15`, `linux-raw-sys 0.9.4`, `minimal-lexical 0.2.1`, `num_enum 0.7.6`, `num_enum_derive 0.7.6`, `once_cell 1.21.4`, `ordered-stream 0.2.0`, `parking 2.2.1`, `paste 1.0.15`, `pastey 0.2.3`, `pin-project-internal 1.1.13`, `pin-project-lite 0.2.17`, `pin-project 1.1.13`, `piper 0.2.5`, `polling 3.11.0`, `portable-atomic-util 0.2.8`, `portable-atomic 1.15.0`, `proc-macro-crate 3.5.0`, `proc-macro2 1.0.107`, `quote 1.0.47`, `ref-cast-impl 1.0.27`, `ref-cast 1.0.27`, `rustc-hash 1.1.0`, `rustix-linux-procfs 0.1.1`, `rustix 0.38.44`, `rustix 1.1.5`, `rustversion 1.0.23`, `semver 1.0.28`, `serde 1.0.229`, `serde_core 1.0.229`, `serde_derive 1.0.229`, `serde_derive_internals 0.30.0`, `serde_json 1.0.151`, `serde_repr 0.1.21`, `simd_cesu8 1.2.0`, `smol_str 0.2.2`, `syn 1.0.109`, `syn 2.0.119`, `syn 3.0.6`, `system-deps 7.0.8`, `thiserror-impl 1.0.69`, `thiserror-impl 2.0.21`, `thiserror 1.0.69`, `thiserror 2.0.21`, `typeid 1.0.3`, `unicode-ident 1.0.26`, `wasi 0.11.1+wasi-snapshot-preview1`, `wasip2 1.0.4+wasi-0.2.12`, `wit-bindgen 0.57.1`, `x11-dl 2.21.0`, `zbus-lockstep-macros 0.5.2`, `zbus-lockstep 0.5.2`, `zmij 1.0.23`, `zvariant_utils 4.2.0`

### MIT License

Used by: `glow 0.17.0`

### MIT License

Used by: `allocator-api2 0.2.21`

### MIT License

Used by: `winnow 1.0.4`

### MIT License

Used by: `raw-window-metal 1.1.0`, `siphasher 1.0.4`

### MIT License

Used by: `tinyvec 1.13.3`

### MIT License

Used by: `sary-ui-egui 0.10.0`

### MIT License

Used by: `android_system_properties 0.1.6`

### MIT License

Used by: `spin 0.9.9`

### MIT License

Used by: `tracing-core 0.1.36`

### MIT License

Used by: `typenum 1.20.1`

### MIT License

Used by: `xml-rs 0.8.29`, `xml 1.4.0`

### MIT License

Used by: `phf 0.13.1`, `phf_generator 0.13.1`, `phf_macros 0.13.1`, `phf_shared 0.13.1`

### MIT License

Used by: `base64 0.22.1`

### MIT License

Used by: `byteorder-lite 0.1.0`, `memchr 2.8.3`, `walkdir 2.5.0`

### MIT License

Used by: `strsim 0.11.1`

### MIT License

Used by: `raw-cpuid 11.6.0`

### MIT License

Used by: `twox-hash 2.1.4`

### MIT License

Used by: `combine 4.6.8`

### MIT License

Used by: `shlex 1.3.0`, `shlex 2.0.1`

### MIT License

Used by: `security-framework-sys 2.17.0`, `security-framework 3.7.0`

### MIT License

Used by: `gif 0.13.3`, `gif 0.14.2`

### MIT License

Used by: `orbclient 0.3.55`

### MIT License

Used by: `aws-lc-sys 0.46.0`

### MIT License

Used by: `pem 3.0.6`

### MIT License

Used by: `bit_field 0.10.3`

### MIT License

Used by: `color_quant 1.1.0`

### MIT License

Used by: `jni 0.21.1`

### MIT License

Used by: `same-file 1.0.6`, `winapi-util 0.1.11`

### MIT License

Used by: `simplecss 0.2.2`, `unicode-vo 0.1.0`

### MIT License

Used by: `roxmltree 0.20.0`

### MIT License

Used by: `guillotiere 0.7.0`

### MIT License

Used by: `xmlwriter 0.1.0`

### MIT License

Used by: `crossbeam-channel 0.5.17`, `crossbeam-deque 0.8.8`, `crossbeam-epoch 0.9.21`, `crossbeam-utils 0.8.23`

### MIT License

Used by: `fontdb 0.23.0`, `unicode-bidi-mirroring 0.4.0`, `unicode-ccc 0.4.0`

### MIT License

Used by: `async-broadcast 0.7.2`

### MIT License

Used by: `xkbcommon-dl 0.4.2`

### MIT License

Used by: `base64 0.23.1`

### MIT License

Used by: `harfrust 0.12.0`, `rustybuzz 0.20.1`

### MIT License

Used by: `weezl 0.1.12`

### MIT License

Used by: `crunchy 0.2.4`

### MIT License

Used by: `os_pipe 1.2.3`

### MIT License

Used by: `open 5.4.4`

### MIT License

Used by: `version_check 0.9.5`

### MIT License

Used by: `generic-array 0.14.7`

### MIT License

Used by: `quick-xml 0.41.0`

### MIT No Attribution

Used by: `dunce 1.0.5`

### MIT No Attribution

Used by: `constant_time_eq 0.4.2`

### SIL Open Font License 1.1

Used by: `epaint_default_fonts 0.36.2`

### Ubuntu Font Licence v1.0

Used by: `epaint_default_fonts 0.36.2`

### Unicode License v3

Used by: `unicode-ident 1.0.26`

### Unicode License v3

Used by: `icu_collections 2.3.0`, `icu_locale_core 2.3.0`, `icu_locale_fallback 2.3.0`, `icu_locale_fallback_data 2.3.0`, `icu_normalizer 2.3.0`, `icu_normalizer_data 2.3.0`, `icu_properties 2.3.0`, `icu_properties_data 2.3.0`, `icu_provider 2.3.1`, `icu_segmenter 2.3.0`, `icu_segmenter_data 2.3.0`, `litemap 0.8.3`, `potential_utf 0.1.6`, `tinystr 0.8.4`, `writeable 0.6.4`, `yoke-derive 0.8.3`, `yoke 0.8.3`, `zerofrom-derive 0.1.8`, `zerofrom 0.1.8`, `zerotrie 0.2.5`, `zerovec-derive 0.11.6`, `zerovec 0.11.8`

### zlib License

Used by: `slotmap 1.1.1`

### zlib License

Used by: `foldhash 0.1.5`, `foldhash 0.2.0`


## `apps/taratasy`

- **MIT License** (677)
- **Apache License 2.0** (33)
- **ISC License** (22)
- **Unicode License v3** (19)
- **BSD 3-Clause "New" or "Revised" License** (12)
- **Boost Software License 1.0** (2)
- **zlib License** (2)
- **BSD 2-Clause "Simplified" License** (1)
- **Community Data License Agreement Permissive 2.0** (1)
- **MIT No Attribution** (1)
- **SIL Open Font License 1.1** (1)
- **Ubuntu Font Licence v1.0** (1)

### Apache License 2.0

Used by: `flagset 0.4.7`

### Apache License 2.0

Used by: `ring 0.17.14`

### Apache License 2.0

Used by: `moxcms 0.8.1`, `pic-scale 0.7.12`, `pxfm 0.1.30`

### Apache License 2.0

Used by: `codespan-reporting 0.13.1`, `self_cell 1.3.0`

### Apache License 2.0

Used by: `unicode-general-category 1.1.0`

### Apache License 2.0

Used by: `gethostname 1.1.0`, `ring 0.17.14`

### Apache License 2.0

Used by: `glutin 0.32.3`, `glutin_egl_sys 0.7.1`, `glutin_glx_sys 0.6.1`, `glutin_wgl_sys 0.6.1`

### Apache License 2.0

Used by: `winit 0.30.13`, `dpi 0.1.2`

### Apache License 2.0

Used by: `accesskit_winit 0.32.2`, `aws-lc-sys 0.45.0`, `flatbuffers 24.12.23`, `gl_generator 0.14.0`, `khronos_api 3.1.0`, `ryu-js 1.0.3`, `ryu 1.0.23`, `sigstore-bundle 0.6.6`, `sigstore-crypto 0.6.6`, `sigstore-merkle 0.6.6`, `sigstore-rekor 0.6.6`, `sigstore-trust-root 0.6.6`, `sigstore-tsa 0.6.6`, `sigstore-types 0.6.6`, `sigstore-verify 0.6.6`, `spirv 0.4.0+sdk-1.4.341.0`, `sync_wrapper 1.0.2`

### BSD 2-Clause "Simplified" License

Used by: `arrayref 0.3.9`

### BSD 3-Clause "New" or "Revised" License

Used by: `encoding_rs 0.8.42`

### BSD 3-Clause "New" or "Revised" License

Used by: `winprint 0.2.1`

### BSD 3-Clause "New" or "Revised" License

Used by: `tiny-skia-path 0.11.4`, `tiny-skia 0.11.4`

### BSD 3-Clause "New" or "Revised" License

Used by: `alloc-no-stdlib 2.0.4`, `brotli 8.0.4`

### BSD 3-Clause "New" or "Revised" License

Used by: `subtle 2.6.1`

### BSD 3-Clause "New" or "Revised" License

Used by: `curve25519-dalek 5.0.0`

### BSD 3-Clause "New" or "Revised" License

Used by: `ed25519-dalek 3.0.0`

### BSD 3-Clause "New" or "Revised" License

Used by: `alloc-stdlib 0.2.4`, `aws-lc-sys 0.45.0`

### BSD 3-Clause "New" or "Revised" License

Used by: `encoding_rs 0.8.42`

### Boost Software License 1.0

Used by: `error-code 3.4.0`

### Boost Software License 1.0

Used by: `clipboard-win 5.4.1`

### Community Data License Agreement Permissive 2.0

Used by: `webpki-root-certs 1.0.9`

### ISC License

Used by: `ring 0.17.14`

### ISC License

Used by: `ring 0.17.14`

### ISC License

Used by: `untrusted 0.7.1`, `untrusted 0.9.0`

### ISC License

Used by: `ring 0.17.14`

### ISC License

Used by: `ring 0.17.14`

### ISC License

Used by: `ring 0.17.14`

### ISC License

Used by: `ring 0.17.14`

### ISC License

Used by: `ring 0.17.14`

### ISC License

Used by: `ring 0.17.14`

### ISC License

Used by: `ring 0.17.14`

### ISC License

Used by: `ring 0.17.14`

### ISC License

Used by: `ring 0.17.14`

### ISC License

Used by: `ring 0.17.14`

### ISC License

Used by: `ring 0.17.14`

### ISC License

Used by: `ring 0.17.14`

### ISC License

Used by: `ring 0.17.14`

### ISC License

Used by: `ring 0.17.14`

### ISC License

Used by: `libloading 0.8.9`

### ISC License

Used by: `rustls-webpki 0.103.15`

### ISC License

Used by: `aws-lc-rs 1.18.1`, `aws-lc-sys 0.45.0`

### MIT License

Used by: `uds_windows 1.2.1`

### MIT License

Used by: `cfg_aliases 0.2.2`

### MIT License

Used by: `atomic-waker 1.1.2`, `futures-lite 2.6.1`

### MIT License

Used by: `sha1 0.10.7`, `sha2 0.10.9`

### MIT License

Used by: `brotli 8.0.4`

### MIT License

Used by: `rayon-core 1.13.0`, `rayon 1.12.0`

### MIT License

Used by: `cgl 0.3.2`, `core-foundation-sys 0.8.7`, `core-foundation 0.10.1`, `core-foundation 0.9.4`, `core-graphics-types 0.1.3`, `core-graphics 0.23.2`, `euclid 0.22.14`

### MIT License

Used by: `hex 0.4.3`

### MIT License

Used by: `form_urlencoded 1.2.2`

### MIT License

Used by: `data-url 0.3.2`, `idna 1.1.0`, `percent-encoding 2.3.2`, `url 2.5.8`

### MIT License

Used by: `cc 1.5.1`, `cfg-if 1.0.5`, `cmake 0.1.58`, `filetime 0.2.29`, `find-msvc-tools 0.1.14`, `jobserver 0.1.35`, `js-sys 0.3.106`, `openssl-probe 0.2.1`, `pkg-config 0.3.34`, `scoped-tls 1.0.1`, `socket2 0.6.5`, `wasm-bindgen-futures 0.4.79`, `wasm-bindgen-macro-support 0.2.129`, `wasm-bindgen-macro 0.2.129`, `wasm-bindgen-shared 0.2.129`, `wasm-bindgen 0.2.129`, `web-sys 0.3.106`

### MIT License

Used by: `mio 1.2.4`

### MIT License

Used by: `errno 0.3.14`

### MIT License

Used by: `mime 0.3.17`

### MIT License

Used by: `base64ct 1.8.3`

### MIT License

Used by: `base16ct 1.0.0`

### MIT License

Used by: `bitflags 1.3.2`, `bitflags 2.13.2`, `log 0.4.34`, `num-bigint 0.5.1`, `num-integer 0.1.47`, `num-traits 0.2.19`

### MIT License

Used by: `uuid 1.26.1`

### MIT License

Used by: `nom 8.0.0`

### MIT License

Used by: `float-cmp 0.9.0`

### MIT License

Used by: `flate2 1.1.10`

### MIT License

Used by: `hyper 1.12.0`

### MIT License

Used by: `either 1.18.0`, `itertools 0.15.0`

### MIT License

Used by: `wayland-backend 0.3.17`, `wayland-client 0.31.15`, `wayland-cursor 0.31.14`, `wayland-protocols-experimental 20250721.0.1`, `wayland-protocols-misc 0.3.12`, `wayland-protocols-plasma 0.3.12`, `wayland-protocols-wlr 0.3.12`, `wayland-protocols 0.32.13`, `wayland-scanner 0.31.11`, `wayland-sys 0.31.11`

### MIT License

Used by: `ordered-float 5.5.0`

### MIT License

Used by: `tempfile 3.27.0`, `xattr 1.6.1`

### MIT License

Used by: `heck 0.5.0`, `unicode-bidi 0.3.18`, `unicode-normalization 0.1.25`, `unicode-properties 0.1.4`, `unicode-segmentation 1.13.3`, `unicode-width 0.2.2`

### MIT License

Used by: `quick-error 2.0.1`

### MIT License

Used by: `jni-sys 0.3.1`, `jni-sys 0.4.1`

### MIT License

Used by: `dlib 0.5.3`

### MIT License

Used by: `png 0.17.16`, `png 0.18.1`

### MIT License

Used by: `schannel 0.1.29`

### MIT License

Used by: `fixedbitset 0.5.7`

### MIT License

Used by: `httparse 1.10.1`, `num_cpus 1.17.0`

### MIT License

Used by: `ash 0.38.0+1.3.281`

### MIT License

Used by: `futures-channel 0.3.34`, `futures-core 0.3.34`, `futures-io 0.3.34`, `futures-macro 0.3.34`, `futures-sink 0.3.34`, `futures-task 0.3.34`, `futures-util 0.3.34`

### MIT License

Used by: `hashbrown 0.14.5`, `hashbrown 0.16.1`, `hashbrown 0.17.1`, `intrusive-collections 0.10.3`

### MIT License

Used by: `serde_urlencoded 0.7.1`

### MIT License

Used by: `hyper-rustls 0.27.10`, `rustls-native-certs 0.8.4`, `rustls 0.23.46`

### MIT License

Used by: `lock_api 0.4.14`, `parking_lot 0.12.5`, `parking_lot_core 0.9.12`, `rustc_version 0.4.1`

### MIT License

Used by: `android_log-sys 0.3.2`

### MIT License

Used by: `android_logger 0.15.1`

### MIT License

Used by: `indexmap 2.14.2`

### MIT License

Used by: `equivalent 1.0.2`

### MIT License

Used by: `scopeguard 1.2.0`

### MIT License

Used by: `cipher 0.5.2`

### MIT License

Used by: `reqwest 0.13.5`

### MIT License

Used by: `md-5 0.11.0`, `sha1 0.11.0`, `sha2 0.11.0`

### MIT License

Used by: `digest 0.10.7`, `hmac 0.13.0`

### MIT License

Used by: `pbkdf2 0.13.0`

### MIT License

Used by: `fnv 1.0.7`

### MIT License

Used by: `memoffset 0.9.1`

### MIT License

Used by: `enumflags2_derive 0.7.12`

### MIT License

Used by: `plain 0.2.3`

### MIT License

Used by: `keyboard-types 0.8.3`

### MIT License

Used by: `ron 0.12.2`

### MIT License

Used by: `redox_syscall 0.4.1`, `redox_syscall 0.5.18`, `redox_syscall 0.9.4`

### MIT License

Used by: `stable_deref_trait 1.2.1`

### MIT License

Used by: `foreign-types-macros 0.2.4`, `foreign-types-shared 0.3.1`, `foreign-types 0.5.0`

### MIT License

Used by: `stringprep 0.1.5`

### MIT License

Used by: `h2 0.4.20`

### MIT License

Used by: `http 1.5.0`

### MIT License

Used by: `tokio-rustls 0.26.6`

### MIT License

Used by: `signal-hook-registry 1.4.8`

### MIT License

Used by: `core_detect 1.0.0`

### MIT License

Used by: `enumflags2 0.7.12`

### MIT License

Used by: `rc2 0.9.0`

### MIT License

Used by: `des 0.9.0`

### MIT License

Used by: `digest 0.11.3`

### MIT License

Used by: `bytes 1.12.1`

### MIT License

Used by: `autocfg 1.5.1`

### MIT License

Used by: `smithay-clipboard 0.7.3`

### MIT License

Used by: `fearless_simd 0.4.1`, `kurbo 0.11.3`, `kurbo 0.13.1`, `peniko 0.6.1`, `polycool 0.4.0`

### MIT License

Used by: `smallvec 1.16.2`

### MIT License

Used by: `ahash 0.8.12`

### MIT License

Used by: `smithay-client-toolkit 0.19.2`, `smithay-client-toolkit 0.20.0`

### MIT License

Used by: `calloop 0.13.0`, `calloop 0.14.4`

### MIT License

Used by: `svgtypes 0.15.3`

### MIT License

Used by: `want 0.3.2`

### MIT License

Used by: `block-buffer 0.10.4`

### MIT License

Used by: `egui-wgpu 0.36.2`

### MIT License

Used by: `cbc 0.2.1`

### MIT License

Used by: `signature 2.2.0`

### MIT License

Used by: `try-lock 0.2.5`

### MIT License

Used by: `aes 0.9.3`

### MIT License

Used by: `getrandom 0.2.17`

### MIT License

Used by: `block-buffer 0.12.1`, `block-padding 0.4.2`

### MIT License

Used by: `getrandom 0.3.4`

### MIT License

Used by: `ecdsa 0.17.0`, `ed25519 3.0.0`, `rfc6979 0.6.0`, `signature 3.0.0`

### MIT License

Used by: `rand_core 0.10.1`

### MIT License

Used by: `keccak 0.2.2`, `zeroize 1.9.0`

### MIT License

Used by: `getrandom 0.4.3`

### MIT License

Used by: `slab 0.4.12`

### MIT License

Used by: `presser 0.3.1`

### MIT License

Used by: `font-types 0.12.5`, `read-fonts 0.41.0`, `read-fonts 0.44.0`, `skrifa 0.44.0`, `skrifa 0.47.0`

### MIT License

Used by: `bumpalo 3.20.3`

### MIT License

Used by: `tracing-attributes 0.1.31`, `tracing-core 0.1.36`, `tracing 0.1.44`

### MIT License

Used by: `tower-layer 0.3.3`, `tower-service 0.3.3`, `tower 0.5.3`

### MIT License

Used by: `pico-args 0.5.0`

### MIT License

Used by: `tower-http 0.6.11`

### MIT License

Used by: `http-body-util 0.1.5`, `http-body 1.1.0`

### MIT License

Used by: `chacha20 0.10.2`, `zeroize_derive 1.5.0`

### MIT License

Used by: `winsafe 0.0.29`

### MIT License

Used by: `iana-time-zone-haiku 0.1.2`, `iana-time-zone 0.1.65`

### MIT License

Used by: `downcast-rs 1.2.1`

### MIT License

Used by: `document-features 0.2.12`

### MIT License

Used by: `litrs 1.0.0`

### MIT License

Used by: `tls_codec 0.4.2`, `tls_codec_derive 0.4.2`

### MIT License

Used by: `memmap2 0.9.11`

### MIT License

Used by: `pollster 0.4.0`, `pollster 1.0.1`

### MIT License

Used by: `p384 0.14.0`

### MIT License

Used by: `const-oid 0.9.6`, `der_derive 0.7.3`

### MIT License

Used by: `primeorder 0.14.0`

### MIT License

Used by: `der 0.7.10`

### MIT License

Used by: `cpufeatures 0.2.17`

### MIT License

Used by: `elliptic-curve 0.14.1`, `p256 0.14.0`, `p521 0.14.0`

### MIT License

Used by: `const-oid 0.10.2`, `cpufeatures 0.3.1`, `der 0.8.2`

### MIT License

Used by: `sha3 0.12.0`

### MIT License

Used by: `num_threads 0.1.7`

### MIT License

Used by: `crypto-common 0.1.7`

### MIT License

Used by: `pem-rfc7468 0.7.0`, `x509-cert 0.2.5`

### MIT License

Used by: `gpu-allocator 0.28.0`

### MIT License

Used by: `bp256 0.14.0`, `bp384 0.14.0`

### MIT License

Used by: `spki 0.7.3`

### MIT License

Used by: `crypto-common 0.2.2`

### MIT License

Used by: `crypto-bigint 0.7.5`, `ripemd 0.2.0`, `sec1 0.8.1`, `spki 0.8.0`

### MIT License

Used by: `renderdoc-sys 1.1.0`

### MIT License

Used by: `atspi-common 0.13.0`, `atspi-proxies 0.13.0`, `atspi 0.29.0`

### MIT License

Used by: `strict-num 0.1.1`

### MIT License

Used by: `xkeysym 0.2.1`

### MIT License

Used by: `inout 0.2.2`

### MIT License

Used by: `cmov 0.5.4`, `hybrid-array 0.4.15`

### MIT License

Used by: `rustls-pki-types 1.15.1`

### MIT License

Used by: `powerfmt 0.2.0`

### MIT License

Used by: `calloop-wayland-source 0.3.0`, `calloop-wayland-source 0.4.1`

### MIT License

Used by: `cursor-icon 1.2.0`, `wayland-csd-frame 0.3.0`

### MIT License

Used by: `bit-vec 0.9.1`

### MIT License

Used by: `cmpv2 0.2.0`, `crmf 0.2.0`, `x509-tsp 0.1.0`

### MIT License

Used by: `hyper-util 0.1.21`

### MIT License

Used by: `cpubits 0.1.1`

### MIT License

Used by: `deranged 0.5.8`

### MIT License

Used by: `system-configuration-sys 0.6.0`, `system-configuration 0.7.0`

### MIT License

Used by: `zbus 5.19.0`, `zbus_macros 5.19.0`, `zbus_names 4.3.4`, `zbus_xml 5.2.1`, `zvariant 5.15.0`, `zvariant_derive 5.15.0`

### MIT License

Used by: `primefield 0.14.0`

### MIT License

Used by: `ctutils 0.4.2`

### MIT License

Used by: `ecb 0.2.1`

### MIT License

Used by: `wnaf 0.14.1`

### MIT License

Used by: `bit-set 0.10.0`

### MIT License

Used by: `sponge-cursor 0.1.0`

### MIT License

Used by: `zcheapstr 1.1.0`

### MIT License

Used by: `env_filter 0.1.4`, `serde_spanned 1.1.1`, `toml 1.1.6+spec-1.1.0`, `toml_datetime 1.1.1+spec-1.1.0`, `toml_edit 0.25.15+spec-1.1.0`, `toml_parser 1.1.3+spec-1.1.0`, `toml_writer 1.1.2+spec-1.1.0`

### MIT License

Used by: `num-conv 0.2.2`

### MIT License

Used by: `time-core 0.1.9`, `time-macros 0.2.32`, `time 0.3.55`

### MIT License

Used by: `hayro 0.7.1`, `hayro-interpret 0.7.0`, `hayro-jbig2 0.3.0`, `hayro-syntax 0.7.2`, `hayro-ccitt 0.3.0`, `hayro-cmap 0.1.0`, `hayro-jpeg2000 0.3.5`, `hayro-postscript 0.1.0`

### MIT License

Used by: `libc 0.2.189`

### MIT License

Used by: `idna_adapter 1.2.2`

### MIT License

Used by: `tar 0.4.46`

### MIT License

Used by: `arrayvec 0.7.8`

### MIT License

Used by: `winresource 0.1.31`

### MIT License

Used by: `synstructure 0.14.0`

### MIT License

Used by: `ipnet 2.12.2`

### MIT License

Used by: `resvg 0.45.1`, `usvg 0.45.1`

### MIT License

Used by: `rand 0.10.3`, `rand_core 0.6.4`

### MIT License

Used by: `nohash-hasher 0.2.0`

### MIT License

Used by: `rangemap 1.8.0`

### MIT License

Used by: `as-raw-xcb-connection 1.0.1`

### MIT License

Used by: `x11rb-protocol 0.13.2`, `x11rb 0.13.2`

### MIT License

Used by: `vello_common 0.0.8`, `vello_common 0.1.0`, `vello_cpu 0.0.8`, `vello_cpu 0.1.0`

### MIT License

Used by: `ureq-proto 0.6.4`

### MIT License

Used by: `crypto-primes 0.7.2`

### MIT License

Used by: `zerocopy-derive 0.8.59`, `zerocopy 0.8.59`

### MIT License

Used by: `encoding_rs 0.8.42`, `multiversion_no_op 1.0.0`, `utf16_iter 1.0.5`, `utf8_iter 1.0.4`, `write16 1.0.0`

### MIT License

Used by: `wayland-protocols 0.32.13`

### MIT License

Used by: `fax 0.2.7`

### MIT License

Used by: `glutin-winit 0.5.0`

### MIT License

Used by: `epaint_default_fonts 0.36.2`

### MIT License

Used by: `lopdf 0.45.0`

### MIT License

Used by: `fs_extra 1.3.0`

### MIT License

Used by: `tap 1.0.1`

### MIT License

Used by: `static_assertions 1.1.0`

### MIT License

Used by: `tiff 0.11.3`

### MIT License

Used by: `crc32fast 1.5.2`

### MIT License

Used by: `dashmap 6.2.1`

### MIT License

Used by: `bytemuck 1.25.2`, `bytemuck_derive 1.12.1`

### MIT License

Used by: `rgb 0.8.53`

### MIT License

Used by: `ureq 3.4.2`

### MIT License

Used by: `raw-window-handle 0.6.2`

### MIT License

Used by: `strum 0.28.0`, `strum_macros 0.28.0`

### MIT License

Used by: `proc-macro-error-attr 1.0.4`, `proc-macro-error 1.0.4`

### MIT License

Used by: `rfd 0.17.2`

### MIT License

Used by: `cfg_aliases 0.2.2`

### MIT License

Used by: `android-properties 0.2.2`

### MIT License

Used by: `xcursor 0.3.11`

### MIT License

Used by: `regress 0.12.0`

### MIT License

Used by: `sys-locale 0.3.2`

### MIT License

Used by: `rustls-platform-verifier 0.7.1`

### MIT License

Used by: `rfd 0.17.2`

### MIT License

Used by: `arboard 3.6.1`

### MIT License

Used by: `aligned-vec 0.6.4`

### MIT License

Used by: `muda 0.21.2`

### MIT License

Used by: `libredox 0.1.25`

### MIT License

Used by: `range-alloc 0.1.5`

### MIT License

Used by: `web-time 1.1.0`

### MIT License

Used by: `equator-macro 0.4.2`, `equator 0.4.2`

### MIT License

Used by: `naga-types 30.0.1`, `naga 30.0.1`, `wgpu-core-deps-apple 30.0.1`, `wgpu-core-deps-emscripten 30.0.1`, `wgpu-core-deps-wasm 30.0.1`, `wgpu-core-deps-windows-linux-android 30.0.1`, `wgpu-core 30.0.1`, `wgpu-hal 30.0.1`, `wgpu-naga-bridge 30.0.1`, `wgpu-types 30.0.1`, `wgpu 30.0.1`

### MIT License

Used by: `craft-winprint-gdi 0.1.0`

### MIT License

Used by: `fmv-macos-events 0.1.0`

### MIT License

Used by: `taratasy 0.9.0`, `taratasy-android 0.9.0`, `taratasy-cli 0.9.0`, `taratasy-web 0.9.0`, `taratasy-a11y 0.9.0`, `taratasy-annot 0.9.0`, `taratasy-audit 0.9.0`, `taratasy-automation 0.9.0`, `taratasy-compare 0.9.0`, `taratasy-content 0.9.0`, `taratasy-cos 0.9.0`, `taratasy-create 0.9.0`, `taratasy-crypt 0.9.0`, `taratasy-edit 0.9.0`, `taratasy-engine 0.9.0`, `taratasy-export 0.9.0`, `taratasy-filters 0.9.0`, `taratasy-fonts 0.9.0`, `taratasy-forms 0.9.0`, `taratasy-geom 0.9.0`, `taratasy-js 0.9.0`, `taratasy-measure 0.9.0`, `taratasy-model 0.9.0`, `taratasy-ocr 0.9.0`, `taratasy-optimize 0.9.0`, `taratasy-oracle 0.9.0`, `taratasy-organize 0.9.0`, `taratasy-platform 0.9.0`, `taratasy-preflight 0.9.0`, `taratasy-print 0.9.0`, `taratasy-redact 0.9.0`, `taratasy-render 0.9.0`, `taratasy-sign 0.9.0`, `taratasy-testkit 0.9.0`, `taratasy-ui-egui 0.9.0`, `taratasy-xfa 0.9.0`, `taratasy-xfdf 0.9.0`, `xtask 0.1.0`, `mahay-android 0.1.0`, `mahay-docpath 0.1.0`, `mahay-print 0.1.0`, `mahay-quit 0.1.0`, `mahay-ui 0.1.0`, `accesskit 0.24.1`, `accesskit_atspi_common 0.18.1`, `accesskit_consumer 0.35.0`, `accesskit_consumer 0.36.0`, `accesskit_consumer 0.38.0`, `accesskit_macos 0.26.3`, `accesskit_unix 0.21.1`, `accesskit_windows 0.32.1`, `block2 0.5.1`, `block2 0.6.2`, `boa_ast 0.22.0`, `boa_engine 0.22.0`, `boa_gc 0.22.0`, `boa_interner 0.22.0`, `boa_macros 0.22.0`, `boa_parser 0.22.0`, `boa_string 0.22.0`, `brotli-decompressor 5.0.3`, `cesu8 1.1.0`, `chrono 0.4.45`, `cms 0.2.3`, `dispatch2 0.3.1`, `dispatch 0.2.0`, `dpi 0.1.2`, `ecolor 0.36.2`, `eframe 0.36.2`, `egui-winit 0.36.2`, `egui 0.36.2`, `egui_extras 0.36.2`, `egui_glow 0.36.2`, `emath 0.36.2`, `epaint 0.36.2`, `fmt-derive-proc 0.1.2`, `fmt-derive 0.1.2`, `jni-macros 0.22.4`, `jni-sys-macros 0.4.1`, `jni 0.22.4`, `libm 0.2.16`, `ndk-context 0.1.1`, `ndk-sys 0.6.0+11769913`, `ndk 0.9.0`, `objc-sys 0.3.5`, `objc2-app-kit 0.2.2`, `objc2-app-kit 0.3.2`, `objc2-core-foundation 0.3.2`, `objc2-core-graphics 0.3.2`, `objc2-encode 4.1.0`, `objc2-foundation 0.2.2`, `objc2-foundation 0.3.2`, `objc2-metal 0.3.2`, `objc2-quartz-core 0.3.2`, `objc2-ui-kit 0.2.2`, `objc2-ui-kit 0.3.2`, `objc2 0.5.2`, `objc2 0.6.4`, `ocrs 0.13.1`, `profiling 1.0.18`, `r-efi 5.3.0`, `r-efi 6.0.0`, `rten-base 0.26.0`, `rten-gemm 0.26.0`, `rten-imageproc 0.26.0`, `rten-model-file 0.26.0`, `rten-parallel 0.26.0`, `rten-shape-inference 0.26.0`, `rten-simd 0.26.0`, `rten-tensor 0.26.0`, `rten-vecmath 0.26.0`, `rten 0.26.0`, `rustls-cng 0.7.1`, `rustls-platform-verifier-android 0.2.0`, `small_btree 0.1.0`, `tag_ptr 0.1.0`, `type-map 0.5.1`, `windows-collections 0.3.2`, `windows-core 0.62.2`, `windows-future 0.3.2`, `windows-implement 0.60.2`, `windows-interface 0.59.3`, `windows-link 0.2.1`, `windows-numerics 0.3.1`, `windows-registry 0.6.1`, `windows-result 0.4.1`, `windows-strings 0.5.1`, `windows-sys 0.45.0`, `windows-sys 0.52.0`, `windows-sys 0.61.2`, `windows-targets 0.42.2`, `windows-targets 0.52.6`, `windows-threading 0.2.1`, `windows 0.62.2`, `windows_aarch64_gnullvm 0.42.2`, `windows_aarch64_gnullvm 0.52.6`, `windows_aarch64_msvc 0.42.2`, `windows_aarch64_msvc 0.52.6`, `windows_i686_gnu 0.42.2`, `windows_i686_gnu 0.52.6`, `windows_i686_gnullvm 0.52.6`, `windows_i686_msvc 0.42.2`, `windows_i686_msvc 0.52.6`, `windows_x86_64_gnu 0.42.2`, `windows_x86_64_gnu 0.52.6`, `windows_x86_64_gnullvm 0.42.2`, `windows_x86_64_gnullvm 0.52.6`, `windows_x86_64_msvc 0.42.2`, `windows_x86_64_msvc 0.52.6`

### MIT License

Used by: `enum-map-derive 0.17.0`, `enum-map 2.7.3`

### MIT License

Used by: `tokio-util 0.7.20`, `tokio 1.53.1`

### MIT License

Used by: `simd-adler32 0.3.10`

### MIT License

Used by: `zune-core 0.5.3`, `zune-jpeg 0.5.15`

### MIT License

Used by: `miniz_oxide 0.8.9`, `miniz_oxide 0.9.1`

### MIT License

Used by: `miniz_oxide 0.8.9`, `miniz_oxide 0.9.1`

### MIT License

Used by: `fdeflate 0.3.7`, `image 0.25.10`

### MIT License

Used by: `simdutf8 0.1.5`

### MIT License

Used by: `android-activity 0.6.1`, `color 0.3.3`, `half 2.7.1`, `linebender_resource_handle 0.1.1`

### MIT License

Used by: `imagesize 0.13.0`

### MIT License

Used by: `cow-utils 0.1.3`

### MIT License

Used by: `fontconfig-parser 0.5.8`

### MIT License

Used by: `serde_json_canonicalizer 0.3.2`

### MIT License

Used by: `wayland-protocols-plasma 0.3.12`

### MIT License

Used by: `async-recursion 1.1.1`, `rsa 0.10.0-rc.18`, `rustc-hash 2.1.3`

### MIT License

Used by: `adler2 2.0.1`, `anyhow 1.0.104`, `async-channel 2.5.0`, `async-executor 1.14.0`, `async-io 2.6.0`, `async-lock 3.4.2`, `async-process 2.5.0`, `async-signal 0.2.14`, `async-task 4.7.1`, `async-trait 0.1.92`, `atomic-waker 1.1.2`, `blocking 1.7.0`, `concurrent-queue 2.5.0`, `curve25519-dalek-derive 0.1.1`, `displaydoc 0.2.7`, `endi 1.1.1`, `enumn 0.1.14`, `event-listener-strategy 0.5.4`, `event-listener 5.4.2`, `fast-float2 0.2.4`, `fastrand 2.5.0`, `futures-lite 2.6.1`, `group 0.14.0`, `hermit-abi 0.5.3`, `itoa 1.0.18`, `khronos-egl 6.0.0`, `linux-raw-sys 0.12.1`, `linux-raw-sys 0.4.15`, `num_enum 0.7.6`, `num_enum_derive 0.7.6`, `once_cell 1.21.4`, `oneshot 0.2.1`, `ordered-stream 0.2.0`, `parking 2.2.1`, `pastey 0.2.3`, `pin-project-internal 1.1.13`, `pin-project-lite 0.2.17`, `pin-project 1.1.13`, `piper 0.2.5`, `polling 3.11.0`, `portable-atomic-util 0.2.8`, `portable-atomic 1.15.0`, `proc-macro-crate 3.5.0`, `proc-macro2 1.0.107`, `quote 1.0.47`, `rustc-hash 1.1.0`, `rustix 0.38.44`, `rustix 1.1.5`, `rustversion 1.0.23`, `semver 1.0.28`, `serde 1.0.229`, `serde_core 1.0.229`, `serde_derive 1.0.229`, `serde_json 1.0.151`, `serde_repr 0.1.21`, `simd_cesu8 1.2.0`, `smol_str 0.2.2`, `syn 1.0.109`, `syn 2.0.119`, `syn 3.0.6`, `thin-vec 0.2.20`, `thiserror-impl 1.0.69`, `thiserror-impl 2.0.21`, `thiserror 1.0.69`, `thiserror 2.0.21`, `typeid 1.0.3`, `unicode-ident 1.0.26`, `utf8-zero 0.8.1`, `wasi 0.11.1+wasi-snapshot-preview1`, `wasip2 1.0.4+wasi-0.2.12`, `wit-bindgen 0.57.1`, `x11-dl 2.21.0`, `zbus-lockstep-macros 0.5.2`, `zbus-lockstep 0.5.2`, `zmij 1.0.23`, `zvariant_utils 4.2.0`

### MIT License

Used by: `glow 0.17.0`

### MIT License

Used by: `allocator-api2 0.2.21`

### MIT License

Used by: `winnow 1.0.4`

### MIT License

Used by: `dynify-macros 0.1.2`, `dynify 0.1.2`

### MIT License

Used by: `raw-window-metal 1.1.0`, `siphasher 1.0.4`

### MIT License

Used by: `tinyvec 1.13.3`

### MIT License

Used by: `android_system_properties 0.1.6`

### MIT License

Used by: `tracing-core 0.1.36`

### MIT License

Used by: `typenum 1.20.1`

### MIT License

Used by: `xml-rs 0.8.29`, `xml 1.4.0`

### MIT License

Used by: `phf 0.13.1`, `phf 0.14.0`, `phf_generator 0.13.1`, `phf_generator 0.14.0`, `phf_macros 0.13.1`, `phf_macros 0.14.0`, `phf_shared 0.13.1`, `phf_shared 0.14.0`

### MIT License

Used by: `base64 0.22.1`

### MIT License

Used by: `byteorder-lite 0.1.0`, `memchr 2.8.3`, `walkdir 2.5.0`

### MIT License

Used by: `combine 4.6.8`

### MIT License

Used by: `shlex 2.0.1`

### MIT License

Used by: `security-framework-sys 2.17.0`, `security-framework 3.7.0`

### MIT License

Used by: `gif 0.14.2`

### MIT License

Used by: `orbclient 0.3.55`

### MIT License

Used by: `aws-lc-sys 0.45.0`

### MIT License

Used by: `fiat-crypto 0.3.0`

### MIT License

Used by: `webbrowser 1.2.4`

### MIT License

Used by: `pem 3.0.6`

### MIT License

Used by: `color_quant 1.1.0`

### MIT License

Used by: `jni 0.21.1`

### MIT License

Used by: `same-file 1.0.6`, `winapi-util 0.1.11`

### MIT License

Used by: `ff 0.14.0`

### MIT License

Used by: `simplecss 0.2.2`

### MIT License

Used by: `roxmltree 0.20.0`

### MIT License

Used by: `guillotiere 0.7.0`

### MIT License

Used by: `xmlwriter 0.1.0`

### MIT License

Used by: `crossbeam-channel 0.5.17`, `crossbeam-deque 0.8.8`, `crossbeam-epoch 0.9.21`, `crossbeam-utils 0.8.23`

### MIT License

Used by: `async-broadcast 0.7.2`, `futures-concurrency 7.7.1`

### MIT License

Used by: `xkbcommon-dl 0.4.2`

### MIT License

Used by: `base64 0.23.1`

### MIT License

Used by: `harfrust 0.12.0`

### MIT License

Used by: `weezl 0.1.12`, `weezl 0.2.1`

### MIT License

Used by: `crunchy 0.2.4`

### MIT License

Used by: `version_check 0.9.5`

### MIT License

Used by: `generic-array 0.14.7`

### MIT License

Used by: `quick-xml 0.41.0`

### MIT No Attribution

Used by: `dunce 1.0.5`

### SIL Open Font License 1.1

Used by: `epaint_default_fonts 0.36.2`

### Ubuntu Font Licence v1.0

Used by: `epaint_default_fonts 0.36.2`

### Unicode License v3

Used by: `unicode-ident 1.0.26`

### Unicode License v3

Used by: `icu_collections 2.3.0`, `icu_locale_core 2.3.0`, `icu_normalizer 2.3.0`, `icu_normalizer_data 2.3.0`, `icu_properties 2.3.0`, `icu_properties_data 2.3.0`, `icu_provider 2.3.1`, `litemap 0.8.3`, `potential_utf 0.1.6`, `tinystr 0.8.4`, `writeable 0.6.4`, `yoke-derive 0.8.4`, `yoke 0.8.3`, `zerofrom-derive 0.1.8`, `zerofrom 0.1.8`, `zerotrie 0.2.5`, `zerovec-derive 0.11.6`, `zerovec 0.11.8`

### zlib License

Used by: `slotmap 1.1.1`

### zlib License

Used by: `foldhash 0.2.0`


## `apps/soratra`

- **MIT License** (572)
- **Apache License 2.0** (28)
- **Unicode License v3** (19)
- **BSD 3-Clause "New" or "Revised" License** (7)
- **ISC License** (6)
- **zlib License** (4)
- **Boost Software License 1.0** (2)
- **BSD 2-Clause "Simplified" License** (1)
- **Community Data License Agreement Permissive 2.0** (1)
- **MIT No Attribution** (1)
- **SIL Open Font License 1.1** (1)
- **Ubuntu Font Licence v1.0** (1)

### Apache License 2.0

Used by: `flagset 0.4.7`

### Apache License 2.0

Used by: `zopfli 0.8.3`

### Apache License 2.0

Used by: `moxcms 0.8.1`, `pxfm 0.1.30`

### Apache License 2.0

Used by: `codespan-reporting 0.13.1`, `self_cell 1.3.0`, `unicode-linebreak 0.1.5`

### Apache License 2.0

Used by: `unicode-general-category 1.1.0`

### Apache License 2.0

Used by: `gethostname 1.1.0`

### Apache License 2.0

Used by: `glutin_wgl_sys 0.6.1`

### Apache License 2.0

Used by: `dpi 0.1.2`, `winit 0.30.13`

### Apache License 2.0

Used by: `accesskit_winit 0.32.2`, `aws-lc-sys 0.46.0`, `gl_generator 0.14.0`, `khronos_api 3.1.0`, `ryu-js 1.0.3`, `ryu 1.0.23`, `sigstore-bundle 0.6.6`, `sigstore-crypto 0.6.6`, `sigstore-merkle 0.6.6`, `sigstore-rekor 0.6.6`, `sigstore-trust-root 0.6.6`, `sigstore-tsa 0.6.6`, `sigstore-types 0.6.6`, `sigstore-verify 0.6.6`, `spirv 0.4.0+sdk-1.4.341.0`, `sync_wrapper 1.0.2`

### BSD 2-Clause "Simplified" License

Used by: `arrayref 0.3.9`

### BSD 3-Clause "New" or "Revised" License

Used by: `encoding_rs 0.8.42`

### BSD 3-Clause "New" or "Revised" License

Used by: `winprint 0.2.1`

### BSD 3-Clause "New" or "Revised" License

Used by: `tiny-skia-path 0.12.0`

### BSD 3-Clause "New" or "Revised" License

Used by: `alloc-no-stdlib 2.0.4`

### BSD 3-Clause "New" or "Revised" License

Used by: `subtle 2.6.1`

### BSD 3-Clause "New" or "Revised" License

Used by: `aws-lc-sys 0.46.0`

### BSD 3-Clause "New" or "Revised" License

Used by: `encoding_rs 0.8.42`

### Boost Software License 1.0

Used by: `error-code 3.4.0`

### Boost Software License 1.0

Used by: `clipboard-win 5.4.1`

### Community Data License Agreement Permissive 2.0

Used by: `webpki-root-certs 1.0.9`

### ISC License

Used by: `untrusted 0.7.1`, `untrusted 0.9.0`

### ISC License

Used by: `libloading 0.8.9`

### ISC License

Used by: `rustls-webpki 0.103.15`

### ISC License

Used by: `aws-lc-rs 1.18.2`, `aws-lc-sys 0.46.0`

### MIT License

Used by: `uds_windows 1.2.1`

### MIT License

Used by: `cfg_aliases 0.2.2`

### MIT License

Used by: `atomic-waker 1.1.2`, `futures-lite 2.6.1`

### MIT License

Used by: `sha1 0.10.7`, `sha2 0.10.9`

### MIT License

Used by: `rayon-core 1.13.0`, `rayon 1.12.0`

### MIT License

Used by: `core-foundation-sys 0.8.7`, `core-foundation 0.10.1`, `core-foundation 0.9.4`, `core-graphics-types 0.1.3`, `core-graphics 0.23.2`, `euclid 0.22.14`

### MIT License

Used by: `hex 0.4.3`

### MIT License

Used by: `form_urlencoded 1.2.2`

### MIT License

Used by: `idna 1.1.0`, `percent-encoding 2.3.2`, `url 2.5.8`

### MIT License

Used by: `cc 1.6.0`, `cfg-if 1.0.5`, `cmake 0.1.58`, `filetime 0.2.29`, `find-msvc-tools 0.1.14`, `jobserver 0.1.35`, `js-sys 0.3.106`, `openssl-probe 0.2.1`, `pkg-config 0.3.34`, `scoped-tls 1.0.1`, `socket2 0.6.5`, `wasm-bindgen-futures 0.4.79`, `wasm-bindgen-macro-support 0.2.129`, `wasm-bindgen-macro 0.2.129`, `wasm-bindgen-shared 0.2.129`, `wasm-bindgen 0.2.129`, `web-sys 0.3.106`

### MIT License

Used by: `mio 1.2.4`

### MIT License

Used by: `errno 0.3.14`

### MIT License

Used by: `mime 0.3.17`

### MIT License

Used by: `base64ct 1.8.3`

### MIT License

Used by: `bitflags 1.3.2`, `bitflags 2.13.2`, `log 0.4.34`, `num-traits 0.2.19`, `regex-automata 0.4.18`, `regex-syntax 0.8.11`, `regex 1.13.1`

### MIT License

Used by: `uuid 1.27.0`

### MIT License

Used by: `nom 8.0.0`

### MIT License

Used by: `float-cmp 0.9.0`

### MIT License

Used by: `flate2 1.1.10`

### MIT License

Used by: `hyper 1.12.0`

### MIT License

Used by: `either 1.19.0`, `itertools 0.15.0`, `petgraph 0.8.3`

### MIT License

Used by: `wayland-backend 0.3.17`, `wayland-client 0.31.15`, `wayland-cursor 0.31.14`, `wayland-protocols-experimental 20250721.0.1`, `wayland-protocols-misc 0.3.12`, `wayland-protocols-plasma 0.3.12`, `wayland-protocols-wlr 0.3.12`, `wayland-protocols 0.32.13`, `wayland-scanner 0.31.11`, `wayland-sys 0.31.11`

### MIT License

Used by: `ordered-float 5.5.0`

### MIT License

Used by: `tempfile 3.27.0`, `xattr 1.6.1`

### MIT License

Used by: `heck 0.5.0`, `unicode-bidi 0.3.18`, `unicode-segmentation 1.13.3`, `unicode-width 0.2.2`

### MIT License

Used by: `quick-error 2.0.1`

### MIT License

Used by: `jni-sys 0.3.1`, `jni-sys 0.4.1`

### MIT License

Used by: `dlib 0.5.3`

### MIT License

Used by: `png 0.18.1`

### MIT License

Used by: `schannel 0.1.30`

### MIT License

Used by: `fixedbitset 0.5.7`

### MIT License

Used by: `httparse 1.10.1`

### MIT License

Used by: `ash 0.38.0+1.3.281`

### MIT License

Used by: `futures-channel 0.3.34`, `futures-core 0.3.34`, `futures-io 0.3.34`, `futures-macro 0.3.34`, `futures-sink 0.3.34`, `futures-task 0.3.34`, `futures-util 0.3.34`

### MIT License

Used by: `hashbrown 0.15.5`, `hashbrown 0.16.1`, `hashbrown 0.17.1`

### MIT License

Used by: `serde_urlencoded 0.7.1`

### MIT License

Used by: `hyper-rustls 0.27.10`, `rustls-native-certs 0.8.5`, `rustls 0.23.46`

### MIT License

Used by: `lock_api 0.4.14`, `parking_lot 0.12.5`, `parking_lot_core 0.9.12`, `rustc_version 0.4.1`, `thread_local 1.1.10`

### MIT License

Used by: `android_log-sys 0.3.2`

### MIT License

Used by: `android_logger 0.15.1`

### MIT License

Used by: `indexmap 2.14.2`

### MIT License

Used by: `equivalent 1.0.2`

### MIT License

Used by: `scopeguard 1.2.0`

### MIT License

Used by: `cipher 0.5.2`

### MIT License

Used by: `reqwest 0.13.5`

### MIT License

Used by: `sha1 0.11.0`, `sha2 0.11.0`

### MIT License

Used by: `digest 0.10.7`, `hmac 0.13.0`

### MIT License

Used by: `fnv 1.0.7`

### MIT License

Used by: `memoffset 0.9.1`

### MIT License

Used by: `enumflags2_derive 0.7.12`

### MIT License

Used by: `plain 0.2.3`

### MIT License

Used by: `redox_syscall 0.4.1`, `redox_syscall 0.5.18`, `redox_syscall 0.9.4`

### MIT License

Used by: `stable_deref_trait 1.2.1`

### MIT License

Used by: `foreign-types-macros 0.2.4`, `foreign-types-shared 0.3.1`, `foreign-types 0.5.0`

### MIT License

Used by: `h2 0.4.20`

### MIT License

Used by: `http 1.5.0`

### MIT License

Used by: `tokio-rustls 0.26.6`

### MIT License

Used by: `signal-hook-registry 1.4.8`

### MIT License

Used by: `core_detect 1.0.0`

### MIT License

Used by: `enumflags2 0.7.12`

### MIT License

Used by: `digest 0.11.3`

### MIT License

Used by: `bytes 1.12.1`

### MIT License

Used by: `autocfg 1.5.1`

### MIT License

Used by: `smithay-clipboard 0.7.3`

### MIT License

Used by: `fearless_simd 0.4.1`, `kurbo 0.13.1`, `peniko 0.6.1`, `polycool 0.4.0`

### MIT License

Used by: `smallvec 1.16.2`

### MIT License

Used by: `ahash 0.8.12`

### MIT License

Used by: `smithay-client-toolkit 0.19.2`, `smithay-client-toolkit 0.20.0`

### MIT License

Used by: `calloop 0.13.0`, `calloop 0.14.5`

### MIT License

Used by: `want 0.3.2`

### MIT License

Used by: `block-buffer 0.10.4`

### MIT License

Used by: `signature 2.2.0`

### MIT License

Used by: `try-lock 0.2.5`

### MIT License

Used by: `aes 0.9.3`

### MIT License

Used by: `block-buffer 0.12.1`

### MIT License

Used by: `getrandom 0.3.4`

### MIT License

Used by: `rand_core 0.10.1`

### MIT License

Used by: `zeroize 1.9.1`

### MIT License

Used by: `getrandom 0.4.3`

### MIT License

Used by: `slab 0.4.12`

### MIT License

Used by: `font-types 0.11.3`, `read-fonts 0.39.2`, `skrifa 0.42.1`, `write-fonts 0.48.1`

### MIT License

Used by: `presser 0.3.1`

### MIT License

Used by: `font-types 0.12.6`, `read-fonts 0.41.0`, `read-fonts 0.43.3`, `read-fonts 0.44.0`, `skrifa 0.44.0`, `skrifa 0.47.0`

### MIT License

Used by: `wl-clipboard-rs 0.9.4`

### MIT License

Used by: `bumpalo 3.20.3`

### MIT License

Used by: `tracing-attributes 0.1.31`, `tracing-core 0.1.36`, `tracing 0.1.44`

### MIT License

Used by: `tower-layer 0.3.3`, `tower-service 0.3.3`, `tower 0.5.3`

### MIT License

Used by: `tower-http 0.6.11`

### MIT License

Used by: `http-body-util 0.1.5`, `http-body 1.1.0`

### MIT License

Used by: `chacha20 0.10.2`, `zeroize_derive 1.5.0`

### MIT License

Used by: `winsafe 0.0.29`

### MIT License

Used by: `iana-time-zone-haiku 0.1.2`, `iana-time-zone 0.1.65`

### MIT License

Used by: `downcast-rs 1.2.1`

### MIT License

Used by: `document-features 0.2.12`

### MIT License

Used by: `litrs 1.0.0`

### MIT License

Used by: `tls_codec 0.4.2`, `tls_codec_derive 0.4.2`

### MIT License

Used by: `memmap2 0.9.11`

### MIT License

Used by: `pollster 0.4.0`, `pollster 1.0.1`

### MIT License

Used by: `const-oid 0.9.6`, `der_derive 0.7.3`

### MIT License

Used by: `der 0.7.10`

### MIT License

Used by: `cpufeatures 0.2.17`

### MIT License

Used by: `const-oid 0.10.2`, `cpufeatures 0.3.1`

### MIT License

Used by: `crypto-common 0.1.7`

### MIT License

Used by: `pem-rfc7468 0.7.0`, `x509-cert 0.2.5`

### MIT License

Used by: `gpu-allocator 0.28.0`

### MIT License

Used by: `spki 0.7.3`

### MIT License

Used by: `crypto-common 0.2.2`

### MIT License

Used by: `renderdoc-sys 1.1.0`

### MIT License

Used by: `atspi-common 0.13.0`, `atspi-proxies 0.13.0`, `atspi 0.29.0`

### MIT License

Used by: `strict-num 0.1.1`

### MIT License

Used by: `xkeysym 0.2.1`

### MIT License

Used by: `inout 0.2.2`

### MIT License

Used by: `cmov 0.5.4`, `hybrid-array 0.4.15`

### MIT License

Used by: `rustls-pki-types 1.15.1`

### MIT License

Used by: `calloop-wayland-source 0.3.0`, `calloop-wayland-source 0.4.1`

### MIT License

Used by: `cursor-icon 1.2.0`, `wayland-csd-frame 0.3.0`

### MIT License

Used by: `bit-vec 0.9.1`

### MIT License

Used by: `cmpv2 0.2.0`, `crmf 0.2.0`, `x509-tsp 0.1.0`

### MIT License

Used by: `hyper-util 0.1.21`

### MIT License

Used by: `cpubits 0.1.1`

### MIT License

Used by: `system-configuration-sys 0.6.0`, `system-configuration 0.7.0`

### MIT License

Used by: `zbus 5.19.0`, `zbus_macros 5.19.0`, `zbus_names 4.3.4`, `zbus_xml 5.2.1`, `zvariant 5.15.0`, `zvariant_derive 5.15.0`

### MIT License

Used by: `ctutils 0.4.3`

### MIT License

Used by: `bit-set 0.10.0`

### MIT License

Used by: `zcheapstr 1.1.0`

### MIT License

Used by: `env_filter 0.1.4`, `serde_spanned 1.1.1`, `toml 1.1.6+spec-1.1.0`, `toml_datetime 1.1.1+spec-1.1.0`, `toml_edit 0.25.15+spec-1.1.0`, `toml_parser 1.1.3+spec-1.1.0`, `toml_writer 1.1.2+spec-1.1.0`

### MIT License

Used by: `libc 0.2.190`

### MIT License

Used by: `idna_adapter 1.2.2`

### MIT License

Used by: `tar 0.4.46`

### MIT License

Used by: `arrayvec 0.7.8`

### MIT License

Used by: `winresource 0.1.31`

### MIT License

Used by: `synstructure 0.14.0`

### MIT License

Used by: `ipnet 2.12.2`

### MIT License

Used by: `rand 0.10.3`, `rand_core 0.6.4`

### MIT License

Used by: `nohash-hasher 0.2.0`

### MIT License

Used by: `as-raw-xcb-connection 1.0.1`

### MIT License

Used by: `x11rb-protocol 0.13.2`, `x11rb 0.13.2`

### MIT License

Used by: `vello_common 0.1.0`, `vello_common 0.2.0`, `vello_cpu 0.1.0`, `vello_cpu 0.2.0`

### MIT License

Used by: `zerocopy-derive 0.8.61`, `zerocopy 0.8.61`

### MIT License

Used by: `glifo 0.3.0`

### MIT License

Used by: `chardetng 1.0.0`, `encoding_rs 0.8.42`, `multiversion_no_op 1.0.0`, `utf8_iter 1.0.4`

### MIT License

Used by: `wayland-protocols 0.32.13`

### MIT License

Used by: `fax 0.2.7`

### MIT License

Used by: `epaint_default_fonts 0.36.2`

### MIT License

Used by: `tree_magic_mini 3.2.2`

### MIT License

Used by: `fs_extra 1.3.0`

### MIT License

Used by: `cfb 0.15.0`

### MIT License

Used by: `static_assertions 1.1.0`

### MIT License

Used by: `tiff 0.11.3`

### MIT License

Used by: `crc32fast 1.5.2`

### MIT License

Used by: `bytemuck 1.25.2`, `bytemuck_derive 1.12.1`

### MIT License

Used by: `arbitrary 1.4.2`, `derive_arbitrary 1.4.2`

### MIT License

Used by: `raw-window-handle 0.6.2`

### MIT License

Used by: `strum 0.28.0`, `strum_macros 0.28.0`

### MIT License

Used by: `proc-macro-error-attr 1.0.4`, `proc-macro-error 1.0.4`

### MIT License

Used by: `rfd 0.17.2`

### MIT License

Used by: `cfg_aliases 0.2.2`

### MIT License

Used by: `android-properties 0.2.2`

### MIT License

Used by: `xcursor 0.3.11`

### MIT License

Used by: `sys-locale 0.3.2`

### MIT License

Used by: `rustls-platform-verifier 0.7.1`

### MIT License

Used by: `rfd 0.17.2`

### MIT License

Used by: `arboard 3.6.1`

### MIT License

Used by: `libredox 0.1.25`

### MIT License

Used by: `is-docker 0.2.0`, `is-wsl 0.4.0`

### MIT License

Used by: `range-alloc 0.1.5`

### MIT License

Used by: `web-time 1.1.0`

### MIT License

Used by: `naga-types 30.0.1`, `naga 30.0.1`, `wgpu-core-deps-apple 30.0.1`, `wgpu-core-deps-emscripten 30.0.1`, `wgpu-core-deps-wasm 30.0.1`, `wgpu-core-deps-windows-linux-android 30.0.1`, `wgpu-core 30.0.1`, `wgpu-hal 30.0.1`, `wgpu-naga-bridge 30.0.1`, `wgpu-types 30.0.1`, `wgpu 30.0.1`

### MIT License

Used by: `craft-winprint-gdi 0.1.0`

### MIT License

Used by: `fmv-macos-events 0.1.0`

### MIT License

Used by: `soratra 0.8.0`, `soratra-android 0.8.0`, `soratra-cli 0.8.0`, `soratra-web 0.8.0`, `soratra-control-key 0.8.0`, `soratra-doc 0.8.0`, `soratra-docbin 0.8.0`, `soratra-docx 0.8.0`, `soratra-engine 0.8.0`, `soratra-fonts 0.8.0`, `soratra-formats 0.8.0`, `soratra-geom 0.8.0`, `soratra-layout 0.8.0`, `soratra-mcp 0.8.0`, `soratra-metafile 0.8.0`, `soratra-pdf 0.8.0`, `soratra-proof 0.8.0`, `soratra-render 0.8.0`, `soratra-ui-egui 0.8.0`, `soratra-zotero 0.8.0`, `xtask 0.8.0`, `mahay-android 0.1.0`, `mahay-pdf 0.1.0`, `mahay-print 0.1.0`, `mahay-ui 0.1.0`, `krilla 0.8.2`, `accesskit 0.24.1`, `accesskit_atspi_common 0.18.1`, `accesskit_consumer 0.35.0`, `accesskit_consumer 0.36.0`, `accesskit_consumer 0.38.0`, `accesskit_macos 0.26.3`, `accesskit_unix 0.21.1`, `accesskit_windows 0.32.1`, `block2 0.5.1`, `block2 0.6.2`, `brotli-decompressor 5.0.3`, `cesu8 1.1.0`, `chrono 0.4.45`, `cms 0.2.3`, `dispatch2 0.3.1`, `dispatch 0.2.0`, `dpi 0.1.2`, `ecolor 0.36.2`, `eframe 0.36.2`, `egui-wgpu 0.36.2`, `egui-winit 0.36.2`, `egui 0.36.2`, `emath 0.36.2`, `epaint 0.36.2`, `fmt-derive-proc 0.1.2`, `fmt-derive 0.1.2`, `jni-macros 0.22.4`, `jni-sys-macros 0.4.1`, `jni 0.22.4`, `libm 0.2.16`, `ndk-context 0.1.1`, `ndk-sys 0.6.0+11769913`, `ndk 0.9.0`, `objc-sys 0.3.5`, `objc2-app-kit 0.2.2`, `objc2-app-kit 0.3.2`, `objc2-core-foundation 0.3.2`, `objc2-core-graphics 0.3.2`, `objc2-encode 4.1.0`, `objc2-foundation 0.2.2`, `objc2-foundation 0.3.2`, `objc2-metal 0.3.2`, `objc2-quartz-core 0.3.2`, `objc2-ui-kit 0.2.2`, `objc2-ui-kit 0.3.2`, `objc2 0.5.3`, `objc2 0.6.5`, `profiling 1.0.18`, `r-efi 5.3.0`, `r-efi 6.0.0`, `rustls-platform-verifier-android 0.2.0`, `type-map 0.5.1`, `windows-collections 0.3.2`, `windows-core 0.62.2`, `windows-future 0.3.2`, `windows-implement 0.60.2`, `windows-interface 0.59.3`, `windows-link 0.2.1`, `windows-numerics 0.3.1`, `windows-registry 0.6.1`, `windows-result 0.4.1`, `windows-strings 0.5.1`, `windows-sys 0.45.0`, `windows-sys 0.52.0`, `windows-sys 0.59.0`, `windows-sys 0.60.2`, `windows-sys 0.61.2`, `windows-targets 0.42.2`, `windows-targets 0.52.6`, `windows-targets 0.53.5`, `windows-threading 0.2.1`, `windows 0.62.2`, `windows_aarch64_gnullvm 0.42.2`, `windows_aarch64_gnullvm 0.52.6`, `windows_aarch64_gnullvm 0.53.1`, `windows_aarch64_msvc 0.42.2`, `windows_aarch64_msvc 0.52.6`, `windows_aarch64_msvc 0.53.1`, `windows_i686_gnu 0.42.2`, `windows_i686_gnu 0.52.6`, `windows_i686_gnu 0.53.1`, `windows_i686_gnullvm 0.52.6`, `windows_i686_gnullvm 0.53.1`, `windows_i686_msvc 0.42.2`, `windows_i686_msvc 0.52.6`, `windows_i686_msvc 0.53.1`, `windows_x86_64_gnu 0.42.2`, `windows_x86_64_gnu 0.52.6`, `windows_x86_64_gnu 0.53.1`, `windows_x86_64_gnullvm 0.42.2`, `windows_x86_64_gnullvm 0.52.6`, `windows_x86_64_gnullvm 0.53.1`, `windows_x86_64_msvc 0.42.2`, `windows_x86_64_msvc 0.52.6`, `windows_x86_64_msvc 0.53.1`, `wuff 0.2.9`

### MIT License

Used by: `tokio-util 0.7.20`, `tokio 1.53.2`

### MIT License

Used by: `simd-adler32 0.3.10`

### MIT License

Used by: `zune-core 0.5.3`, `zune-jpeg 0.5.15`

### MIT License

Used by: `miniz_oxide 0.8.9`, `miniz_oxide 0.9.1`

### MIT License

Used by: `miniz_oxide 0.8.9`, `miniz_oxide 0.9.1`

### MIT License

Used by: `fdeflate 0.3.7`, `image-webp 0.2.4`, `image 0.25.10`

### MIT License

Used by: `simdutf8 0.1.5`

### MIT License

Used by: `android-activity 0.6.1`, `color 0.3.3`, `half 2.7.1`, `linebender_resource_handle 0.1.1`, `pdf-writer 0.15.0`, `subsetter 0.2.6`, `xmp-writer 0.3.3`

### MIT License

Used by: `imagesize 0.14.0`

### MIT License

Used by: `unicode-script 0.5.8`

### MIT License

Used by: `serde_json_canonicalizer 0.3.2`

### MIT License

Used by: `wayland-protocols-plasma 0.3.12`

### MIT License

Used by: `rustc-hash 2.1.3`

### MIT License

Used by: `adler2 2.0.1`, `async-channel 2.5.0`, `async-executor 1.14.0`, `async-io 2.6.0`, `async-lock 3.4.2`, `async-process 2.5.0`, `async-signal 0.2.14`, `async-task 4.7.1`, `async-trait 0.1.92`, `atomic-waker 1.1.2`, `blocking 1.7.0`, `concurrent-queue 2.5.0`, `displaydoc 0.2.7`, `endi 1.1.1`, `event-listener-strategy 0.5.4`, `event-listener 5.4.2`, `fastrand 2.5.0`, `futures-lite 2.6.1`, `hermit-abi 0.5.3`, `itoa 1.0.18`, `khronos-egl 6.0.0`, `linux-raw-sys 0.12.1`, `linux-raw-sys 0.4.15`, `num_enum 0.7.6`, `num_enum_derive 0.7.6`, `once_cell 1.21.4`, `ordered-stream 0.2.0`, `parking 2.2.1`, `pin-project-internal 1.1.13`, `pin-project-lite 0.2.17`, `pin-project 1.1.13`, `piper 0.2.5`, `polling 3.11.0`, `portable-atomic-util 0.2.8`, `portable-atomic 1.15.0`, `proc-macro-crate 3.5.0`, `proc-macro2 1.0.107`, `quote 1.0.47`, `rustc-hash 1.1.0`, `rustix 0.38.44`, `rustix 1.1.5`, `rustversion 1.0.23`, `semver 1.0.28`, `serde 1.0.229`, `serde_core 1.0.229`, `serde_derive 1.0.229`, `serde_json 1.0.151`, `serde_repr 0.1.21`, `simd_cesu8 1.2.0`, `smol_str 0.2.2`, `syn 1.0.109`, `syn 2.0.119`, `syn 3.0.6`, `thiserror-impl 1.0.69`, `thiserror-impl 2.0.21`, `thiserror 1.0.69`, `thiserror 2.0.21`, `unicode-ident 1.0.26`, `wasi 0.11.1+wasi-snapshot-preview1`, `wasip2 1.0.4+wasi-0.2.12`, `wit-bindgen 0.57.1`, `x11-dl 2.21.0`, `zbus-lockstep-macros 0.5.2`, `zbus-lockstep 0.5.2`, `zmij 1.0.23`, `zvariant_utils 4.2.0`

### MIT License

Used by: `async-recursion 1.2.0`, `glow 0.17.0`

### MIT License

Used by: `allocator-api2 0.2.21`

### MIT License

Used by: `winnow 1.0.4`

### MIT License

Used by: `raw-window-metal 1.1.0`, `siphasher 1.0.4`

### MIT License

Used by: `android_system_properties 0.1.6`

### MIT License

Used by: `tracing-core 0.1.36`

### MIT License

Used by: `zip 3.0.0`

### MIT License

Used by: `typenum 1.20.1`

### MIT License

Used by: `xml-rs 0.8.29`, `xml 1.4.0`

### MIT License

Used by: `phf 0.13.1`, `phf_generator 0.13.1`, `phf_macros 0.13.1`, `phf_shared 0.13.1`

### MIT License

Used by: `base64 0.22.1`

### MIT License

Used by: `aho-corasick 1.1.5`, `byteorder-lite 0.1.0`, `memchr 2.8.3`, `walkdir 2.5.0`

### MIT License

Used by: `combine 4.6.8`

### MIT License

Used by: `shlex 2.0.1`

### MIT License

Used by: `security-framework-sys 2.17.0`, `security-framework 3.7.0`

### MIT License

Used by: `gif 0.14.2`

### MIT License

Used by: `orbclient 0.3.55`

### MIT License

Used by: `aws-lc-sys 0.46.0`

### MIT License

Used by: `webbrowser 1.2.4`

### MIT License

Used by: `pem 3.0.6`

### MIT License

Used by: `color_quant 1.1.0`

### MIT License

Used by: `jni 0.21.1`

### MIT License

Used by: `same-file 1.0.6`, `winapi-util 0.1.11`

### MIT License

Used by: `roxmltree 0.20.0`

### MIT License

Used by: `guillotiere 0.7.0`

### MIT License

Used by: `crossbeam-channel 0.5.17`, `crossbeam-deque 0.8.8`, `crossbeam-epoch 0.9.21`, `crossbeam-utils 0.8.23`

### MIT License

Used by: `async-broadcast 0.7.2`

### MIT License

Used by: `xkbcommon-dl 0.4.2`

### MIT License

Used by: `base64 0.23.1`

### MIT License

Used by: `harfrust 0.12.0`, `harfrust 0.13.3`

### MIT License

Used by: `weezl 0.1.12`

### MIT License

Used by: `crunchy 0.2.4`

### MIT License

Used by: `os_pipe 1.2.3`

### MIT License

Used by: `open 5.4.4`

### MIT License

Used by: `version_check 0.9.5`

### MIT License

Used by: `generic-array 0.14.7`

### MIT License

Used by: `quick-xml 0.41.0`

### MIT License

Used by: `ordered-channel 1.2.0`

### MIT No Attribution

Used by: `dunce 1.0.5`

### SIL Open Font License 1.1

Used by: `epaint_default_fonts 0.36.2`

### Ubuntu Font Licence v1.0

Used by: `epaint_default_fonts 0.36.2`

### Unicode License v3

Used by: `unicode-ident 1.0.26`

### Unicode License v3

Used by: `icu_collections 2.3.0`, `icu_locale_core 2.3.0`, `icu_normalizer 2.3.0`, `icu_normalizer_data 2.3.0`, `icu_properties 2.3.0`, `icu_properties_data 2.3.0`, `icu_provider 2.3.1`, `litemap 0.8.3`, `potential_utf 0.1.6`, `tinystr 0.8.4`, `writeable 0.6.4`, `yoke-derive 0.8.4`, `yoke 0.8.3`, `zerofrom-derive 0.1.8`, `zerofrom 0.1.8`, `zerotrie 0.2.5`, `zerovec-derive 0.11.6`, `zerovec 0.11.8`

### zlib License

Used by: `zlib-rs 0.6.8`

### zlib License

Used by: `slotmap 1.1.1`

### zlib License

Used by: `foldhash 0.1.5`, `foldhash 0.2.0`


## `apps/tabilao`

- **MIT License** (518)
- **Apache License 2.0** (27)
- **Unicode License v3** (19)
- **ISC License** (6)
- **BSD 3-Clause "New" or "Revised" License** (5)
- **zlib License** (4)
- **Boost Software License 1.0** (2)
- **Community Data License Agreement Permissive 2.0** (1)
- **MIT No Attribution** (1)
- **SIL Open Font License 1.1** (1)
- **Ubuntu Font Licence v1.0** (1)

### Apache License 2.0

Used by: `flagset 0.4.7`

### Apache License 2.0

Used by: `zopfli 0.8.3`

### Apache License 2.0

Used by: `moxcms 0.8.1`, `pxfm 0.1.30`

### Apache License 2.0

Used by: `codespan-reporting 0.13.1`, `self_cell 1.3.0`

### Apache License 2.0

Used by: `unicode-general-category 1.1.0`

### Apache License 2.0

Used by: `gethostname 1.1.0`

### Apache License 2.0

Used by: `glutin_wgl_sys 0.6.1`

### Apache License 2.0

Used by: `dpi 0.1.2`, `winit 0.30.13`

### Apache License 2.0

Used by: `accesskit_winit 0.32.2`, `aws-lc-sys 0.46.0`, `gl_generator 0.14.0`, `khronos_api 3.1.0`, `ryu-js 1.0.3`, `ryu 1.0.23`, `sigstore-bundle 0.6.6`, `sigstore-crypto 0.6.6`, `sigstore-merkle 0.6.6`, `sigstore-rekor 0.6.6`, `sigstore-trust-root 0.6.6`, `sigstore-tsa 0.6.6`, `sigstore-types 0.6.6`, `sigstore-verify 0.6.6`, `spirv 0.4.0+sdk-1.4.341.0`, `sync_wrapper 1.0.2`

### BSD 3-Clause "New" or "Revised" License

Used by: `encoding_rs 0.8.42`

### BSD 3-Clause "New" or "Revised" License

Used by: `winprint 0.2.1`

### BSD 3-Clause "New" or "Revised" License

Used by: `subtle 2.6.1`

### BSD 3-Clause "New" or "Revised" License

Used by: `aws-lc-sys 0.46.0`

### BSD 3-Clause "New" or "Revised" License

Used by: `encoding_rs 0.8.42`

### Boost Software License 1.0

Used by: `error-code 3.4.0`

### Boost Software License 1.0

Used by: `clipboard-win 5.4.1`

### Community Data License Agreement Permissive 2.0

Used by: `webpki-root-certs 1.0.9`

### ISC License

Used by: `untrusted 0.7.1`, `untrusted 0.9.0`

### ISC License

Used by: `libloading 0.8.9`

### ISC License

Used by: `rustls-webpki 0.103.15`

### ISC License

Used by: `aws-lc-rs 1.18.2`, `aws-lc-sys 0.46.0`

### MIT License

Used by: `uds_windows 1.2.1`

### MIT License

Used by: `cfg_aliases 0.2.2`

### MIT License

Used by: `atomic-waker 1.1.2`, `futures-lite 2.6.1`

### MIT License

Used by: `sha1 0.10.7`, `sha2 0.10.9`

### MIT License

Used by: `core-foundation-sys 0.8.7`, `core-foundation 0.10.1`, `core-foundation 0.9.4`, `core-graphics-types 0.1.3`, `core-graphics 0.23.2`, `euclid 0.22.14`

### MIT License

Used by: `hex 0.4.3`

### MIT License

Used by: `form_urlencoded 1.2.2`

### MIT License

Used by: `idna 1.1.0`, `percent-encoding 2.3.2`, `url 2.5.8`

### MIT License

Used by: `cc 1.6.0`, `cfg-if 1.0.5`, `cmake 0.1.58`, `filetime 0.2.29`, `find-msvc-tools 0.1.14`, `jobserver 0.1.35`, `js-sys 0.3.106`, `openssl-probe 0.2.1`, `pkg-config 0.3.34`, `scoped-tls 1.0.1`, `socket2 0.6.5`, `wasm-bindgen-futures 0.4.79`, `wasm-bindgen-macro-support 0.2.129`, `wasm-bindgen-macro 0.2.129`, `wasm-bindgen-shared 0.2.129`, `wasm-bindgen 0.2.129`, `web-sys 0.3.106`

### MIT License

Used by: `mio 1.2.4`

### MIT License

Used by: `errno 0.3.14`

### MIT License

Used by: `mime 0.3.17`

### MIT License

Used by: `base64ct 1.8.3`

### MIT License

Used by: `bitflags 1.3.2`, `bitflags 2.13.2`, `log 0.4.34`, `num-traits 0.2.19`, `regex-automata 0.4.18`, `regex-syntax 0.8.11`, `regex 1.13.1`

### MIT License

Used by: `uuid 1.27.0`

### MIT License

Used by: `nom 8.0.0`

### MIT License

Used by: `flate2 1.1.10`

### MIT License

Used by: `hyper 1.12.0`

### MIT License

Used by: `either 1.19.0`, `itertools 0.15.0`, `petgraph 0.8.3`

### MIT License

Used by: `wayland-backend 0.3.17`, `wayland-client 0.31.15`, `wayland-cursor 0.31.14`, `wayland-protocols-experimental 20250721.0.1`, `wayland-protocols-misc 0.3.12`, `wayland-protocols-plasma 0.3.12`, `wayland-protocols-wlr 0.3.12`, `wayland-protocols 0.32.13`, `wayland-scanner 0.31.11`, `wayland-sys 0.31.11`

### MIT License

Used by: `ordered-float 5.5.0`

### MIT License

Used by: `tempfile 3.27.0`, `xattr 1.6.1`

### MIT License

Used by: `heck 0.5.0`, `unicode-segmentation 1.13.3`, `unicode-width 0.2.2`

### MIT License

Used by: `quick-error 2.0.1`

### MIT License

Used by: `jni-sys 0.3.1`, `jni-sys 0.4.1`

### MIT License

Used by: `dlib 0.5.3`

### MIT License

Used by: `png 0.18.1`

### MIT License

Used by: `schannel 0.1.30`

### MIT License

Used by: `fixedbitset 0.5.7`

### MIT License

Used by: `httparse 1.10.1`

### MIT License

Used by: `ash 0.38.0+1.3.281`

### MIT License

Used by: `futures-channel 0.3.34`, `futures-core 0.3.34`, `futures-io 0.3.34`, `futures-macro 0.3.34`, `futures-sink 0.3.34`, `futures-task 0.3.34`, `futures-util 0.3.34`

### MIT License

Used by: `hashbrown 0.15.5`, `hashbrown 0.16.1`, `hashbrown 0.17.1`

### MIT License

Used by: `serde_urlencoded 0.7.1`

### MIT License

Used by: `hyper-rustls 0.27.10`, `rustls-native-certs 0.8.5`, `rustls 0.23.46`

### MIT License

Used by: `lock_api 0.4.14`, `parking_lot 0.12.5`, `parking_lot_core 0.9.12`, `rustc_version 0.4.1`

### MIT License

Used by: `android_log-sys 0.3.2`

### MIT License

Used by: `android_logger 0.15.1`

### MIT License

Used by: `indexmap 2.14.2`

### MIT License

Used by: `equivalent 1.0.2`

### MIT License

Used by: `scopeguard 1.2.0`

### MIT License

Used by: `cipher 0.5.2`

### MIT License

Used by: `reqwest 0.13.5`

### MIT License

Used by: `sha1 0.11.0`, `sha2 0.11.0`

### MIT License

Used by: `digest 0.10.7`

### MIT License

Used by: `fnv 1.0.7`

### MIT License

Used by: `memoffset 0.9.1`

### MIT License

Used by: `enumflags2_derive 0.7.12`

### MIT License

Used by: `plain 0.2.3`

### MIT License

Used by: `keyboard-types 0.8.3`

### MIT License

Used by: `redox_syscall 0.4.1`, `redox_syscall 0.5.18`, `redox_syscall 0.9.4`

### MIT License

Used by: `stable_deref_trait 1.2.1`

### MIT License

Used by: `foreign-types-macros 0.2.4`, `foreign-types-shared 0.3.1`, `foreign-types 0.5.0`

### MIT License

Used by: `h2 0.4.20`

### MIT License

Used by: `http 1.5.0`

### MIT License

Used by: `tokio-rustls 0.26.6`

### MIT License

Used by: `signal-hook-registry 1.4.8`

### MIT License

Used by: `core_detect 1.0.0`

### MIT License

Used by: `enumflags2 0.7.12`

### MIT License

Used by: `digest 0.11.3`

### MIT License

Used by: `bytes 1.12.1`

### MIT License

Used by: `autocfg 1.5.1`

### MIT License

Used by: `smithay-clipboard 0.7.3`

### MIT License

Used by: `fearless_simd 0.4.1`, `kurbo 0.13.1`, `peniko 0.6.1`, `polycool 0.4.0`

### MIT License

Used by: `smallvec 1.16.2`

### MIT License

Used by: `ahash 0.8.12`

### MIT License

Used by: `smithay-client-toolkit 0.19.2`, `smithay-client-toolkit 0.20.0`

### MIT License

Used by: `calloop 0.13.0`, `calloop 0.14.5`

### MIT License

Used by: `want 0.3.2`

### MIT License

Used by: `block-buffer 0.10.4`

### MIT License

Used by: `signature 2.2.0`

### MIT License

Used by: `try-lock 0.2.5`

### MIT License

Used by: `aes 0.9.3`

### MIT License

Used by: `block-buffer 0.12.1`

### MIT License

Used by: `getrandom 0.3.4`

### MIT License

Used by: `rand_core 0.10.1`

### MIT License

Used by: `zeroize 1.9.1`

### MIT License

Used by: `getrandom 0.4.3`

### MIT License

Used by: `slab 0.4.12`

### MIT License

Used by: `presser 0.3.1`

### MIT License

Used by: `font-types 0.12.6`, `read-fonts 0.41.0`, `skrifa 0.44.0`

### MIT License

Used by: `wl-clipboard-rs 0.9.4`

### MIT License

Used by: `bumpalo 3.20.3`

### MIT License

Used by: `tracing-attributes 0.1.31`, `tracing-core 0.1.36`, `tracing 0.1.44`

### MIT License

Used by: `tower-layer 0.3.3`, `tower-service 0.3.3`, `tower 0.5.3`

### MIT License

Used by: `tower-http 0.6.11`

### MIT License

Used by: `http-body-util 0.1.5`, `http-body 1.1.0`

### MIT License

Used by: `chacha20 0.10.2`, `zeroize_derive 1.5.0`

### MIT License

Used by: `winsafe 0.0.29`

### MIT License

Used by: `iana-time-zone-haiku 0.1.2`, `iana-time-zone 0.1.65`

### MIT License

Used by: `downcast-rs 1.2.1`

### MIT License

Used by: `document-features 0.2.12`

### MIT License

Used by: `litrs 1.0.0`

### MIT License

Used by: `tls_codec 0.4.2`, `tls_codec_derive 0.4.2`

### MIT License

Used by: `memmap2 0.9.11`

### MIT License

Used by: `pollster 0.4.0`, `pollster 1.0.1`

### MIT License

Used by: `const-oid 0.9.6`, `der_derive 0.7.3`

### MIT License

Used by: `der 0.7.10`

### MIT License

Used by: `cpufeatures 0.2.17`

### MIT License

Used by: `cpufeatures 0.3.1`

### MIT License

Used by: `crypto-common 0.1.7`

### MIT License

Used by: `pem-rfc7468 0.7.0`, `x509-cert 0.2.5`

### MIT License

Used by: `gpu-allocator 0.28.0`

### MIT License

Used by: `spki 0.7.3`

### MIT License

Used by: `crypto-common 0.2.2`

### MIT License

Used by: `renderdoc-sys 1.1.0`

### MIT License

Used by: `atspi-common 0.13.0`, `atspi-proxies 0.13.0`, `atspi 0.29.0`

### MIT License

Used by: `xkeysym 0.2.1`

### MIT License

Used by: `inout 0.2.2`

### MIT License

Used by: `hybrid-array 0.4.15`

### MIT License

Used by: `rustls-pki-types 1.15.1`

### MIT License

Used by: `calloop-wayland-source 0.3.0`, `calloop-wayland-source 0.4.1`

### MIT License

Used by: `cursor-icon 1.2.0`, `wayland-csd-frame 0.3.0`

### MIT License

Used by: `bit-vec 0.9.1`

### MIT License

Used by: `cmpv2 0.2.0`, `crmf 0.2.0`, `x509-tsp 0.1.0`

### MIT License

Used by: `hyper-util 0.1.21`

### MIT License

Used by: `cpubits 0.1.1`

### MIT License

Used by: `system-configuration-sys 0.6.0`, `system-configuration 0.7.0`

### MIT License

Used by: `zbus 5.19.0`, `zbus_macros 5.19.0`, `zbus_names 4.3.4`, `zbus_xml 5.2.1`, `zvariant 5.15.0`, `zvariant_derive 5.15.0`

### MIT License

Used by: `bit-set 0.10.0`

### MIT License

Used by: `zcheapstr 1.1.0`

### MIT License

Used by: `env_filter 0.1.4`, `serde_spanned 1.1.1`, `toml 1.1.6+spec-1.1.0`, `toml_datetime 1.1.1+spec-1.1.0`, `toml_edit 0.25.15+spec-1.1.0`, `toml_parser 1.1.3+spec-1.1.0`, `toml_writer 1.1.2+spec-1.1.0`

### MIT License

Used by: `libc 0.2.190`

### MIT License

Used by: `idna_adapter 1.2.2`

### MIT License

Used by: `tar 0.4.46`

### MIT License

Used by: `arrayvec 0.7.8`

### MIT License

Used by: `winresource 0.1.31`

### MIT License

Used by: `synstructure 0.14.0`

### MIT License

Used by: `ipnet 2.12.2`

### MIT License

Used by: `rand 0.10.3`, `rand_core 0.6.4`

### MIT License

Used by: `nohash-hasher 0.2.0`

### MIT License

Used by: `as-raw-xcb-connection 1.0.1`

### MIT License

Used by: `x11rb-protocol 0.13.2`, `x11rb 0.13.2`

### MIT License

Used by: `vello_common 0.1.0`, `vello_cpu 0.1.0`

### MIT License

Used by: `zerocopy-derive 0.8.61`, `zerocopy 0.8.61`

### MIT License

Used by: `encoding_rs 0.8.42`, `multiversion_no_op 1.0.0`, `utf8_iter 1.0.4`

### MIT License

Used by: `wayland-protocols 0.32.13`

### MIT License

Used by: `fax 0.2.7`

### MIT License

Used by: `epaint_default_fonts 0.36.2`

### MIT License

Used by: `tree_magic_mini 3.2.2`

### MIT License

Used by: `fs_extra 1.3.0`

### MIT License

Used by: `static_assertions 1.1.0`

### MIT License

Used by: `tiff 0.11.3`

### MIT License

Used by: `crc32fast 1.5.2`

### MIT License

Used by: `bytemuck 1.25.2`, `bytemuck_derive 1.12.1`

### MIT License

Used by: `arbitrary 1.5.0`, `derive_arbitrary 1.5.0`

### MIT License

Used by: `raw-window-handle 0.6.2`

### MIT License

Used by: `strum 0.28.0`, `strum_macros 0.28.0`

### MIT License

Used by: `proc-macro-error-attr 1.0.4`, `proc-macro-error 1.0.4`

### MIT License

Used by: `rfd 0.17.2`

### MIT License

Used by: `cfg_aliases 0.2.2`

### MIT License

Used by: `android-properties 0.2.2`

### MIT License

Used by: `xcursor 0.3.11`

### MIT License

Used by: `sys-locale 0.3.2`

### MIT License

Used by: `rustls-platform-verifier 0.7.1`

### MIT License

Used by: `rfd 0.17.2`

### MIT License

Used by: `arboard 3.6.1`

### MIT License

Used by: `muda 0.21.1`

### MIT License

Used by: `libredox 0.1.25`

### MIT License

Used by: `range-alloc 0.1.5`

### MIT License

Used by: `web-time 1.1.0`

### MIT License

Used by: `naga-types 30.0.1`, `naga 30.0.1`, `wgpu-core-deps-apple 30.0.1`, `wgpu-core-deps-emscripten 30.0.1`, `wgpu-core-deps-wasm 30.0.1`, `wgpu-core-deps-windows-linux-android 30.0.1`, `wgpu-core 30.0.1`, `wgpu-hal 30.0.1`, `wgpu-naga-bridge 30.0.1`, `wgpu-types 30.0.1`, `wgpu 30.0.1`

### MIT License

Used by: `craft-winprint-gdi 0.1.0`

### MIT License

Used by: `fmv-macos-events 0.1.0`

### MIT License

Used by: `tabilao 0.8.0`, `tabilao-android 0.8.0`, `tabilao-cli 0.8.0`, `tabilao-web 0.8.0`, `tabilao-calc 0.8.0`, `tabilao-chart 0.8.0`, `tabilao-core 0.8.0`, `tabilao-engine 0.8.0`, `tabilao-formula 0.8.0`, `tabilao-functions 0.8.0`, `tabilao-mcp 0.8.0`, `tabilao-model 0.8.0`, `tabilao-numfmt 0.8.0`, `tabilao-pdf 0.8.0`, `tabilao-ui-egui 0.8.0`, `tabilao-xlsx 0.8.0`, `xtask 0.8.0`, `mahay-android 0.1.0`, `mahay-print 0.1.0`, `mahay-ui 0.1.0`, `accesskit 0.24.1`, `accesskit_atspi_common 0.18.1`, `accesskit_consumer 0.35.0`, `accesskit_consumer 0.36.0`, `accesskit_consumer 0.38.0`, `accesskit_macos 0.26.3`, `accesskit_unix 0.21.1`, `accesskit_windows 0.32.1`, `block2 0.5.1`, `block2 0.6.2`, `cesu8 1.1.0`, `chrono 0.4.45`, `cms 0.2.3`, `dispatch2 0.3.1`, `dispatch 0.2.0`, `dpi 0.1.2`, `ecolor 0.36.2`, `eframe 0.36.2`, `egui-wgpu 0.36.2`, `egui-winit 0.36.2`, `egui 0.36.2`, `emath 0.36.2`, `epaint 0.36.2`, `fmt-derive-proc 0.1.2`, `fmt-derive 0.1.2`, `jni-macros 0.22.4`, `jni-sys-macros 0.4.1`, `jni 0.22.4`, `libm 0.2.16`, `ndk-context 0.1.1`, `ndk-sys 0.6.0+11769913`, `ndk 0.9.0`, `objc-sys 0.3.5`, `objc2-app-kit 0.2.2`, `objc2-app-kit 0.3.2`, `objc2-core-foundation 0.3.2`, `objc2-core-graphics 0.3.2`, `objc2-encode 4.1.0`, `objc2-foundation 0.2.2`, `objc2-foundation 0.3.2`, `objc2-metal 0.3.2`, `objc2-quartz-core 0.3.2`, `objc2-ui-kit 0.2.2`, `objc2-ui-kit 0.3.2`, `objc2 0.5.3`, `objc2 0.6.5`, `profiling 1.0.18`, `r-efi 5.3.0`, `r-efi 6.0.0`, `rustls-platform-verifier-android 0.2.0`, `type-map 0.5.1`, `windows-collections 0.3.2`, `windows-core 0.62.2`, `windows-future 0.3.2`, `windows-implement 0.60.2`, `windows-interface 0.59.3`, `windows-link 0.2.1`, `windows-numerics 0.3.1`, `windows-registry 0.6.1`, `windows-result 0.4.1`, `windows-strings 0.5.1`, `windows-sys 0.45.0`, `windows-sys 0.52.0`, `windows-sys 0.59.0`, `windows-sys 0.61.2`, `windows-targets 0.42.2`, `windows-targets 0.52.6`, `windows-threading 0.2.1`, `windows 0.62.2`, `windows_aarch64_gnullvm 0.42.2`, `windows_aarch64_gnullvm 0.52.6`, `windows_aarch64_msvc 0.42.2`, `windows_aarch64_msvc 0.52.6`, `windows_i686_gnu 0.42.2`, `windows_i686_gnu 0.52.6`, `windows_i686_gnullvm 0.52.6`, `windows_i686_msvc 0.42.2`, `windows_i686_msvc 0.52.6`, `windows_x86_64_gnu 0.42.2`, `windows_x86_64_gnu 0.52.6`, `windows_x86_64_gnullvm 0.42.2`, `windows_x86_64_gnullvm 0.52.6`, `windows_x86_64_msvc 0.42.2`, `windows_x86_64_msvc 0.52.6`

### MIT License

Used by: `tokio-util 0.7.20`, `tokio 1.53.2`

### MIT License

Used by: `simd-adler32 0.3.10`

### MIT License

Used by: `zune-core 0.5.3`, `zune-jpeg 0.5.15`

### MIT License

Used by: `miniz_oxide 0.8.9`, `miniz_oxide 0.9.1`

### MIT License

Used by: `miniz_oxide 0.8.9`, `miniz_oxide 0.9.1`

### MIT License

Used by: `fdeflate 0.3.7`, `image 0.25.10`

### MIT License

Used by: `simdutf8 0.1.5`

### MIT License

Used by: `android-activity 0.6.1`, `color 0.3.3`, `half 2.7.1`, `linebender_resource_handle 0.1.1`

### MIT License

Used by: `serde_json_canonicalizer 0.3.2`

### MIT License

Used by: `wayland-protocols-plasma 0.3.12`

### MIT License

Used by: `rustc-hash 2.1.3`

### MIT License

Used by: `adler2 2.0.1`, `async-channel 2.5.0`, `async-executor 1.14.0`, `async-io 2.6.0`, `async-lock 3.4.2`, `async-process 2.5.0`, `async-signal 0.2.14`, `async-task 4.7.1`, `async-trait 0.1.92`, `atomic-waker 1.1.2`, `blocking 1.7.0`, `concurrent-queue 2.5.0`, `displaydoc 0.2.7`, `endi 1.1.1`, `event-listener-strategy 0.5.4`, `event-listener 5.4.2`, `fastrand 2.5.0`, `futures-lite 2.6.1`, `hermit-abi 0.5.3`, `itoa 1.0.18`, `khronos-egl 6.0.0`, `linux-raw-sys 0.12.1`, `linux-raw-sys 0.4.15`, `num_enum 0.7.6`, `num_enum_derive 0.7.6`, `once_cell 1.21.4`, `ordered-stream 0.2.0`, `parking 2.2.1`, `pin-project-internal 1.1.13`, `pin-project-lite 0.2.17`, `pin-project 1.1.13`, `piper 0.2.5`, `polling 3.11.0`, `portable-atomic-util 0.2.8`, `portable-atomic 1.15.0`, `proc-macro-crate 3.5.0`, `proc-macro2 1.0.107`, `quote 1.0.47`, `rustc-hash 1.1.0`, `rustix 0.38.44`, `rustix 1.1.5`, `rustversion 1.0.23`, `semver 1.0.28`, `serde 1.0.229`, `serde_core 1.0.229`, `serde_derive 1.0.229`, `serde_json 1.0.151`, `serde_repr 0.1.21`, `simd_cesu8 1.2.0`, `smol_str 0.2.2`, `syn 1.0.109`, `syn 2.0.119`, `syn 3.0.6`, `thiserror-impl 1.0.69`, `thiserror-impl 2.0.21`, `thiserror 1.0.69`, `thiserror 2.0.21`, `unicode-ident 1.0.26`, `wasi 0.11.1+wasi-snapshot-preview1`, `wasip2 1.0.4+wasi-0.2.12`, `wit-bindgen 0.57.1`, `x11-dl 2.21.0`, `zbus-lockstep-macros 0.5.2`, `zbus-lockstep 0.5.2`, `zmij 1.0.23`, `zvariant_utils 4.2.0`

### MIT License

Used by: `async-recursion 1.2.0`, `glow 0.17.0`

### MIT License

Used by: `allocator-api2 0.2.21`

### MIT License

Used by: `winnow 1.0.4`

### MIT License

Used by: `raw-window-metal 1.1.0`, `siphasher 1.0.4`

### MIT License

Used by: `android_system_properties 0.1.6`

### MIT License

Used by: `tracing-core 0.1.36`

### MIT License

Used by: `zip 3.0.0`

### MIT License

Used by: `typenum 1.20.1`

### MIT License

Used by: `xml-rs 0.8.29`, `xml 1.4.0`

### MIT License

Used by: `phf 0.13.1`, `phf_generator 0.13.1`, `phf_macros 0.13.1`, `phf_shared 0.13.1`

### MIT License

Used by: `base64 0.22.1`

### MIT License

Used by: `aho-corasick 1.1.5`, `byteorder-lite 0.1.0`, `memchr 2.8.3`, `walkdir 2.5.0`

### MIT License

Used by: `combine 4.6.8`

### MIT License

Used by: `shlex 2.0.1`

### MIT License

Used by: `security-framework-sys 2.17.0`, `security-framework 3.7.0`

### MIT License

Used by: `orbclient 0.3.55`

### MIT License

Used by: `aws-lc-sys 0.46.0`

### MIT License

Used by: `pem 3.0.6`

### MIT License

Used by: `jni 0.21.1`

### MIT License

Used by: `same-file 1.0.6`, `winapi-util 0.1.11`

### MIT License

Used by: `roxmltree 0.20.0`

### MIT License

Used by: `guillotiere 0.7.0`

### MIT License

Used by: `crossbeam-channel 0.5.17`, `crossbeam-utils 0.8.23`

### MIT License

Used by: `async-broadcast 0.7.2`

### MIT License

Used by: `xkbcommon-dl 0.4.2`

### MIT License

Used by: `base64 0.23.1`

### MIT License

Used by: `harfrust 0.12.0`

### MIT License

Used by: `weezl 0.1.12`

### MIT License

Used by: `crunchy 0.2.4`

### MIT License

Used by: `os_pipe 1.2.3`

### MIT License

Used by: `version_check 0.9.5`

### MIT License

Used by: `generic-array 0.14.7`

### MIT License

Used by: `quick-xml 0.41.0`

### MIT No Attribution

Used by: `dunce 1.0.5`

### SIL Open Font License 1.1

Used by: `epaint_default_fonts 0.36.2`

### Ubuntu Font Licence v1.0

Used by: `epaint_default_fonts 0.36.2`

### Unicode License v3

Used by: `unicode-ident 1.0.26`

### Unicode License v3

Used by: `icu_collections 2.3.0`, `icu_locale_core 2.3.0`, `icu_normalizer 2.3.0`, `icu_normalizer_data 2.3.0`, `icu_properties 2.3.0`, `icu_properties_data 2.3.0`, `icu_provider 2.3.1`, `litemap 0.8.3`, `potential_utf 0.1.6`, `tinystr 0.8.4`, `writeable 0.6.4`, `yoke-derive 0.8.4`, `yoke 0.8.3`, `zerofrom-derive 0.1.8`, `zerofrom 0.1.8`, `zerotrie 0.2.5`, `zerovec-derive 0.11.6`, `zerovec 0.11.8`

### zlib License

Used by: `zlib-rs 0.6.8`

### zlib License

Used by: `slotmap 1.1.1`

### zlib License

Used by: `foldhash 0.1.5`, `foldhash 0.2.0`


## `apps/seho`

- **MIT License** (481)
- **Unicode License v3** (19)
- **Apache License 2.0** (17)
- **Mozilla Public License 2.0** (16)
- **BSD 3-Clause "New" or "Revised" License** (3)
- **zlib License** (3)
- **Boost Software License 1.0** (2)
- **BSD 2-Clause "Simplified" License** (1)
- **ISC License** (1)
- **SIL Open Font License 1.1** (1)
- **Ubuntu Font Licence v1.0** (1)

### Apache License 2.0

Used by: `zopfli 0.8.3`

### Apache License 2.0

Used by: `moxcms 0.8.1`, `pxfm 0.1.30`

### Apache License 2.0

Used by: `codespan-reporting 0.13.1`, `cpal 0.16.0`, `self_cell 1.3.0`, `unicode-linebreak 0.1.5`

### Apache License 2.0

Used by: `unicode-general-category 1.1.0`

### Apache License 2.0

Used by: `gethostname 1.1.0`

### Apache License 2.0

Used by: `glutin_wgl_sys 0.6.1`

### Apache License 2.0

Used by: `dpi 0.1.2`, `winit 0.30.13`

### Apache License 2.0

Used by: `accesskit_winit 0.32.2`, `gl_generator 0.14.0`, `khronos_api 3.1.0`, `ryu 1.0.23`, `spirv 0.4.0+sdk-1.4.341.0`

### BSD 2-Clause "Simplified" License

Used by: `arrayref 0.3.9`

### BSD 3-Clause "New" or "Revised" License

Used by: `encoding_rs 0.8.42`

### BSD 3-Clause "New" or "Revised" License

Used by: `tiny-skia-path 0.12.0`

### BSD 3-Clause "New" or "Revised" License

Used by: `encoding_rs 0.8.42`

### Boost Software License 1.0

Used by: `error-code 3.4.0`

### Boost Software License 1.0

Used by: `clipboard-win 5.4.1`

### ISC License

Used by: `libloading 0.8.9`

### MIT License

Used by: `rustybuzz 0.20.1`

### MIT License

Used by: `uds_windows 1.2.1`

### MIT License

Used by: `cfg_aliases 0.2.2`

### MIT License

Used by: `atomic-waker 1.1.2`, `futures-lite 2.6.1`

### MIT License

Used by: `rayon-core 1.13.0`, `rayon 1.12.0`

### MIT License

Used by: `core-foundation-sys 0.8.7`, `core-foundation 0.9.4`, `core-graphics-types 0.1.3`, `core-graphics 0.23.2`, `euclid 0.22.14`

### MIT License

Used by: `hex 0.4.3`

### MIT License

Used by: `form_urlencoded 1.2.2`

### MIT License

Used by: `idna 1.1.0`, `percent-encoding 2.3.2`, `url 2.5.8`

### MIT License

Used by: `cc 1.6.0`, `cfg-if 1.0.5`, `find-msvc-tools 0.1.14`, `jobserver 0.1.35`, `js-sys 0.3.106`, `pkg-config 0.3.34`, `scoped-tls 1.0.1`, `wasm-bindgen-futures 0.4.79`, `wasm-bindgen-macro-support 0.2.129`, `wasm-bindgen-macro 0.2.129`, `wasm-bindgen-shared 0.2.129`, `wasm-bindgen 0.2.129`, `web-sys 0.3.106`

### MIT License

Used by: `errno 0.3.14`

### MIT License

Used by: `bitflags 1.3.2`, `bitflags 2.13.2`, `log 0.4.34`, `num-derive 0.4.2`, `num-traits 0.2.19`

### MIT License

Used by: `uuid 1.27.0`

### MIT License

Used by: `float-cmp 0.9.0`

### MIT License

Used by: `flate2 1.1.10`

### MIT License

Used by: `coreaudio-rs 0.13.0`, `either 1.19.0`, `itertools 0.15.0`

### MIT License

Used by: `wayland-backend 0.3.17`, `wayland-client 0.31.15`, `wayland-cursor 0.31.14`, `wayland-protocols-experimental 20250721.0.1`, `wayland-protocols-misc 0.3.12`, `wayland-protocols-plasma 0.3.12`, `wayland-protocols-wlr 0.3.12`, `wayland-protocols 0.32.13`, `wayland-scanner 0.31.11`, `wayland-sys 0.31.11`

### MIT License

Used by: `ordered-float 5.5.0`

### MIT License

Used by: `tempfile 3.27.0`

### MIT License

Used by: `unicode-bidi 0.3.18`, `unicode-properties 0.1.4`, `unicode-segmentation 1.13.3`, `unicode-width 0.2.2`

### MIT License

Used by: `quick-error 2.0.1`

### MIT License

Used by: `jni-sys 0.3.1`, `jni-sys 0.4.1`

### MIT License

Used by: `dlib 0.5.3`

### MIT License

Used by: `png 0.18.1`

### MIT License

Used by: `ash 0.38.0+1.3.281`

### MIT License

Used by: `futures-core 0.3.34`, `futures-io 0.3.34`, `futures-macro 0.3.34`, `futures-task 0.3.34`, `futures-util 0.3.34`

### MIT License

Used by: `hashbrown 0.16.1`, `hashbrown 0.17.1`

### MIT License

Used by: `lock_api 0.4.14`, `parking_lot 0.12.5`, `parking_lot_core 0.9.12`, `rustc_version 0.4.1`, `thread_local 1.1.10`

### MIT License

Used by: `android_log-sys 0.3.2`

### MIT License

Used by: `android_logger 0.15.1`

### MIT License

Used by: `indexmap 2.14.2`

### MIT License

Used by: `equivalent 1.0.2`

### MIT License

Used by: `scopeguard 1.2.0`

### MIT License

Used by: `memoffset 0.9.1`

### MIT License

Used by: `enumflags2_derive 0.7.12`

### MIT License

Used by: `plain 0.2.3`

### MIT License

Used by: `redox_syscall 0.4.1`, `redox_syscall 0.5.18`, `redox_syscall 0.9.4`

### MIT License

Used by: `stable_deref_trait 1.2.1`

### MIT License

Used by: `foreign-types-macros 0.2.4`, `foreign-types-shared 0.3.1`, `foreign-types 0.5.0`

### MIT License

Used by: `signal-hook-registry 1.4.8`

### MIT License

Used by: `core_detect 1.0.0`

### MIT License

Used by: `enumflags2 0.7.12`

### MIT License

Used by: `bytes 1.12.1`

### MIT License

Used by: `autocfg 1.5.1`

### MIT License

Used by: `smithay-clipboard 0.7.3`

### MIT License

Used by: `fearless_simd 0.4.1`, `kurbo 0.13.1`, `peniko 0.6.1`, `polycool 0.4.0`

### MIT License

Used by: `smallvec 1.16.2`

### MIT License

Used by: `ahash 0.8.12`

### MIT License

Used by: `smithay-client-toolkit 0.19.2`, `smithay-client-toolkit 0.20.0`

### MIT License

Used by: `calloop 0.13.0`, `calloop 0.14.5`

### MIT License

Used by: `ttf-parser 0.25.1`

### MIT License

Used by: `getrandom 0.3.4`

### MIT License

Used by: `getrandom 0.4.3`

### MIT License

Used by: `slab 0.4.12`

### MIT License

Used by: `font-types 0.11.3`, `read-fonts 0.39.2`, `skrifa 0.42.1`, `write-fonts 0.48.1`

### MIT License

Used by: `presser 0.3.1`

### MIT License

Used by: `font-types 0.12.6`, `read-fonts 0.41.0`, `read-fonts 0.43.3`, `read-fonts 0.44.0`, `skrifa 0.44.0`, `skrifa 0.47.0`

### MIT License

Used by: `bumpalo 3.20.3`

### MIT License

Used by: `mach2 0.4.3`

### MIT License

Used by: `tracing-attributes 0.1.31`, `tracing-core 0.1.36`, `tracing 0.1.44`

### MIT License

Used by: `iana-time-zone-haiku 0.1.2`, `iana-time-zone 0.1.65`

### MIT License

Used by: `downcast-rs 1.2.1`

### MIT License

Used by: `document-features 0.2.12`

### MIT License

Used by: `litrs 1.0.0`

### MIT License

Used by: `memmap2 0.9.11`

### MIT License

Used by: `pollster 0.4.0`, `pollster 1.0.1`

### MIT License

Used by: `gpu-allocator 0.28.0`

### MIT License

Used by: `renderdoc-sys 1.1.0`

### MIT License

Used by: `atspi-common 0.13.0`, `atspi-proxies 0.13.0`, `atspi 0.29.0`

### MIT License

Used by: `strict-num 0.1.1`

### MIT License

Used by: `xkeysym 0.2.1`

### MIT License

Used by: `calloop-wayland-source 0.3.0`, `calloop-wayland-source 0.4.1`

### MIT License

Used by: `cursor-icon 1.2.0`, `wayland-csd-frame 0.3.0`

### MIT License

Used by: `bit-vec 0.9.1`

### MIT License

Used by: `zbus 5.19.0`, `zbus_macros 5.19.0`, `zbus_names 4.3.4`, `zbus_xml 5.2.1`, `zvariant 5.15.0`, `zvariant_derive 5.15.0`

### MIT License

Used by: `bit-set 0.10.0`

### MIT License

Used by: `zcheapstr 1.1.0`

### MIT License

Used by: `env_filter 0.1.4`, `serde_spanned 1.1.1`, `toml 1.1.6+spec-1.1.0`, `toml_datetime 1.1.1+spec-1.1.0`, `toml_edit 0.25.15+spec-1.1.0`, `toml_parser 1.1.3+spec-1.1.0`, `toml_writer 1.1.2+spec-1.1.0`

### MIT License

Used by: `libc 0.2.190`

### MIT License

Used by: `idna_adapter 1.2.2`

### MIT License

Used by: `arrayvec 0.7.8`

### MIT License

Used by: `winresource 0.1.31`

### MIT License

Used by: `synstructure 0.14.0`

### MIT License

Used by: `nohash-hasher 0.2.0`

### MIT License

Used by: `as-raw-xcb-connection 1.0.1`

### MIT License

Used by: `x11rb-protocol 0.13.2`, `x11rb 0.13.2`

### MIT License

Used by: `vello_common 0.1.0`, `vello_common 0.2.0`, `vello_cpu 0.1.0`, `vello_cpu 0.2.0`

### MIT License

Used by: `extended 0.1.0`

### MIT License

Used by: `zerocopy-derive 0.8.61`, `zerocopy 0.8.61`

### MIT License

Used by: `glifo 0.3.0`

### MIT License

Used by: `encoding_rs 0.8.42`, `multiversion_no_op 1.0.0`, `utf8_iter 1.0.4`

### MIT License

Used by: `wayland-protocols 0.32.13`

### MIT License

Used by: `fax 0.2.7`

### MIT License

Used by: `epaint_default_fonts 0.36.2`

### MIT License

Used by: `alsa 0.9.1`

### MIT License

Used by: `static_assertions 1.1.0`

### MIT License

Used by: `tiff 0.11.3`

### MIT License

Used by: `crc32fast 1.5.2`

### MIT License

Used by: `alsa-sys 0.3.1`

### MIT License

Used by: `bytemuck 1.25.2`, `bytemuck_derive 1.12.1`

### MIT License

Used by: `arbitrary 1.5.0`, `derive_arbitrary 1.5.0`

### MIT License

Used by: `raw-window-handle 0.6.2`

### MIT License

Used by: `rfd 0.17.2`

### MIT License

Used by: `cfg_aliases 0.2.2`

### MIT License

Used by: `android-properties 0.2.2`

### MIT License

Used by: `xcursor 0.3.11`

### MIT License

Used by: `rfd 0.17.2`

### MIT License

Used by: `arboard 3.6.1`

### MIT License

Used by: `libredox 0.1.25`

### MIT License

Used by: `range-alloc 0.1.5`

### MIT License

Used by: `web-time 1.1.0`

### MIT License

Used by: `i_float 5.0.0`, `i_shape 5.0.0`

### MIT License

Used by: `core_maths 0.1.1`

### MIT License

Used by: `i_tree 0.19.0`

### MIT License

Used by: `naga-types 30.0.1`, `naga 30.0.1`, `wgpu-core-deps-apple 30.0.1`, `wgpu-core-deps-emscripten 30.0.1`, `wgpu-core-deps-wasm 30.0.1`, `wgpu-core-deps-windows-linux-android 30.0.1`, `wgpu-core 30.0.1`, `wgpu-hal 30.0.1`, `wgpu-naga-bridge 30.0.1`, `wgpu-types 30.0.1`, `wgpu 30.0.1`

### MIT License

Used by: `seho 0.8.0`, `seho-android 0.8.0`, `seho-cli 0.8.0`, `seho-web 0.8.0`, `seho-anim 0.8.0`, `seho-av1 0.8.0`, `seho-bitstream 0.8.0`, `seho-color 0.8.0`, `seho-engine 0.8.0`, `seho-fonts 0.8.0`, `seho-format 0.8.0`, `seho-geom 0.8.0`, `seho-h264 0.8.0`, `seho-hevc 0.8.0`, `seho-isobmff 0.8.0`, `seho-matroska 0.8.0`, `seho-mcp 0.8.0`, `seho-media 0.8.0`, `seho-metafile 0.8.0`, `seho-model 0.8.0`, `seho-ogg 0.8.0`, `seho-opus 0.8.0`, `seho-pdf 0.8.0`, `seho-pptx 0.8.0`, `seho-render 0.8.0`, `seho-text 0.8.0`, `seho-ui-egui 0.8.0`, `seho-vp9 0.8.0`, `xtask 0.8.0`, `mahay-android 0.1.0`, `mahay-pdf 0.1.0`, `krilla 0.8.2`, `accesskit 0.24.1`, `accesskit_atspi_common 0.18.1`, `accesskit_consumer 0.35.0`, `accesskit_consumer 0.36.0`, `accesskit_consumer 0.38.0`, `accesskit_macos 0.26.3`, `accesskit_unix 0.21.1`, `accesskit_windows 0.32.1`, `block2 0.5.1`, `block2 0.6.2`, `cesu8 1.1.0`, `chrono 0.4.45`, `dasp_sample 0.11.0`, `dispatch2 0.3.1`, `dispatch 0.2.0`, `dpi 0.1.2`, `ecolor 0.36.2`, `eframe 0.36.2`, `egui-wgpu 0.36.2`, `egui-winit 0.36.2`, `egui 0.36.2`, `emath 0.36.2`, `epaint 0.36.2`, `i_key_sort 0.11.0`, `i_overlay 9.0.0`, `jni-macros 0.22.4`, `jni-sys-macros 0.4.1`, `jni 0.22.4`, `libm 0.2.16`, `ndk-context 0.1.1`, `ndk-sys 0.6.0+11769913`, `ndk 0.9.0`, `objc-sys 0.3.5`, `objc2-app-kit 0.2.2`, `objc2-app-kit 0.3.2`, `objc2-audio-toolbox 0.3.2`, `objc2-core-audio-types 0.3.2`, `objc2-core-audio 0.3.2`, `objc2-core-foundation 0.3.2`, `objc2-core-graphics 0.3.2`, `objc2-encode 4.1.0`, `objc2-foundation 0.2.2`, `objc2-foundation 0.3.2`, `objc2-metal 0.3.2`, `objc2-quartz-core 0.3.2`, `objc2-ui-kit 0.2.2`, `objc2-ui-kit 0.3.2`, `objc2 0.5.3`, `objc2 0.6.5`, `profiling 1.0.18`, `r-efi 5.3.0`, `r-efi 6.0.0`, `type-map 0.5.1`, `windows-collections 0.3.2`, `windows-core 0.54.0`, `windows-core 0.62.2`, `windows-future 0.3.2`, `windows-implement 0.60.2`, `windows-interface 0.59.3`, `windows-link 0.2.1`, `windows-numerics 0.3.1`, `windows-result 0.1.2`, `windows-result 0.4.1`, `windows-strings 0.5.1`, `windows-sys 0.45.0`, `windows-sys 0.52.0`, `windows-sys 0.59.0`, `windows-sys 0.60.2`, `windows-sys 0.61.2`, `windows-targets 0.42.2`, `windows-targets 0.52.6`, `windows-targets 0.53.5`, `windows-threading 0.2.1`, `windows 0.54.0`, `windows 0.62.2`, `windows_aarch64_gnullvm 0.42.2`, `windows_aarch64_gnullvm 0.52.6`, `windows_aarch64_gnullvm 0.53.1`, `windows_aarch64_msvc 0.42.2`, `windows_aarch64_msvc 0.52.6`, `windows_aarch64_msvc 0.53.1`, `windows_i686_gnu 0.42.2`, `windows_i686_gnu 0.52.6`, `windows_i686_gnu 0.53.1`, `windows_i686_gnullvm 0.52.6`, `windows_i686_gnullvm 0.53.1`, `windows_i686_msvc 0.42.2`, `windows_i686_msvc 0.52.6`, `windows_i686_msvc 0.53.1`, `windows_x86_64_gnu 0.42.2`, `windows_x86_64_gnu 0.52.6`, `windows_x86_64_gnu 0.53.1`, `windows_x86_64_gnullvm 0.42.2`, `windows_x86_64_gnullvm 0.52.6`, `windows_x86_64_gnullvm 0.53.1`, `windows_x86_64_msvc 0.42.2`, `windows_x86_64_msvc 0.52.6`, `windows_x86_64_msvc 0.53.1`

### MIT License

Used by: `tokio 1.53.2`

### MIT License

Used by: `simd-adler32 0.3.10`

### MIT License

Used by: `zune-core 0.5.3`, `zune-jpeg 0.5.15`

### MIT License

Used by: `miniz_oxide 0.8.9`, `miniz_oxide 0.9.1`

### MIT License

Used by: `miniz_oxide 0.8.9`, `miniz_oxide 0.9.1`

### MIT License

Used by: `fdeflate 0.3.7`, `image-webp 0.2.4`, `image 0.25.10`

### MIT License

Used by: `simdutf8 0.1.5`

### MIT License

Used by: `android-activity 0.6.1`, `color 0.3.3`, `half 2.7.1`, `linebender_resource_handle 0.1.1`, `pdf-writer 0.15.0`, `subsetter 0.2.6`, `xmp-writer 0.3.3`

### MIT License

Used by: `imagesize 0.14.0`

### MIT License

Used by: `unicode-script 0.5.8`

### MIT License

Used by: `wayland-protocols-plasma 0.3.12`

### MIT License

Used by: `rustc-hash 2.1.3`

### MIT License

Used by: `adler2 2.0.1`, `async-channel 2.5.0`, `async-executor 1.14.0`, `async-io 2.6.0`, `async-lock 3.4.2`, `async-process 2.5.0`, `async-signal 0.2.14`, `async-task 4.7.1`, `async-trait 0.1.92`, `atomic-waker 1.1.2`, `blocking 1.7.0`, `concurrent-queue 2.5.0`, `displaydoc 0.2.7`, `endi 1.1.1`, `event-listener-strategy 0.5.4`, `event-listener 5.4.2`, `fastrand 2.5.0`, `futures-lite 2.6.1`, `hermit-abi 0.5.3`, `itoa 1.0.18`, `khronos-egl 6.0.0`, `lazy_static 1.5.1`, `linux-raw-sys 0.12.1`, `linux-raw-sys 0.4.15`, `num_enum 0.7.6`, `num_enum_derive 0.7.6`, `once_cell 1.21.4`, `ordered-stream 0.2.0`, `parking 2.2.1`, `pin-project-internal 1.1.13`, `pin-project-lite 0.2.17`, `pin-project 1.1.13`, `piper 0.2.5`, `polling 3.11.0`, `portable-atomic-util 0.2.8`, `portable-atomic 1.15.0`, `proc-macro-crate 3.5.0`, `proc-macro2 1.0.107`, `quote 1.0.47`, `rustc-hash 1.1.0`, `rustix 0.38.44`, `rustix 1.1.5`, `rustversion 1.0.23`, `semver 1.0.28`, `serde 1.0.229`, `serde_core 1.0.229`, `serde_derive 1.0.229`, `serde_json 1.0.151`, `serde_repr 0.1.21`, `simd_cesu8 1.2.0`, `smol_str 0.2.2`, `syn 2.0.119`, `syn 3.0.6`, `thiserror-impl 1.0.69`, `thiserror-impl 2.0.21`, `thiserror 1.0.69`, `thiserror 2.0.21`, `unicode-ident 1.0.26`, `wasip2 1.0.4+wasi-0.2.12`, `wit-bindgen 0.57.1`, `x11-dl 2.21.0`, `zbus-lockstep-macros 0.5.2`, `zbus-lockstep 0.5.2`, `zmij 1.0.23`, `zvariant_utils 4.2.0`

### MIT License

Used by: `async-recursion 1.2.0`, `glow 0.17.0`

### MIT License

Used by: `allocator-api2 0.2.21`

### MIT License

Used by: `winnow 1.0.4`

### MIT License

Used by: `raw-window-metal 1.1.0`, `siphasher 1.0.4`

### MIT License

Used by: `android_system_properties 0.1.6`

### MIT License

Used by: `tracing-core 0.1.36`

### MIT License

Used by: `zip 3.0.0`

### MIT License

Used by: `xml-rs 0.8.29`

### MIT License

Used by: `phf 0.13.1`, `phf_generator 0.13.1`, `phf_macros 0.13.1`, `phf_shared 0.13.1`

### MIT License

Used by: `base64 0.22.1`

### MIT License

Used by: `byteorder-lite 0.1.0`, `memchr 2.8.3`, `walkdir 2.5.0`

### MIT License

Used by: `combine 4.6.8`

### MIT License

Used by: `shlex 2.0.1`

### MIT License

Used by: `gif 0.14.2`

### MIT License

Used by: `orbclient 0.3.55`

### MIT License

Used by: `webbrowser 1.2.4`

### MIT License

Used by: `color_quant 1.1.0`

### MIT License

Used by: `jni 0.21.1`

### MIT License

Used by: `same-file 1.0.6`, `winapi-util 0.1.11`

### MIT License

Used by: `guillotiere 0.7.0`

### MIT License

Used by: `crossbeam-channel 0.5.17`, `crossbeam-deque 0.8.8`, `crossbeam-epoch 0.9.21`, `crossbeam-utils 0.8.23`

### MIT License

Used by: `unicode-bidi-mirroring 0.4.0`, `unicode-ccc 0.4.0`

### MIT License

Used by: `async-broadcast 0.7.2`

### MIT License

Used by: `xkbcommon-dl 0.4.2`

### MIT License

Used by: `harfrust 0.12.0`, `harfrust 0.13.3`, `rustybuzz 0.20.1`

### MIT License

Used by: `weezl 0.1.12`

### MIT License

Used by: `crunchy 0.2.4`

### MIT License

Used by: `version_check 0.9.5`

### MIT License

Used by: `quick-xml 0.41.0`

### MIT License

Used by: `ordered-channel 1.2.0`

### Mozilla Public License 2.0

Used by: `symphonia-bundle-flac 0.5.5`, `symphonia-bundle-mp3 0.5.5`, `symphonia-codec-aac 0.5.5`, `symphonia-codec-adpcm 0.5.5`, `symphonia-codec-alac 0.5.5`, `symphonia-codec-pcm 0.5.5`, `symphonia-codec-vorbis 0.5.5`, `symphonia-core 0.5.5`, `symphonia-format-caf 0.5.5`, `symphonia-format-isomp4 0.5.5`, `symphonia-format-mkv 0.5.5`, `symphonia-format-ogg 0.5.5`, `symphonia-format-riff 0.5.5`, `symphonia-metadata 0.5.5`, `symphonia-utils-xiph 0.5.5`, `symphonia 0.5.5`

### SIL Open Font License 1.1

Used by: `epaint_default_fonts 0.36.2`

### Ubuntu Font Licence v1.0

Used by: `epaint_default_fonts 0.36.2`

### Unicode License v3

Used by: `unicode-ident 1.0.26`

### Unicode License v3

Used by: `icu_collections 2.3.0`, `icu_locale_core 2.3.0`, `icu_normalizer 2.3.0`, `icu_normalizer_data 2.3.0`, `icu_properties 2.3.0`, `icu_properties_data 2.3.0`, `icu_provider 2.3.1`, `litemap 0.8.3`, `potential_utf 0.1.6`, `tinystr 0.8.4`, `writeable 0.6.4`, `yoke-derive 0.8.4`, `yoke 0.8.3`, `zerofrom-derive 0.1.8`, `zerofrom 0.1.8`, `zerotrie 0.2.5`, `zerovec-derive 0.11.6`, `zerovec 0.11.8`

### zlib License

Used by: `zlib-rs 0.6.8`

### zlib License

Used by: `slotmap 1.1.1`

### zlib License

Used by: `foldhash 0.2.0`


