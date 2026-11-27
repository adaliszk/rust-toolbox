// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0

use proc_macro::TokenStream;

mod resolvable;

/// Derive [`Resolvable`] for a type so it can be pulled out of a container by type.
///
/// Generates an implementation that prefers the type's factory and falls back to cloning its
/// bound value out of the `adaliszk_anymap::Container`, so the type must be `Clone` (and
/// `Send + Sync + 'static`).
#[proc_macro_derive(Resolvable)]
pub fn resolvable(item: TokenStream) -> TokenStream
{
    resolvable::expand(item)
}
