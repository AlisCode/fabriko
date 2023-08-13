mod groups;
mod todos;

pub trait ExecuteAction<CTX> {
    type Output;
    fn execute(self, ctx: &mut CTX) -> Self::Output;
}
