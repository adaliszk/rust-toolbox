// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

pub fn expand(item: TokenStream) -> TokenStream
{
    let input = parse_macro_input!(item as DeriveInput);
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics ::adaliszk_anymap::Resolvable for #ident #ty_generics #where_clause {
            fn resolve(
                container: &::adaliszk_anymap::Container,
            ) -> ::core::option::Option<Self> {
                ::core::option::Option::or_else(
                    container.make::<Self>(), || container.get::<Self>().cloned(),
                )
            }
        }
    };

    expanded.into()
}
