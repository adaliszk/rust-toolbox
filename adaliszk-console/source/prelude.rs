// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0

use crate::{AsyncError, ConsoleOptions, ConsoleRegistration, ConsoleResult};
use adaliszk_prelude::config::{ParseError, parse_files, resolve_paths};
use clap::{ArgMatches, Args, Command, FromArgMatches};
use serde::de::DeserializeOwned;

/// A matched command context returned by [`resolve_console`].
pub struct ResolvedConsoleCommand
{
    registration: &'static dyn ConsoleRegistration,
    config: String,
    params: ArgMatches,
}

impl ResolvedConsoleCommand
{
    /// Resolve the global `--config` glob and layer the matching files into `CONFIG`.
    ///
    /// The glob expands to a sorted file list, then each file merges left to
    /// right so a later file overrides an earlier one.
    pub fn load_config<CONFIG: DeserializeOwned>(&self) -> Result<CONFIG, ParseError>
    {
        parse_files(&resolve_paths(&self.config)?)
    }

    /// Resolve the command result by running the handler with the parsed params.
    pub async fn resolve(&self) -> ConsoleResult
    {
        self.registration.resolve(&self.params).await
    }
}

/// Parse the process arguments and resolve the invoked command.
///
/// Builds a clap CLI named `name`, augments it with [`ConsoleOptions`] and every registered
/// inventory command then parses `std::env::args`.
///
/// Returns the matched command with context as [`ResolvedConsoleCommand`].
///
/// # Usage
///
/// While it handles most of the clap-related things, it leaves out the last step for you:
///
/// ```rust
/// use adaliszk_console::{workspace_crates, resolve_console};
/// use adaliszk_prelude::config::LoggingConfig;
/// use adaliszk_prelude::tracing::init_tracing;
///
/// // Normally declared in the project's API/Prelude/Foundation workspace:
/// #[derive(serde::Deserialize)]
/// struct AppConfig
/// {
///     logging: LoggingConfig,
/// }
///
/// // Import workspace crates so their inventory could register:
/// use example_crate as _;
///
/// #[tokio::main]
/// async fn main() -> ConsoleResult
/// {
///     let app = resolve_console("cli")?;
///     let config: AppConfig = app.load_config()?;
///     let _logging = init_tracing(workspace_crates!(), &config.logging)?;
///     // ^^^ Note the guard here needs to stay bound to keep trace logging alive!
///     app.resolve().await
/// }
/// ```
///
/// # Errors
///
/// Returns the clap error when configuration parsing or argument matching fails.
///
/// # Panics
///
/// Panics when clap matched no subcommand, which clap normally prevents.
pub fn resolve_console(name: &'static str) -> Result<ResolvedConsoleCommand, AsyncError>
{
    let mut cli = Command::new(name);
    cli = ConsoleOptions::augment_args(cli)
        // TODO: add description and version for the main command
        .arg_required_else_help(true)
        .subcommand_required(true);

    for registration in inventory::iter::<&'static dyn ConsoleRegistration> {
        let command = Command::new(registration.signature());
        cli = cli.subcommand(registration.augment_command(command))
    }

    let matches = cli.get_matches();
    let options = ConsoleOptions::from_arg_matches(&matches)?;
    let (name, params) = matches.subcommand().unwrap();

    for registration in inventory::iter::<&'static dyn ConsoleRegistration> {
        if registration.signature() == name {
            return Ok(ResolvedConsoleCommand {
                registration: *registration,
                config: options.config.clone(),
                params: params.clone(),
            });
        }
    }

    unreachable!("clap should have ensured to have at least one subcommand");
}
