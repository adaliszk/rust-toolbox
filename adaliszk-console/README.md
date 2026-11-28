The clap front-end for [`adaliszk-commands`](https://crates.io/crates/adaliszk-commands): builds
the CLI from the registered commands, parses the process arguments, loads the configuration, and
binds both into the dependency-injection container
([`adaliszk-anymap`](https://crates.io/crates/adaliszk-anymap)) that the matched command resolves
its parameters from.

## Example Usage

You would declare in your own scope an application config that can resolve from the container:

```rust
use serde::Deserialize;
use adaliszk_anymap::Resolvable; // <- the derive needs adaliszk-anymap as a direct dependency

#[derive(Deserialize, Resolvable, Clone)]
pub struct AppConfig
{
    // Whatever your application sets globally
}
```

Then declare a command as a free `async fn` and, when it takes flags, its args struct. The
command's name comes from the function (or `name = "..."`), its `--help` text from the first
`///` line (or `about = "..."`), and `args = TheArgs` links the flags; every parameter is
resolved from the container by type:

```rust
use adaliszk_console::{console_args, console_command, CommandResult};
use your_foundation::AppConfig;

#[console_args] // <- derives clap::Args, Resolvable, and Clone
pub struct ExampleArgs
{
    #[arg(long)]
    force: bool,
}

/// Does the example thing
#[console_command(args = ExampleArgs)] // registers as `example` and links the args
async fn example(config: &AppConfig, args: &ExampleArgs) -> CommandResult
{
    todo!()
}
```

Resolve the invoked command, read the config, then run it:

```rust
use adaliszk_console::{CommandResult, resolve_console};
use adaliszk_prelude::tracing::{init_tracing, workspace_crates};
use your_foundation::AppConfig;

// Load all the crates that register commands:
use your_example_crate as _;

#[tokio::main]
async fn main() -> CommandResult
{
    let mut app = resolve_console("cli", std::env::args_os())?;
    let config: AppConfig = app.load_config()?; // also bound into `app.container`
    let crates = workspace_crates!();
    let _logging = init_tracing(&crates, &config.logging)?;
    tracing::info!("Loaded modules: {}", crates.join(", "));
    app.resolve().await
}
```

The resolved command carries the container: `load_config` binds the loaded config into it, the
console adds the command's parsed args, and the handler pulls each parameter out by type, so
the config type derives `Resolvable` and every command in the application shares one config
type. Commands that need no CLI flags can use the clap-free `#[command]` from
`adaliszk-commands` directly.

## Why do this?

Short answer: purely a tool for my own projects.

Long answer: every CLI repeats the same wiring. It declares subcommands, parses a config file,
and dispatches to a handler. This crate formalizes that once, so a new command is just a free
`async fn`, with no central registry to maintain.

## License & Disclosure

Licensed under the Mozilla Public License 2.0 (MPL-2.0).

Development was done with LLM as a search engine and debugger (meaning it is for learning only),
NO SOURCE LINES where generated or changed without direct human decisions and work.

Contributions expected to follow the same **No-Automated-LLM Contributions** policy with strict
limitations to retain the quality and understanding!
