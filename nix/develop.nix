# SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
# SPDX-License-Identifier: MPL-2.0
#
{ nixpkgs, rust-overlay }:
{
  system,
  pkgs ? import nixpkgs {
    inherit system;
    overlays = [ (import rust-overlay) ];
  },
  # Path to a rust-toolchain.toml; builds the toolchain from it when
  # `rustToolchain` is not supplied directly.
  toolchainFile ? null,
  rustToolchain ?
    (
      if toolchainFile != null then
        pkgs.rust-bin.fromRustupToolchainFile toolchainFile
      else
        pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
            "rustfmt"
            "clippy"
          ];
        }
    ),
  formatter ? null,
  extraPackages ? [ ],
  binaries ? [ ],
}:
let
  # A `<name>` shortcut for each binary: `cargo run --bin <name>`.
  binRunner = name: pkgs.writeShellScriptBin name ''exec cargo run --bin ${name} -- "$@"'';
  # Generic watch loop: `watch <bin> [args...]` reruns the bin on every change.
  cargoWatch = pkgs.writeShellScriptBin "watch" ''
    bin="''${1:?usage: watch <bin> [args...]}"
    shift
    exec cargo watch -x "run --bin $bin -- $*"
  '';
in
pkgs.mkShell {
  packages = [
    rustToolchain
  ]
  ++ (with pkgs; [
    pkg-config
    cargo-deny
    cargo-nextest
    cargo-watch
    cargo-release
  ])
  ++ pkgs.lib.optional (formatter != null) formatter
  ++ extraPackages
  ++ map binRunner binaries
  ++ [ cargoWatch ];
  buildInputs = with pkgs; [
    openssl
  ];
  env = {
    RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
  }
  // pkgs.lib.optionalAttrs (formatter != null) {
    RUSTFMT = "${formatter}/bin/adaliszkfmt";
  };
  shellHook = ''
    export GIT_EMAIL="$(git config user.email)"
    echo "> Compiler: $(rustc --version)"
    echo "> Cargo: $(cargo --version)"
    echo "> Clippy: $(cargo clippy --version)"
    echo "> Fmt: $(cargo fmt --version)"
  '';
}
