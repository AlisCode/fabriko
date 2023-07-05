use crate::FactoryContext;

pub trait FactoryBundle<CTX: FactoryContext>: Sized {
    fn create_bundle(cx: &mut CTX) -> Result<Self, CTX::Error>;
}
