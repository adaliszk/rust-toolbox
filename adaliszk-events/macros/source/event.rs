// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0

use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemStruct;

#[doc(hidden)] // Internal use only
pub fn expand(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream
{
    parse(attr.into(), item.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

pub fn parse(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream>
{
    if !attr.is_empty() {
        return Err(syn::Error::new_spanned(attr, "#[event] takes no options"));
    }
    let item: ItemStruct = syn::parse2(item)?;
    let ident = item.ident.clone();

    Ok(quote! {
        // Attach the resolution derives, so the user does not repeat `Resolvable` and
        // `Clone` on every event struct:
        #[derive(::adaliszk_anymap::Resolvable, ::core::clone::Clone)]
        #item

        impl adaliszk_events::Dispatchable for #ident {}
    })
}

#[cfg(test)]
mod tests
{
    use super::*;
    use quote::quote;

    #[test]
    fn parse_attaches_the_derives_and_dispatchable()
    {
        let generated = parse(
            quote!(),
            quote! {
                struct MessageArrived { message: String }
            },
        )
        .unwrap()
        .to_string();

        assert!(generated.contains("Resolvable"));
        assert!(generated.contains("Clone"));
        assert!(generated.contains("Dispatchable for MessageArrived"));
    }

    #[test]
    fn parse_rejects_options()
    {
        assert!(parse(quote!("name"), quote! { struct Nope; }).is_err());
    }

    #[test]
    fn parse_rejects_a_non_struct_item()
    {
        assert!(parse(quote!(), quote! { enum Nope { A } }).is_err());
    }
}
