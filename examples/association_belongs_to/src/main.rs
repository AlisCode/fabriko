use fabriko::{
    BelongingToLink, BuildResource, Factory, FactoryBundle, FactoryContext, WithIdentifier,
    WithRelatedResources,
};

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

impl WithIdentifier for Todo {
    type ID = i32;
    fn extract_id(&self) -> Self::ID {
        self.id
    }
}

#[derive(Debug, Factory)]
#[factory(factory = "TodoFactory")]
pub struct TodoDefinition {
    #[factory(into, default = "\"My Todo\".to_string()")]
    title: String,
    done: bool,
    #[factory(belongs_to(factory = "TodoGroupFactory"))]
    todo_group: i32,
}

impl BuildResource<TestContext> for TodoDefinition {
    type Output = Todo;

    fn build_resource(
        self,
        ctx: &mut TestContext,
    ) -> Result<Self::Output, <TestContext as fabriko::FactoryContext>::Error> {
        let TodoDefinition {
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

#[derive(Debug)]
#[allow(dead_code)] // Because we're not doing anything with those models
pub struct TodoGroup {
    id: i32,
    title: String,
}

impl WithIdentifier for TodoGroup {
    type ID = i32;
    fn extract_id(&self) -> Self::ID {
        self.id
    }
}

#[derive(Debug, Factory)]
#[factory(factory = "TodoGroupFactory")]
#[factory(has_many(factory = "TodoFactory", setter = "todo_group", name = "todo"))]
pub struct TodoGroupDefinition {
    #[factory(into)]
    title: String,
}

impl BuildResource<TestContext> for TodoGroupDefinition {
    type Output = TodoGroup;

    fn build_resource(
        self,
        ctx: &mut TestContext,
    ) -> Result<Self::Output, <TestContext as FactoryContext>::Error> {
        let TodoGroupDefinition { title } = self;
        Ok(TodoGroup {
            id: ctx.next_todo_group_id(),
            title,
        })
    }
}

// When you want to share the same test setup between various tests, you can create
// your own FactoryBundle to reduce the boilerplate to a bare minimum.
//
// See the example below.
#[derive(Debug, FactoryBundle)]
pub struct MyTestBundle {
    #[bundle(factory = "TodoGroupFactory", attributes(title = "\"Todo Group\""))]
    todo_group: TodoGroup,
    #[bundle(
        factory = "TodoFactory",
        attributes(title = "\"Todo One\"", todo_group = "todo_group.id")
    )]
    todo_one: Todo,
    #[bundle(
        factory = "TodoFactory",
        attributes(title = "\"Todo Two\"", todo_group = "todo_group.id")
    )]
    todo_two: Todo,
}

fn main() {
    let mut cx = TestContext::default();

    // In this example, a Todo belongs to a Todo Group. This means that a Todo can not be created
    // before a TodoGroup is created because our data model does not allow that.

    // Fabriko allows the user to effortlessly create resources associated with each other :
    // * By automatically declaring the "container" that this resource depends on, if relevant.
    // The default attributes will be used, but it is easy to customize the "container" if needed.
    // * By making it easy to create associated resources ("children") - e.g. create todos
    // belonging to a group

    // Fabriko allows all possible flow of declaration of resources so that your test fixtures can match
    // your domain language, all the while reusing the implementation of other factories. Here are
    // some examples :

    // The user can create a container resource that has many resources (a group, here)
    let todo_group = TodoGroupFactory::default()
        .title("My TodoGroup")
        .create(&mut cx)
        .expect("Failed to create a todo group");
    // .. and then create other resources that will be contained inside the container
    // (do_this and do_that are contained inside the group)
    let do_this = TodoFactory::default()
        .todo_group(todo_group.id)
        .title("Do this")
        .create(&mut cx)
        .expect("Failed to create a todo in the todo group");
    let do_that = TodoFactory::default()
        .todo_group(todo_group.id)
        .title("Do that")
        .create(&mut cx)
        .expect("Failed to create a todo in the todo group");
    dbg!(todo_group);
    dbg!(do_this);
    dbg!(do_that);

    // Alternatively, the user can create a container (group) and declare resources (todos)
    // that belongs to it. We then get access to those resources.
    let (tg, (todo_one, todo_two)) = TodoGroupFactory::default()
        .title("The TodoGroup".to_string())
        .with_related_resources()
        .with_todo(|todo| todo.title("Todo one"))
        .with_todo(|todo| todo.title("Todo two"))
        .create(&mut cx)
        .expect("Failed to create tg and todo_in_tg");
    dbg!(tg);
    dbg!(todo_one);
    dbg!(todo_two);

    // You can also use bundles to create the container, and the two todos belonging to it
    let MyTestBundle {
        todo_group,
        todo_one,
        todo_two,
    } = MyTestBundle::create_bundle(&mut cx).expect("Failed to create test setup");
    dbg!(todo_group);
    dbg!(todo_one);
    dbg!(todo_two);

    // The user can easily create a resource that needs to belong to a container (here, a todo
    // belonging to a group), **without** explicitly defining the container. This is great to keep
    // your tests concise if you don't care about the container resource.
    // The group will be created using the default arguments of the referenced factory (TodoGroupFactory)
    let todo_in_anonymous_group = TodoFactory::default()
        .title("Todo in anonymous group")
        .create(&mut cx)
        .expect("Failed to create a todo contained in an anonymous group");
    dbg!(todo_in_anonymous_group);

    // The user can easily create a resource that needs to belong to a container, and customize the group it belongs to
    let todo_in_named_group = TodoFactory::default()
        .belonging_to_todo_group(|tg| tg.title("Named Group"))
        .title("My todo")
        .create(&mut cx)
        .expect("Failed to create a todo contained in a named group");
    dbg!(todo_in_named_group);
}
