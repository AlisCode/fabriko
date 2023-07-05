use darling::{ast::Fields, FromField};
use proc_macro2::TokenStream;
use syn::{Expr, Ident, Type};

use super::associations::belongs_to::BelongsToAssociation;

#[derive(FromField)]
#[darling(attributes(factory))]
/// TODO: Document
/// TODO: support into
/// TODO: support skip
pub(crate) struct FactoryDeriveField {
    pub(crate) ident: Option<Ident>,
    pub(crate) ty: Type,
    #[darling(default)]
    pub(crate) mixin: bool,
    pub(crate) dependant: Option<Expr>,
    pub(crate) belongs_to: Option<BelongsToAssociation>,
}

impl FactoryDeriveField {
    /// TODO: Remove ?
    pub(crate) fn is_factory_attribute(&self) -> bool {
        true
    }

    pub(crate) fn should_derive_setter(&self) -> bool {
        !self.mixin
    }

    /// If this is an attribute field, returns a TokenStream to allow to destructure the field
    /// TODO: Rename
    pub(crate) fn as_factory_field(&self) -> Option<TokenStream> {
        let field_ident = &self.ident;
        if self.dependant.is_some() {
            None
        } else {
            Some(quote::quote!(#field_ident,))
        }
    }
}

//   let #factory_ident {
//    #destructure_attributes_fields
//    ..
//} = self;
pub(crate) fn destructure_factory_fields(fields: &Fields<FactoryDeriveField>) -> TokenStream {
    fields
        .iter()
        .filter_map(FactoryDeriveField::as_factory_field)
        .collect()
}

// let __resource = #attributes_ty_path {
//                     #attributes_fields
//                 }
//                 .build_resource(ctx)?;
//
pub(crate) fn destructure_attributes_fields(fields: &Fields<FactoryDeriveField>) -> TokenStream {
    fields
        .iter()
        .filter_map(|field| {
            if field.is_factory_attribute() {
                let field_ident = &field.ident;
                Some(quote::quote!(#field_ident,))
            } else {
                None
            }
        })
        .collect()
}

pub(crate) fn reassign_dependant_attributes(fields: &Fields<FactoryDeriveField>) -> TokenStream {
    fields
        .iter()
        .filter_map(
            |FactoryDeriveField {
                 ident,
                 ty,
                 dependant,
                 ..
             }| {
                dependant.as_ref().map(|expr| {
                    quote::quote!(
                        let #ident: #ty = #expr;
                    )
                })
            },
        )
        .collect()
}
