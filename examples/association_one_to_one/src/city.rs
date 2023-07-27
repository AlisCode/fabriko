use crate::country::{Country, CountryFactory, CountryId};
use crate::TestContext;
use fabriko::{BelongingTo, BuildResource, Factory, FactoryContext, WithIdentifier};
use nutype::nutype;

#[nutype]
#[derive(*)]
pub struct CityId(i32);

#[derive(Debug)]
pub struct City {
    pub id: CityId,
    pub name: String,
    pub population: u32,
    pub country: CountryId,
}

impl WithIdentifier for City {
    type ID = CityId;

    fn extract_id(&self) -> Self::ID {
        self.id
    }
}

/// TODO: Is this necessary ?
impl BelongingTo<Country> for CityFactory {
    fn belonging_to(mut self, resource: &Country) -> Self {
        self.country = ::fabriko::BelongsTo::Created(resource.extract_id());
        self
    }
}

#[derive(Factory)]
#[factory(associations = "CityFactoryAssociations")]
#[factory(factory = "CityFactory")]
pub struct CityDefinition {
    #[factory(into)]
    name: String,
    population: u32,
    #[factory(belongs_to(factory = "CountryFactory"))]
    country: CountryId,
}

impl BuildResource<TestContext> for CityDefinition {
    type Output = City;

    fn build_resource(
        self,
        ctx: &mut TestContext,
    ) -> Result<Self::Output, <TestContext as FactoryContext>::Error> {
        let CityDefinition {
            name,
            population,
            country,
        } = self;
        Ok(City {
            id: ctx.next_city_id(),
            name,
            population,
            country,
        })
    }
}
