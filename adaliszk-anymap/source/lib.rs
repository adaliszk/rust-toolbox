// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0
//!
//! Laravel-inspired, dependency-injection-friendly wrapper over
//! [`anymap3`](https://crates.io/crates/anymap3).
//!
//! A [`Container`] holds at most one provider per type: a bound value
//! ([`bind`](Container::bind)), a transient factory ([`bind_with`](Container::bind_with)) run
//! on every resolve, or a lazy singleton ([`singleton_with`](Container::singleton_with)) built
//! once and cached. The [`Resolvable`] derive lets a type pull itself out of a container, the
//! building block for the project's handler dependency injection.
//!
//! ```rust
//! use adaliszk_anymap::{Container, Resolvable};
//!
//! #[derive(Resolvable, Clone)]
//! struct Something(String);
//!
//! let mut container = Container::default();
//! container.bind(Something("foo".to_owned()));
//!
//! let resolved: Something = container.require::<Something>().expect("bound above");
//! ```

/// Re-export of [`anymap3`](https://crates.io/crates/anymap3), the backing store.
pub use anymap3;

mod container;
pub use container::*;

/// Derive [`Resolvable`] so a type can be pulled out of a [`Container`] by type.
///
/// The generated implementation prefers the type's factory and falls back to cloning its
/// bound value, so the type must be `Clone`.
pub use adaliszk_anymap_macros::Resolvable;
