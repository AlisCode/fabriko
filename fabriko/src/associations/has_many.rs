use crate::factory::{Factory, FactoryContext};
use crate::BelongingTo;

use crate::tuple_hack::AppendTuple;

#[derive(Default)]
pub struct HasMany<F>(Vec<F>);

impl<F: Default> HasMany<F> {
    pub fn with<FF: FnOnce(F) -> F>(mut self, func: FF) -> Self {
        self.0.push(func(Default::default()));
        self
    }
}

impl<CTX: FactoryContext, F: Factory<CTX>> Factory<CTX> for HasMany<F> {
    type Output = Vec<F::Output>;

    fn create(self, ctx: &mut CTX) -> Result<Self::Output, <CTX as FactoryContext>::Error> {
        self.0.into_iter().map(|f| f.create(ctx)).collect()
    }
}

impl<R, F: BelongingTo<R>> BelongingTo<R> for HasMany<F> {
    fn belonging_to(self, resource: &R) -> Self {
        let factories_belonging_to = self
            .0
            .into_iter()
            .map(|f| f.belonging_to(resource))
            .collect();
        HasMany(factories_belonging_to)
    }
}

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
    type Associations: Default;
    fn with_related_resources<ASSOCIATIONS, F: FnOnce(Self::Associations) -> ASSOCIATIONS>(
        self,
        func: F,
    ) -> FactoryWithResources<Self, ASSOCIATIONS> {
        FactoryWithResources {
            factory: self,
            resources: func(Default::default()),
        }
    }
}
