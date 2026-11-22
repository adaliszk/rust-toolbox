> Toolbox wiring up the well-known packages to the format for my personal thinking.
> Feel free to use it for your own, but be aware this is mainly for my usage.

# AdaLiszk's Rust Toolbox

Working with the Rust ecosystem, I noticed that my patterns are repeated across my projects,
so this wrapper repository is there to formalize those abstractions. You can think of these
crates as one of the many possible ways to use the tools and orchestrate them together or
as a personal lightweight framework.

This also contains my preferred coding style that I've been using professionally for well over
a decade now. Nothing crazy, 99% the official with a tiny tweak here and there.

### Prerequisites

- `nix >= 2.34` + `flake` as a reproducible working environment
- `rust >= 1.100` as the minimum toolchain language

## Packages (Crates)

| Name                                               | Type    | Purpose                                          | Crate                                                                                                                              |
|:---------------------------------------------------|:--------|:-------------------------------------------------|:-----------------------------------------------------------------------------------------------------------------------------------|
| [`adaliszkfmt`](./adaliszkfmt/README.md)           | CLI     | Drop-in `rustfmt` shim bundling a personal style | [![v](https://img.shields.io/crates/v/adaliszkfmt?logo=&label=&style=flat-square)](https://crates.io/crates/adaliszkfmt)           |
| [`adaliszk-prelude`](./adaliszk-prelude/README.md) | Library | Configure, Log, and Monitor applications         | [![v](https://img.shields.io/crates/v/adaliszk-prelude?logo=&label=&style=flat-square)](https://crates.io/crates/adaliszk-prelude) |
| [`adaliszk-console`](./adaliszk-console/README.md) | Library | Inventory for Clap-based CLI applications        | [![v](https://img.shields.io/crates/v/adaliszk-console?logo=&label=&style=flat-square)](https://crates.io/crates/adaliszk-console) |
| `adaliszk-commands`                                | Library | _Inventory for Command-builder for integrations_ | —                                                                                                                                  |
| `adlaiszk-terminal`                                | Library | _Inventory for Rattatui-based CLI applications_  | —                                                                                                                                  |
| `adlaiszk-events`                                  | Library | _Inventory for Tokoi-based eventing_             | —                                                                                                                                  |

### Design & Concept

While each crate explain what they do individually, the overall design can be found
in [DESIGN.md](./DESIGN.md). In a nutshell; most commands expose their inventory, macros, and
prelude methods for easy wiring while keeping the domain-code simple.

## Nix & Containerization

A flake in [`nix`](nix) provides a development shell and a reusable container builder.

A complete consumer flake that reuses both looks like:

```nix
{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    toolbox.url = "github:adaliszk/rust-toolbox?dir=nix";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs =
    { nixpkgs, flake-utils, toolbox, ... }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        # Reuse the toolbox development shell as-is.
        devShells.default = toolbox.devShells.${system}.default;
        # Multi-arch image (amd64 + arm64): `system` is only the build host, the targets
        # come from `systems` (default: ["x86_64-linux", "aarch64-linux"]).
        packages.api = toolbox.lib.mkRustContainer {
          inherit system;
          registry = "ghcr.io/adaliszk";
          pname = "api";
          src = ./.;
        };
      }
    );
}
```

Build and push the multi-arch image with `nix run .#api` that auto-tags with the cargo version.

## License & Disclosure

Licensed under the Mozilla Public License 2.0 (MPL-2.0). See [LICENSE](./LICENSE.md)

Development was done with LLM as a search engine and debugger (meaning it is for learning only),
NO SOURCE LINES where generated or changed without direct human decisions and work.

Contributions expected to follow the same **No-Automated-LLM Contributions** policy with strict
limitations to retain the quality and understanding!

## Contributions

Refinements and fixes are generally accepted with [**No-Automated-LLM
Contributions** policy](AGENTS.md), where we explicitly disallow the usage of LLM tools to
automatically make changes, but they may be used to aid your own skills.
