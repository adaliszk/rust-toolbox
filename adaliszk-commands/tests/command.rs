// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0

use adaliszk_anymap::Resolvable;
use adaliszk_commands::{command, CommandRegistration, CommandResult, Container};
use std::sync::Mutex;

#[derive(Resolvable, Clone)]
struct Greeting
{
    text: String,
}

#[derive(Resolvable, Clone)]
struct Punctuation
{
    mark: char,
}

static SEEN: Mutex<Option<String>> = Mutex::new(None);

/// Greets with punctuation
#[command]
async fn greet(greeting: &Greeting, punctuation: &Punctuation) -> CommandResult
{
    *SEEN.lock().unwrap() = Some(format!("{}{}", greeting.text, punctuation.mark));
    Ok(())
}

#[command(name = "noop")]
async fn without_dependencies() -> CommandResult
{
    Ok(())
}

/// this doc line is overridden
#[command(name = "described", about = "explicit about wins")]
async fn described_command() -> CommandResult
{
    Ok(())
}

static SEEN_OWNED: Mutex<Option<String>> = Mutex::new(None);

/// Takes its dependency by value instead of by reference
#[command(name = "owned")]
async fn with_owned_parameter(greeting: Greeting) -> CommandResult
{
    *SEEN_OWNED.lock().unwrap() = Some(greeting.text);
    Ok(())
}

fn registration(name: &str) -> &'static dyn CommandRegistration
{
    *adaliszk_commands::inventory::iter::<&'static dyn CommandRegistration>
        .into_iter()
        .find(|registration| registration.name() == name)
        .expect("command is registered")
}

#[tokio::test]
async fn resolves_every_parameter_from_the_container_and_runs()
{
    let mut container = Container::default();
    container.bind(Greeting { text: "hi".to_owned() });
    container.bind(Punctuation { mark: '!' });

    registration("greet")
        .resolve(&container)
        .await
        .expect("resolve succeeds");

    assert_eq!(SEEN.lock().unwrap().as_deref(), Some("hi!"));
}

#[tokio::test]
async fn a_missing_dependency_is_an_error()
{
    let mut container = Container::default();
    container.bind(Greeting { text: "hi".to_owned() }); // Punctuation left unbound

    let result = registration("greet").resolve(&container).await;

    assert!(result.is_err());
}

#[test]
fn name_is_the_function_name_or_overridden()
{
    assert_eq!(registration("greet").name(), "greet");
    assert_eq!(registration("noop").name(), "noop");
}

#[test]
fn about_comes_from_the_doc_line_or_the_attribute()
{
    assert_eq!(registration("greet").about(), Some("Greets with punctuation"));
    assert_eq!(registration("described").about(), Some("explicit about wins"));
    assert_eq!(registration("noop").about(), None);
}

#[tokio::test]
async fn a_command_with_no_dependencies_runs()
{
    let container = Container::default();

    registration("noop")
        .resolve(&container)
        .await
        .expect("resolve succeeds");
}

#[tokio::test]
async fn resolve_commands_finds_and_runs_a_command_by_name()
{
    let mut cmd = adaliszk_commands::resolve_commands("noop").expect("the command is found");

    cmd.container.bind(Greeting { text: "unused".to_owned() });
    cmd.resolve().await.expect("resolve succeeds");
}

#[test]
fn resolve_commands_errors_on_an_unknown_name()
{
    let error = match adaliszk_commands::resolve_commands("does-not-exist") {
        Err(error) => error,
        Ok(_) => panic!("an unknown command must not resolve"),
    };

    assert!(error.to_string().contains("does-not-exist"));
}

#[tokio::test]
async fn a_by_value_parameter_resolves_to_an_owned_value()
{
    let mut container = Container::default();
    container.bind(Greeting { text: "owned".to_owned() });

    registration("owned")
        .resolve(&container)
        .await
        .expect("resolve succeeds");

    assert_eq!(SEEN_OWNED.lock().unwrap().as_deref(), Some("owned"));
}
