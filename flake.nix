{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      perSystem =
        { system, self', ... }:
        let
          pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [ (import inputs.rust-overlay) ];
          };
          rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;
          src = craneLib.cleanCargoSource ./.;

          nativeBuildInputs = with pkgs; [
            git
          ];
          buildInputs = [
            rustToolchain
          ];
          devInputs = with pkgs; [
            nixd
            rust-analyzer
          ];

          commonArgs = {
            inherit src buildInputs nativeBuildInputs;
          };
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;
          bokeh = craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });
        in
        {
          packages.default = bokeh;

          checks = {
            bokeh-clippy = craneLib.cargoClippy (
              commonArgs
              // {
                inherit cargoArtifacts;
                cargoClippyExtraArgs = "--all-targets -- -W clippy::pedantic --deny warnings";
              }
            );

            bokeh-doc = craneLib.cargoDoc (
              commonArgs
              // {
                inherit cargoArtifacts;
                env.RUSTDOCFLAGS = "--deny warnings";
              }
            );

            bokeh-fmt = craneLib.cargoFmt {
              inherit src;
            };

            bokeh-toml-fmt = craneLib.taploFmt {
              src = pkgs.lib.sources.sourceFilesBySuffices src [ ".toml" ];
            };

            bokeh-nix-fmt = pkgs.runCommand "nix-fmt-check" { } ''
              ${pkgs.nixfmt}/bin/nixfmt --check ${./flake.nix}
              touch $out
            '';

            bokeh-nextest = craneLib.cargoNextest (
              commonArgs
              // {
                inherit cargoArtifacts;
                partitions = 1;
                partitionType = "count";
                cargoNextestPartitionsExtraArgs = "--no-tests=pass";
              }
            );
          };

          devShells.default = craneLib.devShell {
            checks = self'.checks;
            buildInputs = devInputs;
          };
        };
    };
}
