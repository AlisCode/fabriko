use fabriko::{BuildResource, Factory, FactoryContext};

struct TestContext;
impl FactoryContext for TestContext {
    type Error = ();
}

#[derive(Debug)]
pub struct User {
    firstname: String,
    lastname: String,
    email: String,
}

impl User {
    fn description(&self) -> String {
        let User {
            firstname,
            lastname,
            email,
        } = self;
        format!("{firstname} {lastname} <{email}>")
    }
}

#[derive(Debug, Factory)]
#[factory(factory = "UserFactory")]
pub struct UserDefinition {
    #[factory(into, default = "\"Alice\".into()")]
    firstname: String,
    #[factory(into, default = "\"Cooper\".into()")]
    lastname: String,
    #[factory(into, dependant = format!("{firstname}.{lastname}@test.com"))]
    email: String,
}

impl BuildResource<TestContext> for UserDefinition {
    type Output = User;

    fn build_resource(self, _ctx: &mut TestContext) -> Result<Self::Output, ()> {
        let UserDefinition {
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

fn main() {
    let mut cx = TestContext;
    let alice = UserFactory::default()
        .create(&mut cx)
        .expect("Failed to create alice");
    let alice_description = alice.description();
    println!("{alice_description}");
    assert_eq!(alice_description, "Alice Cooper <Alice.Cooper@test.com>");

    let bob: User = UserFactory::default()
        .firstname("Bob")
        .lastname("Marley")
        .create(&mut cx)
        .expect("Failed to create bob");
    let bob_description = bob.description();
    println!("{bob_description}");
    assert_eq!(bob.description(), "Bob Marley <Bob.Marley@test.com>");
}
