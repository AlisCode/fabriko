use crate::country::{Country, CountryFactory, CountryId};
use crate::TestContext;
use fabriko::{
    BelongingTo, BelongsTo, BuildResource, FactoryContext, ResolveDependency, WithIdentifier,
};
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

// #[derive(Factory)]
// #[factory(factory = "CityFactory")]
pub struct CityDefinition {
    // #[factory(into)]
    name: String,
    population: u32,
    // #[factory(belongs_to(factory = "CountryFactory"))]
    country: CountryId,
}

#[derive(Default)]
pub struct CityFactory {
    name: String,
    population: u32,
    country: ::fabriko::BelongsTo<CountryFactory, CountryId>,
}

impl<CTX: ::fabriko::FactoryContext> ::fabriko::Factory<CTX> for CityFactory
where
    CityDefinition: ::fabriko::BuildResource<CTX>,
    BelongsTo<CountryFactory, CountryId>: ResolveDependency<CTX, Output = CountryId>,
{
    type Output = <CityDefinition as ::fabriko::BuildResource<CTX>>::Output;
    fn create(self, ctx: &mut CTX) -> Result<Self::Output, CTX::Error> {
        let CityFactory {
            name,
            population,
            country,
            ..
        } = self;
        let country = country.resolve_dependency(ctx)?;
        let __resource = CityDefinition {
            name,
            population,
            country,
        }
        .build_resource(ctx)?;
        Ok(__resource)
    }
}
impl CityFactory {
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
    pub fn population(mut self, population: u32) -> Self {
        self.population = population;
        self
    }
    pub fn country<F: FnOnce(CountryFactory) -> CountryFactory>(mut self, func: F) -> Self {
        self.country = BelongsTo::Create(func(CountryFactory::default()));
        self
    }
    pub fn country_id(mut self, country: CountryId) -> Self {
        self.country = BelongsTo::Created(country);
        self
    }
}

impl BelongingTo<Country> for CityFactory {
    fn belonging_to(mut self, resource: &Country) -> Self {
        self.country = BelongsTo::Created(resource.extract_id());
        self
    }
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
