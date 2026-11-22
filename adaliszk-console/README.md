A small command framework over [`clap`](https://crates.io/crates/clap) and
[`inventory`](https://crates.io/crates/inventory). Commands register themselves at compile time,
so a binary discovers them without a central list, and every command receives the application's
parsed configuration from a common type declared in user-space.

## Example Usage

A command is a struct with its own flags plus a handler:

```rust
use adaliszk_console::{ConsoleArgs, ConsoleHandler, ConsoleResult, console};

#[derive(ConsoleArgs)]
struct Serve;

#[console(name = "serve")]
impl ConsoleHandler<AppConfig> for Serve {
    async fn handle(&self, config: &AppConfig) -> ConsoleResult {
        Ok(())
    }
}
```

Resolve the invoked command in `main`, read the config, then run it:

```rust
#[tokio::main]
async fn main() -> ConsoleResult {
    let app = resolve_console::<AppConfig>("cli")?;
    let _logging = init_tracing(workspace_crates!(), &app.config().logging)?;
    app.resolve().await
}
```

`AppConfig` is your own configuration type, declared and shared by every command. The
`#[console]` macro reads it from the handler and passes the parsed config.

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
