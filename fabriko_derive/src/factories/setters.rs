use darling::ast::Fields;
use proc_macro2::TokenStream;
use syn::Ident;

use super::field::FactoryDeriveField;

impl FactoryDeriveField {
    pub(crate) fn derive_setter(&self) -> Option<TokenStream> {
        if self.should_derive_setter() {
            let FactoryDeriveField {
                ident,
                ty,
                belongs_to,
                into,
                mixin: _,
                dependant: _,
            } = self;
            match belongs_to {
                Some(belongs_to) => {
                    return Some(
                        super::associations::belongs_to::declare_fields_belonging_to(
                            belongs_to, ident,
                        ),
                    );
                }
                None => {
                    if *into {
                        return Some(quote::quote!(
                            pub fn #ident<T: Into<#ty>>(mut self, #ident: T) -> Self {
                                self.#ident = #ident.into();
                                self
                            }
                        ));
                    } else {
                        return Some(quote::quote!(
                            pub fn #ident(mut self, #ident: #ty) -> Self {
                                self.#ident = #ident;
                                self
                            }
                        ));
                    }
                }
            }
        }
        None
    }
}

pub(crate) fn derive_setters_implementations(
    factory_ident: &Ident,
    fields: &Fields<FactoryDeriveField>,
) -> darling::Result<TokenStream> {
    let setters: TokenStream = fields
        .iter()
        .flat_map(|field| field.derive_setter())
        .collect();
    Ok(quote::quote! {
        impl #factory_ident {
            #setters
        }
    })
}
