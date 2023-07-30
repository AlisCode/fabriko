use darling::{
    ast::{Data, Fields},
    FromDeriveInput,
};
use proc_macro2::TokenStream;
use syn::{DeriveInput, Ident};

use self::associations::{has_many::HasManyAssociation, has_one::HasOneAssociation};
use self::field::FactoryDeriveField;

mod associations;
mod field;
mod mixins;
mod setters;

#[derive(FromDeriveInput)]
#[darling(supports(struct_named))]
#[darling(attributes(factory))]
/// TODO: Document
/// TODO: Split into own module
struct FactoryDeriveInput {
    ident: Ident,
    data: Data<darling::util::Ignored, field::FactoryDeriveField>,
    #[darling(rename = "factory")]
    factory_ident: Ident,
    #[darling(rename = "associations")]
    associations_ident: Option<Ident>,
    #[darling(multiple)]
    has_many: Vec<HasManyAssociation>,
    #[darling(multiple)]
    has_one: Vec<HasOneAssociation>,
}

impl FactoryDeriveInput {
    pub fn derive(&self) -> darling::Result<TokenStream> {
        let FactoryDeriveInput {
            ident: attributes_ident,
            data,
            factory_ident,
            associations_ident,
            has_many,
            has_one,
        } = self;
        let fields = match data {
            Data::Enum(_) => panic!("The only supported mode is struct with named fields"),
            Data::Struct(fields) => fields,
        };

        let mixin_implementations =
            self::mixins::derive_mixin_implementations(factory_ident, fields)?;
        let setter_implementations =
            self::setters::derive_setters_implementations(factory_ident, fields)?;
        let factory_definition = derive_factory_definition(factory_ident, fields);
        let factory_implementation =
            derive_factory_implementation(attributes_ident, factory_ident, fields)?;
        let associated_resources_definition_and_implementation =
            associations_ident.as_ref().map(|associations_ident| {
                self::associations::derive_associations(
                    has_many,
                    has_one,
                    &associations_ident,
                    factory_ident,
                )
            });
        let belonging_to_link_implementations =
            self::associations::belongs_to::derive_belonging_to_link_implementations(
                factory_ident,
                fields,
            );

        Ok(quote::quote! {
            #factory_definition
            #factory_implementation
            #mixin_implementations
            #setter_implementations
            #belonging_to_link_implementations
            #associated_resources_definition_and_implementation
        })
    }
}

fn derive_factory_definition(
    factory_ident: &Ident,
    fields: &Fields<FactoryDeriveField>,
) -> TokenStream {
    let factory_fields: TokenStream = fields
        .iter()
        .map(
            |FactoryDeriveField {
                 ident,
                 ty,
                 mixin: _,
                 into: _,
                 dependant: _,
                 belongs_to,
                 default: _,
             }| {
                let ident = ident.as_ref().expect("Only named structs are supported");
                match belongs_to {
                    Some(belongs_to) => belongs_to.field_definition(ident, ty),
                    None => quote::quote!(#ident: #ty,),
                }
            },
        )
        .collect();
    // TODO: split to own module ?
    let factory_default_fields: TokenStream = fields
        .iter()
        .map(
            |FactoryDeriveField {
                 ident,
                 ty: _,
                 mixin: _,
                 into: _,
                 dependant: _,
                 belongs_to: _,
                 default,
             }| {
                let ident = ident.as_ref().expect("Only named structs are supported");
                match default {
                    Some(expr) => quote::quote!(#ident: #expr,),
                    None => quote::quote!(#ident: Default::default(),),
                }
            },
        )
        .collect();
    quote::quote!(
        pub struct #factory_ident {
            #factory_fields
        }

        impl Default for #factory_ident {
            fn default() -> Self {
                #factory_ident {
                    #factory_default_fields
                }
            }
        }
    )
}

fn derive_factory_implementation(
    attributes_ident: &Ident,
    factory_ident: &Ident,
    fields: &Fields<FactoryDeriveField>,
) -> darling::Result<TokenStream> {
    let mut impl_block_conditions: Vec<TokenStream> = Vec::new();

    let destructured_factory_fields = self::field::destructure_factory_fields(fields);
    let associations_pre_create =
        self::associations::belongs_to::resolve_belongs_to_assocations_and_add_conditions(
            &mut impl_block_conditions,
            fields,
        );
    let reassign_dependant_attributes = self::field::reassign_dependant_attributes(fields);
    let destructured_attributes_fields = self::field::destructure_attributes_fields(fields);

    impl_block_conditions.push(quote::quote!(#attributes_ident: ::fabriko::BuildResource<CTX>,));
    let where_clause: TokenStream = impl_block_conditions.into_iter().collect();
    Ok(quote::quote! {
        impl<CTX: ::fabriko::FactoryContext> ::fabriko::Factory<CTX> for #factory_ident
        where
            #where_clause
        {
            type Output = <#attributes_ident as ::fabriko::BuildResource<CTX>>::Output;

            fn create(self, ctx: &mut CTX) -> Result<Self::Output, CTX::Error> {
                let #factory_ident {
                    #destructured_factory_fields
                    ..
                } = self;

                // Resolves associations
                use ::fabriko::ResolveDependency;
                #associations_pre_create

                // Reassigns dependant attributes
                #reassign_dependant_attributes

                // Build resource
                let __resource = #attributes_ident {
                    #destructured_attributes_fields
                }
                .build_resource(ctx)?;

                Ok(__resource)
            }
        }
    })
}

pub(crate) fn do_derive_factory(input: &DeriveInput) -> darling::Result<TokenStream> {
    let factory_derive_input = FactoryDeriveInput::from_derive_input(input)?;
    factory_derive_input.derive()
}
