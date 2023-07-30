use crate::factory::{Factory, FactoryContext};
use crate::{BelongingTo, BelongingToLink, FactoryBelongingTo, WithIdentifier};

#[derive(Default)]
pub struct HasMany<const N: u64, F>(Vec<FactoryBelongingTo<N, F>>);

impl<const N: u64, F: Default> HasMany<N, F> {
    pub fn with<const LINK: u64, FF: FnOnce(F) -> F>(mut self, func: FF) -> Self {
        self.0.push(FactoryBelongingTo {
            factory: func(Default::default()),
        });
        self
    }
}

impl<const N: u64, CTX: FactoryContext, F: Factory<CTX>> Factory<CTX> for HasMany<N, F> {
    type Output = Vec<F::Output>;

    fn create(self, ctx: &mut CTX) -> Result<Self::Output, <CTX as FactoryContext>::Error> {
        self.0.into_iter().map(|f| f.create(ctx)).collect()
    }
}

impl<const N: u64, R: WithIdentifier, F: BelongingToLink<N, ID = <R as WithIdentifier>::ID>>
    BelongingTo<R> for HasMany<N, F>
{
    fn belonging_to(self, resource: &R) -> Self {
        let factories_belonging_to = self
            .0
            .into_iter()
            .map(|f| f.belonging_to(resource))
            .collect();
        HasMany(factories_belonging_to)
    }
}
