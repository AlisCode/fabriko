use darling::FromMeta;
use proc_macro2::Span;
use syn::{Ident, Path};

use super::{AssociationsCodegen, AssociationsSetter};

#[derive(FromMeta)]
/// TODO: Document
pub(crate) struct HasManyAssociation {
    #[darling(rename = "factory")]
    pub(crate) for_factory: Path,
    pub(crate) name: Ident,
    pub(crate) setter: Ident,
}

impl AssociationsCodegen for HasManyAssociation {
    fn derive_setters(
        &self,
        structure: &super::AssociationAttributesStructure,
    ) -> proc_macro2::TokenStream {
        let HasManyAssociation {
            for_factory,
            name,
            setter,
        } = self;
        let setter_fn_name = Ident::new(&format!("with_{setter}"), Span::call_site());
        let setter_fn = quote::quote!(#setter_fn_name<FUNC: FnOnce(#for_factory) -> #for_factory>);
        AssociationsSetter {
            field_ident: name,
            setter_fn,
            argument_of_setter: quote::quote!(__func: FUNC),
            create_set_type_of_association: quote::quote!(#name.with(__func)),
            default_type_of_association: quote::quote!(::fabriko::HasMany<#for_factory>),
            set_type_of_association: quote::quote!(::fabriko::HasMany<#for_factory>),
        }
        .derive_setter(structure)
    }
}

/*
impl HasManyAssociation {
    /// TODO: this function and derive_fn_implementation can be factored
    fn derive_fn_definition(&self, factory_ident: &Ident) -> TokenStream {
        let HasManyAssociation {
            for_factory,
            name,
            setter,
        } = self;
        let fn_ident = Ident::new(&format!("with_{name}"), name.span());

        let mut hasher = fnv::FnvHasher::default();
        setter.to_string().hash(&mut hasher);
        let hash_setter_name = hasher.finish();

        quote::quote! {
            fn #fn_ident<F: FnOnce(#for_factory) -> #for_factory>(
                self,
                func: F,
            ) -> ::fabriko::FactoryWithResources<
                #factory_ident,
                <Self::R as ::fabriko::AppendTuple>::Output<
                    // Ensure field #name is a link
                    ::fabriko::FactoryBelongingTo<{ #hash_setter_name }, #for_factory>,
                >,
            >;
        }
    }

    /// TODO: this function and derive_fn_definition can be factored
    fn derive_fn_implementation(&self, factory_ident: &Ident) -> TokenStream {
        let HasManyAssociation {
            for_factory,
            name,
            setter,
        } = self;
        let fn_ident = Ident::new(&format!("with_{name}"), name.span());

        let mut hasher = fnv::FnvHasher::default();
        setter.to_string().hash(&mut hasher);
        let hash_setter_name = hasher.finish();

        quote::quote! {
            fn #fn_ident<F: FnOnce(#for_factory) -> #for_factory>(
                self,
                func: F,
            ) -> ::fabriko::FactoryWithResources<
                #factory_ident,
                <Self::R as ::fabriko::AppendTuple>::Output<
                    // Ensure field #name is a link
                    ::fabriko::FactoryBelongingTo<{ #hash_setter_name }, #for_factory>,
                >,
            > {
                let factory = func(#for_factory::default());
                let factory = ::fabriko::FactoryBelongingTo { factory };
                self.with_resource(factory)
            }
        }
    }
}
*/
