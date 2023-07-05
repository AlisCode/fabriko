use std::collections::HashMap;

use darling::{
    ast::{Data, Fields},
    FromDeriveInput, FromField,
};
use proc_macro2::TokenStream;
use syn::{DeriveInput, Expr, Ident, Path, Type};

#[derive(FromDeriveInput)]
pub(crate) struct BundleDeriveInput {
    ident: Ident,
    data: darling::ast::Data<(), BundleDeriveField>,
}

#[derive(FromField)]
#[darling(attributes(bundle))]
pub(crate) struct BundleDeriveField {
    ident: Option<Ident>,
    ty: Type,
    factory: Path,
    attributes: HashMap<Ident, Expr>,
}

impl BundleDeriveField {
    pub fn where_clause(&self) -> TokenStream {
        let BundleDeriveField {
            ident: _,
            ty,
            factory,
            attributes: _,
        } = self;
        quote::quote! {
            #factory: ::fabriko::Factory<CTX, Output = #ty>,
        }
    }
}

fn build_bundle_struct(ident: &Ident, fields: &Fields<BundleDeriveField>) -> TokenStream {
    let fields: TokenStream = fields
        .iter()
        .map(
            |BundleDeriveField {
                 ident,
                 ty: _,
                 factory: _,
                 attributes: _,
             }| quote::quote!(#ident,),
        )
        .collect();
    quote::quote!(
        #ident {
            #fields
        }
    )
}

fn instantiate_bundle_fields(fields: &Fields<BundleDeriveField>) -> TokenStream {
    fields
        .iter()
        .map(
            |BundleDeriveField {
                 ident,
                 ty: _,
                 factory,
                 attributes,
             }| {
                let attributes_customization: TokenStream = attributes
                    .into_iter()
                    .map(|(method, expr)| quote::quote!(.#method(#expr)))
                    .collect();
                quote::quote!(
                    let #ident = #factory::default()
                    #attributes_customization
                    .create(cx)?;
                )
            },
        )
        .collect()
}

impl BundleDeriveInput {
    pub(crate) fn derive_factory_bundle_implementation(self) -> darling::Result<TokenStream> {
        let BundleDeriveInput { ident, data } = self;

        let fields = match data {
            Data::Enum(_) => unimplemented!(), // TODO: Proper error
            Data::Struct(fields) => fields,
        };

        let where_clause: TokenStream =
            fields.iter().map(BundleDeriveField::where_clause).collect();
        let instantiated_bundle_fields = instantiate_bundle_fields(&fields);
        let returned_bundle_struct = build_bundle_struct(&ident, &fields);

        Ok(quote::quote! {
            impl<CTX: ::fabriko::FactoryContext> ::fabriko::FactoryBundle<CTX> for #ident
            where
                #where_clause
            {
                fn create_bundle(cx: &mut CTX) -> Result<Self, CTX::Error> {
                    #instantiated_bundle_fields
                    Ok(#returned_bundle_struct)
                }
            }
        })
    }
}

pub(crate) fn do_derive_bundle(input: &DeriveInput) -> darling::Result<TokenStream> {
    let bundle_derive_input = BundleDeriveInput::from_derive_input(input)?;
    bundle_derive_input.derive_factory_bundle_implementation()
}
