use city::{CityFactory, CityId};
use country::{CountryFactory, CountryId};
use fabriko::{Fabriko, FactoryContext, WithRelatedResources};

use crate::country::CountryFactoryAssociations;

mod city;
mod country;

#[derive(Default, Debug, Fabriko)]
#[fabriko(wrapper = "Factories")]
#[fabriko(factory(factory = "CountryFactory", function = "country"))]
#[fabriko(factory(factory = "CityFactory", function = "city"))]
pub struct TestContext {
    seq_city: i32,
    seq_country: i32,
}

impl TestContext {
    pub fn next_city_id(&mut self) -> CityId {
        self.seq_city += 1;
        CityId::new(self.seq_city)
    }

    pub fn next_country_id(&mut self) -> CountryId {
        self.seq_country += 1;
        CountryId::new(self.seq_country)
    }
}

impl FactoryContext for TestContext {
    type Error = std::convert::Infallible;
}

fn main() {
    let mut f = Factories::default();

    let (
        france,
        CountryFactoryAssociations {
            capital_city: paris,
            cities: french_cities,
        },
    ) = f.country(|c| {
        c.name("France".into()).with_related_resources(|rr| {
            rr.capital_city(|c| c.name("Paris".into()))
                .with_city(|c| c.name("Lyon".into()))
                .with_city(|c| c.name("Marseille".into()))
        })
    });
    dbg!(france);
    dbg!(paris);
    dbg!(french_cities);

    let barcelona = f.city(|city| {
        city.name("Barcelona".into())
            .country(|country| country.name("Spain".into()))
    });
    dbg!(barcelona);
}
