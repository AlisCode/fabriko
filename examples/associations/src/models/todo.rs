use std::time::Instant;

use fabriko::{BuildResource, Factory, FactoryContext, WithIdentifier};
use nutype::nutype;

use super::todo_group::{TodoGroupFactory, TodoGroupId};
use crate::{context::TestContext, mixins::EditionTimestampMixin};

#[nutype]
#[derive(*)]
pub struct TodoId(i32);

#[derive(Debug)]
pub struct Todo {
    pub id: TodoId,
    pub title: String,
    pub done: bool,
    pub todo_group_id: TodoGroupId,
    pub created_at: Instant,
    pub updated_at: Instant,
}

impl WithIdentifier for Todo {
    type ID = TodoId;
    fn extract_id(&self) -> Self::ID {
        self.id
    }
}

#[derive(Debug, Factory)]
#[factory(factory = "TodoFactory", associations = "TodoAssociations")]
pub struct TodoDefinition {
    #[factory(into, default = "\"My Todo\".to_string()")]
    title: String,
    done: bool,
    #[factory(belongs_to(factory = "TodoGroupFactory"))]
    todo_group: TodoGroupId,
    #[factory(mixin)]
    timestamps: EditionTimestampMixin,
}

impl BuildResource<TestContext> for TodoDefinition {
    type Output = Todo;

    fn build_resource(
        self,
        ctx: &mut TestContext,
    ) -> Result<Self::Output, <TestContext as FactoryContext>::Error> {
        let TodoDefinition {
            title,
            done,
            todo_group: todo_group_id,
            timestamps:
                EditionTimestampMixin {
                    created_at,
                    updated_at,
                },
        } = self;
        Ok(Todo {
            id: ctx.state().next_todo_id(),
            title,
            done,
            todo_group_id,
            created_at,
            updated_at,
        })
    }
}
