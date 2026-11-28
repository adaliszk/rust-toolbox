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

/// The CLI identity shown by clap's help and version output.
///
/// Fill it from the application's Cargo metadata: the `CARGO_PKG_*` macros expand in the
/// application crate, so the values come from its own Cargo.toml:
///
/// ```rust
/// use adaliszk_console::ConsoleInfo;
///
/// let info = ConsoleInfo {
///     name: env!("CARGO_PKG_NAME"),
///     version: option_env!("CARGO_PKG_VERSION"),
///     about: option_env!("CARGO_PKG_DESCRIPTION"),
/// };
/// ```
///
/// Or construct it by hand for custom text; unset fields come from [`Default`], and a plain
/// `&'static str` converts into a name-only identity.
#[derive(Default)]
pub struct ConsoleInfo
{
    /// The binary name shown in the usage line.
    pub name: &'static str,
    /// Enables clap's `-V`/`--version` flag when set.
    pub version: Option<&'static str>,
    /// The description shown at the top of `--help` when set.
    pub about: Option<&'static str>,
}

impl From<&'static str> for ConsoleInfo
{
    fn from(name: &'static str) -> Self
    {
        Self { name, ..Self::default() }
    }
}

/// Parse the given arguments and resolve the invoked command.
///
/// Builds a clap CLI carrying the given [`ConsoleInfo`] identity (or a plain name) from
/// [`ConsoleOptions`] plus every registered command, where each subcommand carries its `about`
/// and the flags of the args struct linked with [`console_command`](crate::console_command),
/// then parses `argv` (normally `std::env::args_os()`).
///
/// Returns the matched command with context as [`ResolvedConsoleCommand`].
///
/// # Usage
///
/// While it handles most of the clap-related things, it leaves out the last step for you:
///
/// ```rust,no_run
/// use adaliszk_console::{resolve_console, ConsoleInfo};
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
///     let info = ConsoleInfo {
///         name: env!("CARGO_PKG_NAME"),
///         version: option_env!("CARGO_PKG_VERSION"),
///         about: option_env!("CARGO_PKG_DESCRIPTION"),
///     };
///
///     let mut app = resolve_console(info, std::env::args_os())?;
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
    info: impl Into<ConsoleInfo>,
    argv: impl IntoIterator<Item = ARG>,
) -> Result<ResolvedConsoleCommand, AsyncError>
where
    ARG: Into<OsString> + Clone,
{
    let mut cli = build_cli(info.into());

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

fn build_cli(info: ConsoleInfo) -> Command
{
    // The identity comes after the augmentation, as the derive applies its own about reset.
    // Cargo sets CARGO_PKG_DESCRIPTION to "" when absent, so an empty about is dropped here:
    ConsoleOptions::augment_args(Command::new(info.name))
        .version(info.version)
        .about(info.about.filter(|about| !about.is_empty()))
        .arg_required_else_help(true)
        .subcommand_required(true)
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn the_console_info_reaches_the_cli()
    {
        let cli = build_cli(ConsoleInfo {
            name: "app",
            version: Some("1.2.3"),
            about: Some("Does things"),
        });

        assert_eq!(cli.get_name(), "app");
        assert_eq!(cli.get_version(), Some("1.2.3"));
        assert_eq!(
            cli.get_about().map(ToString::to_string),
            Some("Does things".into())
        );
    }

    #[test]
    fn a_name_only_identity_adds_no_version_or_about()
    {
        let cli = build_cli("app".into());

        assert_eq!(cli.get_version(), None);
        assert_eq!(cli.get_about(), None);
    }

    #[test]
    fn an_empty_cargo_description_is_dropped_from_the_about()
    {
        let cli = build_cli(ConsoleInfo {
            name: "app",
            about: Some(""),
            ..ConsoleInfo::default()
        });

        assert_eq!(cli.get_about(), None);
    }
}
