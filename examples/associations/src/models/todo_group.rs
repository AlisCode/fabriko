use fabriko::WithIdentifier;
use nutype::nutype;

#[nutype]
#[derive(*)]
pub struct TodoGroupId(i32);

#[derive(Debug)]
pub struct TodoGroup {
    pub id: TodoGroupId,
    pub title: String,
}

impl WithIdentifier for TodoGroup {
    type ID = TodoGroupId;
    fn extract_id(&self) -> Self::ID {
        self.id
    }
}
