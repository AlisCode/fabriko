//! In this example, a Todo belongs to a Todo Group. This means that a Todo can not be created
//! before a TodoGroup is created because our data model does not allow that.
//!
//! Fabriko allows the user to effortlessly create resources associated with each other :
//! * By automatically declaring the "container" that this resource depends on, if relevant.
//! The default attributes will be used, but it is easy to customize the "container" if needed.
//! * By making it easy to create associated resources ("children") - e.g. create todos
//! belonging to a group
//!
//! Fabriko allows all possible flow of declaration of resources so that your test fixtures can match
//! your domain language, all the while reusing the implementation of other factories. Here are
//! some examples :

use fabriko::{
    BelongingToLink, BuildResource, Factory, FactoryBundle, FactoryContext, WithRelatedResources,
};

use context::{TestContext, TestContextFabriko};
use models::todo::Todo;
use models::todo_group::{TodoGroup, TodoGroupId};

mod context;
mod models;

#[derive(Debug, Factory)]
#[factory(factory = "TodoFactory")]
pub struct TodoDefinition {
    #[factory(into, default = "\"My Todo\".to_string()")]
    title: String,
    done: bool,
    #[factory(belongs_to(factory = "TodoGroupFactory"))]
    todo_group: TodoGroupId,
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
    // Create a testing context and the wrapper that goes with it
    let mut f = TestContextFabriko::default();

    // The user can create a container resource that will contain many resources
    let todo_group = f.todo_group(|tg| tg.title("My TodoGroup"));
    // .. and then create other resources that will be contained inside the container
    // (do_this and do_that are contained inside the group)
    let do_this = f.todo(|t| t.todo_group(todo_group.id).title("Do this"));
    let do_that = f.todo(|t| t.todo_group(todo_group.id).title("Do that"));
    dbg!(todo_group);
    dbg!(do_this);
    dbg!(do_that);

    // Alternatively, the user can create a container (group) and declare resources (todos)
    // that belongs to it. We then get access to those resources.
    let (todo_group, (todo_one, todo_two)) = f.todo_group(|tg| {
        tg.title("TG")
            .with_related_resources()
            .with_todo(|t| t.title("Todo one").done(true))
            .with_todo(|t| t.title("Todo two"))
    });
    dbg!(todo_group);
    dbg!(todo_one);
    dbg!(todo_two);

    // You can also use bundles to create the container, and the two todos belonging to it
    let MyTestBundle {
        todo_group,
        todo_one,
        todo_two,
    } = f.bundle();
    dbg!(todo_group);
    dbg!(todo_one);
    dbg!(todo_two);

    // The user can easily create a resource that needs to belong to a container (here, a todo
    // belonging to a group), **without** explicitly defining the container. This is great to keep
    // your tests concise if you don't care about the container resource.
    // The group will be created using the default arguments of the referenced factory (TodoGroupFactory)
    let todo_in_anonymous_group = f.todo(|t| t);
    dbg!(todo_in_anonymous_group);

    // The user can easily create a resource that needs to belong to a container, and customize the group it belongs to
    let todo_in_named_group =
        f.todo(|t| t.belonging_to_todo_group(|group| group.title("Named group")));
    dbg!(todo_in_named_group);
}
