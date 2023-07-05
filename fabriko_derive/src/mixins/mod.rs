use darling::{ast::Data, FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use syn::{DeriveInput, Ident, Type};

#[derive(FromDeriveInput)]
pub(crate) struct MixinDeriveInput {
    ident: Ident,
    data: darling::ast::Data<(), MixinDeriveField>,
}

#[derive(FromField)]
#[darling(attributes(mixin))]
/// TODO: Document
struct MixinDeriveField {
    ident: Option<Ident>,
    ty: Type,
    #[darling(default)]
    /// When `into` is set, the generated setter will be generic over any
    /// type that implements Into<T> where T is the type of the field
    into: bool,
}

impl MixinDeriveInput {
    pub(crate) fn derive_mixin_trait_definition_and_impl(self) -> darling::Result<TokenStream> {
        let MixinDeriveInput { ident, data } = self;
        let fields = match data {
            Data::Enum(_) => unimplemented!(), // TODO: Proper error
            Data::Struct(fields) => fields,
        };

        // TODO: Refactor, could be just one iteration
        let setter_declarations: TokenStream = fields
            .iter()
            .map(MixinDeriveField::derive_setter_declaration)
            .collect();
        let setter_implementations: TokenStream = fields
            .iter()
            .map(MixinDeriveField::derive_setter_implementation)
            .collect();
        let setter_implementations_for_generic_t: TokenStream = fields
            .iter()
            .map(MixinDeriveField::derive_setter_implementation_for_generic_t)
            .collect();

        let trait_ident = Ident::new(&format!("{ident}Mixin"), ident.span());
        Ok(quote::quote!(
            pub trait #trait_ident {
                #setter_declarations
            }

            impl #trait_ident for #ident {
                #setter_implementations
            }

            impl<T: ::fabriko::WithMixin<#ident>> #trait_ident for T {
                #setter_implementations_for_generic_t
            }
        ))
    }
}

impl MixinDeriveField {
    fn derive_setter_declaration(&self) -> TokenStream {
        let MixinDeriveField { ident, ty, into } = self;
        if *into {
            quote::quote!(
                fn #ident<INTO_T: Into<#ty>>(self, #ident: INTO_T) -> Self;
            )
        } else {
            quote::quote!(
                fn #ident(self, #ident: #ty) -> Self;
            )
        }
    }

    fn derive_setter_implementation(&self) -> TokenStream {
        let MixinDeriveField { ident, ty, into } = self;
        if *into {
            quote::quote!(
                fn #ident<INTO_T: Into<#ty>>(mut self, #ident: INTO_T) -> Self {
                    self.#ident = #ident.into();
                    self
                }
            )
        } else {
            quote::quote!(
                fn #ident(mut self, #ident: #ty) -> Self {
                    self.#ident = #ident;
                    self
                }
            )
        }
    }

    fn derive_setter_implementation_for_generic_t(&self) -> TokenStream {
        let MixinDeriveField { ident, ty, into } = self;
        if *into {
            quote::quote!(
                fn #ident<INTO_T: Into<#ty>>(self, #ident: INTO_T) -> Self {
                    self.with_mixin(|__mixin| __mixin.#ident(#ident))
                }
            )
        } else {
            quote::quote!(
                fn #ident(self, #ident: #ty) -> Self {
                    self.with_mixin(|__mixin| __mixin.#ident(#ident))
                }
            )
        }
    }
}

pub(crate) fn do_derive_mixin(input: &DeriveInput) -> darling::Result<TokenStream> {
    let mixin_derive_input = MixinDeriveInput::from_derive_input(input)?;
    mixin_derive_input.derive_mixin_trait_definition_and_impl()
}
