use crate::{
    factory::{Factory, FactoryContext},
    tuple_hack::UnitTuple,
};

#[derive(Debug)]
pub enum BelongsTo<F, ID> {
    Create(F),
    Created(ID),
}

/// A helper for procedural macros
pub trait BelongsToInfo {
    type Factory;
    type ID;
}

impl<F, ID> BelongsToInfo for BelongsTo<F, ID> {
    type Factory = F;
    type ID = ID;
}

impl<F: Default, ID> Default for BelongsTo<F, ID> {
    fn default() -> Self {
        BelongsTo::Create(F::default())
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

pub trait CreateBelongingTo<CTX: FactoryContext> {
    type FactoryOutput;
    type ID;
    fn create_belonging_to<E: FnOnce(Self::FactoryOutput) -> Self::ID>(
        self,
        ctx: &mut CTX,
        extract: E,
    ) -> Result<Self::ID, CTX::Error>;
}

impl<CTX, F, ID> CreateBelongingTo<CTX> for BelongsTo<F, ID>
where
    CTX: FactoryContext,
    F: Factory<CTX>,
{
    type FactoryOutput = F::Output;
    type ID = ID;
    fn create_belonging_to<E: FnOnce(Self::FactoryOutput) -> ID>(
        self,
        ctx: &mut CTX,
        extract: E,
    ) -> Result<Self::ID, <CTX as FactoryContext>::Error> {
        match self {
            BelongsTo::Create(factory) => {
                let resource = factory.create(ctx)?;
                Ok(extract(resource))
            }
            BelongsTo::Created(id) => Ok(id),
        }
    }
}
