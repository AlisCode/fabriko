use proc_macro2::{Span, TokenStream};
use syn::Ident;

use self::{has_many::HasManyAssociation, has_one::HasOneAssociation};

pub(crate) mod belongs_to;
pub(crate) mod has_many;
pub(crate) mod has_one;

/// Safe entrypoint for the step of the derive that will
/// codegen what is necessary to make it possible for a Factory to declare
/// related resources that directly depends on what the Factory is building.
///
/// The following attributes are relevant to this step :
/// * #[factory(has_many( .. ))] -> declares that the derived factory has many such resources
/// * #[factory(has_one( .. ))] -> declares that the derived factory has exactly one such resources
/// * #[factory(associations = " .. ")] -> the Ident of struct that will contain the
/// related resources
pub(crate) struct AssociationsDeriveAttributes<'a> {
    has_many: &'a [HasManyAssociation],
    has_one: &'a [HasOneAssociation],
    associations_ty: &'a Ident,
}

pub(crate) fn derive_associations(
    has_many: &[HasManyAssociation],
    has_one: &[HasOneAssociation],
    associations_ty: &Ident,
    factory_ident: &Ident,
) -> TokenStream {
    AssociationsDeriveAttributes {
        has_many,
        has_one,
        associations_ty,
    }
    .derive_related_resources(factory_ident)
}

/// The supported associations annotations
enum AssociationKind<'a> {
    HasMany(&'a HasManyAssociation),
    HasOne(&'a HasOneAssociation),
}

/// One field of an [`AssocaitionAttributesStructure`].
/// This is in essence a declaration of an association, in a format that makes it easy
/// for the codegen.
struct AssociationAttributesStructureField<'a> {
    field_ident: &'a Ident,
    generic: Ident,
    kind: AssociationKind<'a>,
}

/// The definition of the structure that will hold all associations
/// to related resources for the Factory being derived
struct AssociationAttributesStructure<'a> {
    ident: &'a Ident,
    fields: Vec<AssociationAttributesStructureField<'a>>,
}

impl<'a> AssociationsDeriveAttributes<'a> {
    fn as_structure(&self) -> AssociationAttributesStructure<'a> {
        let AssociationsDeriveAttributes {
            has_many,
            has_one,
            associations_ty,
        } = self;

        // TODO: There has to be a better way to write that
        // Intention :
        // * index = 0 -> 'A' (as an ident)
        // * index = 1 -> 'B' (as an ident)
        // * .. and so on
        let index_as_generic_char = |index: usize| {
            let generic_char_index = 'A' as u8 as usize + index;
            Ident::new(
                &char::from_u32(
                    generic_char_index
                        .try_into()
                        .expect("Failed to transform usize into u32"),
                )
                .into_iter()
                .collect::<String>(),
                Span::call_site(),
            )
        };

        let has_manies = has_many.iter().enumerate().map(|(index, has_many)| {
            AssociationAttributesStructureField {
                field_ident: &has_many.name,
                generic: index_as_generic_char(index),
                kind: AssociationKind::HasMany(has_many),
            }
        });
        // TODO: get rid of this by chaining has_many and has_one, and mapping them to an AssociationKind
        // .. since AssociationKind holds all the data we need anyway
        let has_manies_len = has_manies.len();
        let has_ones = has_one.iter().enumerate().map(|(index, has_one)| {
            AssociationAttributesStructureField {
                field_ident: &has_one.name,
                generic: index_as_generic_char(index + has_manies_len),
                kind: AssociationKind::HasOne(has_one),
            }
        });

        AssociationAttributesStructure {
            ident: associations_ty,
            fields: has_manies.chain(has_ones).collect(),
        }
    }
}

impl<'a> AssociationsDeriveAttributes<'a> {
    /// Derives all the code that will make it possible for a Factory to declare related resources
    /// that directly depends on this resource
    pub(crate) fn derive_related_resources(&self, factory_ident: &Ident) -> TokenStream {
        let association_attributes_structure = self.as_structure();

        let structure_decl = association_attributes_structure.derive_structure_declaration();
        let with_related_resources_impl =
            association_attributes_structure.derive_with_related_resources_impl(factory_ident);
        // TODO: Setters for the Associations structure
        // TODO: Factory (or something else?) implementation for the Associations structure

        quote::quote!(
            #structure_decl
            #with_related_resources_impl
        )
    }
}

impl<'a> AssociationAttributesStructure<'a> {
    /// TODO: tests
    fn derive_structure_declaration(&self) -> TokenStream {
        let AssociationAttributesStructure { ident, fields } = self;
        let all_generics: TokenStream = fields
            .iter()
            .map(
                |AssociationAttributesStructureField {
                     field_ident: _,
                     generic,
                     kind: _,
                 }| quote::quote!(#generic, ),
            )
            .collect();
        let all_fields: TokenStream = fields
            .iter()
            .map(
                |AssociationAttributesStructureField {
                     field_ident,
                     generic,
                     kind: _,
                 }| quote::quote!(pub #field_ident: #generic, ),
            )
            .collect();
        quote::quote!(
            #[derive(Default)]
            pub struct #ident<#all_generics> {
                #all_fields
            }
        )
    }

    /// TODO: tests
    fn derive_with_related_resources_impl(&self, factory_ident: &Ident) -> TokenStream {
        let AssociationAttributesStructure { ident, fields } = self;
        let generics_of_associations_type: TokenStream = fields
            .iter()
            .map(
                |AssociationAttributesStructureField {
                     field_ident: _,
                     generic: _,
                     kind,
                 }| {
                    let generic = match kind {
                        AssociationKind::HasMany(HasManyAssociation {
                            for_factory,
                            name: _,
                            // setter: _,
                        }) => quote::quote!(::fabriko::HasMany<#for_factory>),
                        AssociationKind::HasOne(HasOneAssociation {
                            for_factory,
                            name: _,
                            // setter: _,
                        }) => quote::quote!(::fabriko::HasOneDefault<#for_factory>),
                    };
                    quote::quote!(#generic,)
                },
            )
            .collect();
        quote::quote!(
            impl ::fabriko::WithRelatedResources for #factory_ident {
                type DefaultAssociations = #ident<#generics_of_associations_type>;
            }
        )
    }
}
