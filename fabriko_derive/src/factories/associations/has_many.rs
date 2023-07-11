use std::hash::{Hash, Hasher};

use darling::FromMeta;
use proc_macro2::TokenStream;
use syn::{Ident, Path};

#[derive(FromMeta)]
/// TODO: Document
/// TODO: Split into own module
pub(crate) struct HasManyAssociation {
    #[darling(rename = "factory")]
    for_factory: Path,
    name: Ident,
    setter: Ident,
}

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

/*
 impl fabriko::WithRelatedResources for TodoGroupFactory {}
pub trait TodoGroupFactoryAssociatedResources<R: fabriko::AppendTuple> {
    fn with_todo<F: FnOnce(TodoFactory) -> TodoFactory>(
        self,
        func: F,
    ) -> ::fabriko::FactoryWithResources<
        TodoGroupFactory,
        <R as ::fabriko::AppendTuple>::Output<
            ::fabriko::FactoryBelongingTo<{ TODO_GROUP }, TodoFactory>,
        >,
    >;
}
impl<R: ::fabriko::AppendTuple> TodoGroupFactoryAssociatedResources<R>
    for ::fabriko::FactoryWithResources<TodoGroupFactory, R>
{
    fn with_todo<F: FnOnce(TodoFactory) -> TodoFactory>(
        self,
        func: F,
    ) -> ::fabriko::FactoryWithResources<
        TodoGroupFactory,
        <R as ::fabriko::AppendTuple>::Output<
            ::fabriko::FactoryBelongingTo<{ TODO_GROUP }, TodoFactory>,
        >,
    > {
        let factory = func(TodoFactory::default());
        let factory = FactoryBelongingTo { factory };
        self.with_resource(factory)
    }
}
 */

pub(crate) fn derive_factory_associated_resources_and_implementation(
    factory_ident: &Ident,
    has_many: &[HasManyAssociation],
) -> darling::Result<TokenStream> {
    let trait_identifier = Ident::new(
        &format!("{factory_ident}AssociatedResources"),
        factory_ident.span(),
    );

    let trait_function_definitions: TokenStream = has_many
        .iter()
        .map(|hma| hma.derive_fn_definition(factory_ident))
        .collect();
    let trait_function_implementations: TokenStream = has_many
        .iter()
        .map(|hma| hma.derive_fn_implementation(factory_ident))
        .collect();

    Ok(quote::quote! {
        impl fabriko::WithRelatedResources for #factory_ident {}

        pub trait TodoGroupFactoryAssociatedResources {
            type R: fabriko::AppendTuple;
            #trait_function_definitions
        }

        impl<R: ::fabriko::AppendTuple> #trait_identifier
            for ::fabriko::FactoryWithResources<#factory_ident, R>
        {
            type R = R;
            #trait_function_implementations
        }
    })
}
