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
        return Err(syn::Error::new_spanned(
            attr,
            "#[console_args] takes no options; link the struct to its command with \
             #[console_command(args = TheArgs)]",
        ));
    }
    let item: ItemStruct = syn::parse2(item)?;

    Ok(quote! {
        // Attach the parsing and resolution derives, so the user does not repeat
        // `clap::Args`, `Resolvable`, and `Clone` on every args struct:
        #[derive(
            ::clap::Args,
            ::adaliszk_anymap::Resolvable,
            ::core::clone::Clone,
        )]
        #item
    })
}

#[cfg(test)]
mod tests
{
    use super::*;
    use quote::quote;

    #[test]
    fn parse_attaches_the_derives()
    {
        let generated = parse(
            quote!(),
            quote! {
                struct ExampleArgs { force: bool }
            },
        )
        .unwrap()
        .to_string();

        assert!(generated.contains("clap :: Args"));
        assert!(generated.contains("Resolvable"));
        assert!(generated.contains("Clone"));
        assert!(generated.contains("struct ExampleArgs"));
    }

    #[test]
    fn parse_rejects_options()
    {
        assert!(parse(quote!("example"), quote! { struct Nope; }).is_err());
    }

    #[test]
    fn parse_rejects_a_non_struct_item()
    {
        assert!(parse(quote!(), quote! { enum Nope { A } }).is_err());
    }
}
