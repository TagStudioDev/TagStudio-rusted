{
  inputs = {
    devenv.url = "github:cachix/devenv";

    devenv-root = {
      url = "file+file:///dev/null";
      flake = false;
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };

    nix2container = {
      url = "github:nlewo/nix2container";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    systems.url = "github:nix-systems/default-linux";
  };

  outputs =
    { flake-parts, nixpkgs, ... }@inputs:
    let
      inherit (nixpkgs) lib;
    in
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ inputs.devenv.flakeModule ];

      systems = import inputs.systems;

      perSystem =
        { pkgs, ... }:
        {
          devenv.shells = rec {
            default = tagstudio-rusted;

            tagstudio-rusted = {
              devenv.root =
                let
                  devenvRoot = builtins.readFile inputs.devenv-root.outPath;
                in
                # If not overriden (/dev/null), --impure is necessary.
                lib.mkIf (devenvRoot != "") devenvRoot;

              name = "TagStudioRusted";

              packages = with pkgs; [
                cargo-deny
                cargo-edit
                cargo-expand
                cargo-msrv
                cargo-udeps
                libsoup_3
                openssl
                pkg-config
                webkitgtk_4_1

                (cargo-tauri.overrideAttrs (o: rec {
                  version = "2.0.0-rc.11";

                  src = fetchFromGitHub {
                    owner = "tauri-apps";
                    repo = "tauri";
                    rev = "tauri-v${version}";
                    hash = "sha256-7usmUaAK9HIjrBvq+wquAlI5KuAQA/VFhDp6trxvDAQ=";
                  };

                  # tests::run_app fails with permission denied.
                  doCheck = false;

                  sourceRoot = null;

                  cargoDeps = pkgs.rustPlatform.importCargoLock {
                    lockFile = src + /Cargo.lock;
                    outputHashes = {
                      "schemars_derive-0.8.21" = "sha256-AmxBKZXm2Eb+w8/hLQWTol5f22uP8UqaIh+LVLbS20g=";
                    };
                  };
                  /*
                    cargoDeps = o.cargoDeps.overrideAttrs (_: {
                      inherit src sourceRoot;

                      name = "cargo-tauri-${version}-vendor.tar.gz";
                      outputHash = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";
                    });
                  */

                  buildInputs = builtins.map (
                    pkg: if pkg == pkgs.webkitgtk then pkgs.webkitgtk_4_1 else pkg
                  ) o.buildInputs;
                }))
              ];

              languages = {
                javascript = {
                  enable = true;
                  pnpm = {
                    enable = true;
                    install.enable = true;
                  };
                };
                nix.enable = true;
                rust = {
                  enable = true;
                  channel = "stable";
                };
              };

              pre-commit = {
                hooks = {
                  clippy.enable = true;
                  deadnix.enable = true;
                  flake-checker.enable = true;
                  nixfmt-rfc-style.enable = true;
                  rustfmt.enable = true;
                  statix.enable = true;
                };
                settings.rust.cargoManifestPath = "src-tauri/Cargo.toml";
              };
            };
          };

          formatter = pkgs.nixfmt-rfc-style;
        };
    };
}
