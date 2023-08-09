use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use fabriko::{Fabriko, FactoryContext};

use crate::models::todo::{TodoFactory, TodoId};
use crate::models::todo_group::{TodoGroupFactory, TodoGroupId};
use crate::models::user::{UserFactory, UserId};
use crate::models::user_group::{UserGroupFactory, UserGroupId};

#[derive(Debug, Default, Fabriko)]
#[fabriko(wrapper = "TestContextFabriko")]
#[fabriko(factory(factory = "TodoFactory", function = "todo"))]
#[fabriko(factory(factory = "TodoGroupFactory", function = "todo_group"))]
#[fabriko(factory(factory = "UserFactory", function = "user"))]
#[fabriko(factory(factory = "UserGroupFactory", function = "user_group"))]
pub struct TestContext(Rc<RefCell<AppState>>);

impl TestContext {
    pub fn new(state: Rc<RefCell<AppState>>) -> Self {
        Self(state)
    }
    pub fn state(&mut self) -> RefMut<AppState> {
        self.0.borrow_mut()
    }
}

#[derive(Debug, Default)]
pub struct AppState {
    seq_todos: i32,
    seq_todo_groups: i32,
    seq_users: i32,
    seq_user_groups: i32,
}

impl AppState {
    pub fn next_todo_id(&mut self) -> TodoId {
        self.seq_todos += 1;
        TodoId::new(self.seq_todos)
    }

    pub fn next_todo_group_id(&mut self) -> TodoGroupId {
        self.seq_todo_groups += 1;
        TodoGroupId::new(self.seq_todo_groups)
    }

    pub fn next_user_id(&mut self) -> UserId {
        self.seq_users += 1;
        UserId::new(self.seq_users)
    }

    pub fn next_user_group_id(&mut self) -> UserGroupId {
        self.seq_user_groups += 1;
        UserGroupId::new(self.seq_user_groups)
    }
}

impl FactoryContext for TestContext {
    type Error = std::convert::Infallible;
}
