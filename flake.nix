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
      overlay = final: prev: {
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
      }
    );
}
