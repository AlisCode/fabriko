use darling::FromMeta;
use syn::{Ident, Path};

#[derive(FromMeta)]
/// TODO: Document
pub(crate) struct HasOneAssociation {
    #[darling(rename = "factory")]
    pub(crate) for_factory: Path,
    pub(crate) name: Ident,
    // pub(crate) setter: Ident,
}
