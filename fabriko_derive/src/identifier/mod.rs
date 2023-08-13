use darling::{ast::Data, FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use syn::{Attribute, DeriveInput, Ident, Type};

#[derive(Debug, FromDeriveInput)]
#[darling(supports(struct_named))]
// TODO: Support unnamed structures
struct WithIdentifierDeriveInput {
    ident: Ident,
    data: Data<darling::util::Ignored, WithIdentifierField>,
}

#[derive(Debug, FromField)]
// #[darling(attributes(identifier))]
#[darling(forward_attrs(identifier))]
struct WithIdentifierField {
    ident: Option<Ident>,
    ty: Type,
    attrs: Vec<Attribute>,
}

pub(crate) fn do_derive_with_identifier(
    input: &DeriveInput,
) -> Result<TokenStream, darling::Error> {
    let WithIdentifierDeriveInput { ident, data } =
        WithIdentifierDeriveInput::from_derive_input(input)?;

    let fields = match data {
        Data::Enum(_) => unimplemented!("Enum not supported"),
        Data::Struct(fields) => fields,
    };

    let id_fields: Vec<(&Type, &Option<Ident>)> = fields
        .iter()
        .filter_map(|WithIdentifierField { ident, ty, attrs }| {
            if !attrs.is_empty() {
                Some((ty, ident))
                // Some(quote::quote!(#ty,))
            } else {
                None
            }
        })
        .collect();

    let typedef = if id_fields.len() == 1 {
        let (ty, _) = id_fields[0];
        quote::quote!(#ty)
    } else {
        let tys: TokenStream = id_fields
            .iter()
            .map(|(ty, _)| quote::quote!(#ty,))
            .collect();
        quote::quote!((#tys))
    };

    let extract_fields = if id_fields.len() == 1 {
        let (_, ident) = id_fields[0];
        quote::quote!(self.#ident)
    } else {
        let idents: TokenStream = id_fields
            .iter()
            .map(|(_, ident)| quote::quote!(self.#ident,))
            .collect();
        quote::quote!((#idents))
    };

    Ok(quote::quote! {
        impl ::fabriko::WithIdentifier for #ident {
            type ID = #typedef;

            fn extract_id(&self) -> Self::ID {
                #extract_fields
            }
        }
    })
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn should_derive_with_identifier_trait_for_struct() {
        let input: DeriveInput = syn::parse_quote!(
            #[derive(WithIdentifier)]
            pub struct Person {
                #[identifier]
                id: i32,
                age: u8,
                name: String,
            }
        );
        let derived_impl =
            do_derive_with_identifier(&input).expect("Failed to derive implementation");
        let expected_impl = quote::quote!(impl ::fabriko::WithIdentifier for Person {
            type ID = i32;

            fn extract_id(&self) -> Self::ID {
                self.id
            }
        });
        assert_eq!(derived_impl.to_string(), expected_impl.to_string());
    }

    #[test]
    fn should_derive_with_identifier_trait_for_struct_composite_key() {
        let input: DeriveInput = syn::parse_quote!(
            #[derive(WithIdentifier)]
            pub struct Person {
                email: i32,
                #[identifier]
                age: u8,
                #[identifier]
                name: String,
            }
        );
        let derived_impl =
            do_derive_with_identifier(&input).expect("Failed to derive implementation");
        let expected_impl = quote::quote!(impl ::fabriko::WithIdentifier for Person {
            type ID = (u8,String,);

            fn extract_id(&self) -> Self::ID {
                (self.age,self.name,)
            }
        });
        assert_eq!(derived_impl.to_string(), expected_impl.to_string());
    }
}
