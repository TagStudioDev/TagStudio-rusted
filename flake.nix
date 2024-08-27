{
  description = "TagStudioRusted";

  inputs = {
    devenv.url = "github:cachix/devenv";

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

  outputs = { flake-parts, nixpkgs, self, systems, ... }@inputs:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ inputs.devenv.flakeModule ];

      systems = import systems;

      perSystem = { pkgs, system, ... }: {
        devenv.shells = rec {
          default = tagstudio;

          tagstudio = {
            name = "TagStudioRusted";

            dotenv.disableHint = true;

            packages = with pkgs; [
              cargo-expand
              cargo-msrv
              (cargo-tauri.overrideAttrs (o: rec {
                version = "2.0.0-rc.6";
                src = fetchFromGitHub {
                  owner = "tauri-apps";
                  repo = "tauri";
                  rev = "tauri-v${version}";
                  hash = "sha256-y8Y9En1r1HU9sZcYHFhB+botVQBZfzqoDrlgp98ltrY=";
                };
                cargoDeps = o.cargoDeps.overrideAttrs (_: {
                  inherit src;

                  name = "cargo-tauri-${version}-vendor.tar.gz";
                  outputHash = "sha256-3/fMCOmbwjtia/T4uLJ4oaNnquJPCvMFc0AcSLEg0XQ=";
                });
              }))
              cargo-udeps
              cargo-update
              libsoup_3
              openssl
              pkg-config
              webkitgtk_4_1
            ];

            languages = {
              javascript = {
                enable = true;
                pnpm = {
                  enable = true;
                  install.enable = true;
                };
              };
              rust = {
                enable = true;
                channel = "stable";
              };
            };
          };
        };
      };
    };
}
