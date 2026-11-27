> The design consist of multiple optional parts but shared through a common plugin system

# Application layouts:

The idea is to support multiple type of apps, on a high level:

```
CLI
└─ adaliszk-console  ........ clap shell: CLI build, parse, bind into container
   ├─ adaliszk-commands  .... command inventory with Dependency resolution
   │  └─ adaliszk-anymap  ... command dependencies: resolves signature structs
   ├─ adaliszk-anymap  ...... clap dependencies: resolves application and clap structs
   └─ adaliszk-prelude  ..... parses configurations and initializes logging
```

# Handle dependencies via `adaliszk-anymap` container

A `Container` is a type-keyed map (backed by [anymap3](https://crates.io/crates/anymap3)) holding
one provider per type. A dependency is provided one of three Laravel-style ways:

```rust
use adaliszk_anymap::{Container, Resolvable};

#[derive(Resolvable, Clone)]
struct Config { token: String }

let mut container = Container::default();

container.bind(Config { token: "secret".to_owned() });
container.bind_with(|_| Config { token: "fresh".to_owned() });
container.singleton_with(|_| Config { token: "once".to_owned() });

let config: Config = container.require::<Config>().expect("bound above");
```

# Main configuration via "prelude":

All applications need a configuration, logging, and monitoring, which could be set up
by using the `adaliszk-prelude`:

```rust
use adaliszk_prelude::config::{LoggingConfig, parse_files};
use adaliszk_prelude::tracing::{workspace_crates, init_tracing};
use serde::Deserialize;

// `#[serde(default)]` makes the struct's `Default` the base layer, so config
// files only need to specify the keys they override:
#[derive(Deserialize, Default)]
#[serde(default)]
struct ApplicationConfig {
    timezone: String,
    logging: LoggingConfig,
}

// Parse one or more config files (format picked by extension) into the struct:
let config: ApplicationConfig = parse_files(&["config.yaml".into()]).expect("parse config");

// Initialize the tracing using the config:
let guard = init_tracing(&workspace_crates!(), &config.logging);
```

The particular application is expected to be split into multiple local crates, where
[inventory](https://crates.io/crates/inventory) is used under the hood to distribute plugins
that has been declared by the various use-case libraries.

# CLI interface using "console" and "commands":

Common use-case is to provide a CLI interface to have various console commands, this is split
into two crates: `adaliszk-commands` registers clap-free command functions whose parameters are
resolved from the container, and `adaliszk-console` owns clap — it builds the CLI, parses the
arguments, and binds the config plus the parsed args into the container.

1. Declare your command anywhere so that [inventory](https://crates.io/crates/inventory)
   could collect it. The name comes from the function (or `name = "..."`), the `--help` text
   from the first `///` line (or `about = "..."`), and `args = TheArgs` links the flags:
   ```rust
   use adaliszk_console::{console_args, console_command, CommandResult};
   use project_foundation::ApplicationConfig;

   #[console_args] // <- derives clap::Args, Resolvable, and Clone
   struct ExampleArgs
   {
       #[arg(long)]
       force: bool,
   }

   /// Does the example thing
   #[console_command(args = ExampleArgs)] // <- registers as `example` and links the args
   async fn example(config: &ApplicationConfig, args: &ExampleArgs) -> CommandResult
   {
       Ok(())
   }
   ```
   A command without CLI flags can use the clap-free `#[command]` from `adaliszk-commands`
   directly; the config type derives `Resolvable` (and `Clone`) so it resolves from the
   container.

2. Resolve the invoked command in `main`, read the config, then run it. The resolved command
   carries the container: `load_config` binds the config into it, services can be added via
   `app.container`, and the console adds the command's parsed arguments:
   ```rust
   use adaliszk_prelude::tracing::{workspace_crates, init_tracing};
   use adaliszk_console::{CommandResult, resolve_console};
   use project_foundation::ApplicationConfig;

   #[tokio::main]
   async fn main() -> CommandResult
   {
       let mut app = resolve_console("cli", std::env::args_os())?;
       let config: ApplicationConfig = app.load_config()?; // also bound into `app.container`
       let _logging = init_tracing(&workspace_crates!(), &config.logging)?;
       //  ^^^ guard must stay bound to keep trace logging alive

       app.resolve().await
   }
   ```