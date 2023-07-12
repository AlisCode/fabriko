use fabriko::{Fabriko, FactoryContext};

use crate::models::todo::TodoId;
use crate::models::todo_group::TodoGroupId;
use crate::{TodoFactory, TodoGroupFactory};

#[derive(Debug, Default, Fabriko)]
#[fabriko(wrapper = "TestContextFabriko")]
#[fabriko(factory(factory = "TodoFactory", function = "todo"))]
#[fabriko(factory(factory = "TodoGroupFactory", function = "todo_group"))]
pub struct TestContext {
    seq_todos: i32,
    seq_todo_groups: i32,
}

impl TestContext {
    pub fn next_todo_id(&mut self) -> TodoId {
        self.seq_todos += 1;
        TodoId::new(self.seq_todos)
    }

    pub fn next_todo_group_id(&mut self) -> TodoGroupId {
        self.seq_todo_groups += 1;
        TodoGroupId::new(self.seq_todo_groups)
    }
}

impl FactoryContext for TestContext {
    type Error = std::convert::Infallible;
}
