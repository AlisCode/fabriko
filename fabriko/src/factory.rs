pub trait FactoryContext {
    type Error;
}

pub trait Factory<CTX: FactoryContext> {
    type Output;
    fn create(self, ctx: &mut CTX) -> Result<Self::Output, CTX::Error>;
}

pub trait BuildResource<CTX: FactoryContext> {
    type Output;
    fn build_resource(self, ctx: &mut CTX) -> Result<Self::Output, CTX::Error>;
}
