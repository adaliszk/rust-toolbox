// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0

use adaliszk_anymap::Resolvable;
use adaliszk_console::{console_args, console_command, resolve_console, CommandResult};
use serde::Deserialize;
use std::sync::Mutex;

#[derive(Deserialize, Resolvable, Clone)]
struct TestConfig
{
    token: String,
}

#[console_args]
struct RecordArgs
{
    #[arg(long)]
    shout: bool,
}

/// Helper to capture what the handler received.
static RECEIVED: Mutex<Option<String>> = Mutex::new(None);

/// Records the config token and the parsed flag it received
#[console_command(name = "record", args = RecordArgs)]
async fn record_command(config: &TestConfig, args: &RecordArgs) -> CommandResult
{
    let mut token = config.token.clone();
    if args.shout {
        token = token.to_uppercase();
    }
    *RECEIVED.lock().unwrap() = Some(token);
    Ok(())
}

/// End-to-end: argv is parsed, the config is loaded from the `--config` file, both are bound
/// into the container, and the handler resolves them by type.
#[tokio::test]
async fn dispatch_binds_config_and_parsed_args_into_the_container()
{
    let mut app = resolve_console(
        "app",
        ["app", "--config", "tests/fixtures/dispatch.yaml", "record", "--shout"],
    )
    .expect("the command resolves");

    // `load_config` also binds the loaded config into `app.container`:
    let _config: TestConfig = app.load_config().expect("the fixture parses");
    app.resolve().await.expect("dispatch succeeds");

    assert_eq!(RECEIVED.lock().unwrap().as_deref(), Some("SECRET"));
}
