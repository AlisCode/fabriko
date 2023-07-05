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
}

impl HasManyAssociation {
    /// TODO: this function and derive_fn_implementation can be factored
    fn derive_fn_definition(&self, factory_ident: &Ident) -> TokenStream {
        let HasManyAssociation { for_factory, name } = self;
        let fn_ident = Ident::new(&format!("with_{name}"), name.span());
        quote::quote! {
            fn #fn_ident<F: FnOnce(#for_factory) -> #for_factory>(
                self,
                func: F,
            ) -> ::fabriko::FactoryWithResources<#factory_ident, <Self::R as ::fabriko::AppendTuple>::Output<#for_factory>>;
        }
    }

    /// TODO: this function and derive_fn_definition can be factored
    fn derive_fn_implementation(&self, factory_ident: &Ident) -> TokenStream {
        let HasManyAssociation { for_factory, name } = self;
        let fn_ident = Ident::new(&format!("with_{name}"), name.span());
        quote::quote! {
            fn #fn_ident<F: FnOnce(#for_factory) -> #for_factory>(
                self,
                func: F,
            ) -> ::fabriko::FactoryWithResources<#factory_ident, <Self::R as ::fabriko::AppendTuple>::Output<#for_factory>> {
                self.with_resource(func(#for_factory::default()))
            }
        }
    }
}

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
