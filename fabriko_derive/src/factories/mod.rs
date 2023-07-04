use darling::{
    ast::{Data, Fields},
    FromDeriveInput, FromField, FromMeta,
};
use proc_macro2::TokenStream;
use syn::{DeriveInput, Expr, Ident, Path, Type};

#[derive(FromDeriveInput)]
#[darling(supports(struct_named))]
#[darling(attributes(factory))]
/// TODO: Document
/// TODO: Split into own module
struct FactoryDeriveInput {
    ident: Ident,
    data: Data<darling::util::Ignored, FactoryDeriveField>,
    attributes: Path,
    #[darling(multiple)]
    has_many: Vec<HasManyAssociation>,
}

#[derive(FromField)]
#[darling(attributes(factory))]
/// TODO: Document
/// TODO: Split into own module
struct FactoryDeriveField {
    ident: Option<Ident>,
    ty: Type,
    #[darling(default)]
    mixin: bool,
    dependant: Option<Expr>,
    belongs_to: Option<BelongsToAssociation>,
}

#[derive(FromMeta)]
/// TODO: Document
/// TODO: Split into own module
struct HasManyAssociation {
    #[darling(rename = "factory")]
    for_factory: Path,
    name: Ident,
}

#[derive(FromMeta)]
/// TODO: Document
/// TODO: Split into own module
struct BelongsToAssociation {
    ty: Path,
    field: Ident,
    /// TODO: Probably unnecessary
    id_ty: Path,
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
            Data::Enum(_) => unimplemented!(),
            Data::Struct(fields) => fields,
        };

        let mixin_implementations = derive_mixin_implementations(ident, fields)?;
        let setter_implementations = derive_setters_implementations(ident, fields)?;
        let factory_implementation = derive_factory_implementation(ident, attributes, fields)?;
        // We don't need the code for associated resources if we don't have any
        let associated_resources_definition_and_implementation = if !has_many.is_empty() {
            derive_factory_associated_resources_and_implementation(ident, has_many)?
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

/// TODO: Split into own module
fn derive_factory_associated_resources_and_implementation(
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

fn derive_mixin_implementations(
    factory_ident: &Ident,
    fields: &Fields<FactoryDeriveField>,
) -> darling::Result<TokenStream> {
    let mixin_impls: TokenStream = fields
        .iter()
        .flat_map(|field| field.derive_mixin_field(factory_ident))
        .collect();

    Ok(mixin_impls)
}

fn derive_setters_implementations(
    factory_ident: &Ident,
    fields: &Fields<FactoryDeriveField>,
) -> darling::Result<TokenStream> {
    let setters: TokenStream = fields
        .iter()
        .flat_map(|field| field.derive_setter())
        .collect();
    Ok(quote::quote! {
        impl #factory_ident {
            #setters
        }
    })
}

fn derive_factory_implementation(
    factory_ident: &Ident,
    attributes_ty_path: &Path,
    fields: &Fields<FactoryDeriveField>,
) -> darling::Result<TokenStream> {
    let destructure_attributes_fields: TokenStream = fields
        .iter()
        .filter_map(|field| {
            let field_ident = &field.ident;
            if field.dependant.is_some() {
                None
            } else {
                Some(quote::quote!(#field_ident,))
            }
        })
        .collect();

    let attributes_fields: TokenStream = fields
        .iter()
        .filter_map(|field| {
            if field.is_factory_attribute() {
                let field_ident = &field.ident;
                Some(quote::quote!(#field_ident,))
            } else {
                None
            }
        })
        .collect();

    let dependant_attributes: TokenStream = fields
        .iter()
        .filter_map(
            |FactoryDeriveField {
                 ident,
                 ty,
                 dependant,
                 ..
             }| {
                dependant.as_ref().map(|expr| {
                    quote::quote!(
                        let #ident: #ty = #expr;
                    )
                })
            },
        )
        .collect();

    let mut conditions: Vec<TokenStream> = Vec::new();
    let associations_pre_create: TokenStream = fields
        .iter()
        .filter_map(
            |FactoryDeriveField {
                 ident, ty, belongs_to, ..
             }| {
                belongs_to
                    .as_ref()
                    .map(|BelongsToAssociation { ty: belongs_to_ty, field, id_ty }| {
                        conditions.push(quote::quote! { #ty: ::fabriko::CreateBelongingTo<CTX, FactoryOutput = #belongs_to_ty, ID = #id_ty>, });
                        quote::quote! {
                            let #ident = #ident.create_belonging_to(ctx, |__res: #belongs_to_ty| __res.#field)?;
                        }
                    })
            },
        )
        .collect();

    let conditions: TokenStream = conditions.into_iter().collect();
    Ok(quote::quote! {
        impl<CTX: ::fabriko::FactoryContext> ::fabriko::Factory<CTX> for #factory_ident
        where
            #attributes_ty_path: ::fabriko::BuildResource<CTX>,
            #conditions
        {
            type Output = <#attributes_ty_path as BuildResource<CTX>>::Output;

            fn create(self, ctx: &mut CTX) -> Result<Self::Output, CTX::Error> {
                let #factory_ident {
                    #destructure_attributes_fields
                    ..
                } = self;

                // Associations (pre-create)
                use ::fabriko::CreateBelongingTo;
                #associations_pre_create

                // Dependant attributes (pre-create)
                #dependant_attributes

                // Build resource
                let __resource = #attributes_ty_path {
                    #attributes_fields
                }
                .build_resource(ctx)?;

                Ok(__resource)
            }
        }
    })
}

impl FactoryDeriveField {
    /// TODO: Remove ?
    pub fn is_factory_attribute(&self) -> bool {
        true
    }

    pub fn should_derive_setter(&self) -> bool {
        !self.mixin
    }

    pub fn derive_mixin_field(&self, factory_ident: &Ident) -> Option<TokenStream> {
        let FactoryDeriveField {
            ident, ty, mixin, ..
        } = self;
        if *mixin {
            return Some(quote::quote! {
                 impl ::fabriko::WithMixin<#ty> for #factory_ident {
                     fn with_mixin<F: FnOnce(#ty) -> #ty>(mut self, f: F) -> Self {
                         self.#ident = f(self.#ident);
                         self
                     }
                 }
            });
        }
        None
    }

    pub fn derive_setter(&self) -> Option<TokenStream> {
        if self.should_derive_setter() {
            let FactoryDeriveField {
                ident,
                ty,
                belongs_to,
                ..
            } = self;
            match belongs_to {
                Some(BelongsToAssociation {
                    ty: _,
                    field: _,
                    id_ty: _,
                }) => {
                    let ident = ident.as_ref().expect("Only named structs are supported");
                    let setter_belonging_to =
                        Ident::new(&format!("belonging_to_{}", ident), ident.span());
                    return Some(quote::quote!(
                        pub fn #setter_belonging_to<F: FnOnce(<#ty as ::fabriko::BelongsToInfo>::Factory) -> <#ty as ::fabriko::BelongsToInfo>::Factory>(mut self, f: F) -> Self {
                            self.#ident = ::fabriko::BelongsTo::Create(f(Default::default()));
                            self
                        }
                        pub fn #ident(mut self, id: <#ty as ::fabriko::BelongsToInfo>::ID) -> Self {
                            self.#ident = ::fabriko::BelongsTo::Created(id);
                            self
                        }
                    ));
                }
                None => {
                    return Some(quote::quote!(
                        pub fn #ident(mut self, #ident: #ty) -> Self {
                            self.#ident = #ident;
                            self
                        }
                    ));
                }
            }
        }
        None
    }
}

pub(crate) fn do_derive_factory(input: &DeriveInput) -> darling::Result<TokenStream> {
    let factory_derive_input = FactoryDeriveInput::from_derive_input(input)?;
    factory_derive_input.derive()
}
