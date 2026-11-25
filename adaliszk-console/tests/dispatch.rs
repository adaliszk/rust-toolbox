// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0

use adaliszk_console::{
    ConsoleArgs, ConsoleHandler, ConsoleOptions, ConsoleRegistration, ConsoleResult, console,
};
use clap::Command;
use serde::Deserialize;
use std::sync::Mutex;

#[derive(Deserialize, Clone)]
struct TestConfig
{
    token: String,
}

/// Helper to track if handle was called or not
static RECEIVED: Mutex<Option<String>> = Mutex::new(None);

#[derive(ConsoleArgs)]
struct TestCommand;

#[console(name = "test")]
impl ConsoleHandler<TestConfig> for TestCommand
{
    async fn handle(&self, config: &TestConfig) -> ConsoleResult
    {
        *RECEIVED.lock().unwrap() = Some(config.token.clone());
        Ok(())
    }
}

/// End-to-end: the global `--config` is parsed from a file and passed to the handler
#[tokio::test]
async fn dispatch_passes_parsed_config_to_handler()
{
    let registration = inventory::iter::<&'static dyn ConsoleRegistration>
        .into_iter()
        .find(|registration| registration.signature() == "test")
        .expect("record command is registered");

    let cli = ConsoleOptions::augment_args(Command::new("app"));
    let cli =
        cli.subcommand(registration.augment_command(Command::new(registration.signature())));

    let config_path = "tests/fixtures/dispatch.yaml";
    let matches = cli.get_matches_from([
        "app",
        "--config",
        config_path,
        "test",
        "--config",
        config_path,
    ]);
    let (_, params) = matches.subcommand().expect("a subcommand was matched");

    registration
        .resolve(params)
        .await
        .expect("dispatch succeeds");

    assert_eq!(RECEIVED.lock().unwrap().as_deref(), Some("secret"));
}
