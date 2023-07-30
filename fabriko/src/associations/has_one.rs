use crate::{
    BelongingTo, BelongingToLink, Factory, FactoryBelongingTo, FactoryContext, WithIdentifier,
};

#[derive(Debug, Default)]
pub struct HasOneDefault<F>(F);

impl<CTX: FactoryContext, F: Factory<CTX>> Factory<CTX> for HasOneDefault<F> {
    type Output = <F as Factory<CTX>>::Output;

    fn create(self, ctx: &mut CTX) -> Result<Self::Output, <CTX as FactoryContext>::Error> {
        self.0.create(ctx)
    }
}

impl<R, F: BelongingTo<R>> BelongingTo<R> for HasOneDefault<F> {
    fn belonging_to(self, resource: &R) -> Self {
        HasOneDefault(self.0.belonging_to(resource))
    }
}

#[derive(Debug)]
pub struct HasOneCreated<ID>(ID);

impl<ID> HasOneCreated<ID> {
    pub fn new(id: ID) -> Self {
        HasOneCreated(id)
    }
}

impl<CTX: FactoryContext, ID> Factory<CTX> for HasOneCreated<ID> {
    type Output = ID;

    fn create(self, _ctx: &mut CTX) -> Result<Self::Output, <CTX as FactoryContext>::Error> {
        Ok(self.0)
    }
}

impl<R, ID> BelongingTo<R> for HasOneCreated<ID> {
    fn belonging_to(self, _resource: &R) -> Self {
        self
    }
}

#[derive(Debug)]
pub struct HasOneToCreate<const N: u64, F>(FactoryBelongingTo<N, F>);

impl<const N: u64, F> HasOneToCreate<N, F> {
    pub fn new(factory: F) -> Self {
        HasOneToCreate(FactoryBelongingTo { factory })
    }
}

impl<const N: u64, CTX: FactoryContext, F: Factory<CTX>> Factory<CTX> for HasOneToCreate<N, F>
where
    <F as Factory<CTX>>::Output: WithIdentifier,
{
    type Output = <<F as Factory<CTX>>::Output as WithIdentifier>::ID;

    fn create(self, cx: &mut CTX) -> Result<Self::Output, <CTX as FactoryContext>::Error> {
        self.0.create(cx).map(|resource| resource.extract_id())
    }
}

impl<const N: u64, R: WithIdentifier, F: BelongingToLink<N, ID = <R as WithIdentifier>::ID>>
    BelongingTo<R> for HasOneToCreate<N, F>
{
    fn belonging_to(mut self, resource: &R) -> Self {
        self.0 = self.0.belonging_to(resource);
        self
    }
}
