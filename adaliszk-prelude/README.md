A small foundation for my Rust projects that wires up the basics I repeat everywhere. It
provides
shared configuration types, YAML configuration parsing, and an opinionated `tracing` setup that
writes to both the console and a rolling log file. A YAML config maps straight onto the types:

```yaml
timezone: "Europe/Budapest"
logging:
  level: info # trace | debug | info | warn | error
  format: structured # pretty | compact | structured
  dir: "/var/lib/application/logs"
```

## Installation & Usage

```sh
cargo add adaliszk-prelude
```

Parse a config file into a typed struct, then initialize logging from it:

```rust
use adaliszk_prelude::{LoggingConfig, parse_config, init_tracing, workspace_crates};
use serde::Deserialize;

#[derive(Deserialize)]
struct AppConfig {
    timezone: String,
    logging: LoggingConfig,
}

let config: AppConfig = parse_config("config.yaml".as_ref())?;
let guard = init_tracing(workspace_crates!(), &config.logging)?;
```

## License & Disclosure

Licensed under the Mozilla Public License 2.0 (MPL-2.0).

Development was done with LLM as a search engine and debugger (meaning it is for learning only),
NO SOURCE LINES where generated or changed without direct human decisions and work.

Contributions expected to follow the same **No-Automated-LLM Contributions** policy with strict
limitations to retain the quality and understanding!
