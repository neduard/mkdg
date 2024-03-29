{
  description = "make a static site digital garden";
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

          meta = with pkgs.lib; {
            homepage = "https://github.com/neduard/mkdg";
            license = licenses.mit;
          };

          cargoLock = {
            lockFile = ./Cargo.lock;
          };
        };

        packages.default = self.packages.${system}.mkdg;

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            cargo
            clippy
            rustc
            rustfmt
          ];
        };
      }
    );
}
