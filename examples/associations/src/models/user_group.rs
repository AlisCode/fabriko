use fabriko::{BuildResource, Factory, WithIdentifier};
use nutype::nutype;

use crate::context::TestContext;

use super::user::{UserFactory, UserId};

#[nutype]
#[derive(*)]
pub struct UserGroupId(i32);

#[derive(Debug, PartialEq, Eq)]
pub struct UserGroup {
    id: UserGroupId,
    name: String,
}

impl WithIdentifier for UserGroup {
    type ID = UserGroupId;

    fn extract_id(&self) -> Self::ID {
        self.id
    }
}

#[derive(Debug, Factory)]
#[factory(factory = "UserGroupFactory", associations = "UserGroupAssociations")]
#[factory(has_many(factory = "UserInGroupFactory", link = "user_group_id", name = "user"))]
pub struct UserGroupDefinition {
    #[factory(into)]
    name: String,
}

impl BuildResource<TestContext> for UserGroupDefinition {
    type Output = UserGroup;

    fn build_resource(
        self,
        ctx: &mut TestContext,
    ) -> Result<Self::Output, <TestContext as fabriko::FactoryContext>::Error> {
        let UserGroupDefinition { name } = self;
        Ok(UserGroup {
            id: ctx.state().next_user_group_id(),
            name,
        })
    }
}

#[derive(Debug)]
pub struct UserInGroup {
    pub user_id: UserId,
    pub user_group_id: UserGroupId,
}

#[derive(Debug, Factory)]
#[factory(factory = "UserInGroupFactory")]
pub struct UserInGroupDefinition {
    #[factory(belongs_to(factory = "UserFactory"))]
    user_id: UserId,
    #[factory(belongs_to(factory = "UserGroupFactory"))]
    user_group_id: UserGroupId,
}

impl BuildResource<TestContext> for UserInGroupDefinition {
    type Output = UserInGroup;

    fn build_resource(
        self,
        _ctx: &mut TestContext,
    ) -> Result<Self::Output, <TestContext as fabriko::FactoryContext>::Error> {
        let UserInGroupDefinition {
            user_id,
            user_group_id,
        } = self;
        Ok(UserInGroup {
            user_id,
            user_group_id,
        })
    }
}
