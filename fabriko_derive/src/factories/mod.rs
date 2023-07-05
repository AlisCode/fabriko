use darling::{
    ast::{Data, Fields},
    FromDeriveInput,
};
use proc_macro2::TokenStream;
use syn::{DeriveInput, Ident, Path};

use self::associations::has_many::HasManyAssociation;
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
    attributes: Path,
    #[darling(multiple)]
    has_many: Vec<HasManyAssociation>,
}

impl FactoryDeriveInput {
    pub fn derive(&self) -> darling::Result<TokenStream> {
        let FactoryDeriveInput {
            ident,
            data,
            attributes,
            has_many,
        } = self;
        let fields = match data {
            Data::Enum(_) => unimplemented!(), // TODO: Proper error
            Data::Struct(fields) => fields,
        };

        let mixin_implementations = self::mixins::derive_mixin_implementations(ident, fields)?;
        let setter_implementations = self::setters::derive_setters_implementations(ident, fields)?;
        let factory_implementation = derive_factory_implementation(ident, attributes, fields)?;
        // We don't need the code for associated resources if we don't have any
        let associated_resources_definition_and_implementation = if !has_many.is_empty() {
            self::associations::has_many::derive_factory_associated_resources_and_implementation(
                ident, has_many,
            )?
        } else {
            TokenStream::default()
        };

        Ok(quote::quote! {
            #mixin_implementations
            #setter_implementations
            #factory_implementation
            #associated_resources_definition_and_implementation
        })
    }
}

fn derive_factory_implementation(
    factory_ident: &Ident,
    attributes_ty_path: &Path,
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

    impl_block_conditions.push(quote::quote!(#attributes_ty_path: ::fabriko::BuildResource<CTX>,));
    let where_clause: TokenStream = impl_block_conditions.into_iter().collect();
    Ok(quote::quote! {
        impl<CTX: ::fabriko::FactoryContext> ::fabriko::Factory<CTX> for #factory_ident
        where
            #where_clause
        {
            type Output = <#attributes_ty_path as BuildResource<CTX>>::Output;

            fn create(self, ctx: &mut CTX) -> Result<Self::Output, CTX::Error> {
                let #factory_ident {
                    #destructured_factory_fields
                    ..
                } = self;

                // Resolves associations
                use ::fabriko::CreateBelongingTo;
                #associations_pre_create

                // Reassigns dependant attributes
                #reassign_dependant_attributes

                // Build resource
                let __resource = #attributes_ty_path {
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
