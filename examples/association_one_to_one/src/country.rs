use fabriko::{
    BelongingTo, BuildResource, Factory, FactoryContext, HasMany, HasOneCreated, HasOneDefault,
    HasOneToCreate, WithIdentifier,
};
use nutype::nutype;

use crate::{
    city::{CityFactory, CityId},
    TestContext,
};

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
#[factory(has_one(factory = "CityFactory", name = "capital_city"))]
#[factory(has_many(factory = "CityFactory", name = "cities"))]
pub struct CountryDefinition {
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

impl<A> CountryFactoryAssociations<A, HasOneDefault<CityFactory>> {
    pub fn capital_city_id(
        self,
        city_id: CityId,
    ) -> CountryFactoryAssociations<A, HasOneCreated<CityId>> {
        let CountryFactoryAssociations {
            capital_city: _,
            cities,
        } = self;
        let capital_city = HasOneCreated::new(city_id);
        CountryFactoryAssociations {
            capital_city,
            cities,
        }
    }

    pub fn capital_city<F: FnOnce(CityFactory) -> CityFactory>(
        self,
        func: F,
    ) -> CountryFactoryAssociations<A, HasOneToCreate<CityFactory>> {
        let CountryFactoryAssociations {
            capital_city: _,
            cities,
        } = self;
        let capital_city = HasOneToCreate::new(func(Default::default()));
        CountryFactoryAssociations {
            capital_city,
            cities,
        }
    }
}

impl<B> CountryFactoryAssociations<HasMany<CityFactory>, B> {
    pub fn with_city<F: FnOnce(CityFactory) -> CityFactory>(
        mut self,
        func: F,
    ) -> CountryFactoryAssociations<HasMany<CityFactory>, B> {
        self.cities = self.cities.with(func);
        self
    }
}

impl<CTX: FactoryContext, A: Factory<CTX>, B: Factory<CTX>> Factory<CTX>
    for CountryFactoryAssociations<A, B>
{
    type Output = CountryFactoryAssociations<A::Output, B::Output>;

    fn create(self, ctx: &mut CTX) -> Result<Self::Output, <CTX as FactoryContext>::Error> {
        let CountryFactoryAssociations {
            capital_city,
            cities,
        } = self;
        let capital_city = capital_city.create(ctx)?;
        let cities = cities.create(ctx)?;
        Ok(CountryFactoryAssociations {
            capital_city,
            cities,
        })
    }
}
