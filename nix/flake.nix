# SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
# SPDX-License-Identifier: MPL-2.0
#
{
  description = "Development environment and container builders for the AdaLiszk's Toolbox";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nix2container = {
      url = "github:nlewo/nix2container";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      nix2container,
    }:
    let
      mkRustContainer = import ./container.nix { inherit nix2container nixpkgs; };
      mkDevShell = import ./develop.nix;
    in
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [
          (import rust-overlay)
        ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustToolchain =
          if builtins.pathExists ../rust-toolchain.toml then
            pkgs.rust-bin.fromRustupToolchainFile ../rust-toolchain.toml
          else
            pkgs.rust-bin.stable.latest.default.override {
              extensions = [
                "rust-src"
                "rust-analyzer"
                "rustfmt"
                "clippy"
              ];
            };
      in
      {
        devShells.default = mkDevShell { inherit pkgs rustToolchain; };
      }
    )
    // {
      lib = {
        inherit mkRustContainer mkDevShell;
      };
    };
}
