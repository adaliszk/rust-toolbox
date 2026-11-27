Laravel-inspired, dependency-injection-friendly wrapper over
[`anymap3`](https://crates.io/crates/anymap3): a type-keyed `Container` that holds one provider
per type, plus a `#[derive(Resolvable)]` so a type can pull itself out of a container by type.

## Installation & Usage

```sh
cargo add adaliszk-anymap
```

A dependency can be provided three ways, then resolved back by type:

```rust
use adaliszk_anymap::{Container, Resolvable};

#[derive(Resolvable, Clone)]
struct Config
{
    token: String,
}

let mut container = Container::default();

// 1: fixed value, cloned to every usage:
container.bind(Config { token: "secret".to_owned() });

// 2: transient value, the callback builds a fresh one on every usage:
container.bind_with(|_| Config { token: "fresh".to_owned() });

// 3: lazy singleton value, the callback runs once, then the value is cached:
container.singleton_with(|_| Config { token: "once".to_owned() });

// Use the value:
let config: Config = container.require::<Config>().expect("bound above");
```

## Why do this?

Short answer: purely a tool for my own projects.

Long answer: reflection-based dependency injection is a very useful tool that exist in multiple
crates. This is no different using `anyhow` and the only addition is the wrapper methods and
macro to facilitate a Laravel-style mental logic.

## License & Disclosure

Licensed under the Mozilla Public License 2.0 (MPL-2.0).

The initial version was LLM generated, but iterations and refinements from there only involves
LLM as a search engine and debugger (meaning it is for learning only).

Contributions expected to follow **No-Automated-LLM Contributions** policy with strict
limitations to retain the quality and understanding!

