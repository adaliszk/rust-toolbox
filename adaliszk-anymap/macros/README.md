# adaliszk-anymap-macros

Procedural-macro companion for [`adaliszk-anymap`](https://crates.io/crates/adaliszk-anymap).

Provides `#[derive(Resolvable)]`, which implements `adaliszk_anymap::Resolvable` to prefer the
type's factory and fall back to cloning its bound value out of a `Container`. Use
`adaliszk-anymap` directly; this crate is re-exported from it.
