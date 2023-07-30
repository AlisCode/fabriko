use crate::{
    factory::{Factory, FactoryContext},
    tuple_hack::UnitTuple,
    FactorySetter, WithIdentifier,
};

use super::ResolveDependency;

#[derive(Debug)]
pub enum BelongsTo<F, ID> {
    Create(F),
    Created(ID),
}

impl<F: Default, ID> Default for BelongsTo<F, ID> {
    fn default() -> Self {
        BelongsTo::Create(F::default())
    }
}

impl<CTX: FactoryContext, F: Factory<CTX>, ID> ResolveDependency<CTX> for BelongsTo<F, ID>
where
    <F as Factory<CTX>>::Output: WithIdentifier<ID = ID>,
{
    type Output = ID;
    fn resolve_dependency(self, cx: &mut CTX) -> Result<Self::Output, CTX::Error> {
        let id = match self {
            BelongsTo::Create(factory) => factory.create(cx)?.extract_id(),
            BelongsTo::Created(id) => id,
        };
        Ok(id)
    }
}

pub trait BelongingToLink<const N: u64> {
    type ID;
    const SETTER: FactorySetter<Self, Self::ID>;
}

#[derive(Debug)]
pub struct FactoryBelongingTo<const N: u64, F> {
    pub factory: F,
}

impl<const N: u64, R: WithIdentifier, F: BelongingToLink<N, ID = <R as WithIdentifier>::ID>>
    BelongingTo<R> for FactoryBelongingTo<N, F>
{
    fn belonging_to(self, resource: &R) -> Self {
        let FactoryBelongingTo { factory } = self;
        let factory = BelongingToLink::<{ N }>::SETTER(factory, resource.extract_id());
        FactoryBelongingTo { factory }
    }
}

impl<const N: u64, CTX: FactoryContext, F: Factory<CTX>> Factory<CTX>
    for FactoryBelongingTo<{ N }, F>
{
    type Output = <F as Factory<CTX>>::Output;

    fn create(self, ctx: &mut CTX) -> Result<Self::Output, <CTX as FactoryContext>::Error> {
        self.factory.create(ctx)
    }
}

pub trait BelongingTo<R> {
    fn belonging_to(self, resource: &R) -> Self;
}

impl<RES, T: BelongingTo<RES>> BelongingTo<RES> for UnitTuple<T> {
    fn belonging_to(mut self, resource: &RES) -> Self {
        self.0 = self.0.belonging_to(resource);
        self
    }
}

// impl<RES, A: BelongingTo<RES>, B: BelongingTo<RES>> BelongingTo<RES> for (A, B) {
//     fn belonging_to(self, resource: &RES) -> Self {
//         let (a, b) = self;
//         (a.belonging_to(resource), b.belonging_to(resource))
//     }
// }

macro_rules! impl_belonging_to_tuple {
    ($($T:ident),*) => {
        impl<RES, $($T: BelongingTo<RES>),*> BelongingTo<RES> for ($($T),*) {
            #[allow(non_snake_case)]
            fn belonging_to(self, resource: &RES) -> Self {
                let ($($T),*) = self;
                ($($T.belonging_to(resource)),*)
            }
        }
    };
}

impl_belonging_to_tuple!(A, B);
impl_belonging_to_tuple!(A, B, C);
impl_belonging_to_tuple!(A, B, C, D);
impl_belonging_to_tuple!(A, B, C, D, E);
impl_belonging_to_tuple!(A, B, C, D, E, F);
impl_belonging_to_tuple!(A, B, C, D, E, F, G);
impl_belonging_to_tuple!(A, B, C, D, E, F, G, H);
impl_belonging_to_tuple!(A, B, C, D, E, F, G, H, I);
impl_belonging_to_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_belonging_to_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_belonging_to_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
