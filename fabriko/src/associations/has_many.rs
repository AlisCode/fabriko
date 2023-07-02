use crate::factory::{Factory, FactoryContext};

use crate::associations::belongs_to::BelongingTo;
use crate::tuple_hack::AppendTuple;

#[derive(Debug, Default)]
pub struct HasMany<F>(Vec<F>);

pub trait CreateHasMany<CTX: FactoryContext, R> {
    type Output;
    fn create_has_many(self, ctx: &mut CTX, resource: &R) -> Result<Vec<Self::Output>, CTX::Error>;
}

impl<CTX, F, R> CreateHasMany<CTX, R> for HasMany<F>
where
    CTX: FactoryContext,
    F: Factory<CTX> + BelongingTo<R>,
{
    type Output = <F as Factory<CTX>>::Output;
    fn create_has_many(self, ctx: &mut CTX, resource: &R) -> Result<Vec<Self::Output>, CTX::Error> {
        self.0
            .into_iter()
            .map(|factory| factory.belonging_to(resource).create(ctx))
            .collect()
    }
}

pub struct FactoryWithResources<F, R> {
    pub factory: F,
    pub resources: R,
}

impl<CTX: FactoryContext, F, R> Factory<CTX> for FactoryWithResources<F, R>
where
    F: Factory<CTX>,
    R: BelongingTo<<F as Factory<CTX>>::Output> + Factory<CTX>,
{
    type Output = (<F as Factory<CTX>>::Output, <R as Factory<CTX>>::Output);

    fn create(self, ctx: &mut CTX) -> Result<Self::Output, <CTX as FactoryContext>::Error> {
        let FactoryWithResources { factory, resources } = self;
        let resource = factory.create(ctx)?;
        let resources = resources.belonging_to(&resource).create(ctx)?;
        Ok((resource, resources))
    }
}

impl<F, R: AppendTuple> FactoryWithResources<F, R> {
    pub fn with_resource<RR>(
        self,
        resource: RR,
    ) -> FactoryWithResources<F, <R as AppendTuple>::Output<RR>> {
        let FactoryWithResources { factory, resources } = self;
        FactoryWithResources {
            factory,
            resources: resources.append(resource),
        }
    }
}

pub trait WithRelatedResources: Sized {
    fn with_related_resources(self) -> FactoryWithResources<Self, ()> {
        FactoryWithResources {
            factory: self,
            resources: (),
        }
    }
}
