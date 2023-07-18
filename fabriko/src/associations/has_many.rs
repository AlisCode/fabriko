use crate::factory::{Factory, FactoryContext};
use crate::BelongingTo;

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
