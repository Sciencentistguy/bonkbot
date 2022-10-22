{
  inputs = {
    # github example, also supported gitlab:
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = github:edolstra/flake-compat;
      flake = false;
    };
  };
  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  }:
    {
      overlays.default = final: prev: {
        bonkbot = self.packages.${prev.system}.default;
      };
    }
    // flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
        inherit (pkgs) lib;
        bonkbot = {
          lib,
          openssl,
          pkg-config,
          rustPlatform,
        }:
          rustPlatform.buildRustPackage {
            name = "bonkbot";
            src = lib.cleanSource ./.;
            cargoLock.lockFile = ./Cargo.lock;
            nativeBuildInputs = [
              pkg-config
              rustPlatform.bindgenHook
            ];
            buildInputs = [openssl];
            meta = with lib; {
              license = licenses.mpl20;
              homepage = "https://github.com/Sciencentistguy/bonkbot";
              platforms = platforms.all;
            };
          };
      in {
        packages.bonkbot = pkgs.callPackage bonkbot {};

        packages.default = self.packages.${system}.bonkbot;
        devShells.default = self.packages.${system}.default.overrideAttrs (super: {
          nativeBuildInputs = with pkgs;
            super.nativeBuildInputs
            ++ [
              cargo-edit
              clippy
              rustfmt
            ];
          RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
        });

        nixosModules.bonkbot = {
          pkgs,
          config,
          lib,
          ...
        }: let
          inherit (lib) mkOption mkIf mkEnableOption types;
          cfg = config.services.bonkbot;
        in {
          options = {
            services.bonkbot = {
              enable = mkEnableOption "bonkbot";
              package = mkOption {
                type = types.package;
                default = self.packages.${system}.default;
                defaultText = "pkgs.bonkbot";
                description = "The package to use for the bonkbot service.";
              };
              tokenPath = mkOption {
                example = "/run/secrets/bonkbot_token";
                type = types.str;
              };
              appIdPath = mkOption {
                example = "/run/secrets/bonkbot_appid";
                type = types.str;
              };
            };
          };

          config = mkIf cfg.enable {
            users = {
              users.bonkbot = {
                group = "bonkbot";
                description = "bonkbot user";
                isSystemUser = true;
              };
              groups.bonkbot = {};
            };

            systemd.services.bonkbot = {
              description = "bonkbot";
              wantedBy = ["multi-user.target"];
              after = ["network.target"];
              serviceConfig = {
                ExecStart = "${cfg.package}/bin/bonkbot ${cfg.tokenPath} ${cfg.appIdPath}";
                User = "bonkbot";
                Group = "bonkbot";
                Restart = "always";
                RestartSec = 5;
              };
            };
          };
        };
      }
    );
}
