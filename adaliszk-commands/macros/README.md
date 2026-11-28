> [!WARNING]
> Depend on [`adaliszk-commands`](https://crates.io/crates/adaliszk-commands) instead.
> The commands re-exports every macro from here, so you never name this crate directly.

## What it provides

`#[command(name = "...", about = "...", tag = "...")]`

The attribute registers a free `async fn` in `inventory` and
generates a `CommandRegistration` that resolves each parameter from a `Container` by type. The
optional `tag` makes the command opt-in: it resolves only when the application enables the tag.
Use `adaliszk-commands` directly; this crate is re-exported from it.

## License & Disclosure

Licensed under the Mozilla Public License 2.0 (MPL-2.0).

Development was done with LLM as a search engine and debugger (meaning it is for learning only),
NO SOURCE LINES where generated or changed without direct human decisions and work.

Contributions expected to follow the same **No-Automated-LLM Contributions** policy with strict
limitations to retain the quality and understanding!
