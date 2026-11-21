// SPDX-FileCopyrightText: 2026, Ádám Liszkai "Kicsivazz" and Contributors
// SPDX-License-Identifier: MPL-2.0
//!
//! ## About this crate
//!
//! This crate holds the procedural macros that `adaliszk-prelude` requires for configuration
//! parsing or logging configuration.
//!
//! [`adaliszk-prelude`]: https://docs.rs/adaliszk-prelude
//!

use proc_macro::TokenStream;
use quote::quote;

mod workspace;
///
/// # Collect the workspace member crate names
///
/// For setting the logging in `adaliszk_prelude::init_tracing()` correctly, the used workspaces
/// is intended so that only the scopes required are set, not all dependencies all the way down:
///
/// ```rust
/// use adaliszk_prelude_macros::workspace_crates;
/// let crates: Vec<String> = workspace_crates!();
/// ```
///
/// ## Panics
/// - Fails compilation when `cargo metadata` cannot run, such as `cargo` missing from `PATH`
///   or an unreadable workspace manifest.
///
#[proc_macro]
pub fn workspace_crates(_input: TokenStream) -> TokenStream
{
    let members = workspace::collect_members();
    let names = members.iter().map(String::as_str);
    quote! { vec![#(#names.to_string()),*] }.into()
}
