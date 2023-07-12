use darling::FromMeta;
use proc_macro2::TokenStream;
use syn::{Ident, Path};

#[derive(FromMeta)]
pub(crate) struct FabrikoDeriveAttribute {
    factory: Path,
    function: Ident,
}

impl FabrikoDeriveAttribute {
    pub(crate) fn derive_factory_fn_for_wrapper(self, context_ident: &Ident) -> TokenStream {
        let FabrikoDeriveAttribute { factory, function } = self;
        quote::quote!(
            pub fn #function<FF: ::fabriko::Factory<#context_ident>, CB: FnOnce(#factory) -> FF>(
                &mut self,
                callback: CB,
            ) -> <FF as ::fabriko::Factory<#context_ident>>::Output {
                self.0.factory(callback)
            }
        )
    }
}

#[cfg(test)]
pub mod tests {
    use darling::FromMeta;
    use proc_macro2::Span;
    use syn::{Ident, Meta};

    use super::FabrikoDeriveAttribute;

    #[test]
    fn should_derive_factory_fn_for_wrapper() {
        let meta: Meta = syn::parse_quote!(fabriko(factory = "MyFactory", function = "my_factory"));
        let fabriko_derive_attribute = FabrikoDeriveAttribute::from_meta(&meta)
            .expect("Failed to parse FabrikoDeriveAttribute");

        let expected = quote::quote!(
            pub fn my_factory<FF: ::fabriko::Factory<MyContext>, CB: FnOnce(MyFactory) -> FF>(
                &mut self,
                callback: CB,
            ) -> <FF as ::fabriko::Factory<MyContext>>::Output {
                self.0.factory(callback)
            }
        );
        let actual = fabriko_derive_attribute
            .derive_factory_fn_for_wrapper(&Ident::new("MyContext", Span::call_site()));
        assert_eq!(expected.to_string(), actual.to_string());
    }
}
