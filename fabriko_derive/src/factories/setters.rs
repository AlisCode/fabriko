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
                ..
            } = self;
            match belongs_to {
                Some(_) => {
                    let ident = ident.as_ref().expect("Only named structs are supported");
                    let setter_belonging_to =
                        Ident::new(&format!("belonging_to_{}", ident), ident.span());
                    return Some(quote::quote!(
                        pub fn #setter_belonging_to<F: FnOnce(<#ty as ::fabriko::BelongsToInfo>::Factory) -> <#ty as ::fabriko::BelongsToInfo>::Factory>(mut self, f: F) -> Self {
                            self.#ident = ::fabriko::BelongsTo::Create(f(Default::default()));
                            self
                        }
                        pub fn #ident(mut self, id: <#ty as ::fabriko::BelongsToInfo>::ID) -> Self {
                            self.#ident = ::fabriko::BelongsTo::Created(id);
                            self
                        }
                    ));
                }
                None => {
                    return Some(quote::quote!(
                        pub fn #ident(mut self, #ident: #ty) -> Self {
                            self.#ident = #ident;
                            self
                        }
                    ));
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
