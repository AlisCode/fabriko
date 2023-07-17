use crate::tuple_hack::UnitTuple;

pub trait FactoryContext {
    type Error;
}

pub trait Factory<CTX: FactoryContext> {
    type Output;
    fn create(self, ctx: &mut CTX) -> Result<Self::Output, CTX::Error>;
}

impl<CTX: FactoryContext, F: Factory<CTX>> Factory<CTX> for UnitTuple<F> {
    type Output = F::Output;

    fn create(self, ctx: &mut CTX) -> Result<Self::Output, <CTX as FactoryContext>::Error> {
        self.0.create(ctx)
    }
}

impl<CTX: FactoryContext, F: Factory<CTX> + std::any::Any> Factory<CTX> for Box<F> {
    type Output = F::Output;

    fn create(self, ctx: &mut CTX) -> Result<Self::Output, <CTX as FactoryContext>::Error> {
        (*self).create(ctx)
    }
}

macro_rules! impl_factory_tuple {
    ($($T:ident),*) => {
        impl<CTX: FactoryContext, $($T: Factory<CTX>),*> Factory<CTX> for ($($T),*) {
            type Output = ($(<$T as Factory<CTX>>::Output),*);
            #[allow(non_snake_case)]
            fn create(self, ctx: &mut CTX) -> Result<Self::Output, <CTX as FactoryContext>::Error> {
                let ($($T),*) = self;
                $(
                    let $T = $T.create(ctx)?;
                )*
                Ok(($($T),*))
            }
        }
    };
}

impl_factory_tuple!(A, B);
impl_factory_tuple!(A, B, C);
impl_factory_tuple!(A, B, C, D);
impl_factory_tuple!(A, B, C, D, E);
impl_factory_tuple!(A, B, C, D, E, F);
impl_factory_tuple!(A, B, C, D, E, F, G);
impl_factory_tuple!(A, B, C, D, E, F, G, H);
impl_factory_tuple!(A, B, C, D, E, F, G, H, I);
impl_factory_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_factory_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_factory_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);

pub trait BuildResource<CTX: FactoryContext> {
    type Output;
    fn build_resource(self, ctx: &mut CTX) -> Result<Self::Output, CTX::Error>;
}
