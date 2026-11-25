> The design consist of multiple optional parts but shared through a common plugin system

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
let guard = init_tracing(workspace_crates!(), & config.logging);
```

The particular application is expected to be split into multiple local crates, where
[inventory](https://crates.io/crates/inventory) is used under the hood to distribute plugins
that has been declared by the various use-case libraries.

# CLI interface using "console":

Common use-case is to provide a CLi interface to have various console commands, this can be set
up by two parts.

1. Declare your command in anywhere so that [inventory](https://crates.io/crates/inventory)
   could collect the commands:
   ```rust
   use adaliszk_console::{ConsoleArgs, ConsoleHandler, ConsoleResult, console};
   use project_foundation::ApplicationConfig;
    
   #[derive(ConsoleArgs)]
   struct ExampleCommand;
    
   #[console(name = "example")]
   impl ConsoleHandler<ApplicationConfig> for ExampleCommand
   {
       async fn handle(&self, config: &ApplicationConfig) -> ConsoleResult
       {
           Ok(())
       }
   }
   ```

2. Resolve the invoked command in `main`, read the config, then run it:
   ```rust
   use adaliszk_prelude::tracing::{workspace_crates, init_tracing};
   use adaliszk_console::{ConsoleResult, resolve_console};
   use project_foundation::ApplicationConfig;
   
   #[tokio::main]
   async fn main() -> ConsoleResult
   {
       let app = resolve_console("cli")?;
       let config: ApplicationConfig = app.load_config()?;
       let _logging = init_tracing(workspace_crates!(), &config.logging)?;
       //  ^^^ guard must stay bound to keep trace logging alive
       app.resolve().await
   }
   ```