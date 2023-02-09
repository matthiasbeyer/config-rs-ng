{
  description = "The config-rs-ng crate";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-22.11";
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, crane, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rustTarget = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        craneLib = (crane.mkLib pkgs).overrideToolchain rustTarget;

        tomlInfo = craneLib.crateNameFromCargoToml { cargoToml = ./Cargo.toml; };
        src =
          let
            markdownFilter = path: _type: pkgs.lib.hasSuffix ".md" path;
            filterPath = path: type: builtins.any (f: f path type) [
              markdownFilter
              craneLib.filterCargoSources
              pkgs.lib.cleanSourceFilter
            ];
          in
          pkgs.lib.cleanSourceWith {
            src = ./.;
            filter = filterPath;
          };

        configArtifacts = craneLib.buildDepsOnly {
          inherit src;
          cargoExtraArgs = "--all-features --all";
        };

        config = craneLib.buildPackage {
          inherit (tomlInfo) version;
          inherit src;
          cargoArtifacts = configArtifacts;
          cargoExtraArgs = "--all-features --all";
        };
      in
      rec {
        checks = {
          inherit config;

          config-clippy = craneLib.cargoClippy {
            inherit src;
            cargoArtifacts = configArtifacts;
            cargoExtraArgs = "--all --all-features";
            cargoClippyExtraArgs = "-- --deny warnings";
          };

          config-fmt = craneLib.cargoFmt {
            inherit src;
          };
        };

        packages = {
          inherit config;
          default = packages.config;
        };

        devShells = {
          default = devShells.config;
          config = pkgs.mkShell {
            buildInputs = [ ];

            nativeBuildInputs = [
              rustTarget
              pkgs.gitlint
            ];
          };
        };
      }
    );
}
