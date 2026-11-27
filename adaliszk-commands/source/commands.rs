// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0

use crate::Container;
use async_trait::async_trait;

/// A boxed, thread-safe error returned by command handlers.
///
/// Any [`Error`](std::error::Error) that is [`Send`] + [`Sync`] converts into it with `?`.
pub type AsyncError = Box<dyn std::error::Error + Send + Sync>;

/// The result a command handler returns: `Ok(())` on success, or any [`AsyncError`].
pub type CommandResult = Result<(), AsyncError>;

/// A command collected by [`inventory`], resolved from a [`Container`].
///
/// The [`command`](crate::command) macro implements this for each registered function.
#[async_trait]
pub trait CommandRegistration: Send + Sync
{
    /// The command's name — its inventory and dispatch key.
    fn name(&self) -> &'static str;

    /// The command's one-line description, from its first `///` line or `about = "..."`.
    fn about(&self) -> Option<&'static str>;

    /// Resolve the handler's parameters from `container` by type, then run it.
    ///
    /// Returns an error if a parameter's dependency is not bound in the container.
    async fn resolve(&self, container: &Container) -> CommandResult;
}

inventory::collect!(&'static dyn CommandRegistration);
