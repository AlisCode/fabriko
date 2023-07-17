use crate::{BelongingTo, Factory, FactoryContext, WithIdentifier};

// Two structs :
// HasOneCreated<ID>
// HasOneCreate<F>
pub enum HasOne<F, ID> {
    Create(F),
    Created(ID),
}

impl<CTX: FactoryContext, F: Factory<CTX>, ID> Factory<CTX> for HasOne<F, ID>
where
    <F as Factory<CTX>>::Output: WithIdentifier<ID = ID>,
{
    type Output = ID;

    fn create(self, cx: &mut CTX) -> Result<Self::Output, <CTX as FactoryContext>::Error> {
        let id = match self {
            HasOne::Created(id) => id,
            HasOne::Create(factory) => factory.create(cx)?.extract_id(),
        };
        Ok(id)
    }
}

impl<R, F: BelongingTo<R>, ID> BelongingTo<R> for HasOne<F, ID> {
    fn belonging_to(self, resource: &R) -> Self {
        match self {
            HasOne::Create(fac) => HasOne::Create(fac.belonging_to(resource)),
            HasOne::Created(_) => self,
        }
    }
}
