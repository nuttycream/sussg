{
  description = "sussg";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    flake-parts.url = "github:hercules-ci/flake-parts";

    crane.url = "github:ipetkov/crane";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    flake-parts,
    ...
  } @ inputs:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = nixpkgs.lib.systems.flakeExposed;

      perSystem = {
        pkgs,
        system,
        ...
      }: let
        rustToolchain = inputs.fenix.packages.${system}.stable.toolchain;

        craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;

        versionInfo = craneLib.crateNameFromCargoToml {cargoToml = ./Cargo.toml;};
        src = craneLib.cleanCargoSource ./.;

        commonArgs = {
          inherit (versionInfo) pname version;
          inherit src;
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      in {
        packages = let
          sussg = craneLib.buildPackage (
            commonArgs
            // {
              inherit cargoArtifacts src;
            }
          );
        in {
          inherit sussg;
          default = sussg;
        };

        devShells.default = let
          inherit
            (pkgs)
            mkShell
            ;
        in
          mkShell {
            name = "sussg-shell";
            packages = [
              rustToolchain
            ];
          };
      };
    };
}
