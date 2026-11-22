# SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
# SPDX-License-Identifier: MPL-2.0
#
{
  pkgs,
  rustToolchain,
  formatter ? null,
}:
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
  ++ pkgs.lib.optional (formatter != null) formatter;
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
