use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use fabriko::{Fabriko, FactoryContext};

use crate::models::todo::{Todo, TodoFactory, TodoId};
use crate::models::todo_group::{TodoGroup, TodoGroupFactory, TodoGroupId};
use crate::models::user::{User, UserFactory, UserId};
use crate::models::user_group::{UserGroup, UserGroupFactory, UserGroupId, UserInGroup};

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
    todos: Vec<Todo>,
    seq_todo_groups: i32,
    todo_groups: Vec<TodoGroup>,
    seq_users: i32,
    users: Vec<User>,
    seq_user_groups: i32,
    user_groups: Vec<UserGroup>,
    user_in_groups: Vec<UserInGroup>,
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

    pub fn todos(&mut self) -> &mut Vec<Todo> {
        &mut self.todos
    }
}

impl FactoryContext for TestContext {
    type Error = std::convert::Infallible;
}
