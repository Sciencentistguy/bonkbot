{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
  };
  outputs = {
    self,
    nixpkgs,
    flake-utils,
    crane,
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

        craneLib = crane.lib.${system};

        bonkbot = craneLib.buildPackage {
          name = "bonkbot";
          src = craneLib.cleanCargoSource ./.;
          nativeBuildInputs = with pkgs; [
            pkg-config
            rustPlatform.bindgenHook
          ];
          buildInputs = with pkgs; [openssl];
        };
      in {
        packages.bonkbot = bonkbot;

        packages.default = self.packages.${system}.bonkbot;
        devShells.default = self.packages.${system}.default.overrideAttrs (super: {
          nativeBuildInputs = with pkgs;
            super.nativeBuildInputs
            ++ [
              cargo-edit
              clippy
              rustfmt
              rustc
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
