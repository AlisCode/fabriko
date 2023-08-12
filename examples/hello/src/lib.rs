//! A "Hello world" example of how to setup and use Fabriko to define a Factory that allows us to persist
//! an instance of a Person in our application's state

use fabriko::{BuildResource, Fabriko, Factory, FactoryContext};

#[derive(Debug, Default)]
/// Our application state : An in-memory database where we persist our `Person` instances and store
/// the ID of the last instance created
pub struct AppState {
    seq_persons: i32,
    persons: Vec<Person>,
}

#[derive(Debug, Default, Fabriko)]
/// This instructs Fabriko to name our Factory wrapper `TestContextFabriko`.
/// This wrapper will be our entrypoint that we will initialize before every test.
#[fabriko(wrapper = "TestContextFabriko")]
// This instructs Fabriko to add a `person` function. This will create a `PersonFactory`
// for the user to customize, and the resource will then be persisted and returned.
#[fabriko(factory(factory = "PersonFactory", function = "person"))]
/// It is where all factories are declared.
/// It contains whatever context we want to use to persist our resources.
///
/// In this example, the context is the App's state (stored in memory), but in practice, you
/// would probably want it to be a Database Connection, or a DB Connection Pool, or a link to any service, etc...
pub struct TestContext(AppState);

impl TestContextFabriko {
    /// TODO: Should probably be derived?
    pub fn into_inner(self) -> TestContext {
        self.0.into_inner()
    }
}

impl TestContext {
    /// Consumes the TestContext to get back the state of our application
    pub fn into_app_state(self) -> AppState {
        self.0
    }

    /// Mutably borrows the wrapped state to persist data on
    pub fn state(&mut self) -> &mut AppState {
        &mut self.0
    }
}

/// `FactoryContext` is where we define that some type can be used to persist our resources.
///
/// We need to declare the Error type. This is useful in case the persisting step can fail, for example
/// if we are connecting to a database.
impl FactoryContext for TestContext {
    /// In our case, when persisting, we will simply push to a Vec.
    /// This operation should be infallible.
    type Error = std::convert::Infallible;
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// The resource that we are going to build, as it is persisted in our application's state
/// (In memory, or inside a database)
pub struct Person {
    id: i32,
    firstname: String,
    lastname: String,
    age: i32,
}

#[derive(Factory)]
// This instructs Fabriko to create a `PersonFactory` that will use the following definition
#[factory(factory = "PersonFactory")]
/// The properties of our `PersonFactory`.
/// This struct contains everything necessary to build a new instance of a `Person`.
pub struct PersonDefinition {
    // Using `into` instructs Fabriko to create a setter that transforms the type given as
    // an input to the target type defined here (String) using the trait `Into`.
    //
    // This is useful in some cases to provide better ergonomics for your factory.
    // Here for example, without `into` we would need to call either
    // `.firstname("Alice".into())` or `.firstname("Alice".to_string())`
    #[factory(into)]
    firstname: String,
    #[factory(into)]
    lastname: String,
    /// Using `default` instructs Fabriko to make the value of the field default to
    /// some known value when the field is left unspecified.
    #[factory(default = 18)]
    age: i32,
}

/// `BuildResource` is the trait implemented to define how we are persisting a resource
/// on the provided Context.
///
/// The Output type is the kind of resource that we're building.
/// The build may fail depending on how the resource is persisted on the context, therefore we return a Result here.
impl BuildResource<TestContext> for PersonDefinition {
    type Output = Person;

    fn build_resource(
        self,
        ctx: &mut TestContext,
    ) -> Result<Self::Output, <TestContext as FactoryContext>::Error> {
        // Borrow the state, add a new Person, and return that instance
        let state = ctx.state();
        state.seq_persons += 1;
        let PersonDefinition {
            firstname,
            lastname,
            age,
        } = self;
        let person = Person {
            id: state.seq_persons,
            firstname,
            lastname,
            age,
        };
        state.persons.push(person.clone());
        Ok(person)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Checks that a person can be persisted on our Application's state using our TestContext.
    fn should_create_person() {
        // This creates an `AppState` using the `default` implementation
        let mut context = TestContextFabriko::default();

        let person = context.person(|p| p.firstname("Alice").lastname("Cooper"));
        assert_eq!(
            person,
            Person {
                id: 1,
                firstname: "Alice".into(),
                lastname: "Cooper".into(),
                age: 18,
            }
        );

        // Consumes the wrapper to get the state of our application back.
        let state = context.into_inner().into_app_state();

        // We can then verify that Alice has been persisted on the application's state
        assert_eq!(state.seq_persons, person.id);
        assert_eq!(state.persons, vec![person]);
    }
}
