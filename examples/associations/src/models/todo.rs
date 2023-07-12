use fabriko::WithIdentifier;
use nutype::nutype;

use super::todo_group::TodoGroupId;

#[nutype]
#[derive(*)]
pub struct TodoId(i32);

#[derive(Debug)]
pub struct Todo {
    pub id: TodoId,
    pub title: String,
    pub done: bool,
    pub todo_group_id: TodoGroupId,
}

impl WithIdentifier for Todo {
    type ID = TodoId;
    fn extract_id(&self) -> Self::ID {
        self.id
    }
}
