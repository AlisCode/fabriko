use fabriko::{BuildResource, Factory, FactoryBundle, FactoryContext, Mixin};
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
    #[mixin(into)]
    // There is also support for `into` inside mixins
    // Same behavior as with factories
    pub email: String,
}

impl Default for CreationMixin {
    fn default() -> Self {
        Self {
            created_at: Instant::now(),
            updated_at: Instant::now(),
            email: "dummy@test.com".into(),
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
#[factory(factory = "AccountFactory", associations = "AccountAssociations")]
pub struct AccountDefinition {
    #[factory(into, default = "S3kUr3_P@sSw0rd".into())]
    password: String,
    /// Attributes from the mixin get "magically" injected to the factory
    #[factory(mixin)]
    creation: CreationMixin,
}

impl BuildResource<TestContext> for AccountDefinition {
    type Output = Account;

    fn build_resource(
        self,
        _ctx: &mut TestContext,
    ) -> Result<Self::Output, <TestContext as FactoryContext>::Error> {
        let AccountDefinition {
            password,
            creation:
                CreationMixin {
                    created_at,
                    updated_at,
                    email,
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

// FactoryBundle also supports setting attributes defined in Mixins !
// Just import the Mixin trait in your file, and you're all set.
#[derive(Debug, FactoryBundle)]
struct MyTestSetup {
    #[bundle(factory = "AccountFactory", attributes(email = "\"alice@test.com\"", created_at = Instant::now()))]
    alice: Account,
    #[bundle(factory = "AccountFactory", attributes(email = "\"bob@test.com\""))]
    bob: Account,
}

fn main() {
    let mut cx = TestContext;
    let account: Account = AccountFactory::default()
        .password("TestPass")
        .email("my@email.com")
        .created_at(Instant::now())
        .create(&mut cx)
        .expect("Failed to create Account");
    dbg!(account);

    let MyTestSetup { alice, bob } =
        MyTestSetup::create_bundle(&mut cx).expect("Failed to create MyTestSetup");
    dbg!(alice, bob);
}
