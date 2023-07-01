pub trait WithMixin<M> {
    fn with_mixin<F: FnOnce(M) -> M>(self, f: F) -> Self;
}
