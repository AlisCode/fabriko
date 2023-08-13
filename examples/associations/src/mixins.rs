use fabriko::Mixin;
use std::time::Instant;

#[derive(Debug, Mixin)]
pub struct EditionTimestampMixin {
    pub created_at: Instant,
    pub updated_at: Instant,
}

impl Default for EditionTimestampMixin {
    fn default() -> Self {
        let now = Instant::now();
        EditionTimestampMixin {
            created_at: now,
            updated_at: now,
        }
    }
}
