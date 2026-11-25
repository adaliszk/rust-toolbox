// SPDX-FileCopyrightText: 2026, Ádám Liszkai "Kicsivazz" and Contributors
// SPDX-License-Identifier: MPL-2.0

pub use adaliszk_console_macros::console;
use crate::{Async, ConsoleResult};

#[derive(clap::Args)]
pub struct ConsoleOptions
{
    #[arg(
        long,
        help = "Load configuration from a file",
        default_value = "config.yaml",
        global = true,
    )]
    pub config: String,
}

/// Each command implements a handler that can ship its own clap arguments.
///
/// Annotate the implementation with [`console`] to register the command, like:
///
/// ```rust
/// use adaliszk_console::{ConsoleHandler, ConsoleResult, console};
///
/// #[derive(serde::Deserialize)]
/// struct AppConfig; // <- Declared in the project's API/Prelude/Foundation workspace
///
/// #[console(name = "example")]
/// pub struct MyConsoleCommand;
///
/// impl ConsoleHandler<AppConfig> for MyConsoleCommand
/// {
///     async fn handle(&self, _config: &AppConfig) -> ConsoleResult { todo!() }
/// }
/// ```
#[Async]
pub trait ConsoleHandler<CONFIG>
{
    async fn handle(&self, config: &CONFIG) -> ConsoleResult;
}

#[Async]
#[doc(hidden)] // This is internal between the macro and the package
pub trait ConsoleRegistration: Send + Sync
{
    fn signature(&self) -> &'static str;
    fn augment_command(&self, cmd: clap::Command) -> clap::Command;
    async fn resolve(&self, args: &clap::ArgMatches) -> ConsoleResult;
}

inventory::collect!(&'static dyn ConsoleRegistration);
