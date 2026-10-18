# Hazavana — the desktop app (`hazavana`) and the CLI/MCP server (`hazavana-cli`), built from
# this repository:
#
#   nix build          # → ./result/bin/hazavana, ./result/bin/hazavana-cli
#
# Kept in step with the release workflow (.github/workflows/release.yml) and packaging/env.sh:
#   * `Cargo.lock` drives the dependencies (`cargoLock`), so there is no vendorHash to bump. The git
#     source (raantss18/mahay-libs) is fetched at the revision the lock file pins;
#   * `MAHAY_FONTS_DIR` embeds the CJK fonts from raantss18/mahay-fonts (the `mahay-fonts`
#     flake input); without it everything builds and runs, but Japanese and Chinese text have no glyphs;
#   * the desktop file, hicolor icons and AppStream metadata are the same files the .deb/.rpm ship
#     (packaging/linux/), so `apt` and NixOS users see one identical Hazavana;
#   * `doCheck` runs `cargo test --workspace`, what `cargo xtask ci` runs. `nix build` runs it too;
#     `pkgs.hazavana.overrideAttrs { doCheck = false; }` skips it for a faster, build-only install.
#
# Everything in the product is pure Rust (no C/C++ dependencies), so this needs no build system
# beyond cargo plus the windowing headers winit's build scripts look for.
{
  lib,
  stdenv,
  rustPlatform,
  pkg-config,
  makeWrapper,
  # Windowing/GPU libraries: needed at build time by the crates that link them, at run time by the
  # ones that open them with dlopen() (see `runtimeLibs`), and by the dev shell (flake.nix).
  wayland,
  wayland-scanner,
  libxkbcommon,
  libx11,
  libxcb,
  libxcursor,
  libxi,
  libxrandr,
  vulkan-loader,
  libglvnd,
  # rfd opens libdbus-1 at run time to reach the xdg-desktop-portal file chooser (Open/Save dialogs).
  dbus,
  # Validates the desktop file and the AppStream metadata we install (as CI does).
  desktop-file-utils,
  appstream,
  # Adds the driver search path (/run/opengl-driver/lib) to the binaries' RPATH so wgpu finds the
  # installed Vulkan driver on NixOS. Optional: harmless on nixpkgs without it.
  autoAddDriverRunpath ? null,
  # raantss18/mahay-fonts checkout (flake input). `null` builds without CJK glyphs.
  mahay-fonts ? null,
  # Release date for the AppStream metadata (YYYY-MM-DD); the flake derives it from its own commit.
  buildDate ? "1970-01-01",
}:

let
  root = ../.;

  # The version lives in exactly one place: `[workspace.package] version` in the root Cargo.toml.
  version = (builtins.fromTOML (builtins.readFile (root + "/Cargo.toml"))).workspace.package.version;

  isLinux = stdenv.hostPlatform.isLinux;

  # AppStream id: also the desktop file name, and the icon name in share/icons/hicolor.
  appId = "org.mahay.hazavana";

  # Libraries the binaries open at run time with dlopen(): nothing links them, so no RPATH points at
  # them — winit loads libwayland-client/libxkbcommon/libxcb, rfd libdbus-1, wgpu the Vulkan loader
  # or libEGL (NixOS patches that loader to find the drivers in /run/opengl-driver). The wrapper
  # below points LD_LIBRARY_PATH here.
  runtimeLibs = lib.optionals isLinux [
    libxkbcommon
    wayland
    libx11
    libxcb
    libxcursor
    libxi
    libxrandr
    vulkan-loader
    libglvnd
    dbus
  ];

  # The two native binaries. The wasm app (apps/hazavana-web) is built by `cargo xtask web`, not
  # here; xtask is tooling.
  binaries = [
    "hazavana"
    "hazavana-cli"
  ];

  # HEIC/HEIF decoding (opt-in upstream: HEVC patents are the distributor's call), as the release builds.
  buildFeatures = [
    "hazavana/heif"
    "hazavana-cli/heif"
  ];
in
rustPlatform.buildRustPackage {
  pname = "hazavana";
  inherit version;

  src = lib.cleanSourceWith {
    src = lib.cleanSource root;
    # The Nix files are not part of the build: editing them must not rebuild any Rust.
    filter =
      path: _type:
      !(builtins.elem (baseNameOf path) [
        "flake.nix"
        "flake.lock"
        "nix"
      ]);
  };

  cargoLock = {
    lockFile = root + "/Cargo.lock";
    # The git dependency (raantss18/mahay-libs) is pinned by revision in Cargo.lock.
    allowBuiltinFetchGit = true;
  };

  # Both binaries, as the .deb/.rpm install them (`packaging/linux/package.sh`).
  cargoBuildFlags = [
    "--locked"
    "-p"
    "hazavana"
    "-p"
    "hazavana-cli"
  ];

  # `cargo xtask ci` runs `cargo test --workspace`. Tests that need what the sandbox cannot have
  # skip themselves with a note: corpus/ (gitignored, `cargo xtask corpus --download`) and the GPU
  # equivalence tests (no adapter).
  cargoTestFlags = [ "--workspace" ];

  # One test thread at a time (RUST_TEST_THREADS=1). `hazavana-catalog`'s `tests_lock` tests share
  # a process-global resource: `flock` is inherited across `fork`, so a child process that another
  # test forked keeps the parent's just-released library lock alive until its `execve` closes the
  # (CLOEXEC) fd — long enough that the sibling test sees a spurious `LockError::InUse`, on ~30 % of
  # runs on a many-core machine. Serializing the tests keeps `nix build` deterministic without
  # touching the repository. The underlying window (any `fork` in a process holding the library
  # lock — the app spawns `xdg-open`/`open`/`explorer` and `sysctl`) is a product-level question:
  # `LibraryLock::acquire` could retry on `WouldBlock` for a few hundred ms before reporting InUse.
  dontUseCargoParallelTests = true;

  strictDeps = true;

  nativeBuildInputs = [
    pkg-config
  ]
  ++ lib.optionals isLinux [
    makeWrapper
    wayland-scanner
    desktop-file-utils
    appstream
  ]
  ++ lib.optionals (isLinux && autoAddDriverRunpath != null) [ autoAddDriverRunpath ];

  buildInputs = lib.optionals isLinux [
    wayland
    libxkbcommon
    libx11
    libxcb
    libxcursor
    libxi
    libxrandr
  ];

  env = lib.optionalAttrs (mahay-fonts != null) {
    MAHAY_FONTS_DIR = toString mahay-fonts;
    # A malformed font input must fail the build rather than silently ship without glyphs.
    MAHAY_FONTS_REQUIRED = "1";
  };

  postInstall = ''
    # Desktop integration: byte-for-byte the files the .deb/.rpm/.AppImage install.
    install -Dm644 packaging/linux/${appId}.desktop $out/share/applications/${appId}.desktop
    install -Dm644 packaging/linux/${appId}.mime.xml $out/share/mime/packages/${appId}.xml
    cp -r assets/app-icon/hicolor $out/share/icons/
    install -d $out/share/metainfo
    substitute packaging/linux/${appId}.metainfo.xml.in \
      $out/share/metainfo/${appId}.metainfo.xml \
      --subst-var-by VERSION ${version} \
      --subst-var-by DATE ${buildDate}

    # Licences for everything embedded in the binaries: Inter, the app icon, each craft font.
    doc=$out/share/doc/hazavana
    install -Dm644 -t "$doc" \
      README.md LICENSE-MIT LICENSE-APACHE NOTICE \
      assets/ATTRIBUTION.md assets/fonts/OFL-Inter.txt
  ''
  + lib.optionalString (mahay-fonts != null) ''
    for lic in ${toString mahay-fonts}/fonts/*/OFL.txt; do
      install -Dm644 "$lic" "$doc/OFL-$(basename "$(dirname "$lic")").txt"
    done
  ''
  + lib.optionalString isLinux ''
    desktop-file-validate $out/share/applications/${appId}.desktop
    XDG_CACHE_HOME=$TMPDIR/.cache appstreamcli validate --no-net \
      $out/share/metainfo/${appId}.metainfo.xml

    # makeWrapper keeps the binaries working when the libs above are only dlopen'd.
    for bin in ${lib.concatStringsSep " " (map (b: "$out/bin/" + b) binaries)}; do
      wrapProgram "$bin" --prefix LD_LIBRARY_PATH : "${lib.makeLibraryPath runtimeLibs}"
    done
  '';

  passthru = {
    # Shared with the dev shell so `cargo run` inside `nix develop` finds the same dlopen'd libs.
    inherit runtimeLibs;
  };

  meta = {
    description = "Photo library and non-destructive raw developer";
    longDescription = ''
      Hazavana organises a photo library and develops raw files non-destructively: masks,
      presets, colour grading, local adjustments and batch export, driven by the same command
      layer as its UI. It ships with hazavana-cli, a headless renderer, command runner and MCP
      server for AI agents.
    '';
    homepage = "https://github.com/raantss18/mahay";
    license = with lib.licenses; [
      mit
      asl20
    ];
    sourceProvenance = with lib.sourceTypes; [ fromSource ];
    platforms = lib.platforms.linux ++ lib.platforms.darwin;
    # `nix run` and the desktop entry both start the GUI.
    mainProgram = "hazavana";
  };
}
