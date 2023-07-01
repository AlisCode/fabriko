use crate::factory::{Factory, FactoryContext};

#[derive(Debug, Default)]
pub struct HasMany<F>(Vec<F>);

impl<F> HasMany<F> {
    pub fn create<CTX: FactoryContext, FUNC: Fn(F) -> F>(
        self,
        ctx: &mut CTX,
        f: FUNC,
    ) -> Result<Vec<<F as Factory<CTX>>::Output>, CTX::Error>
    where
        F: Factory<CTX>,
    {
        self.0
            .into_iter()
            .map(|factory| f(factory).create(ctx))
            .collect()
    }
}

pub trait CreateHasMany<CTX: FactoryContext> {
    type Factory;
    type FactoryOutput;
    fn create_has_many<FUNC: Fn(Self::Factory) -> Self::Factory>(
        self,
        ctx: &mut CTX,
        before_creation: FUNC,
    ) -> Result<Vec<Self::FactoryOutput>, CTX::Error>;
}

impl<CTX: FactoryContext, F: Factory<CTX>> CreateHasMany<CTX> for HasMany<F> {
    type Factory = F;
    type FactoryOutput = <F as Factory<CTX>>::Output;

    fn create_has_many<FUNC: Fn(Self::Factory) -> Self::Factory>(
        self,
        ctx: &mut CTX,
        before_creation: FUNC,
    ) -> Result<Vec<Self::FactoryOutput>, CTX::Error> {
        self.0
            .into_iter()
            .map(|factory| before_creation(factory).create(ctx))
            .collect()
    }
}
