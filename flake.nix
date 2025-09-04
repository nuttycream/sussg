{
  description = "sussg";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
        with pkgs; {
          packages.default = pkgs.rustPlatform.buildRustPackage {
            name = "sussg";
            src = ./.;
            buildInputs = [];
            nativeBuildInputs = [];
            cargoHash = "sha256-KNIHDvCgD2FkM/8N4taFoMC6l3cZZi2VGHmJ//bMTgY=";
          };

          devShells.default = mkShell {
            name = "sussg";
            packages = with pkgs; [
              just
              rust-bin.stable.latest.minimal
              rust-analyzer
              rustfmt
            ];

            buildInputs = [
              openssl
              pkg-config
            ];
          };
        }
    );
}
