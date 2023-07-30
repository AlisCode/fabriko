use std::hash::{Hash, Hasher};

use darling::FromMeta;
use fnv::FnvHasher;
use proc_macro2::{Span, TokenStream};
use syn::{Ident, Path};

use super::{AssociationsCodegen, AssociationsSetter};

#[derive(FromMeta)]
/// TODO: Document
pub(crate) struct HasManyAssociation {
    #[darling(rename = "factory")]
    pub(crate) for_factory: Path,
    pub(crate) name: Ident,
    pub(crate) link: Ident,
}

impl HasManyAssociation {
    pub(crate) fn has_many_type(&self) -> TokenStream {
        let setter_hash = self.link_hash();
        let HasManyAssociation {
            for_factory,
            name: _,
            link: _,
        } = self;
        quote::quote!(::fabriko::HasMany<#setter_hash, #for_factory>)
    }

    pub(crate) fn link_hash(&self) -> u64 {
        let mut hasher = FnvHasher::default();
        self.link.to_string().hash(&mut hasher);
        hasher.finish()
    }
}

impl AssociationsCodegen for HasManyAssociation {
    fn derive_setters(
        &self,
        structure: &super::AssociationAttributesStructure,
    ) -> proc_macro2::TokenStream {
        let has_many_type = self.has_many_type();
        let HasManyAssociation {
            for_factory,
            name,
            link: _,
        } = self;
        let setter_fn_name = Ident::new(&format!("with_{name}"), Span::call_site());
        let setter_fn = quote::quote!(#setter_fn_name<FUNC: FnOnce(#for_factory) -> #for_factory>);
        let setter_hash = self.link_hash();

        AssociationsSetter {
            field_ident: name,
            setter_fn,
            argument_of_setter: quote::quote!(__func: FUNC),
            create_set_type_of_association: quote::quote!(#name.with::<#setter_hash, _>(__func)),
            default_type_of_association: has_many_type.clone(),
            set_type_of_association: has_many_type,
        }
        .derive_setter(structure)
    }
}
