> [!WARNING]
> Depend on [`adaliszk-console`](https://crates.io/crates/adaliszk-console) instead. The console
> re-exports every macro from here, so you never name this crate directly.

## What it provides

`#[console(name = "...", about = "...")]`

An attribute placed on a `ConsoleHandler<CONFIG>` implementation. It registers the command in
[`inventory`](https://docs.rs/inventory), bridges the handler to the internal registration, and
reads the application config for you. The `about` option sets the help text, falling back to
`CARGO_PKG_DESCRIPTION` when omitted.

## License & Disclosure

Licensed under the Mozilla Public License 2.0 (MPL-2.0).

Development was done with LLM as a search engine and debugger (meaning it is for learning only),
NO SOURCE LINES where generated or changed without direct human decisions and work.

Contributions expected to follow the same **No-Automated-LLM Contributions** policy with strict
limitations to retain the quality and understanding!
