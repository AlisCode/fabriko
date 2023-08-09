use std::time::Instant;

use fabriko::{BuildResource, Factory, FactoryContext, WithIdentifier};
use nutype::nutype;

use crate::{context::TestContext, mixins::EditionTimestampMixin};

use super::{
    todo::TodoFactory,
    user::{UserFactory, UserId},
};

#[nutype]
#[derive(*)]
pub struct TodoGroupId(i32);

#[derive(Debug)]
pub struct TodoGroup {
    pub id: TodoGroupId,
    pub title: String,
    pub created_at: Instant,
    pub created_by: UserId,
    pub updated_at: Instant,
}

impl WithIdentifier for TodoGroup {
    type ID = TodoGroupId;
    fn extract_id(&self) -> Self::ID {
        self.id
    }
}

#[derive(Debug, Factory)]
#[factory(factory = "TodoGroupFactory", associations = "TodoGroupAssociations")]
#[factory(has_many(factory = "TodoFactory", link = "todo_group", name = "todos"))]
pub struct TodoGroupDefinition {
    #[factory(into)]
    title: String,
    #[factory(mixin)]
    timestamps: EditionTimestampMixin,
    #[factory(belongs_to(factory = "UserFactory"))]
    created_by: UserId,
}

impl BuildResource<TestContext> for TodoGroupDefinition {
    type Output = TodoGroup;

    fn build_resource(
        self,
        ctx: &mut TestContext,
    ) -> Result<Self::Output, <TestContext as FactoryContext>::Error> {
        let TodoGroupDefinition {
            title,
            timestamps:
                EditionTimestampMixin {
                    created_at,
                    updated_at,
                },
            created_by,
        } = self;
        Ok(TodoGroup {
            id: ctx.state().next_todo_group_id(),
            title,
            created_at,
            created_by,
            updated_at,
        })
    }
}
