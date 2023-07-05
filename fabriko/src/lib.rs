extern crate fabriko_derive;

mod associations;
mod factory;
mod mixins;
mod tuple_hack;

pub use associations::{
    belongs_to::{BelongingTo, BelongsTo, BelongsToInfo, CreateBelongingTo},
    has_many::{CreateHasMany, FactoryWithResources, HasMany, WithRelatedResources},
};
pub use fabriko_derive::{Factory, Mixin};
pub use factory::{BuildResource, Factory, FactoryContext};
pub use mixins::WithMixin;
pub use tuple_hack::AppendTuple;
