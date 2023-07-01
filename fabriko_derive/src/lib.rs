extern crate proc_macro;
use factories::do_derive_factory;
use mixins::do_derive_mixin;
use proc_macro::TokenStream;
use syn::DeriveInput;

mod factories;
mod mixins;

#[proc_macro_derive(Factory, attributes(factory))]
pub fn derive_factory(item: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(item as DeriveInput);
    let token_stream = do_derive_factory(&derive_input);
    unwrap_errors(token_stream)
}

#[proc_macro_derive(Mixin)]
pub fn derive_mixin(item: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(item as DeriveInput);
    let token_stream = do_derive_mixin(&derive_input);
    unwrap_errors(token_stream)
}

fn unwrap_errors(res: darling::Result<proc_macro2::TokenStream>) -> TokenStream {
    match res {
        Ok(tt) => tt,
        Err(err) => err.write_errors(),
    }
    .into()
}
