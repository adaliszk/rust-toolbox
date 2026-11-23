A small command framework over [`clap`](https://crates.io/crates/clap) and
[`inventory`](https://crates.io/crates/inventory). Commands register themselves at compile time,
so a binary discovers them without a central list, and every command receives the application's
parsed configuration from a common type declared in user-space.

## Example Usage

You would declare in your own scope an application config:

```rust
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConfig
{
    // Whatever your application sets globally
}
```

Then declare a command is a struct with its own flags plus a handler:

```rust
use adaliszk_console::{ConsoleArgs, ConsoleHandler, ConsoleResult, console};
use your_foundation::AppConfig;

#[derive(ConsoleArgs)]
pub struct ExampleCommand;

#[console(name = "example")]
impl ConsoleHandler<AppConfig> for ExampleCommand
{
    async fn handle(&self, config: &AppConfig) -> ConsoleResult
    {
        todo!()
    }
}
```

Resolve the registered, read the config, then run it:

```rust
use adaliszk_console::{ConsoleResult, resolve_console};
use adaliszk_prelude::tracing::{init_tracing, workspace_crates};
use your_foundation::AppConfig;

// Load all the crates that register commands:
use your_example_crate as _;

#[tokio::main]
async fn main() -> ConsoleResult
{
    let app = resolve_console::<AppConfig>("cli")?;
    let crates = workspace_crates!();
    let _logging = init_tracing(crates.clone(), &app.config().logging)?;
    tracing::info!("Loaded modules: {}", crates.join(", "));
    app.resolve().await
}
```

## Why do this?

Short answer: purely a tool for my own projects.

Long answer: every CLI repeats the same wiring. It declares subcommands, parses a config file,
and dispatches to a handler. This crate formalizes that once, so a new command is a struct plus
a handler, with no central registry to maintain.

## License & Disclosure

Licensed under the Mozilla Public License 2.0 (MPL-2.0).

Development was done with LLM as a search engine and debugger (meaning it is for learning only),
NO SOURCE LINES where generated or changed without direct human decisions and work.

Contributions expected to follow the same **No-Automated-LLM Contributions** policy with strict
limitations to retain the quality and understanding!
