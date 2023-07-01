use crate::factory::{Factory, FactoryContext};

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

/*
impl<F, ID> BelongsTo<F, ID> {
    pub fn create_belonging_to<CTX: FactoryContext, E: FnOnce(<F as Factory<CTX>>::Output) -> ID>(
        self,
        ctx: &mut CTX,
        extract: E,
    ) -> Result<ID, CTX::Error>
    where
        F: Factory<CTX>,
    {
        match self {
            BelongsTo::Create(factory) => {
                let resource = factory.create(ctx)?;
                Ok(extract(resource))
            }
            BelongsTo::Created(id) => Ok(id),
        }
    }
}
*/

pub trait CreateBelongingTo<CTX: FactoryContext> {
    type FactoryOutput;
    type ID;
    fn create_belonging_to<E: FnOnce(Self::FactoryOutput) -> Self::ID>(
        self,
        ctx: &mut CTX,
        extract: E,
    ) -> Result<Self::ID, CTX::Error>;
}

impl<CTX: FactoryContext, F: Factory<CTX>, ID> CreateBelongingTo<CTX> for BelongsTo<F, ID> {
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
