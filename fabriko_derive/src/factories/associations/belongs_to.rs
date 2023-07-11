use std::hash::{Hash, Hasher};

use darling::{ast::Fields, FromMeta};
use proc_macro2::TokenStream;
use syn::{Ident, Path};

use crate::factories::field::FactoryDeriveField;

#[derive(FromMeta)]
/// TODO: Document
pub(crate) struct BelongsToAssociation {
    /// TODO: Remove when this will become the type of the field
    id_ty: Path,
    factory: Path,
}

impl BelongsToAssociation {
    pub(crate) fn derive_belonging_to_link(
        &self,
        factory_ident: &Ident,
        field_ident: &Option<Ident>,
    ) -> TokenStream {
        let BelongsToAssociation { id_ty, factory: _ } = self;
        let ident = field_ident
            .as_ref()
            .expect("Only named structs are supported");

        let mut hasher = fnv::FnvHasher::default();
        ident.to_string().hash(&mut hasher);
        let ident_hash = hasher.finish();

        quote::quote! {
            impl ::fabriko::BelongingToLink<{ #ident_hash }> for #factory_ident {
                type ID = #id_ty;
                const SETTER: ::fabriko::FactorySetter<Self, Self::ID> = #factory_ident::#field_ident;
            }
        }
    }
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
                belongs_to
                    .as_ref()
                    .map(|BelongsToAssociation { id_ty, factory: _ }| {
                        impl_block_conditions.push(
                        quote::quote! { #ty: ::fabriko::ResolveDependency<CTX, Output = #id_ty>, },
                        // quote::quote! { ::fabriko::BelongsTo<#factory, #ty>: ::fabriko::ResolveDependency<CTX, Output = #ty>, },
                    );
                        quote::quote! {
                            let #ident = #ident.resolve_dependency(ctx)?;
                        }
                    })
            },
        )
        .collect()
}

pub(crate) fn declare_fields_belonging_to(
    belongs_to_association: &BelongsToAssociation,
    field_ident: &Option<Ident>,
) -> TokenStream {
    let ident = field_ident
        .as_ref()
        .expect("Only named structs are supported");

    let mut hasher = fnv::FnvHasher::default();
    ident.to_string().hash(&mut hasher);
    let ident_hash = hasher.finish();

    let BelongsToAssociation { id_ty: _, factory } = belongs_to_association;
    let setter_belonging_to = Ident::new(&format!("belonging_to_{}", ident), ident.span());
    quote::quote!(
        pub fn #setter_belonging_to<F: FnOnce(#factory) -> #factory>(mut self, f: F) -> Self {
            self.#ident = ::fabriko::BelongsTo::Create(f(Default::default()));
            self
        }
        pub fn #ident(mut self, id: <Self as BelongingToLink<{ #ident_hash }>>::ID) -> Self {
            self.#ident = ::fabriko::BelongsTo::Created(id);
            self
        }
    )
}

pub(crate) fn derive_belonging_to_link_implementations(
    factory_ident: &Ident,
    fields: &Fields<FactoryDeriveField>,
) -> TokenStream {
    fields
        .iter()
        .filter_map(
            |FactoryDeriveField {
                 ident: field_ident,
                 ty: _,
                 mixin: _,
                 into: _,
                 dependant: _,
                 belongs_to,
             }| {
                belongs_to.as_ref().map(|belongs_to| {
                    belongs_to.derive_belonging_to_link(factory_ident, field_ident)
                })
            },
        )
        .collect()
}
