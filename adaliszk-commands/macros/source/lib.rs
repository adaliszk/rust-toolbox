// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0

use proc_macro::TokenStream;

mod command;

///
/// # Register a command from a free `async fn`
///
/// Each parameter is resolved from the `adaliszk_commands::Container` by its type, so every
/// parameter type must implement `Resolvable` and be bound in the container before the command
/// runs. The command name is the function name, or `#[command(name = "...")]`. Its one-line
/// description comes from the first `///` line, or `#[command(about = "...")]`.
///
/// ```rust
/// use adaliszk_anymap::Resolvable;
/// use adaliszk_commands::{command, CommandResult};
///
/// #[derive(Resolvable, Clone)]
/// struct AppConfig { verbose: bool }
///
/// /// Does the example thing
/// #[command] // registers as `example`
/// async fn example(_config: &AppConfig) -> CommandResult
/// {
///     Ok(())
/// }
/// ```
///
/// There is no clap involvement here; parsing arguments and binding them into the container is
/// the clap front-end's responsibility.
///
#[proc_macro_attribute]
pub fn command(attr: TokenStream, item: TokenStream) -> TokenStream
{
    command::expand(attr, item)
}
