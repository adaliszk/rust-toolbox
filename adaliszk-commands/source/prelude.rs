// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0

use crate::{AsyncError, CommandRegistration, CommandResult, Container};

/// A matched command returned by [`resolve_commands`].
pub struct ResolvedCommand
{
    /// The dependency-injection container the command resolves its parameters from.
    ///
    /// Bind the command's dependencies (values, `bind_with` factories, or `singleton_with`
    /// singletons) before [`resolve`](Self::resolve).
    pub container: Container,
    registration: &'static dyn CommandRegistration,
}

impl ResolvedCommand
{
    /// Run the command with its dependencies resolved from
    /// [`container`](Self::container) by type.
    pub async fn resolve(self) -> CommandResult
    {
        self.registration.resolve(&self.container).await
    }
}

/// Find a registered command by name, ready to run with a carried [`Container`].
///
/// This is the front-end-free dispatch: no argument parsing and no configuration loading;
/// the caller binds every dependency itself. A CLI front-end like `adaliszk-console` does
/// those parts on top.
///
/// # Usage
///
/// ```rust
/// use adaliszk_anymap::Resolvable;
/// use adaliszk_commands::{command, resolve_commands, CommandResult};
///
/// #[derive(Resolvable, Clone)]
/// struct Greeting
/// {
///     text: String,
/// }
///
/// /// Greets whoever is configured
/// #[command]
/// async fn greet(_greeting: &Greeting) -> CommandResult
/// {
///     Ok(())
/// }
///
/// #[tokio::main]
/// async fn main() -> CommandResult
/// {
///     let mut cmd = resolve_commands("greet")?;
///     cmd.container.bind(Greeting { text: "hi".to_owned() });
///     cmd.resolve().await
/// }
/// ```
///
/// # Errors
///
/// Returns an error when no command with `name` is registered.
pub fn resolve_commands(name: &str) -> Result<ResolvedCommand, AsyncError>
{
    resolve_commands_with_tags(name, &[])
}

/// [`resolve_commands`] with the given tag groups enabled.
///
/// An untagged command always matches; a command registered with `#[command(tag = "...")]`
/// matches only when its tag is in `tags`. Tags behave as opt-in groups, so
/// [`resolve_commands`] (an empty tag set) sees the untagged commands only.
///
/// # Usage
///
/// ```rust
/// use adaliszk_commands::{command, resolve_commands, resolve_commands_with_tags, CommandResult};
///
/// /// Seeds example data; wanted in developer builds only
/// #[command(tag = "dev")]
/// async fn seed() -> CommandResult
/// {
///     Ok(())
/// }
///
/// #[tokio::main]
/// async fn main() -> CommandResult
/// {
///     assert!(resolve_commands("seed").is_err(), "tagged commands are opt-in");
///     resolve_commands_with_tags("seed", &["dev"])?.resolve().await
/// }
/// ```
///
/// # Errors
///
/// Returns an error when no command with `name` is registered within the enabled tags.
pub fn resolve_commands_with_tags(
    name: &str,
    tags: &[&str],
) -> Result<ResolvedCommand, AsyncError>
{
    for registration in inventory::iter::<&'static dyn CommandRegistration> {
        if registration.name() == name && tag_enabled(registration.tag(), tags) {
            return Ok(ResolvedCommand {
                container: Container::default(),
                registration: *registration,
            });
        }
    }

    Err(format!("command `{name}` is not registered").into())
}

#[doc(hidden)] // Shared with the console front-end only
pub fn tag_enabled(tag: Option<&str>, tags: &[&str]) -> bool
{
    tag.is_none_or(|tag| tags.contains(&tag))
}
