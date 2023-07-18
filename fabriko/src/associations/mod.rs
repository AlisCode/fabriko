use crate::FactoryContext;

use self::factory::FactoryWithResources;

pub mod belongs_to;
pub mod factory;
pub mod has_many;
pub mod has_one;
// pub mod may_belong_to;
pub mod with_identifier;

pub trait ResolveDependency<CTX: FactoryContext> {
    type Output;
    fn resolve_dependency(self, cx: &mut CTX) -> Result<Self::Output, CTX::Error>;
}

pub trait WithRelatedResources: Sized {
    type DefaultAssociations: Default;
    fn with_related_resources<
        ASSOCIATIONS,
        F: FnOnce(Self::DefaultAssociations) -> ASSOCIATIONS,
    >(
        self,
        func: F,
    ) -> FactoryWithResources<Self, ASSOCIATIONS> {
        FactoryWithResources {
            factory: self,
            resources: func(Default::default()),
        }
    }
}
