// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0

use adaliszk_anymap::Resolvable;
use adaliszk_console::{
    console_args, console_command, resolve_console, CommandResult, ConsoleRegistration,
};
use serde::Deserialize;

#[derive(Deserialize, Resolvable, Clone)]
struct AppConfig; // <- Normally implemented in the project's API/Prelude/Foundation

#[console_args]
struct SimpleArgs
{
    #[arg(long)]
    force: bool,
}

/// Runs the simple command
#[console_command(args = SimpleArgs)]
async fn simple(_config: &AppConfig, args: &SimpleArgs) -> CommandResult
{
    assert!(args.force, "the parsed flag reaches the handler");
    Ok(())
}

/// A command with no arguments attached still resolves
#[console_command]
async fn bare(_config: &AppConfig) -> CommandResult
{
    Ok(())
}

#[tokio::test]
async fn a_command_with_args_receives_the_parsed_flags()
{
    let mut app = resolve_console("app", ["app", "simple", "--force"])
        .expect("the command resolves");

    app.container.bind(AppConfig);
    app.resolve().await.expect("dispatch succeeds");
}

#[tokio::test]
async fn a_command_without_args_only_needs_the_config()
{
    let mut app = resolve_console("app", ["app", "bare"]).expect("the command resolves");

    app.container.bind(AppConfig);
    app.resolve().await.expect("dispatch succeeds");
}

#[derive(Resolvable, Clone)]
struct Service
{
    id: u32,
}

/// Uses a service bound by the application, not parsed from the CLI
#[console_command]
async fn serviced(_config: &AppConfig, service: &Service) -> CommandResult
{
    assert_eq!(service.id, 7, "the factory-built service reaches the handler");
    Ok(())
}

#[tokio::test]
async fn an_application_bound_service_reaches_the_handler()
{
    let mut app = resolve_console("app", ["app", "serviced"]).expect("the command resolves");

    app.container.bind(AppConfig);
    app.container.singleton_with(|_| Service { id: 7 });
    app.resolve().await.expect("dispatch succeeds");
}

#[test]
fn a_console_registration_points_to_its_command()
{
    let registration = adaliszk_console::inventory::iter::<&'static dyn ConsoleRegistration>
        .into_iter()
        .find(|registration| registration.command_name() == "simple")
        .expect("the args are registered");

    let command = registration.command();
    assert_eq!(command.name(), "simple");
    assert_eq!(command.about(), Some("Runs the simple command"));
}

#[tokio::test]
async fn a_missing_config_binding_is_an_error()
{
    #[derive(Resolvable, Clone)]
    struct OtherConfig;

    let mut app = resolve_console("app", ["app", "bare"]).expect("the command resolves");

    // The handler wants `AppConfig`, but only `OtherConfig` gets bound:
    app.container.bind(OtherConfig);
    let result = app.resolve().await;
    assert!(result.is_err());
}
