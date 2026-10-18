{
  description = "Pejy — page layout and publishing, a free alternative to InDesign, in pure Rust";

  inputs = {
    # Pejy needs edition 2024 and rustc ≥ 1.95, so the flake tracks unstable. Consumers on a
    # release channel can retarget it: `inputs.pejy.inputs.nixpkgs.follows = "nixpkgs";`
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    # Optional build input of the repository: the fonts that the build embeds when `MAHAY_FONTS_DIR`
    # is set. Official releases always build with them; without them everything works but some
    # scripts have no glyphs. Pinned to the revision the release workflow uses — bump deliberately
    # (`nix flake update mahay-fonts`).
    mahay-fonts = {
      url = "github:raantss18/mahay-fonts/abb83316d96aa59c1cf64784289e378fe9fa5695";
      flake = false;
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      mahay-fonts,
    }:
    let
      inherit (nixpkgs) lib;

      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];

      # The AppStream metadata carries a release date. The flake source's own commit date is
      # reproducible, unlike "now"; a tarball or dirty tree without one falls back to the epoch.
      buildDate =
        let
          d = self.lastModifiedDate or "";
        in
        if builtins.stringLength d >= 8 then
          "${builtins.substring 0 4 d}-${builtins.substring 4 2 d}-${builtins.substring 6 2 d}"
        else
          "1970-01-01";

      packageArgs = {
        inherit mahay-fonts buildDate;
      };

      forAllSystems = f: lib.genAttrs systems (system: f (import nixpkgs { inherit system; }) system);
    in
    {
      packages = forAllSystems (
        pkgs: _:
        let
          pejy = pkgs.callPackage ./nix/package.nix packageArgs;
        in
        {
          inherit pejy;
          default = pejy;
        }
      );

      # `nixpkgs.overlays = [ inputs.pejy.overlays.default ]` → `pkgs.pejy` everywhere.
      overlays.default = final: _prev: {
        pejy = final.callPackage ./nix/package.nix packageArgs;
      };

      apps = forAllSystems (
        _: system: {
          default = {
            type = "app";
            program = lib.getExe self.packages.${system}.pejy;
          };
        }
      );

      devShells = forAllSystems (
        pkgs: system:
        let
          pejy = self.packages.${system}.pejy;
        in
        {
          default = pkgs.mkShell (
            {
              # The package's own native inputs, so `cargo build` in the shell needs no extra setup.
              inputsFrom = [ pejy ];
              packages = with pkgs; [
                cargo
                rustc
                clippy
                rustfmt
                rust-analyzer
              ];
              # `cargo run` dlopens the Vulkan loader, libwayland and libxkbcommon like the packaged binary.
              LD_LIBRARY_PATH = lib.makeLibraryPath pejy.passthru.runtimeLibs;
              RUST_BACKTRACE = "1";
            }
            // lib.optionalAttrs (mahay-fonts != null) {
              # Embedded fonts in `cargo run` too; build.rs reads this as MAHAY_FONTS_DIR.
              MAHAY_FONTS_DIR = toString mahay-fonts;
            }
          );
        }
      );

      checks = forAllSystems (
        pkgs: system: {
          pejy = self.packages.${system}.pejy;

          # `nix fmt` formats the flake; this keeps it formatted.
          formatting =
            pkgs.runCommand "pejy-nix-formatting" { nativeBuildInputs = [ pkgs.nixfmt ]; }
              ''
                nixfmt --check ${./flake.nix} ${./nix}
                touch $out
              '';
        }
      );

      formatter = forAllSystems (pkgs: _: pkgs.nixfmt);
    };
}
