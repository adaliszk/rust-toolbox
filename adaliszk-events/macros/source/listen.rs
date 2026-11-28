// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::parse::Parser;
use syn::{FnArg, ItemFn, LitStr, Path, Type};

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
    let (explicit_event, tag_attr) = parse_options(attr)?;
    let func: ItemFn = syn::parse2(item)?;

    if func.sig.asyncness.is_none() {
        return Err(syn::Error::new_spanned(&func.sig, "#[listen] requires an async fn"));
    }

    // Each parameter resolves from the container by its (dereferenced) type, in any position,
    // the event included, since the dispatcher binds it before the listener runs.
    let mut bindings = Vec::new();
    let mut call_args = Vec::new();
    for (index, input) in func.sig.inputs.iter().enumerate() {
        let (inner, by_ref) = parameter_type(input)?;
        let var = format_ident!("__arg{}", index);
        bindings.push(quote! {
            let #var = container.require::<#inner>()?;
        });
        call_args.push(if by_ref { quote! { &#var } } else { quote! { #var } });
    }
    // The attribute names the event (the registry key) like `console_command`'s
    // `args = ...`; the parameters stay uniform and all resolve from the container.
    let Some(event_type) = explicit_event else {
        return Err(syn::Error::new_spanned(
            &func.sig,
            "#[listen] requires the event binding, e.g. #[listen(event = TheEvent)]",
        ));
    };

    let fn_ident = func.sig.ident.clone();
    let instance = format_ident!("__{}_listener", fn_ident);

    // The trait defaults to no tag, so the override is only generated when one is given:
    let tag = tag_attr.map(|text| {
        quote! {
            fn tag(&self) -> ::core::option::Option<&'static str> {
                ::core::option::Option::Some(#text)
            }
        }
    });

    Ok(quote! {
        #func

        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        pub struct #instance;

        #[adaliszk_events::async_trait]
        impl adaliszk_events::EventRegistration for #instance {
            fn event(&self) -> ::core::any::TypeId {
                ::core::any::TypeId::of::<#event_type>()
            }

            #tag

            fn subscribe(&'static self, bus: &mut adaliszk_events::EventDispatcher) {
                bus.attach::<#event_type>(self);
            }

            async fn resolve(
                &self, container: &adaliszk_events::Container
            ) -> adaliszk_events::EventResult {
                #(#bindings)*
                #fn_ident(#(#call_args),*).await
            }
        }

        adaliszk_events::inventory::submit! {
            &#instance as &dyn adaliszk_events::EventRegistration
        }
    })
}

fn parse_options(attr: TokenStream) -> syn::Result<(Option<Path>, Option<LitStr>)>
{
    let mut event: Option<Path> = None;
    let mut tag: Option<LitStr> = None;
    let parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("event") {
            event = Some(meta.value()?.parse()?);
            Ok(())
        } else if meta.path.is_ident("tag") {
            tag = Some(meta.value()?.parse()?);
            Ok(())
        } else {
            Err(meta.error("unknown listen option"))
        }
    });
    parser.parse2(attr)?;
    Ok((event, tag))
}

fn parameter_type(input: &FnArg) -> syn::Result<(Type, bool)>
{
    let pat = match input {
        FnArg::Typed(pat) => pat,
        FnArg::Receiver(_) => {
            return Err(syn::Error::new_spanned(input, "#[listen] must be a free function"));
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

    #[test]
    fn parse_binds_the_event_from_the_attribute()
    {
        let generated = parse(
            quote!(event = MessageArrived),
            quote! {
                async fn on_message(config: &AppConfig, event: &MessageArrived) -> EventResult {
                    Ok(())
                }
            },
        )
        .unwrap()
        .to_string();

        assert!(generated.contains("TypeId :: of :: < MessageArrived >"));
        assert!(generated.contains("attach :: < MessageArrived >"));
        assert!(generated.contains("require :: < MessageArrived >"));
        assert!(generated.contains("require :: < AppConfig >"));
        assert!(generated.contains("EventRegistration"));
    }

    #[test]
    fn parse_requires_the_event_binding()
    {
        let error = parse(
            quote!(),
            quote! { async fn on_message(event: &MessageArrived) -> EventResult { Ok(()) } },
        )
        .unwrap_err();
        assert!(error.to_string().contains("event = "));
    }

    #[test]
    fn parse_rejects_unknown_options()
    {
        assert!(parse(
            quote!(bogus = MessageArrived),
            quote! { async fn on_message(event: &MessageArrived) -> EventResult { Ok(()) } },
        )
        .is_err());
    }

    #[test]
    fn a_tag_overrides_the_trait_default_only_when_given()
    {
        let tagged = parse(
            quote!(event = MessageArrived, tag = "telemetry"),
            quote! { async fn on_message(event: &MessageArrived) -> EventResult { Ok(()) } },
        )
        .unwrap()
        .to_string();
        assert!(tagged.contains("fn tag"));
        assert!(tagged.contains("\"telemetry\""));

        let untagged = parse(
            quote!(event = MessageArrived),
            quote! { async fn on_message(event: &MessageArrived) -> EventResult { Ok(()) } },
        )
        .unwrap()
        .to_string();
        assert!(!untagged.contains("fn tag"), "the trait default stays in place");
    }

    #[test]
    fn parse_rejects_a_non_async_function()
    {
        let error = parse(
            quote!(event = MessageArrived),
            quote! { fn on_message(event: &MessageArrived) -> EventResult { Ok(()) } },
        )
        .unwrap_err();
        assert!(error.to_string().contains("async"));
    }

    #[test]
    fn parse_rejects_a_receiver()
    {
        assert!(parse(
            quote!(event = MessageArrived),
            quote! { async fn on_message(&self) -> EventResult { Ok(()) } },
        )
        .is_err());
    }
}
