> [!WARNING]
> Depend on [`adaliszk-console`](https://crates.io/crates/adaliszk-console) instead.
> The console re-exports every macro from here, so you never name this crate directly.

## What it provides

`#[console_command(name = "...", about = "...", args = TheArgs)]`

A drop-in replacement for `adaliszk-commands`' `#[command]` attribute (which it applies under
the hood, passing `name` and `about` through). The optional `args = TheArgs` links the command's
flags: the console contributes the struct's flags to the clap subcommand and binds the parsed
value into the dependency-injection container.

`#[console_args]`

Attaches `#[derive(clap::Args, Resolvable, Clone)]` to an arguments struct, so none of them are
repeated at every declaration.

## License & Disclosure

Licensed under the Mozilla Public License 2.0 (MPL-2.0).

Development was done with LLM as a search engine and debugger (meaning it is for learning only),
NO SOURCE LINES where generated or changed without direct human decisions and work.

Contributions expected to follow the same **No-Automated-LLM Contributions** policy with strict
limitations to retain the quality and understanding!
