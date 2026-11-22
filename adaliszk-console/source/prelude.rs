// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0

use crate::{AsyncError, ConsoleOptions, ConsoleRegistration, ConsoleResult};
use clap::{ArgMatches, Args, Command, FromArgMatches};
use serde::de::DeserializeOwned;

/// A matched command context returned by [`resolve_console`].
pub struct ResolvedConsoleCommand<CONFIG: DeserializeOwned + Clone + Send + Sync + 'static>
{
    registration: &'static dyn ConsoleRegistration,
    config: CONFIG,
    params: ArgMatches,
}

impl<CONFIG: DeserializeOwned + Clone + Send + Sync + 'static> ResolvedConsoleCommand<CONFIG>
{
    /// The parsed global configuration, for example to pass to `init_tracing`.
    pub fn config(&self) -> &CONFIG
    {
        &self.config
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
/// // Import workspace crates os their inventory could register:
/// use example_crate as _;
///
/// #[tokio::main]
/// async fn main() -> ConsoleResult
/// {
///     let app = resolve_console::<AppConfig>("cli")?;
///     let _logging = init_tracing(workspace_crates!(), &app.config().logging)?;
///     // ^^^ Note the guard here needs to exist toe keep trace logging alive!
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
pub fn resolve_console<CONFIG: DeserializeOwned + Clone + Send + Sync + 'static>(
    name: &'static str,
) -> Result<ResolvedConsoleCommand<CONFIG>, AsyncError>
{
    let mut cli = Command::new(name);
    cli = ConsoleOptions::<CONFIG>::augment_args(cli);

    for registration in inventory::iter::<&'static dyn ConsoleRegistration> {
        let command = Command::new(registration.signature());
        cli = cli.subcommand(registration.augment_command(command))
    }

    let matches = cli.get_matches();
    let options = ConsoleOptions::<CONFIG>::from_arg_matches(&matches)?;
    let (name, params) = matches.subcommand().unwrap();

    for registration in inventory::iter::<&'static dyn ConsoleRegistration> {
        if registration.signature() == name {
            return Ok(ResolvedConsoleCommand {
                registration: *registration,
                config: options.config,
                params: params.clone(),
            });
        }
    }

    unreachable!("clap should have ensured to have at least one subcommand");
}
