//! A workaround to have a tuple and append types inside of it while avoiding nesting

pub struct UnitTuple<T>(pub(crate) T);

pub trait AppendTuple {
    type Output<T>;
    fn append<T>(self, add: T) -> Self::Output<T>;
}

impl AppendTuple for () {
    type Output<T> = UnitTuple<T>;
    fn append<T>(self, add: T) -> Self::Output<T> {
        UnitTuple(add)
    }
}

impl<A> AppendTuple for UnitTuple<A> {
    type Output<T> = (A, T);
    fn append<T>(self, add: T) -> Self::Output<T> {
        (self.0, add)
    }
}

macro_rules! impl_append_tuple {
    ($($T:ident),*) => {
        impl<$($T),*> AppendTuple for ($($T),*) {
            type Output<U> = ($($T),*, U);
            #[allow(non_snake_case)]
            fn append<U>(self, add: U) -> Self::Output<U> {
                let ($($T),*) = self;
                ($($T),*, add)
            }
        }
    };
}

impl_append_tuple!(A, B);
impl_append_tuple!(A, B, C);
impl_append_tuple!(A, B, C, D);
impl_append_tuple!(A, B, C, D, E);
impl_append_tuple!(A, B, C, D, E, F);
impl_append_tuple!(A, B, C, D, E, F, G);
impl_append_tuple!(A, B, C, D, E, F, G, H);
impl_append_tuple!(A, B, C, D, E, F, G, H, I);
impl_append_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_append_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_append_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
