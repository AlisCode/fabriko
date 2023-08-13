use fabriko::{BuildResource, Factory, WithIdentifier};
use nutype::nutype;

use crate::context::TestContext;

#[nutype]
#[derive(*)]
pub struct UserId(i32);

#[derive(Debug, PartialEq, Eq, Clone, WithIdentifier)]
pub struct User {
    #[identifier]
    pub id: UserId,
    pub name: String,
}

#[derive(Debug, Factory)]
#[factory(factory = "UserFactory")]
#[factory(has_many(factory = "UserInGroupFactory", link = "user_id", name = "user_group"))]
pub struct UserDefinition {
    #[factory(into)]
    name: String,
}

impl BuildResource<TestContext> for UserDefinition {
    type Output = User;

    fn build_resource(
        self,
        ctx: &mut TestContext,
    ) -> Result<Self::Output, <TestContext as fabriko::FactoryContext>::Error> {
        let UserDefinition { name } = self;
        let user = User {
            id: ctx.state().next_user_id(),
            name,
        };
        ctx.state().users.push(user.clone());
        Ok(user)
    }
}
