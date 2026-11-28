// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0

use adaliszk_commands::{CommandRegistration, Container};

/// The console's global options, available to every command.
#[derive(clap::Args)]
#[command(about = None)] // clap would otherwise surface the doc comment as the CLI's about
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

#[doc(hidden)] // This is internal between the macro and the package
pub trait ConsoleRegistration: Send + Sync
{
    fn command(&self) -> &'static dyn CommandRegistration;
    fn command_name(&self) -> &'static str
    {
        self.command().name()
    }
    fn augment(&self, cmd: clap::Command) -> clap::Command;
    fn bind(&self, matches: &clap::ArgMatches, container: &mut Container)
        -> Result<(), clap::Error>;
}

inventory::collect!(&'static dyn ConsoleRegistration);
