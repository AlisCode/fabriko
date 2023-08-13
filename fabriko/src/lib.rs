extern crate fabriko_derive;

mod associations;
mod bundle;
mod context;
mod factory;
mod mixins;
mod tuple_hack;

pub use associations::{
    belongs_to::{BelongingTo, BelongingToLink, BelongsTo, FactoryBelongingTo},
    factory::FactoryWithResources,
    has_many::HasMany,
    has_one::{HasOneCreated, HasOneDefault, HasOneToCreate},
    with_identifier::WithIdentifier,
    ResolveDependency, WithRelatedResources,
};
pub use bundle::FactoryBundle;
pub use context::Fabriko;
pub use fabriko_derive::{Fabriko, Factory, FactoryBundle, Mixin, WithIdentifier};
pub use factory::{BuildResource, Factory, FactoryContext};
pub use mixins::WithMixin;
pub use tuple_hack::AppendTuple;

pub type FactorySetter<F, T> = fn(F, T) -> F;
