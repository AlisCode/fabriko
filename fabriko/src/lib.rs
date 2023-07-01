extern crate fabriko_derive;

mod associations;
mod factory;
mod mixins;

pub use associations::{
    belongs_to::{BelongsTo, BelongsToInfo, CreateBelongingTo},
    has_many::{CreateHasMany, HasMany},
};
pub use fabriko_derive::{Factory, Mixin};
pub use factory::{BuildResource, Factory, FactoryContext};
pub use mixins::WithMixin;

/*
#[derive(Debug)]
struct TitleMixin {
    title: String,
}

impl Default for TitleMixin {
    fn default() -> Self {
        Self {
            title: "Some title".to_string(),
        }
    }
}

// DERIVED
trait TitleMixinMixin {
    fn title(self, title: String) -> Self;
}

// DERIVED
impl TitleMixinMixin for TitleMixin {
    fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }
}

// DERIVED
impl<T: WithMixin<TitleMixin>> TitleMixinMixin for T {
    fn title(self, title: String) -> Self {
        self.with_mixin(|mixin| mixin.title(title))
    }
}

#[derive(Debug, Default /* Factory */)]
//#[factory(attributes = "TodoFactoryAttributes")]
pub struct TodoFactory {
    //#[factory(mixin)]
    title: TitleMixin,
    content: Option<String>,
    //#[factory(belongs_to = "TodoGroup.id")]
    todo_group_id: BelongsTo<TodoGroupFactory, i32>,
    done: bool,
}

impl<CTX: FactoryContext> Factory<CTX> for TodoFactory
where
    TodoFactoryAttributes: BuildResource<CTX>,
    TodoGroupFactory: Factory<CTX, Output = TodoGroup>,
{
    type Output = <TodoFactoryAttributes as BuildResource<CTX>>::Output;

    fn create(self, ctx: &mut CTX) -> Result<Self::Output, CTX::Error> {
        let TodoFactory {
            title,
            content,
            todo_group_id,
            done,
        } = self;

        // Associations (pre-create)
        let todo_group_id = todo_group_id.create::<CTX, _>(ctx, |x| x.id)?;

        // Dependant attributes (pre-create)

        // Build resource
        let resource = TodoFactoryAttributes {
            title,
            content,
            todo_group_id,
            done,
        }
        .build_resource(ctx)?;

        // Associations (post-create)

        Ok(resource)
    }
}

impl WithMixin<TitleMixin> for TodoFactory {
    fn with_mixin<F: FnOnce(TitleMixin) -> TitleMixin>(mut self, f: F) -> Self {
        self.title = f(self.title);
        self
    }
}

pub struct TodoFactoryAttributes {
    title: TitleMixin,
    content: Option<String>,
    todo_group_id: i32,
    done: bool,
}

impl BuildResource<()> for TodoFactoryAttributes {
    type Output = Todo;

    fn build_resource(
        self,
        _ctx: &mut (),
    ) -> Result<Self::Output, <() as factory::FactoryContext>::Error> {
        Ok(Todo {
            id: 1, // extracted from context
            title: self.title.title.clone(),
            content: self.content.clone(),
            done: self.done,
            todo_group_id: self.todo_group_id,
        })
    }
}

pub struct Todo {
    pub id: i32,
    pub title: String,
    pub content: Option<String>,
    pub done: bool,
    pub todo_group_id: i32,
}

#[derive(Debug, Default /* Factory */)]
// #[factory(attributes = "TodoGroupFactoryAttributes")]
pub struct TodoGroupFactory {
    //#[factory(mixin)]
    title: TitleMixin,
    //#[has_many(extract = "Todo.id", inject = "todo_group")]
    todos: HasMany<TodoFactory>,
}

pub struct TodoGroupFactoryAttributes {
    title: TitleMixin,
}

pub struct TodoGroup {
    pub id: i32,
    pub title: String,
}

impl BuildResource<()> for TodoGroupFactoryAttributes {
    type Output = TodoGroup;

    fn build_resource(self, _ctx: &mut ()) -> Result<Self::Output, <() as FactoryContext>::Error> {
        Ok(TodoGroup {
            id: 1, // Extract from CTX
            title: self.title.title,
        })
    }
}

// DERIVED
impl WithMixin<TitleMixin> for TodoGroupFactory {
    fn with_mixin<F: FnOnce(TitleMixin) -> TitleMixin>(mut self, f: F) -> Self {
        self.title = f(self.title);
        self
    }
}

// DERIVED
impl<CTX: FactoryContext> Factory<CTX> for TodoGroupFactory
where
    TodoGroupFactoryAttributes: BuildResource<CTX>,
    TodoFactory: Factory<CTX>,
{
    type Output = <TodoGroupFactoryAttributes as BuildResource<CTX>>::Output;

    fn create(self, ctx: &mut CTX) -> Result<Self::Output, <CTX as FactoryContext>::Error> {
        let TodoGroupFactory { title, todos } = self;
        // Associations (pre-create)
        // Dependant attributes (pre-create)

        // Build resource
        let resource = TodoGroupFactoryAttributes { title }.build_resource(ctx)?;

        // Associations (post-create)
        let _ = todos.create(ctx)?;

        Ok(resource)
    }
}

#[derive(Debug, Default /* Factory */)]
// #[factory(attributes = "UserFactoryAttributes")]
pub struct UserFactory {
    pub firstname: String,
    pub lastname: String,
    //#[factory(dependant = "format!("{firstname}.{lastname}@test.com")")]
    pub email: String,
}

pub struct UserFactoryAttributes {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
}

pub struct User {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
}

impl BuildResource<()> for UserFactoryAttributes {
    type Output = User;

    fn build_resource(self, _ctx: &mut ()) -> Result<Self::Output, <() as FactoryContext>::Error> {
        let UserFactoryAttributes {
            firstname,
            lastname,
            email,
        } = self;
        Ok(User {
            firstname,
            lastname,
            email,
        })
    }
}

// DERIVED
#[allow(unused_variables)]
impl Factory<()> for UserFactory {
    type Output = User;

    fn create(self, ctx: &mut ()) -> Result<Self::Output, <() as FactoryContext>::Error> {
        let UserFactory {
            firstname,
            lastname,
            email,
        } = self;

        // Associations (pre-create)

        // Dependant attributes (pre-create)
        let email = format!("{firstname}.{lastname}@test.com");

        // Build resource
        let resource = UserFactoryAttributes {
            firstname,
            lastname,
            email,
        }
        .build_resource(ctx)?;

        // Associations (post-create)

        Ok(resource)
    }
}
*/
