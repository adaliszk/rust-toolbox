// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0

use proc_macro::TokenStream;

mod console;

///
/// # Register a Console Command using Clap via Prelude types
///
/// Derive `ConsoleArgs` on the command struct for its own flags, then place
/// `#[console(name = "...")]` on its `ConsoleHandler<CONFIG>` implementation. The
/// macro registers the command in [`inventory`](https://docs.rs/inventory) and
/// runs it with the global config read for you:
///
/// ```rust
/// use adaliszk_console::{ConsoleArgs, ConsoleHandler, ConsoleResult, console};
/// use adaliszk_prelude::config::LoggingConfig;
///
/// // Normally declared in the project's API/Prelude/Foundation workspace:
/// #[derive(serde::Deserialize)]
/// struct AppConfig
/// {
///     logging: LoggingConfig,
/// }
///
/// #[derive(ConsoleArgs)]
/// struct ExampleCommand;
///
/// #[console(name = "example")]
/// impl ConsoleHandler<AppConfig> for ExampleCommand
/// {
///     async fn handle(&self, config: &AppConfig) -> ConsoleResult
///     {
///         Ok(())
///     }
/// }
/// ```
///
/// ## Running the console
///
/// The application shell resolves the command and reads the parsed config:
///
/// ```rust
/// use adaliszk_console::{workspace_crates, resolve_console};
/// use adaliszk_prelude::config::LoggingConfig;
/// use adaliszk_prelude::tracing::init_tracing;
///
/// // Normally declared in the project's API/Prelude/Foundation workspace:
/// #[derive(serde::Deserialize)]
/// struct AppConfig
/// {
///     logging: LoggingConfig,
/// }
///
/// // Import workspace crates os their inventory could register:
/// use example_crate as _;
///
/// #[tokio::main]
/// async fn main() -> ConsoleResult
/// {
///     let app = resolve_console::<AppConfig>("cli")?;
///     let _logging = init_tracing(workspace_crates!(), &app.config().logging)?;
///     // ^^^ Note the guard here needs to exist toe keep trace logging alive!
///     app.resolve().await
/// }
/// ```
///
/// ## Panics
/// - Fails compilation when `#[console(name = "...")]` is missing or malformed.
/// - Fails compilation when not placed on `impl ConsoleHandler<Config> for Struct`.
///
#[proc_macro_attribute]
pub fn console(attr: TokenStream, item: TokenStream) -> TokenStream
{
    console::expand(attr, item)
}
