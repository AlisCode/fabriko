extern crate proc_macro;
use bundles::do_derive_bundle;
use fabriko::do_derive_fabriko;
use factories::do_derive_factory;
use identifier::do_derive_with_identifier;
use mixins::do_derive_mixin;
use proc_macro::TokenStream;
use syn::DeriveInput;

mod bundles;
mod create;
mod fabriko;
mod factories;
mod identifier;
mod mixins;

#[proc_macro_derive(Factory, attributes(factory))]
pub fn derive_factory(item: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(item as DeriveInput);
    let token_stream = do_derive_factory(&derive_input);
    unwrap_errors(token_stream)
}

#[proc_macro_derive(Mixin, attributes(mixin))]
pub fn derive_mixin(item: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(item as DeriveInput);
    let token_stream = do_derive_mixin(&derive_input);
    unwrap_errors(token_stream)
}

#[proc_macro_derive(FactoryBundle, attributes(bundle))]
pub fn derive_bundle(item: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(item as DeriveInput);
    let token_stream = do_derive_bundle(&derive_input);
    unwrap_errors(token_stream)
}

#[proc_macro_derive(Fabriko, attributes(fabriko))]
pub fn derive_fabriko(item: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(item as DeriveInput);
    let token_stream = do_derive_fabriko(&derive_input);
    unwrap_errors(token_stream)
}

#[proc_macro_derive(WithIdentifier, attributes(identifier))]
pub fn derive_with_identifier(item: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(item as DeriveInput);
    let token_stream = do_derive_with_identifier(&derive_input);
    unwrap_errors(token_stream)
}

#[proc_macro]
/// # Example usage :
///
/// let user = create!(context, todo
///     .title "My todo"
///     .belonging_to_todo_group {
///         .title "My todo group"
///     }
/// )
///
pub fn create(item: TokenStream) -> TokenStream {
    create::do_create(item.into()).into()
}

fn unwrap_errors(res: darling::Result<proc_macro2::TokenStream>) -> TokenStream {
    match res {
        Ok(tt) => tt,
        Err(err) => err.write_errors(),
    }
    .into()
}
