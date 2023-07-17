use fabriko::{
    BelongingTo, BuildResource, DefaultAssociation, Factory, FactoryContext, HasMany, HasOne,
    WithIdentifier, WithRelatedResources,
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

// #[derive(Factory)]
// #[factory(factory = "CountryFactory")]
// #[factory(has_many(factory = "CityFactory", setter = "country", name = "cities"))]
// #[factory(has_one(factory = "CityFactory", name = "capital_city"))]
pub struct CountryDefinition {
    name: String,
}

impl WithRelatedResources for CountryFactory {
    type Associations = CountryAssociations<DefaultAssociation<CityFactory>, HasMany<CityFactory>>;
}

#[derive(Default)]
pub struct CountryFactory {
    name: String,
}

impl<CTX: ::fabriko::FactoryContext> ::fabriko::Factory<CTX> for CountryFactory
where
    CountryDefinition: ::fabriko::BuildResource<CTX>,
{
    type Output = <CountryDefinition as ::fabriko::BuildResource<CTX>>::Output;
    fn create(self, ctx: &mut CTX) -> Result<Self::Output, CTX::Error> {
        let CountryFactory { name, .. } = self;
        let __resource = CountryDefinition { name }.build_resource(ctx)?;
        Ok(__resource)
    }
}

impl CountryFactory {
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
}

#[derive(Default)]
pub struct CountryAssociations<A, B> {
    capital_city: A,
    cities: B,
}

impl<B> CountryAssociations<DefaultAssociation<CityFactory>, B> {
    pub fn capital_city_id(
        self,
        city_id: CityId,
    ) -> CountryAssociations<HasOne<CityFactory, CityId>, B> {
        let CountryAssociations {
            capital_city: _,
            cities,
        } = self;
        let capital_city = HasOne::Created(city_id) as HasOne<CityFactory, CityId>;
        CountryAssociations {
            capital_city,
            cities,
        }
    }

    pub fn capital_city<F: FnOnce(CityFactory) -> CityFactory>(
        self,
        func: F,
    ) -> CountryAssociations<HasOne<CityFactory, CityId>, B> {
        let CountryAssociations {
            capital_city: _,
            cities,
        } = self;
        let capital_city = HasOne::Create(func(Default::default())) as HasOne<CityFactory, CityId>;
        CountryAssociations {
            capital_city,
            cities,
        }
    }
}

impl<A> CountryAssociations<A, HasMany<CityFactory>> {
    pub fn with_city<F: FnOnce(CityFactory) -> CityFactory>(
        mut self,
        func: F,
    ) -> CountryAssociations<A, HasMany<CityFactory>> {
        self.cities = self.cities.with(func);
        self
    }
}

impl<R, A: BelongingTo<R>, B: BelongingTo<R>> BelongingTo<R> for CountryAssociations<A, B> {
    fn belonging_to(self, resource: &R) -> Self {
        let CountryAssociations {
            capital_city,
            cities,
        } = self;
        let capital_city = capital_city.belonging_to(resource);
        let cities = cities.belonging_to(resource);
        CountryAssociations {
            capital_city,
            cities,
        }
    }
}

impl<CTX: FactoryContext, A: Factory<CTX>, B: Factory<CTX>> Factory<CTX>
    for CountryAssociations<A, B>
{
    type Output = (A::Output, B::Output);

    fn create(self, ctx: &mut CTX) -> Result<Self::Output, <CTX as FactoryContext>::Error> {
        let CountryAssociations {
            capital_city,
            cities,
        } = self;
        let capital_city = capital_city.create(ctx)?;
        let cities = cities.create(ctx)?;
        Ok((capital_city, cities))
    }
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
