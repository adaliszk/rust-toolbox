Command registration wired through the
[`adaliszk-anymap`](https://crates.io/crates/adaliszk-anymap) dependency-injection container. A
command is a free `async fn`; `#[command]` registers it in
[`inventory`](https://crates.io/crates/inventory), and at resolve time each parameter is pulled
from a `Container` by type.

## Installation & Usage

```sh
cargo add adaliszk-commands
```

```rust
use adaliszk_anymap::Resolvable; // <- the derive needs adaliszk-anymap as a direct dependency
use adaliszk_commands::{command, resolve_commands, CommandResult};

#[derive(Resolvable, Clone)]
struct AppConfig { verbose: bool }

/// Does the example thing
#[command] // registers as `example`; name is the fn name, about is this `///` line
async fn example(config: &AppConfig) -> CommandResult
{
    if config.verbose { /* ... */ }
    Ok(())
}

// Resolve the command by name, bind its dependencies, then run it:
#[tokio::main]
async fn main() -> CommandResult
{
    let mut cmd = resolve_commands("example")?;
    cmd.container.bind(AppConfig { verbose: true });
    cmd.resolve().await
}
```

Every parameter type must implement `Resolvable` and be bound in the container before the command
runs; a missing dependency is returned as an error.

## Why do this?

Short answer: purely a tool for my own projects.

Long answer: it separates *what a command needs* (its typed parameters, resolved from the
container) from *where those values come from* (CLI arguments, config files, services). This crate
owns only the former and stays free of clap; the clap front-end (`adaliszk-console`) fills the
container, so the same command handler could be driven by something other than a CLI.

## License & Disclosure

Licensed under the Mozilla Public License 2.0 (MPL-2.0).

Development was done with LLM as a search engine and debugger (meaning it is for learning only),
NO SOURCE LINES where generated or changed without direct human decisions and work.

Contributions expected to follow the same **No-Automated-LLM Contributions** policy with strict
limitations to retain the quality and understanding!

