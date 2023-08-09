use std::time::Instant;

use crate::{
    context::AppState,
    models::todo::{Todo, TodoId},
};

use super::ExecuteAction;

#[derive(Debug)]
pub struct TodoChangeset {
    title: Option<String>,
    done: Option<bool>,
}

#[derive(Debug)]
pub struct UpdateTodo {
    todo_id: TodoId,
    changeset: TodoChangeset,
}

#[derive(Debug, PartialEq, Eq)]
pub enum UpdateTodoError {
    TodoDoesNotExist,
}

impl ExecuteAction<AppState> for UpdateTodo {
    type Output = Result<Todo, UpdateTodoError>;

    fn execute(self, ctx: &mut AppState) -> Self::Output {
        let todo = ctx
            .todos()
            .iter_mut()
            .find(|t| t.id == self.todo_id)
            .ok_or_else(|| UpdateTodoError::TodoDoesNotExist)?;

        let TodoChangeset { title, done } = self.changeset;

        let mut todo_has_been_updated = false;
        if let Some(title) = title {
            todo.title = title;
            todo_has_been_updated = true;
        }
        if let Some(done) = done {
            todo.done = done;
            todo_has_been_updated = true;
        }

        if todo_has_been_updated {
            todo.updated_at = Instant::now();
        }

        Ok((*todo).clone())
    }
}

#[cfg(test)]
pub mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        actions::{
            update_todo::{TodoChangeset, UpdateTodo, UpdateTodoError},
            ExecuteAction,
        },
        context::{AppState, TestContext, TestContextFabriko},
        models::todo::TodoId,
    };

    #[test]
    /// Tests that the UpdateTodo action works as expected.
    /// Note how, when testing, we do not need to explicitly create a todo_group.
    /// This action does not care about the todo group.
    fn should_update_todo() {
        let state = Rc::new(RefCell::new(AppState::default()));
        let mut f = TestContextFabriko::new(TestContext::new(state.clone()));

        let todo = f.todo(|t| t.title("My todo").done(false));

        let result = UpdateTodo {
            todo_id: todo.id,
            changeset: TodoChangeset {
                title: Some("My done todo".to_string()),
                done: Some(true),
            },
        }
        .execute(&mut state.borrow_mut());

        let updated_todo = match result {
            Ok(updated_todo) => updated_todo,
            Err(e) => panic!("Unexpected error : {e:#?}"),
        };

        assert_eq!(updated_todo.id, todo.id);
        assert_eq!(updated_todo.title, "My done todo".to_string());
        assert_eq!(updated_todo.done, true);
        assert_eq!(updated_todo.todo_group_id, todo.todo_group_id);
        assert_eq!(updated_todo.created_at, todo.created_at);
        assert_ne!(updated_todo.updated_at, todo.updated_at);
    }

    #[test]
    fn should_fail_to_update_todo_when_it_does_not_exist() {
        let state = Rc::new(RefCell::new(AppState::default()));
        let mut f = TestContextFabriko::new(TestContext::new(state.clone()));

        // Not mandatory, but let's check that we are not updating something random
        // by adding a todo in the state
        let _todo = f.todo(|t| t.title("My todo").done(false));

        let result = UpdateTodo {
            todo_id: TodoId::new(0xDEAD),
            changeset: TodoChangeset {
                title: Some("New title".to_string()),
                done: Some(true),
            },
        }
        .execute(&mut state.borrow_mut());

        assert_eq!(result, Err(UpdateTodoError::TodoDoesNotExist));
    }
}
