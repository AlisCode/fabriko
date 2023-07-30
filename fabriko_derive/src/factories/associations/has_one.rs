use darling::FromMeta;
use fnv::FnvHasher;
use proc_macro2::{Span, TokenStream};
use std::hash::{Hash, Hasher};
use syn::{Ident, Path};

use crate::factories::associations::AssociationsSetter;

use super::{AssociationAttributesStructure, AssociationsCodegen};

#[derive(FromMeta)]
/// TODO: Document
pub(crate) struct HasOneAssociation {
    #[darling(rename = "factory")]
    pub(crate) for_factory: Path,
    pub(crate) name: Ident,
    pub(crate) link: Ident,
}

impl AssociationsCodegen for HasOneAssociation {
    fn derive_setters(&self, structure: &AssociationAttributesStructure) -> TokenStream {
        let predefined = PredefinedSetter(self).derive_setters(structure);
        let factory = FactorySetter(self).derive_setters(structure);
        quote::quote! {
            #predefined
            #factory
        }
    }
}

struct PredefinedSetter<'a>(&'a HasOneAssociation);

impl<'a> AssociationsCodegen for PredefinedSetter<'a> {
    fn derive_setters(&self, structure: &AssociationAttributesStructure) -> TokenStream {
        let HasOneAssociation {
            for_factory,
            name,
            link: setter,
        } = self.0;
        let setter_fn_name = Ident::new(&format!("{name}_id"), Span::call_site());
        let setter_fn = quote::quote!(#setter_fn_name);
        let mut hasher = FnvHasher::default();
        setter.to_string().hash(&mut hasher);
        let setter_hash = hasher.finish();
        let argument_for_setter =
            quote::quote!(<#for_factory as ::fabriko::BelongingToLink<#setter_hash>>::ID);
        AssociationsSetter {
            field_ident: name,
            setter_fn,
            argument_of_setter: quote::quote!(__resource_id: #argument_for_setter),
            create_set_type_of_association: quote::quote!(::fabriko::HasOneCreated::new(
                __resource_id
            )),
            default_type_of_association: quote::quote!(::fabriko::HasOneDefault<#for_factory>),
            set_type_of_association: quote::quote!(::fabriko::HasOneCreated<#argument_for_setter>),
        }
        .derive_setter(structure)
    }
}

struct FactorySetter<'a>(&'a HasOneAssociation);

impl<'a> AssociationsCodegen for FactorySetter<'a> {
    fn derive_setters(&self, structure: &AssociationAttributesStructure) -> TokenStream {
        let HasOneAssociation {
            for_factory,
            name,
            link,
        } = self.0;
        let setter_fn = quote::quote!(#name<FUNC: FnOnce(#for_factory) -> #for_factory>);
        let mut hasher = FnvHasher::default();
        link.to_string().hash(&mut hasher);
        let setter_hash = hasher.finish();

        AssociationsSetter {
            field_ident: name,
            setter_fn,
            argument_of_setter: quote::quote!(__func: FUNC),
            create_set_type_of_association: quote::quote!(::fabriko::HasOneToCreate::new(__func(
                Default::default()
            ))),
            default_type_of_association: quote::quote!(::fabriko::HasOneDefault<#for_factory>),
            set_type_of_association: quote::quote!(::fabriko::HasOneToCreate<#setter_hash, #for_factory>),
        }
        .derive_setter(structure)
    }
}
