use crate::{Factory, FactoryContext, WithIdentifier};

use super::ResolveDependency;

#[derive(Debug)]
pub enum MayBelongTo<F, ID> {
    DoesNotBelongTo,
    Create(F),
    Created(ID),
}

impl<F, ID> Default for MayBelongTo<F, ID> {
    fn default() -> Self {
        MayBelongTo::DoesNotBelongTo
    }
}

impl<CTX: FactoryContext, F: Factory<CTX>, ID> ResolveDependency<CTX> for MayBelongTo<F, ID>
where
    <F as Factory<CTX>>::Output: WithIdentifier<ID = ID>,
{
    type Output = Option<ID>;

    fn resolve_dependency(
        self,
        cx: &mut CTX,
    ) -> Result<Self::Output, <CTX as FactoryContext>::Error> {
        let maybe_id = match self {
            MayBelongTo::DoesNotBelongTo => None,
            MayBelongTo::Create(factory) => {
                let id = factory.create(cx)?.extract_id();
                Some(id)
            }
            MayBelongTo::Created(id) => Some(id),
        };
        Ok(maybe_id)
    }
}
