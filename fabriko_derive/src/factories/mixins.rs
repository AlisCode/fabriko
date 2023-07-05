use darling::ast::Fields;
use proc_macro2::TokenStream;
use syn::Ident;

use super::field::FactoryDeriveField;

pub(crate) fn derive_mixin_implementations(
    factory_ident: &Ident,
    fields: &Fields<FactoryDeriveField>,
) -> darling::Result<TokenStream> {
    let mixin_impls: TokenStream = fields
        .iter()
        .flat_map(|field| field.derive_mixin_field(factory_ident))
        .collect();

    Ok(mixin_impls)
}

impl FactoryDeriveField {
    fn derive_mixin_field(&self, factory_ident: &Ident) -> Option<TokenStream> {
        let FactoryDeriveField {
            ident, ty, mixin, ..
        } = self;
        if *mixin {
            return Some(quote::quote! {
                 impl ::fabriko::WithMixin<#ty> for #factory_ident {
                     fn with_mixin<F: FnOnce(#ty) -> #ty>(mut self, f: F) -> Self {
                         self.#ident = f(self.#ident);
                         self
                     }
                 }
            });
        }
        None
    }
}
