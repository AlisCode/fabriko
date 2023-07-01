use fabriko::{BuildResource, Factory, FactoryContext, Mixin};
use std::time::Instant;

struct TestContext;
impl FactoryContext for TestContext {
    type Error = ();
}

/// Here we define common attributes for factories.
///
/// For example, say in an application we often find ourselves
/// having creation and update timestamp for our resources.
///
/// We can derive Mixin for this structure, and share those
/// attributes through factories that will reference it
#[derive(Debug, Mixin)]
pub struct CreationMixin {
    pub created_at: Instant,
    pub updated_at: Instant,
}

impl Default for CreationMixin {
    fn default() -> Self {
        Self {
            created_at: Instant::now(),
            updated_at: Instant::now(),
        }
    }
}

#[derive(Debug)]
pub struct Account {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub created_at: Instant,
    pub updated_at: Instant,
}

#[derive(Debug, Factory)]
#[factory(attributes = "AccountFactoryAttributes")]
pub struct AccountFactory {
    email: String,
    password: String,
    /// Attributes from the mixin get "magically" injected to the factory
    #[factory(mixin)]
    creation: CreationMixin,
}

impl Default for AccountFactory {
    fn default() -> Self {
        Self {
            email: "dummy@dummy.com".into(),
            password: "password".into(),
            creation: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct AccountFactoryAttributes {
    pub email: String,
    pub password: String,
    pub creation: CreationMixin,
}

impl BuildResource<TestContext> for AccountFactoryAttributes {
    type Output = Account;

    fn build_resource(
        self,
        _ctx: &mut TestContext,
    ) -> Result<Self::Output, <TestContext as FactoryContext>::Error> {
        let AccountFactoryAttributes {
            email,
            password,
            creation:
                CreationMixin {
                    created_at,
                    updated_at,
                },
        } = self;
        Ok(Account {
            id: 1,
            email,
            password,
            created_at,
            updated_at,
        })
    }
}

fn main() {
    let mut cx = TestContext;
    let account: Account = AccountFactory::default()
        .password("TestPass".to_string())
        .created_at(Instant::now())
        .create(&mut cx)
        .expect("Failed to create Account");
    dbg!(account);
}
