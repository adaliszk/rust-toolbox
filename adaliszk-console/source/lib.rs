// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0
//!
//! # AdaLiszk Console
//!
//! A small command framework over [`clap`] and [`inventory`]. Commands register themselves at
//! compile time, so a binary discovers them without a central list, while the commands receive
//! the application's parsed configuration from a common struct declared in the user-space.
//!
//! ## Flow
//!
//! 1. Declare a configuration type in your foundation crate.
//! 2. Implement [`ConsoleHandler`] on a command and annotate it with [`console`].
//! 3. Call [`resolve_console`] in `main`, then [`ResolvedConsoleCommand::resolve`].
//!
//! The [`console`] macro documents the command-authoring side in detail.
//!

/// Re-export of [`inventory`](https://crates.io/crates/inventory)
pub use inventory;

/// Re-export of [`clap`](https://crates.io/crates/clap)
pub use clap;

/// Re-export of [`clap::Args`](https://docs.rs/clap/4.6.9/clap/trait.Args.html)
pub use clap::Args as ConsoleArgs;

/// Re-export of [`async_trait`](https://crates.io/crates/async-trait)
pub use async_trait::async_trait as Async;

mod types;
pub use types::*;

mod console;
pub use console::*;

mod prelude;
pub use prelude::*;
