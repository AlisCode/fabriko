use fabriko::{BelongingTo, BuildResource, Factory, FactoryContext, WithIdentifier};
use nutype::nutype;

use crate::{city::CityFactory, TestContext};

#[nutype]
#[derive(*)]
pub struct CountryId(i32);

#[derive(Debug)]
pub struct Country {
    pub id: CountryId,
    pub name: String,
}

impl WithIdentifier for Country {
    type ID = CountryId;
    fn extract_id(&self) -> Self::ID {
        self.id
    }
}

#[derive(Factory)]
#[factory(
    factory = "CountryFactory",
    associations = "CountryFactoryAssociations"
)]
#[factory(has_one(factory = "CityFactory", name = "capital_city", link = "country"))]
#[factory(has_many(factory = "CityFactory", name = "cities", link = "country"))]
pub struct CountryDefinition {
    #[factory(into)]
    name: String,
}

impl BuildResource<TestContext> for CountryDefinition {
    type Output = Country;

    fn build_resource(
        self,
        ctx: &mut TestContext,
    ) -> Result<Self::Output, <TestContext as FactoryContext>::Error> {
        let CountryDefinition { name } = self;
        Ok(Country {
            id: ctx.next_country_id(),
            name,
        })
    }
}
