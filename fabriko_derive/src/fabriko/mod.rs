use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use syn::{DeriveInput, Ident};

mod attribute;

#[derive(FromDeriveInput)]
#[darling(attributes(fabriko))]
struct FabrikoDeriveInput {
    ident: Ident,
    #[darling(multiple, rename = "factory")]
    factories: Vec<attribute::FabrikoDeriveAttribute>,
    #[darling(rename = "wrapper")]
    fabriko_wrapper_name: Ident,
}

pub(crate) fn do_derive_fabriko(input: &DeriveInput) -> Result<TokenStream, darling::Error> {
    let FabrikoDeriveInput {
        ident: context_ident,
        factories,
        fabriko_wrapper_name,
    } = FabrikoDeriveInput::from_derive_input(input)?;

    let exposed_factories: TokenStream = factories
        .into_iter()
        .map(|attr| attr.derive_factory_fn_for_wrapper(&context_ident))
        .collect();

    Ok(quote::quote!(
        #[derive(Debug, Default)]
        pub struct #fabriko_wrapper_name(::fabriko::Fabriko<#context_ident>);

        impl #fabriko_wrapper_name {
            pub fn new(cx: #context_ident) -> Self {
                #fabriko_wrapper_name(::fabriko::Fabriko::new(cx))
            }

            pub fn bundle<B: ::fabriko::FactoryBundle<#context_ident>>(&mut self) -> B {
                self.0.bundle()
            }

            #exposed_factories
        }
    ))
}
