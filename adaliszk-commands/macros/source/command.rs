// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::parse::Parser;
use syn::{Attribute, FnArg, Ident, ItemFn, LitStr, Type};

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
    let (name_attr, about_attr, tag_attr) = parse_options(attr)?;
    let func: ItemFn = syn::parse2(item)?;

    if func.sig.asyncness.is_none() {
        return Err(syn::Error::new_spanned(
            &func.sig,
            "#[command] requires an async fn",
        ));
    }

    let fn_ident = func.sig.ident.clone();
    let name = command_name(name_attr, &fn_ident);
    let about = match command_about(about_attr, &func.attrs) {
        Some(text) => quote! { ::core::option::Option::Some(#text) },
        None => quote! { ::core::option::Option::None },
    };

    // The trait defaults to no tag, so the override is only generated when one is given:
    let tag = tag_attr.map(|text| {
        quote! {
            fn tag(&self) -> ::core::option::Option<&'static str> {
                ::core::option::Option::Some(#text)
            }
        }
    });

    let instance = format_ident!("__{}_command", fn_ident);

    // Each parameter resolves from the container by its (dereferenced) type.
    let mut bindings = Vec::new();
    let mut call_args = Vec::new();
    for (index, input) in func.sig.inputs.iter().enumerate() {
        let (inner, by_ref) = parameter_type(input)?;
        let var = format_ident!("__arg{}", index);
        bindings.push(quote! {
            let #var = container.require::<#inner>()?;
        });
        call_args.push(if by_ref {
            quote! { &#var }
        } else {
            quote! { #var }
        });
    }

    Ok(quote! {
        #func

        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        pub struct #instance;

        #[adaliszk_commands::async_trait]
        impl adaliszk_commands::CommandRegistration for #instance {
            fn name(&self) -> &'static str { #name }

            fn about(&self) -> ::core::option::Option<&'static str> { #about }

            #tag

            async fn resolve(
                &self, container: &adaliszk_commands::Container
            ) -> adaliszk_commands::CommandResult {
                #(#bindings)*
                #fn_ident(#(#call_args),*).await
            }
        }

        adaliszk_commands::inventory::submit! {
            &#instance as &dyn adaliszk_commands::CommandRegistration
        }
    })
}

fn parse_options(attr: TokenStream) -> syn::Result<(Option<LitStr>, Option<LitStr>, Option<LitStr>)>
{
    let mut name: Option<LitStr> = None;
    let mut about: Option<LitStr> = None;
    let mut tag: Option<LitStr> = None;
    let parser = syn::meta::parser(|meta| {
        match meta.path.get_ident().map(ToString::to_string).as_deref() {
            Some("name") => name = Some(meta.value()?.parse()?),
            Some("about") => about = Some(meta.value()?.parse()?),
            Some("tag") => tag = Some(meta.value()?.parse()?),
            _ => return Err(meta.error("unknown command option")),
        }
        Ok(())
    });
    parser.parse2(attr)?;
    Ok((name, about, tag))
}

fn command_name(name_attr: Option<LitStr>, fn_ident: &Ident) -> String
{
    name_attr
        .map(|literal| literal.value())
        .unwrap_or_else(|| fn_ident.to_string())
}

fn command_about(about_attr: Option<LitStr>, attrs: &[Attribute]) -> Option<String>
{
    about_attr
        .map(|literal| literal.value())
        .or_else(|| first_doc_line(attrs))
}

fn first_doc_line(attrs: &[Attribute]) -> Option<String>
{
    for attr in attrs {
        if !attr.path().is_ident("doc") {
            continue;
        }
        if let syn::Meta::NameValue(meta) = &attr.meta {
            if let syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(text),
                ..
            }) = &meta.value
            {
                let line = text.value().trim().to_owned();
                if !line.is_empty() {
                    return Some(line);
                }
            }
        }
    }
    None
}

fn parameter_type(input: &FnArg) -> syn::Result<(Type, bool)>
{
    let pat = match input {
        FnArg::Typed(pat) => pat,
        FnArg::Receiver(_) => {
            return Err(syn::Error::new_spanned(
                input,
                "#[command] must be a free function",
            ));
        }
    };
    Ok(match &*pat.ty {
        Type::Reference(reference) => ((*reference.elem).clone(), true),
        other => (other.clone(), false),
    })
}

#[cfg(test)]
mod tests
{
    use super::*;
    use quote::quote;
    use syn::parse_quote;

    #[test]
    fn name_defaults_to_the_function_name()
    {
        let ident: Ident = parse_quote!(greet);
        assert_eq!(command_name(None, &ident), "greet");
    }

    #[test]
    fn name_attribute_overrides_the_function_name()
    {
        let ident: Ident = parse_quote!(greet);
        let name: LitStr = parse_quote!("other");
        assert_eq!(command_name(Some(name), &ident), "other");
    }

    #[test]
    fn about_prefers_the_attribute_over_the_doc_line()
    {
        let attrs: Vec<Attribute> = vec![parse_quote!(#[doc = " doc line"])];
        let about: LitStr = parse_quote!("explicit");

        assert_eq!(
            command_about(Some(about), &attrs),
            Some("explicit".to_owned())
        );
        assert_eq!(command_about(None, &attrs), Some("doc line".to_owned()));
        assert_eq!(command_about(None, &[]), None);
    }

    #[test]
    fn first_doc_line_skips_blank_lines_and_trims()
    {
        let attrs: Vec<Attribute> = vec![
            parse_quote!(#[doc = ""]),
            parse_quote!(#[doc = "  first line  "]),
            parse_quote!(#[doc = " second line"]),
        ];
        assert_eq!(first_doc_line(&attrs), Some("first line".to_owned()));
    }

    #[test]
    fn parameter_type_dereferences_and_flags_references()
    {
        let by_ref: FnArg = parse_quote!(config: &AppConfig);
        let (inner, is_ref) = parameter_type(&by_ref).unwrap();
        assert_eq!(quote!(#inner).to_string(), "AppConfig");
        assert!(is_ref);

        let by_value: FnArg = parse_quote!(config: AppConfig);
        let (inner, is_ref) = parameter_type(&by_value).unwrap();
        assert_eq!(quote!(#inner).to_string(), "AppConfig");
        assert!(!is_ref);
    }

    #[test]
    fn parameter_type_rejects_a_receiver()
    {
        let receiver: FnArg = parse_quote!(&self);
        assert!(parameter_type(&receiver).is_err());
    }

    #[test]
    fn parse_options_rejects_unknown_keys()
    {
        assert!(parse_options(quote!(name = "x", bogus = "y")).is_err());
        assert!(parse_options(quote!(name = "x", about = "y")).is_ok());
        assert!(parse_options(quote!()).is_ok());
    }

    #[test]
    fn parse_options_accepts_a_tag()
    {
        let (_, _, tag) = parse_options(quote!(tag = "dev")).unwrap();
        assert_eq!(tag.unwrap().value(), "dev");

        let (_, _, tag) = parse_options(quote!(name = "x")).unwrap();
        assert!(tag.is_none());
    }

    #[test]
    fn a_tag_overrides_the_trait_default_only_when_given()
    {
        let tagged = parse(
            quote!(tag = "dev"),
            quote! { async fn seed() -> CommandResult { Ok(()) } },
        )
        .unwrap()
        .to_string();
        assert!(tagged.contains("fn tag"));
        assert!(tagged.contains("\"dev\""));

        let untagged = parse(quote!(), quote! { async fn seed() -> CommandResult { Ok(()) } })
            .unwrap()
            .to_string();
        assert!(!untagged.contains("fn tag"), "the trait default stays in place");
    }

    #[test]
    fn parse_rejects_a_non_async_function()
    {
        let error =
            parse(quote!(), quote! { fn thing() -> CommandResult { Ok(()) } }).unwrap_err();
        assert!(error.to_string().contains("async"));
    }

    #[test]
    fn parse_generates_a_registration_for_an_async_function()
    {
        let generated = parse(
            quote!(),
            quote! {
                /// Greets warmly
                async fn greet(config: &AppConfig) -> CommandResult { Ok(()) }
            },
        )
        .unwrap()
        .to_string();

        assert!(generated.contains("\"greet\""));
        assert!(generated.contains("Greets warmly"));
        assert!(generated.contains("require :: < AppConfig >"));
        assert!(generated.contains("CommandRegistration"));
    }
}
