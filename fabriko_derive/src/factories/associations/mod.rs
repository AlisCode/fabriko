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

pub(crate) trait AssociationsCodegen {
    fn derive_setters(&self, structure: &AssociationAttributesStructure) -> TokenStream;
}

pub(crate) struct AssociationsSetter<'a> {
    field_ident: &'a Ident,
    setter_fn: TokenStream,
    argument_of_setter: TokenStream,
    create_set_type_of_association: TokenStream,
    default_type_of_association: TokenStream,
    set_type_of_association: TokenStream,
}

impl<'a> AssociationsSetter<'a> {
    fn derive_setter(&self, structure: &AssociationAttributesStructure) -> TokenStream {
        let AssociationsSetter {
            field_ident,
            setter_fn: setter_fn_name,
            argument_of_setter,
            create_set_type_of_association,
            default_type_of_association,
            set_type_of_association,
        } = self;

        let AssociationAttributesStructure { ident, fields } = structure;

        let all_fields_ident: TokenStream = fields
            .iter()
            .map(|field| {
                let ident = field.field_ident;
                quote::quote!(#ident,)
            })
            .collect();
        let generics: TokenStream = fields
            .iter()
            .filter_map(|field| {
                let AssociationAttributesStructureField {
                    field_ident: ident,
                    generic,
                    kind: _,
                } = field;
                if ident != field_ident {
                    return Some(quote::quote!(#generic,));
                }
                None
            })
            .collect();
        let base_generics: TokenStream = fields
            .iter()
            .filter_map(|field| {
                let AssociationAttributesStructureField {
                    field_ident: ident,
                    generic,
                    kind: _,
                } = field;
                if ident == field_ident {
                    return Some(quote::quote!(#default_type_of_association,));
                }
                Some(quote::quote!(#generic,))
            })
            .collect();
        let set_generics: TokenStream = fields
            .iter()
            .filter_map(|field| {
                let AssociationAttributesStructureField {
                    field_ident: ident,
                    generic,
                    kind: _,
                } = field;
                if ident == field_ident {
                    return Some(quote::quote!(#set_type_of_association,));
                }
                Some(quote::quote!(#generic,))
            })
            .collect();

        quote::quote!(
            impl<#generics> #ident<#base_generics> {
                // Because the field being associated might get mutated
                // or reassigned, it might be unused.
                #[allow(unused_variables)]
                pub fn #setter_fn_name(
                    self,
                    #argument_of_setter,
                ) -> #ident<#set_generics> {
                    let #ident {
                        #all_fields_ident
                    } = self;
                    let #field_ident = #create_set_type_of_association;
                    #ident {
                        #all_fields_ident
                    }
                }
            }
        )
    }
}

/*
impl<A> CountryFactoryAssociations<A, HasOneDefault<CityFactory>> {
    pub fn capital_city_id(
        self,
        city_id: CityId,
    ) -> CountryFactoryAssociations<A, HasOneCreated<CityId>> {
        let CountryFactoryAssociations {
            capital_city: _,
            cities,
        } = self;
        let capital_city = HasOneCreated::new(city_id);
        CountryFactoryAssociations {
            capital_city,
            cities,
        }
    }

    pub fn capital_city<F: FnOnce(CityFactory) -> CityFactory>(
        self,
        func: F,
    ) -> CountryFactoryAssociations<A, HasOneToCreate<CityFactory>> {
        let CountryFactoryAssociations {
            capital_city: _,
            cities,
        } = self;
        let capital_city = HasOneToCreate::new(func(Default::default()));
        CountryFactoryAssociations {
            capital_city,
            cities,
        }
    }
}
*/

/// One field of an [`AssocaitionAttributesStructure`].
/// This is in essence a declaration of an association, in a format that makes it easy
/// for the codegen.
struct AssociationAttributesStructureField<'a> {
    field_ident: &'a Ident,
    generic: Ident,
    kind: AssociationKind<'a>,
}

impl<'a> AssociationAttributesStructureField<'a> {
    /// TODO: test
    fn ident_when_destructuring(&self) -> TokenStream {
        let ident = self.field_ident;
        quote::quote!(#ident,)
    }

    /// TODO: test
    fn reassign_belonging_to(&self) -> TokenStream {
        let ident = self.field_ident;
        quote::quote!(let #ident = #ident.belonging_to(__resource);)
    }
}

/// The definition of the structure that will hold all associations
/// to related resources for the Factory being derived
pub(crate) struct AssociationAttributesStructure<'a> {
    ident: &'a Ident,
    fields: Vec<AssociationAttributesStructureField<'a>>,
}

// TODO: There has to be a better way to write that
// Intention :
// * index = 0 -> 'A' (as an ident)
// * index = 1 -> 'B' (as an ident)
// * .. and so on
fn index_as_generic_char(index: usize) -> Ident {
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
}

impl<'a> AssociationsDeriveAttributes<'a> {
    fn as_structure(&self) -> AssociationAttributesStructure<'a> {
        let AssociationsDeriveAttributes {
            has_many,
            has_one,
            associations_ty,
        } = self;

        let fields = has_many
            .iter()
            .map(AssociationKind::HasMany)
            .chain(has_one.iter().map(AssociationKind::HasOne))
            .enumerate()
            .map(|(index, kind)| {
                let name = match kind {
                    AssociationKind::HasOne(one) => &one.name,
                    AssociationKind::HasMany(many) => &many.name,
                };
                AssociationAttributesStructureField {
                    field_ident: &name,
                    generic: index_as_generic_char(index),
                    kind,
                }
            })
            .collect();

        AssociationAttributesStructure {
            ident: associations_ty,
            fields,
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
        let belonging_to_impl =
            association_attributes_structure.derive_belonging_to_implementation_for_associations();
        let setters = association_attributes_structure.derive_setters();
        // TODO: Factory (or something else?) implementation for the Associations structure

        quote::quote!(
            #structure_decl
            #with_related_resources_impl
            #belonging_to_impl
            #setters
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
                            setter: _,
                        }) => quote::quote!(::fabriko::HasMany<#for_factory>),
                        AssociationKind::HasOne(HasOneAssociation {
                            for_factory,
                            name: _,
                            link: _,
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

    /// TODO: tests
    fn derive_belonging_to_implementation_for_associations(&self) -> TokenStream {
        let AssociationAttributesStructure { ident, fields } = self;
        let generics: TokenStream = fields
            .iter()
            .map(|field| {
                let AssociationAttributesStructureField {
                    field_ident: _,
                    generic,
                    kind: _,
                } = field;
                quote::quote!(#generic,)
            })
            .collect();
        let generics_belonging_to: TokenStream = fields
            .iter()
            .map(|field| {
                let AssociationAttributesStructureField {
                    field_ident: _,
                    generic,
                    kind: _,
                } = field;
                quote::quote!(#generic: ::fabriko::BelongingTo<RESOURCE>,)
            })
            .collect();
        let fields_when_destructuring: TokenStream = fields
            .iter()
            .map(AssociationAttributesStructureField::ident_when_destructuring)
            .collect();
        let fields_reassignments: TokenStream = fields
            .iter()
            .map(AssociationAttributesStructureField::reassign_belonging_to)
            .collect();
        quote::quote!(
            impl<RESOURCE, #generics_belonging_to> BelongingTo<RESOURCE>
                for #ident<#generics>
            {
                fn belonging_to(self, __resource: &RESOURCE) -> Self {
                    let #ident {
                        #fields_when_destructuring
                    } = self;
                    #fields_reassignments
                    #ident {
                        #fields_when_destructuring
                    }
                }
            }
        )
    }

    /// TODO: tests
    fn derive_setters(&self) -> TokenStream {
        self.fields
            .iter()
            .map(|field| match field.kind {
                AssociationKind::HasMany(many) => many.derive_setters(self),
                AssociationKind::HasOne(one) => one.derive_setters(self),
            })
            .collect()
    }
}
