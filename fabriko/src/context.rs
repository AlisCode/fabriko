use crate::{Factory, FactoryBundle, FactoryContext};

#[derive(Debug, Default)]
pub struct Fabriko<CTX>(CTX);

impl<CTX: FactoryContext> Fabriko<CTX>
where
    CTX::Error: std::error::Error,
{
    pub fn into_inner(self) -> CTX {
        self.0
    }

    pub fn new(ctx: CTX) -> Self {
        Fabriko(ctx)
    }

    pub fn bundle<B: FactoryBundle<CTX>>(&mut self) -> B {
        match B::create_bundle(&mut self.0) {
            Ok(bundle) => bundle,
            Err(err) => panic!(
                "Failed to create bundle {} : {}",
                std::any::type_name::<B>(),
                err
            ),
        }
    }

    pub fn try_bundle<B: FactoryBundle<CTX>>(&mut self) -> Result<B, CTX::Error> {
        B::create_bundle(&mut self.0)
    }

    pub fn factory<F: Default, FF: Factory<CTX>, CB: FnOnce(F) -> FF>(
        &mut self,
        define_factory: CB,
    ) -> <FF as Factory<CTX>>::Output {
        match define_factory(F::default()).create(&mut self.0) {
            Ok(resource) => resource,
            Err(err) => {
                panic!(
                    "Failed to create resource {} : {}",
                    std::any::type_name::<<FF as Factory<CTX>>::Output>(),
                    err
                )
            }
        }
    }

    pub fn try_factory<F: Default, FF: Factory<CTX>, CB: FnOnce(F) -> FF>(
        &mut self,
        define_factory: CB,
    ) -> Result<<FF as Factory<CTX>>::Output, CTX::Error> {
        define_factory(F::default()).create(&mut self.0)
    }
}
