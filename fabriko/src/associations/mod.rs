use crate::FactoryContext;

pub mod belongs_to;
pub mod default;
pub mod has_many;
pub mod has_one;
pub mod may_belong_to;
pub mod with_identifier;

pub trait ResolveDependency<CTX: FactoryContext> {
    type Output;
    fn resolve_dependency(self, cx: &mut CTX) -> Result<Self::Output, CTX::Error>;
}
