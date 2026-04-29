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
            cargoHash = "sha256-3B4lD2Vgo78xgK2kuSIkkhv1S485Y5KEjO3YV18a//U=";
          };

          devShells.default = mkShell {
            name = "sussg";
            packages = with pkgs; [
              rust-bin.stable.latest.default
            ];

            buildInputs = [
              openssl
              pkg-config
            ];
          };
        }
    );
}
