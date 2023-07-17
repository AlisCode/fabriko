use crate::{BelongingTo, Factory, FactoryContext};

#[derive(Debug, Default)]
/// TODO: rename HasOneDefault<F>
pub struct DefaultAssociation<F>(F);

impl<CTX: FactoryContext, F: Factory<CTX>> Factory<CTX> for DefaultAssociation<F> {
    type Output = <F as Factory<CTX>>::Output;

    fn create(self, ctx: &mut CTX) -> Result<Self::Output, <CTX as FactoryContext>::Error> {
        self.0.create(ctx)
    }
}

impl<R, F: BelongingTo<R>> BelongingTo<R> for DefaultAssociation<F> {
    fn belonging_to(self, resource: &R) -> Self {
        DefaultAssociation(self.0.belonging_to(resource))
    }
}
