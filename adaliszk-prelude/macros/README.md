> [!WARNING]
> Depend on [`adaliszk-prelude`](https://crates.io/crates/adaliszk-prelude) instead. The prelude
> re-exports every macro from here, so you never name this crate directly.

## What it provides

#### `workspace_crates!() -> Vec<String>`

Expands into a list of package names within at compile time. It resolves them through
`cargo metadata`, so globs and real package names resolve correctly. The prelude uses the list
to scope logging filters to first-party crates.

## License & Disclosure

Licensed under the Mozilla Public License 2.0 (MPL-2.0).

Development was done with LLM as a search engine and debugger (meaning it is for learning only),
NO SOURCE OR TEST LINES where generated or changed without direct human decisions and work.

Contributions expected to follow the same **No-Automated-LLM Contributions** policy with strict
limitations to retain the quality and understanding!
