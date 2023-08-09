//! The following example demonstrates how `fabriko` handles associations, by building as
//! an example a todo app that supports grouping todos in groups, and having users access them.
//!
//! * A `todo` belongs to a `todo_group` : there is a one-to-many relationship.
//! * A `user` can belong to one or more `user_group`, and a `user_group` is
//! composed of one or more `user` : there is a many-to-many relationship (the linking table is
//! called `user_in_group`).
//! * A `user` has exactly one `user_detail` that points to it : there is a one-to-one
//! relationship (NOTE: Fabriko currently does *NOT* support one-to-one relationships where each entity
//! points to the other).
//!
//! The resources can be represented by the following diagram :
//!
//!   ┌────────────┐ ┌─────────────────────┐   ┌───────────┐
//!   │user_details│ │todos                │   │todo_groups│
//!   ├────────────┤ ├─────────────────────┤   ├───────────┤
//!   │id   integer│ │id            integer│ ┌►│id  integer│
//!   │email string│ │title          string│ │ │name string│
//! ┌─┤user_id  int│ │done          boolean│ │ └───────────┘
//! │ └────────────┘ │todo_group_id integer├─┘
//! │                └─────────────────────┘
//! │
//! │ ┌────────────┐  ┌─────────────────────┐   ┌───────────┐
//! │ │users       │  │user_in_groups       │   │user_groups│
//! │ ├────────────┤  ├─────────────────────┤   ├───────────┤
//! └►│id   integer│◄─┤user_id       integer│ ┌►│id  integer│
//!   │name  string│  │user_group_id integer├─┘ │name string│
//!   └────────────┘  └─────────────────────┘   └───────────┘
//!
//! In practice, this means that a `todo` can not be created without the `todo_group` it belongs to.
//! Same goes for a `user_detail` because it semantically belongs to a user.
//! A `user` can be created independently from a `user_group`, and vice versa.
//!
//! When writing a test, one typically wants to create a resource, the subject of the test, and
//! the related resource that go with it. E.g. when testing that a `todo` can be renamed, we would
//! create a `todo` and launch the code to rename it.
//!
//! Declaring the whole set of dependencies necessary to the test can be cumbersome and verbose,
//! leading to more code and therefore unreadable tests. Fabriko tries to adress that by
//! allowing you to create and persist these resources in a declarative way.
//!
//! Fabriko allows the user to effortlessly create resources associated with each other :
//! * By automatically declaring the "container" that this resource depends on, if relevant.
//! The default attributes will be used, but it is easy to customize the "container" if needed.
//! * By making it easy to create associated resources ("children") - e.g. create todos
//! belonging to a group
//!

use std::cell::RefCell;
use std::rc::Rc;

use fabriko::{Factory, FactoryBundle, WithRelatedResources};

use context::TestContextFabriko;
use models::todo::{Todo, TodoFactory};
use models::todo_group::{TodoGroup, TodoGroupFactory};

use crate::context::{AppState, TestContext};
use crate::models::todo_group::TodoGroupAssociations;
use crate::models::user_group::UserGroupAssociations;

mod context;
mod mixins;
mod models;

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
    let state = Rc::new(RefCell::new(AppState::default()));
    let mut f = TestContextFabriko::new(TestContext::new(state));

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
    let (todo_group, TodoGroupAssociations { todos }) = f.todo_group(|tg| {
        tg.title("TG").with_related_resources(|tg| {
            tg.with_todos(|t| t.title("Todo one").done(true))
                .with_todos(|t| t.title("Todo two"))
        })
    });
    dbg!(todo_group);
    dbg!(todos);

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

    let alice = f.user(|u| u.name("Alice"));
    let bob = f.user(|u| u.name("Bob"));
    let (group, UserGroupAssociations { user }) = f.user_group(|ug| {
        ug.name("Group name").with_related_resources(|ug| {
            ug.with_user(|uig| uig.user_id(alice.id))
                .with_user(|uig| uig.user_id(bob.id))
        })
    });
    dbg!(group, user);
}
