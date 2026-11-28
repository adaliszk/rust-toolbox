// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0
//!
//! # AdaLiszk Console
//!
//! The clap front-end for [`adaliszk-commands`](adaliszk_commands): it builds the CLI from the
//! registered commands, parses the process arguments, loads the configuration, and binds both
//! into the dependency-injection [`Container`] that the matched command resolves from.
//!
//! ## Flow
//!
//! 1. Declare commands with [`console_command`] (a drop-in for `adaliszk-commands`'
//!    [`command`]): parameters resolved from the container by type, and `args = TheArgs`
//!    links the command's flags.
//! 2. Declare each command's flags as a plain struct with [`console_args`]; the attribute
//!    derives `clap::Args`, `Resolvable`, and `Clone`.
//! 3. Call [`resolve_console`] in `main`, load the config (bound into the carried
//!    [`Container`] for you), add any services, then [`ResolvedConsoleCommand::resolve`].
//!
//! The generated derives expand to [`clap`](https://crates.io/crates/clap) and
//! [`adaliszk-anymap`](https://crates.io/crates/adaliszk-anymap) paths, so both are expected
//! as direct dependencies of the application alongside this crate.
//!

/// Re-exports of the command registration layer from
/// [`adaliszk-commands`](https://crates.io/crates/adaliszk-commands).
pub use adaliszk_commands::{command, AsyncError, CommandRegistration, CommandResult, Container};

/// Re-exports of the macro needed to register console commands:
pub use adaliszk_console_macros::console_command;

/// Re-exports of the macro needed to register console arguments:
pub use adaliszk_console_macros::console_args;

mod console;
pub use console::*;

#[doc(hidden)] // Used by the macro-generated registrations only
pub use inventory;

mod prelude;
pub use prelude::*;
