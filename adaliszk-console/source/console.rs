// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0

use adaliszk_commands::{CommandRegistration, Container};

/// The console's global options, available to every command.
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

/// The console-side registration of a command's clap arguments, collected by [`inventory`].
///
/// The [`console_command`](crate::console_command) macro implements this for each linked
/// `args = ...` struct: it contributes the struct's flags to the command's clap subcommand, and
/// after parsing it binds the parsed value into the [`Container`] so the command handler can
/// resolve it by type.
#[doc(hidden)] // This is internal between the macro and the package
pub trait ConsoleRegistration: Send + Sync
{
    /// The command these arguments belong to — a direct link, since the
    /// [`console_command`](crate::console_command) macro generates both registrations in one
    /// expansion.
    fn command(&self) -> &'static dyn CommandRegistration;

    /// The name of the command these arguments belong to.
    fn command_name(&self) -> &'static str
    {
        self.command().name()
    }

    /// Contribute the args struct's flags to the command's clap subcommand.
    fn augment(&self, cmd: clap::Command) -> clap::Command;

    /// Parse the args struct from the matches and bind it into the container.
    fn bind(&self, matches: &clap::ArgMatches, container: &mut Container)
        -> Result<(), clap::Error>;
}

inventory::collect!(&'static dyn ConsoleRegistration);
