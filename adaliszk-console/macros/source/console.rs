// SPDX-FileCopyrightText: 2026, Ádám Liszkai "Kicsivazz" and Contributors
// SPDX-License-Identifier: MPL-2.0

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{GenericArgument, ItemImpl, LitStr, PathArguments, Type, parse_macro_input};

pub fn expand(attr: TokenStream, item: TokenStream) -> TokenStream
{
    let mut name: Option<LitStr> = None;
    let mut about: Option<LitStr> = None;

    let options = syn::meta::parser(|meta| {
        match meta.path.get_ident().map(ToString::to_string).as_deref() {
            Some("name") => name = Some(meta.value()?.parse()?),
            Some("about") => about = Some(meta.value()?.parse()?),
            _ => return Err(meta.error("unknown console option")),
        }
        Ok(())
    });
    parse_macro_input!(attr with options);

    let block = parse_macro_input!(item as ItemImpl);

    let name = name.expect("#[console] requires `name = \"...\"`");
    let about = match about {
        Some(literal) => quote! { #literal },
        None => quote! { env!("CARGO_PKG_DESCRIPTION") },
    };

    let self_type = block.self_ty.clone();
    let self_ident = match &*self_type {
        Type::Path(path) => path.path.segments.last().unwrap().ident.clone(),
        _ => panic!("#[console] must be placed on `impl ConsoleHandler<C> for Struct`"),
    };
    let instance = format_ident!("{}Instance", self_ident);

    let (trait_path, _) = block
        .trait_
        .as_ref()
        .expect("#[console] requires `impl ConsoleHandler<C> for ...`");

    let config_type = match &trait_path.segments.last().unwrap().arguments {
        PathArguments::AngleBracketed(arguments) => match arguments.args.first() {
            Some(GenericArgument::Type(ty)) => ty.clone(),
            _ => panic!("expected a config type: `impl Command<Config>`"),
        },
        _ => panic!("expected a config type: `impl Command<Config>`"),
    };

    let expanded = quote! {
        #[adaliszk_console::Async]
        #block

        #[doc(hidden)]
        pub struct #instance;

        #[adaliszk_console::Async]
        impl adaliszk_console::ConsoleRegistration for #instance {
            fn signature(&self) -> &'static str { #name }

            fn augment_command(&self, cmd: clap::Command) -> clap::Command {
                <#self_type as clap::Args>::augment_args(cmd)
                    .about(<#self_type as adaliszk_console::ConsoleHelp>::DESCRIPTION)
            }

            async fn resolve(
                &self, args: &clap::ArgMatches
            ) -> adaliszk_console::ConsoleResult {
                let command = <#self_type as clap::FromArgMatches>::from_arg_matches(args)
                    .unwrap_or_else(|error| error.exit());
                let config = args
                    .get_one::<#config_type>("config")
                    .expect("global --config argument");
                <#self_type as adaliszk_console::ConsoleHandler<#config_type>>::handle(&command, config).await
            }
        }

        impl adaliszk_console::ConsoleHelp for #self_type {
            const DESCRIPTION: &'static str = #about;
        }

        adaliszk_console::inventory::submit! {
            &#instance as &dyn adaliszk_console::ConsoleRegistration
        }
    };

    expanded.into()
}
