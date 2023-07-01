use fabriko::{BuildResource, Factory, FactoryContext};

struct TestContext;
impl FactoryContext for TestContext {
    type Error = ();
}

#[derive(Debug, Default, Factory)]
#[factory(attributes = "UserFactoryAttributes")]
pub struct UserFactory {
    firstname: String,
    lastname: String,
    #[factory(dependant = format!("{firstname}.{lastname}@test.com"))]
    email: String,
}

pub struct UserFactoryAttributes {
    firstname: String,
    lastname: String,
    email: String,
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

impl BuildResource<TestContext> for UserFactoryAttributes {
    type Output = User;

    fn build_resource(self, _ctx: &mut TestContext) -> Result<Self::Output, ()> {
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

fn main() {
    let mut cx = TestContext;
    let user: User = UserFactory::default()
        .firstname("Alice".to_string())
        .lastname("Cooper".to_string())
        .create(&mut cx)
        .expect("Failed to create user");
    println!("{}", user.description());
}
