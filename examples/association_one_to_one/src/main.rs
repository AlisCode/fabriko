use city::CityId;
use country::{CountryFactory, CountryId};
use fabriko::{Fabriko, Factory, FactoryContext, WithRelatedResources};

mod city;
mod country;

#[derive(Default, Debug, Fabriko)]
#[fabriko(wrapper = "MyFabriko")]
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
    let mut cx = TestContext::default();

    let (country, (capital, cities)) = CountryFactory::default()
        .name("France".into())
        .with_related_resources(|f| f.capital_city(|c| c.name("Paris".into())))
        .create(&mut cx)
        .expect("Failed to create country");
    dbg!(country);
    dbg!(capital);
    dbg!(cities);
}
