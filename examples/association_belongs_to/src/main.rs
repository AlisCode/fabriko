use fabriko::{BelongsTo, BuildResource, Factory, FactoryContext, HasMany};

#[derive(Debug, Default)]
pub struct TestContext {
    seq_todos: i32,
    seq_todo_groups: i32,
}

impl TestContext {
    pub fn next_todo_id(&mut self) -> i32 {
        self.seq_todos += 1;
        self.seq_todos
    }

    pub fn next_todo_group_id(&mut self) -> i32 {
        self.seq_todo_groups += 1;
        self.seq_todo_groups
    }
}

impl FactoryContext for TestContext {
    type Error = ();
}

#[derive(Debug)]
#[allow(dead_code)] // Because we're not doing anything with those models
pub struct Todo {
    id: i32,
    title: String,
    done: bool,
    todo_group_id: i32,
}

#[derive(Debug)]
#[allow(dead_code)] // Because we're not doing anything with those models
pub struct TodoGroup {
    id: i32,
    title: String,
}

#[derive(Default, Factory)]
#[factory(attributes = "TodoFactoryAttributes")]
pub struct TodoFactory {
    title: String,
    done: bool,
    #[factory(belongs_to(ty = "TodoGroup", field = "id", id_ty = "i32"))]
    todo_group: BelongsTo<TodoGroupFactory, i32>,
}

pub struct TodoFactoryAttributes {
    title: String,
    done: bool,
    todo_group: i32,
}

impl BuildResource<TestContext> for TodoFactoryAttributes {
    type Output = Todo;

    fn build_resource(
        self,
        ctx: &mut TestContext,
    ) -> Result<Self::Output, <TestContext as fabriko::FactoryContext>::Error> {
        let TodoFactoryAttributes {
            title,
            done,
            todo_group: todo_group_id,
        } = self;
        Ok(Todo {
            id: ctx.next_todo_id(),
            title,
            done,
            todo_group_id,
        })
    }
}

#[derive(Default, Factory)]
#[factory(attributes = "TodoGroupFactoryAttributes")]
pub struct TodoGroupFactory {
    title: String,
    #[factory(has_many(extract_ty = "TodoGroup", extract = "id", inject = "todo_group"))]
    todo: HasMany<TodoFactory>,
}

pub struct TodoGroupFactoryAttributes {
    title: String,
}

impl BuildResource<TestContext> for TodoGroupFactoryAttributes {
    type Output = TodoGroup;

    fn build_resource(
        self,
        ctx: &mut TestContext,
    ) -> Result<Self::Output, <TestContext as FactoryContext>::Error> {
        let TodoGroupFactoryAttributes { title } = self;
        Ok(TodoGroup {
            id: ctx.next_todo_group_id(),
            title,
        })
    }
}

fn main() {
    let mut cx = TestContext::default();

    let todo_in_anonymous_group = TodoFactory::default()
        .belonging_to_todo_group(|tg| tg.title("Group 1".to_string()))
        .title("My todo".to_string())
        .create(&mut cx)
        .expect("Failed to create a todo contained in an anonymous group");
    dbg!(todo_in_anonymous_group);

    let todo_group = TodoGroupFactory::default()
        .title("Group 2".to_string())
        .create(&mut cx)
        .expect("Failed to create a todo group");
    let do_this = TodoFactory::default()
        .todo_group(todo_group.id)
        .title("Do this".to_string())
        .create(&mut cx)
        .expect("Failed to create a todo in the todo group");
    let do_that = TodoFactory::default()
        .todo_group(todo_group.id)
        .title("Do that".to_string())
        .create(&mut cx)
        .expect("Failed to create a todo in the todo group");
    dbg!(todo_group);
    dbg!(do_this);
    dbg!(do_that);
}
