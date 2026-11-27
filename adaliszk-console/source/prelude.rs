// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0

use crate::{ConsoleOptions, ConsoleRegistration};
use adaliszk_commands::{AsyncError, CommandRegistration, CommandResult, Container};
use adaliszk_prelude::config::{ParseError, parse_files, resolve_paths};
use clap::{ArgMatches, Args, Command, FromArgMatches};
use serde::de::DeserializeOwned;
use std::ffi::OsString;

/// A matched command context returned by [`resolve_console`].
pub struct ResolvedConsoleCommand
{
    /// The dependency-injection container the command resolves its parameters from.
    ///
    /// [`load_config`](Self::load_config) binds the loaded config into it; add services with
    /// `bind_with` (transient) or `singleton_with` (lazy, cached) before
    /// [`resolve`](Self::resolve).
    pub container: Container,
    registration: &'static dyn CommandRegistration,
    config: String,
    params: ArgMatches,
}

impl ResolvedConsoleCommand
{
    /// Resolve the global `--config` glob and layer the matching files into `CONFIG`.
    ///
    /// The glob expands to a sorted file list, then each file merges left to right so a later
    /// file overrides an earlier one. The loaded config is also bound into
    /// [`container`](Self::container), so the command handler can resolve it by type.
    pub fn load_config<CONFIG>(&mut self) -> Result<CONFIG, ParseError>
    where
        CONFIG: DeserializeOwned + Clone + Send + Sync + 'static,
    {
        let config: CONFIG = parse_files(&resolve_paths(&self.config)?)?;
        self.container.bind(config.clone());
        Ok(config)
    }

    /// Run the matched command with its dependencies resolved from the container.
    ///
    /// The console adds every args struct linked to this command with
    /// [`console_command`](crate::console_command)'s `args = ...`, parsed from the matched
    /// arguments; the command handler then resolves its parameters from
    /// [`container`](Self::container) by type.
    pub async fn resolve(mut self) -> CommandResult
    {
        for arguments in inventory::iter::<&'static dyn ConsoleRegistration> {
            if arguments.command_name() == self.registration.name() {
                arguments.bind(&self.params, &mut self.container)?;
            }
        }

        self.registration.resolve(&self.container).await
    }
}

/// Parse the given arguments and resolve the invoked command.
///
/// Builds a clap CLI named `name` from [`ConsoleOptions`] plus every registered command — each
/// subcommand carries its `about` and the flags of the args struct linked with
/// [`console_command`](crate::console_command) — then parses `argv` (normally
/// `std::env::args_os()`).
///
/// Returns the matched command with context as [`ResolvedConsoleCommand`].
///
/// # Usage
///
/// While it handles most of the clap-related things, it leaves out the last step for you:
///
/// ```rust,no_run
/// use adaliszk_console::resolve_console;
/// use adaliszk_prelude::config::LoggingConfig;
/// use adaliszk_prelude::tracing::{init_tracing, workspace_crates};
///
/// // Normally declared in the project's API/Prelude/Foundation workspace, and the
/// // crates registering commands imported so their inventory submissions link in:
/// #[derive(serde::Deserialize, adaliszk_anymap::Resolvable, Clone)]
/// struct AppConfig
/// {
///     logging: LoggingConfig,
/// }
///
/// #[tokio::main]
/// async fn main() -> adaliszk_console::CommandResult
/// {
///     let mut app = resolve_console("cli", std::env::args_os())?;
///     let config: AppConfig = app.load_config()?; // also bound into `app.container`
///     let _logging = init_tracing(&workspace_crates!(), &config.logging)?;
///     // ^^^ Note the guard here needs to stay bound to keep trace logging alive!
///
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
pub fn resolve_console<ARG>(
    name: &'static str,
    argv: impl IntoIterator<Item = ARG>,
) -> Result<ResolvedConsoleCommand, AsyncError>
where
    ARG: Into<OsString> + Clone,
{
    let mut cli = Command::new(name);
    cli = ConsoleOptions::augment_args(cli)
        // TODO: add description and version for the main command
        .arg_required_else_help(true)
        .subcommand_required(true);

    for registration in inventory::iter::<&'static dyn CommandRegistration> {
        let mut command = Command::new(registration.name());
        if let Some(about) = registration.about() {
            command = command.about(about);
        }
        for arguments in inventory::iter::<&'static dyn ConsoleRegistration> {
            if arguments.command_name() == registration.name() {
                command = arguments.augment(command);
            }
        }
        cli = cli.subcommand(command);
    }

    let matches = cli.get_matches_from(argv);
    let options = ConsoleOptions::from_arg_matches(&matches)?;
    let (name, params) = matches.subcommand().unwrap();

    for registration in inventory::iter::<&'static dyn CommandRegistration> {
        if registration.name() == name {
            return Ok(ResolvedConsoleCommand {
                container: Container::default(),
                registration: *registration,
                config: options.config.clone(),
                params: params.clone(),
            });
        }
    }

    unreachable!("clap should have ensured to have at least one subcommand");
}
