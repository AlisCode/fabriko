use crate::{BelongingTo, Factory, FactoryContext};

/// TODO: Documentation
pub struct FactoryWithResources<F, R> {
    pub factory: F,
    pub resources: R,
}

impl<CTX: FactoryContext, F, R> Factory<CTX> for FactoryWithResources<F, R>
where
    F: Factory<CTX>,
    R: Factory<CTX> + BelongingTo<<F as Factory<CTX>>::Output>,
{
    type Output = (<F as Factory<CTX>>::Output, <R as Factory<CTX>>::Output);

    fn create(self, ctx: &mut CTX) -> Result<Self::Output, <CTX as FactoryContext>::Error> {
        let FactoryWithResources { factory, resources } = self;
        let resource = factory.create(ctx)?;
        let resources = resources.belonging_to(&resource).create(ctx)?;
        Ok((resource, resources))
    }
}
