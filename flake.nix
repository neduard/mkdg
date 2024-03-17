{
  description = "make digital garden";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in {
        packages.mkdg = pkgs.rustPlatform.buildRustPackage {
          pname = "mkdg";
          version = "0.1.0";
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };
        };

        defaultPackage = self.packages.${system}.mkdg;

        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rustc
            rustfmt
            cargo
          ];
        };
      }
    );
}
