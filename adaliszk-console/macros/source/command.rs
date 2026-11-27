// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::parse::Parser;
use syn::{ItemFn, LitStr, Path};

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
    let options = parse_options(attr)?;
    let func: ItemFn = syn::parse2(item)?;

    let fn_ident = func.sig.ident.clone();

    // Pass the name/about options through to the clap-free `#[command]` registration:
    let mut passthrough = Vec::new();
    if let Some(name) = &options.name {
        passthrough.push(quote! { name = #name });
    }
    if let Some(about) = &options.about {
        passthrough.push(quote! { about = #about });
    }

    // Link the args struct, when given, as this command's console registration. The chained
    // `#[command]` names its instance `__{fn}_command`, so the link is a direct reference:
    let registration = options.args.map(|args_type| {
        let instance = format_ident!("__{}_console", fn_ident);
        let command_instance = format_ident!("__{}_command", fn_ident);
        quote! {
            #[doc(hidden)]
            #[allow(non_camel_case_types)]
            pub struct #instance;

            impl adaliszk_console::ConsoleRegistration for #instance {
                fn command(&self) -> &'static dyn adaliszk_console::CommandRegistration {
                    &#command_instance
                }

                fn augment(&self, cmd: ::clap::Command) -> ::clap::Command {
                    <#args_type as ::clap::Args>::augment_args(cmd)
                }

                fn bind(
                    &self,
                    matches: &::clap::ArgMatches,
                    container: &mut adaliszk_console::Container,
                ) -> ::core::result::Result<(), ::clap::Error> {
                    let args = <#args_type as ::clap::FromArgMatches>::from_arg_matches(matches)?;
                    container.bind(args);
                    Ok(())
                }
            }

            adaliszk_console::inventory::submit! {
                &#instance as &dyn adaliszk_console::ConsoleRegistration
            }
        }
    });

    Ok(quote! {
        #[adaliszk_console::command(#(#passthrough),*)]
        #func

        #registration
    })
}

struct Options
{
    name: Option<LitStr>,
    about: Option<LitStr>,
    args: Option<Path>,
}

fn parse_options(attr: TokenStream) -> syn::Result<Options>
{
    let mut name: Option<LitStr> = None;
    let mut about: Option<LitStr> = None;
    let mut args: Option<Path> = None;
    let parser = syn::meta::parser(|meta| {
        match meta.path.get_ident().map(ToString::to_string).as_deref() {
            Some("name") => name = Some(meta.value()?.parse()?),
            Some("about") => about = Some(meta.value()?.parse()?),
            Some("args") => args = Some(meta.value()?.parse()?),
            _ => return Err(meta.error("unknown console_command option")),
        }
        Ok(())
    });
    parser.parse2(attr)?;
    Ok(Options { name, about, args })
}

#[cfg(test)]
mod tests
{
    use super::*;
    use quote::quote;

    #[test]
    fn options_accept_name_about_and_args()
    {
        let options = parse_options(quote!(name = "other", about = "text", args = Example)).unwrap();
        assert_eq!(options.name.unwrap().value(), "other");
        assert_eq!(options.about.unwrap().value(), "text");
        assert!(options.args.is_some());

        assert!(parse_options(quote!()).unwrap().args.is_none());
        assert!(parse_options(quote!(bogus = "x")).is_err());
    }

    #[test]
    fn parse_chains_the_command_attribute()
    {
        let generated = parse(
            quote!(),
            quote! { async fn example() -> CommandResult { Ok(()) } },
        )
        .unwrap()
        .to_string();

        assert!(generated.contains("command"));
        assert!(!generated.contains("ConsoleRegistration"), "no args, no registration");
    }

    #[test]
    fn parse_links_the_args_type_to_the_command_instance()
    {
        let generated = parse(
            quote!(args = ExampleArgs),
            quote! { async fn example() -> CommandResult { Ok(()) } },
        )
        .unwrap()
        .to_string();

        assert!(generated.contains("ConsoleRegistration"));
        assert!(generated.contains("__example_command"), "direct link to the command instance");
        assert!(generated.contains("ExampleArgs"));
    }

    #[test]
    fn parse_passes_the_name_override_through()
    {
        let generated = parse(
            quote!(name = "other", args = ExampleArgs),
            quote! { async fn example() -> CommandResult { Ok(()) } },
        )
        .unwrap()
        .to_string();

        assert!(generated.contains("name = \"other\""));
    }
}
