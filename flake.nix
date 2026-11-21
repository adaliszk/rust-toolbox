{
  description = "Development environment for the AdaLiszk's Toolbox";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
    }:
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
          if builtins.pathExists ./rust-toolchain.toml then
            pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml
          else
            pkgs.rust-bin.stable.latest.default.override {
              extensions = [
                "rust-src"
                "rust-analyzer"
                "clippy"
                "rustfmt"
              ];
            };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = [
            rustToolchain
          ]
          ++ (with pkgs; [
            pkg-config
            cargo-watch
            cargo-nextest
            cargo-deny
          ]);
          buildInputs = with pkgs; [
            openssl
          ];
          env = {
            RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          };
          shellHook = ''
            export GIT_EMAIL="$(git config user.email)"
            echo "> Compiler: $(rustc --version)"
            echo "> Cargo: $(cargo --version)"
            echo "> Clippy: $(cargo clippy --version)"
            echo "> Fmt: $(cargo fmt --version)"
          '';
        };
      }
    );
}
