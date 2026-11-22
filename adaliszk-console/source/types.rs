// SPDX-FileCopyrightText: 2026, Ádám Liszkai "Kicsivazz" and Contributors
// SPDX-License-Identifier: MPL-2.0

use std::error::Error;

/// A boxed, thread-safe error returned by console operations.
///
/// Any [`Error`] that is [`Send`] + [`Sync`] converts into it with `?`, so
/// handlers propagate application errors without a shared error enum.
pub type AsyncError = Box<dyn Error + Send + Sync>;

/// Supplies a command's help text.
///
/// The [`console`](crate::console) macro implements this for each command, using
/// the `about = "..."` option or falling back to `CARGO_PKG_DESCRIPTION`.
pub trait ConsoleHelp
{
    /// The one-line description shown in `--help`.
    const DESCRIPTION: &'static str;
}

/// The result a command handler returns: `Ok(())` on success, or any
/// [`AsyncError`].
pub type ConsoleResult = Result<(), AsyncError>;
