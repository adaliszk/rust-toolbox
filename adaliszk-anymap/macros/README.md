> [!WARNING]
> Depend on [`adaliszk-anymap`](https://crates.io/crates/adaliszk-anymap) instead.
> The commands re-exports every macro from here, so you never name this crate directly.

## What it provides

`#[derive(Resolvable)]`

Enforces the implementation of `adaliszk_anymap::Resolvable` to prefer the type's factory and
fall back to cloning its bound value out of a `Container`.

## License & Disclosure

Licensed under the Mozilla Public License 2.0 (MPL-2.0).

Development was done with LLM as a search engine and debugger (meaning it is for learning only),
NO SOURCE LINES where generated or changed without direct human decisions and work.

Contributions expected to follow the same **No-Automated-LLM Contributions** policy with strict
limitations to retain the quality and understanding!
