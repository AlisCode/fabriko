use darling::{ast::Fields, FromMeta};
use proc_macro2::TokenStream;
use syn::{Ident, Path};

use crate::factories::field::FactoryDeriveField;

#[derive(FromMeta)]
/// TODO: Document
pub(crate) struct BelongsToAssociation {
    ty: Path,
    field: Ident,
    /// TODO: Probably unnecessary
    id_ty: Path,
}

/// Writes a TokenStream that creates the resources this factory needs (its "containers"), and
/// makes sure the Where clause of the implementation is filled with the relevant requirements.
/// That is, the Factory requires its dependencies to also be Factory implementors.
///
/// TODO: Strong-type conditions to WhereClause ?
pub(crate) fn resolve_belongs_to_assocations_and_add_conditions(
    impl_block_conditions: &mut Vec<TokenStream>,
    fields: &Fields<FactoryDeriveField>,
) -> TokenStream {
    fields
        .iter()
        .filter_map(
            |FactoryDeriveField {
                 ident,
                 ty,
                 belongs_to,
                 ..
             }| {
                let ident = ident.as_ref().unwrap();
                belongs_to.as_ref().map(|BelongsToAssociation {
                    ty: belongs_to_ty,
                    field,
                    id_ty,
                }| {
                    impl_block_conditions.push(quote::quote! { #ty: ::fabriko::CreateBelongingTo<CTX, FactoryOutput = #belongs_to_ty, ID = #id_ty>, });
                    quote::quote! {
                        let #ident = #ident.create_belonging_to(ctx, |__res: #belongs_to_ty| __res.#field)?;
                    }
                })
            },
        )
        .collect()
}
