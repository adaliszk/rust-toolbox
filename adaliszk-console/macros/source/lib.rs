// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0

use proc_macro::TokenStream;

mod arguments;
mod command;

///
/// # Register a console command, optionally linking its arguments
///
/// A drop-in replacement for `#[command]` (which it applies under the hood, passing `name`,
/// `about`, and `tag` through) that can also link the command's `args` struct; the console then
/// contributes the struct's flags to the command's clap subcommand and binds the parsed value
/// into the dependency-injection container:
///
/// ```rust
/// use adaliszk_console::{console_args, console_command, CommandResult};
///
/// #[console_args]
/// struct ExampleArgs
/// {
///     #[arg(long)]
///     force: bool,
/// }
///
/// /// Does the example thing
/// #[console_command(args = ExampleArgs)] // registers as `example` and links the args
/// async fn example(args: &ExampleArgs) -> CommandResult
/// {
///     Ok(())
/// }
///
/// /// No arguments needed:
/// #[console_command]
/// async fn version() -> CommandResult
/// {
///     Ok(())
/// }
///
/// fn main() {}
/// ```
///
/// ## Panics
/// - Fails compilation on unknown options.
/// - Fails compilation when not placed on an async function (via `#[command]`).
///
#[proc_macro_attribute]
pub fn console_command(attr: TokenStream, item: TokenStream) -> TokenStream
{
    command::expand(attr, item)
}

///
/// # Declare a command's arguments
///
/// Attaches `#[derive(clap::Args, Resolvable, Clone)]` to the struct, so none of them are
/// repeated on every args struct. Link the struct to its command with
/// `#[console_command(args = TheArgs)]`.
///
/// ```rust
/// use adaliszk_console::console_args;
///
/// #[console_args]
/// struct ExampleArgs
/// {
///     #[arg(long)]
///     force: bool,
/// }
///
/// fn main() {}
/// ```
///
/// Do not derive `clap::Args`, `Resolvable`, or `Clone` yourself; the attribute adds them.
/// Other derives (`Debug`, etc.) can still be added in your own `#[derive(...)]` list.
///
/// ## Panics
/// - Fails compilation when given any options.
/// - Fails compilation when not placed on a struct.
///
#[proc_macro_attribute]
pub fn console_args(attr: TokenStream, item: TokenStream) -> TokenStream
{
    arguments::expand(attr, item)
}
