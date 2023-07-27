use std::hash::{Hash, Hasher};

use darling::{ast::Fields, FromMeta};
use proc_macro2::TokenStream;
use syn::{Ident, Path, Type};

use crate::factories::field::FactoryDeriveField;

#[derive(FromMeta)]
/// TODO: Document
pub(crate) struct BelongsToAssociation {
    factory: Path,
}

impl BelongsToAssociation {
    pub(crate) fn derive_belonging_to_link(
        &self,
        factory_ident: &Ident,
        field_ident: &Option<Ident>,
        field_ty: &Type,
    ) -> TokenStream {
        let ident = field_ident
            .as_ref()
            .expect("Only named structs are supported");

        let mut hasher = fnv::FnvHasher::default();
        ident.to_string().hash(&mut hasher);
        let ident_hash = hasher.finish();

        quote::quote! {
            impl ::fabriko::BelongingToLink<{ #ident_hash }> for #factory_ident {
                type ID = #field_ty;
                const SETTER: ::fabriko::FactorySetter<Self, Self::ID> = #factory_ident::#field_ident;
            }
        }
    }

    pub(crate) fn field_definition(&self, ident: &Ident, ty: &Type) -> TokenStream {
        let BelongsToAssociation { factory } = self;
        quote::quote!(#ident: ::fabriko::BelongsTo<#factory, #ty>,)
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
                mixin: _,
                into: _,
                dependant: _,
                default: _,
             }| {
                let ident = ident.as_ref().unwrap();
                belongs_to
                    .as_ref()
                    .map(|BelongsToAssociation { factory }| {
                        impl_block_conditions.push(
                        quote::quote! { ::fabriko::BelongsTo<#factory, #ty>: ::fabriko::ResolveDependency<CTX, Output = #ty>, },
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

    let BelongsToAssociation { factory } = belongs_to_association;
    let setter_belonging_to = Ident::new(&format!("belonging_to_{}", ident), ident.span());
    quote::quote!(
        pub fn #setter_belonging_to<F: FnOnce(#factory) -> #factory>(mut self, f: F) -> Self {
            self.#ident = ::fabriko::BelongsTo::Create(f(Default::default()));
            self
        }
        pub fn #ident(mut self, id: <Self as ::fabriko::BelongingToLink<{ #ident_hash }>>::ID) -> Self {
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
                 ty: field_ty,
                 mixin: _,
                 into: _,
                 dependant: _,
                 belongs_to,
                 default: _,
             }| {
                belongs_to.as_ref().map(|belongs_to| {
                    belongs_to.derive_belonging_to_link(factory_ident, field_ident, field_ty)
                })
            },
        )
        .collect()
}
