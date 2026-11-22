# SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
# SPDX-License-Identifier: MPL-2.0
#
# Compile a Rust crate and ship it as a multi-arch OCI image. It builds one image
# per target architecture (cross-compiled from the build host) and pushes them as
# a single manifest list, so one tag deploys on either arm64 or amd64.
#
# ```nix
# toolbox.lib.mkRustContainer {
#     inherit system;
#     pname = "api";      # crate name; also the binary, override with `binary`
#     src   = ./.;        # the crate/workspace to build
# }
# ```
#
# The result is a pusher: `nix run .#api -- registry.example.com/api:1.0`
#
{ nix2container, nixpkgs }:
{
  system,
  systems ? [ "x86_64-linux" "aarch64-linux" ],
  registry ? null,
  pname,
  # Defaults to the crate's Cargo.toml version; falls back to "latest" when it is
  # absent or workspace-inherited (pass `version` explicitly then).
  version ?
    (
      let
        found = (builtins.fromTOML (builtins.readFile (src + "/Cargo.toml"))).package.version or null;
      in
      if builtins.isString found then found else "latest"
    ),
  binary ? pname,
  src,
  caBundle ? null,
  static ? false,
}:
let
  hostPkgs = import nixpkgs { inherit system; };
  builder = nix2container.packages.${system}.nix2container;

  archOf = {
    "x86_64-linux" = "amd64";
    "aarch64-linux" = "arm64";
  };

  imageFor =
    target:
    let
      pkgs =
        if target == system then
          import nixpkgs { inherit system; }
        else
          import nixpkgs {
            localSystem = system;
            crossSystem = target;
          };
      rustPlatform = if static then pkgs.pkgsStatic.rustPlatform else pkgs.rustPlatform;
      app = rustPlatform.buildRustPackage {
        inherit pname version src;
        cargoLock.lockFile = src + "/Cargo.lock";
      };
      certFile =
        if caBundle != null then caBundle else "${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt";
    in
    builder.buildImage {
      name = pname;
      tag = "${version}-${archOf.${target}}";
      arch = archOf.${target};
      config = {
        entrypoint = [ "${app}/bin/${binary}" ];
        env = [ "SSL_CERT_FILE=${certFile}" ];
      };
    };

  images = map (target: {
    arch = archOf.${target};
    image = imageFor target;
  }) systems;

  platforms = hostPkgs.lib.concatMapStringsSep "," (target: "linux/${archOf.${target}}") systems;

  defaultDest = if registry != null then "${registry}/${pname}:${version}" else "";
in
hostPkgs.writeShellApplication {
  name = "push-${pname}";
  runtimeInputs = [
    hostPkgs.skopeo
    hostPkgs.manifest-tool
  ];
  text = ''
    dest="''${1:-${defaultDest}}"
    if [ -z "$dest" ]; then
      echo "push-${pname}: pass <registry/repo:tag>, or set \`registry\` in mkRustContainer" >&2
      exit 1
    fi

    # Push each architecture to a per-arch tag.
    ${hostPkgs.lib.concatMapStringsSep "\n" ({ arch, image }: ''
      "${image.copyTo}/bin/copy-to" "docker://''${dest}-${arch}"
    '') images}

    # Stitch the per-arch tags into one multi-arch manifest list.
    manifest-tool push from-args \
      --platforms "${platforms}" \
      --template "''${dest}-ARCH" \
      --target "''${dest}"
  '';
}
