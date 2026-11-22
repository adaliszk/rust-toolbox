// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0

use adaliszk_console::{
    ConsoleHandler, ConsoleArgs, ConsoleHelp, ConsoleRegistration, ConsoleResult, console,
};
use inventory; // <- Or adaliszk_console::inventory;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
struct AppConfig; // <- Normally implemented in the project's API/Prelude/Foundation

#[derive(ConsoleArgs)]
struct SimpleCommand; // <- Normally implemented in any of the project's workspaces

#[console(name = "simple")]
impl ConsoleHandler<AppConfig> for SimpleCommand
{
    async fn handle(&self, _config: &AppConfig) -> ConsoleResult
    {
        Ok(())
    }
}

#[test]
fn command_registers_in_inventory()
{
    let registered = inventory::iter::<&'static dyn ConsoleRegistration>
        .into_iter()
        .any(|registration| registration.signature() == "simple");

    assert_eq!(
        <SimpleCommand as ConsoleHelp>::DESCRIPTION,
        env!("CARGO_PKG_DESCRIPTION")
    );
    assert!(registered);
}

#[derive(ConsoleArgs)]
struct DescribedCommand; // <- Normally used when multiple commands implemented per workspace

#[console(
    name = "describe",
    about = "this shows up instead of the CARGO_PKG_DESCRIPTION"
)]
impl ConsoleHandler<AppConfig> for DescribedCommand
{
    async fn handle(&self, _config: &AppConfig) -> ConsoleResult
    {
        Ok(())
    }
}

#[test]
fn command_uses_custom_about_description()
{
    assert_eq!(
        <DescribedCommand as ConsoleHelp>::DESCRIPTION,
        "this shows up instead of the CARGO_PKG_DESCRIPTION"
    );
}
