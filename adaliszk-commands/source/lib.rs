// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0
//!
//! Command registration wired through the [`adaliszk-anymap`](adaliszk_anymap)
//! dependency-injection [`Container`]. A command is a free `async fn`; [`command`] registers it
//! in [`inventory`](https://crates.io/crates/inventory), and at resolve time each parameter is
//! pulled from the container by type.
//!
//! The container must already hold every dependency a command needs. Filling it (parsing CLI
//! arguments, loading config, binding both in) is the job of the clap front-end
//! (`adaliszk-console`); this crate has no clap dependency.
//!

/// Re-export of [`adaliszk-anymap`](https://crates.io/crates/adaliszk-anymap)'s container.
///
/// The `Resolvable` trait and derive are expected from `adaliszk-anymap` directly; the derive
/// expands to that crate's paths, so it must be a dependency of the crate deriving it.
pub use adaliszk_anymap::Container;

/// Register a command from a free `async fn`; see the crate docs.
pub use adaliszk_commands_macros::command;

#[doc(hidden)] // Used by the macro-generated registrations only
pub use async_trait::async_trait;

mod commands;
pub use commands::*;

#[doc(hidden)] // Used by the macro-generated registrations only
pub use inventory;

mod prelude;
pub use prelude::*;
