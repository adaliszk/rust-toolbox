// SPDX-FileCopyrightText: 2026, Ádám Liszkai "Kicsivazz" and Contributors
// SPDX-License-Identifier: MPL-2.0

use crate::{Async, ConsoleResult};
use adaliszk_prelude::config::{ParseError, parse_config};

pub use adaliszk_console_macros::console;
use serde::de::DeserializeOwned;
use std::path::Path;

/// The global arguments shared by every command, parameterized by `CONFIG` type.
///
/// The derived `--config <CONFIG_FILE>` argument loads and deserializes a file into `CONFIG`.
/// It is `global`, so it applies to every subcommand.
///
/// [`resolve_console`](crate::resolve_console) augments the CLI with these options.
#[derive(clap::Args)]
pub struct ConsoleOptions<CONFIG : DeserializeOwned + Clone + Send + Sync + 'static>
{
    /// The parsed configuration, loaded from the `--config` file.
    #[arg(
        long,
        help = "Load configuration from a file",
        value_name = "CONFIG_FILE",
        default_value = "config.yaml",
        value_parser = parse_config_arg::<CONFIG>,
        global = true,
    )]
    pub config: CONFIG,
}

fn parse_config_arg<CONFIG: DeserializeOwned>(path: &str) -> Result<CONFIG, ParseError>
{
    // FUTURE: Switch to extension detection, and parse JSON, YAML, TOML
    // FUTURE: Parse entire folders by merging keys by the path
    parse_config(Path::new(path))
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
