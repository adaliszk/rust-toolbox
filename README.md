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

| Name                                               | Type    | Purpose                                          | Crate                                                                                                                            |
|:---------------------------------------------------|:--------|:-------------------------------------------------|:---------------------------------------------------------------------------------------------------------------------------------|
| [`adaliszkfmt`](./adaliszkfmt/README.md)           | CLI     | Drop-in `rustfmt` shim bundling a personal style | [![v](https://img.shields.io/crates/v/adaliszkfmt?logo=&label=&style=flat-square)](https://crates.io/crates/adaliszkfmt)         |
| [`adaliszk-prelude`](./adaliszk-prelude/README.md) | Library | Configure, Log, and Monitor applications         | [![v](https://img.shields.io/crates/v/adaliszk-prelude?logo=&label=&style=flat-square)](https://crates.io/crates/adaliszk-prelude) |
| `adaliszk-console`                                 | Library | _Inventory for Clap-based CLI applications_      | —                                                                                                                                |
| `adaliszk-commands`                                | Library | _Inventory for Command-builder for integrations_ | —                                                                                                                                |
| `adlaiszk-terminal`                                | Library | _Inventory for Rattatui-based CLI applications_  | —                                                                                                                                |
| `adlaiszk-events`                                  | Library | _Inventory for Tokoi-based eventing_             | —                                                                                                                                |

### Design & Concept

All applications need a configuration parsing, logging, and monitoring, which could be set up
by using the `adaliszk-prelude`:

```rust
use adaliszk_prelude::{LoggingConfig, parse_config, init_tracing};
use serde::Deserialize;

#[derive(Deserialize)]
struct ApplicationConfig {
    logging: LoggingConfig,
}

// Parse the config using TOML, into a serde/strum dict:
let config = parse_config::<ApplicationConfig>(path);

// Initialize the tracing using the config:
init_tracing(config.logging);
```

From there, the particular application is expected to be split into multiple local crates, where
[inventory](https://crates.io/crates/inventory) is used under the hood to distribute plugins
that has been declared by the various use-case libraries.

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
