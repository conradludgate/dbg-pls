use crate::{DebugPls, Formatter};

macro_rules! peel {
    ($name:ident, $($other:ident,)*) => (tuple! { $($other,)* })
}

macro_rules! tuple {
    () => ();
    ( $($name:ident,)* ) => (
        impl<$($name:DebugPls),+> DebugPls for ($($name,)+) where last_type!($($name,)+): ?Sized {
            #[allow(non_snake_case, unused_assignments)]
            fn fmt(&self, f: Formatter<'_>) {
                let ($(ref $name,)+) = *self;
                f.debug_tuple()
                $(
                    .field(&$name)
                )+
                    .finish();
            }
        }
        peel! { $($name,)+ }
    )
}

macro_rules! last_type {
    ($a:ident,) => { $a };
    ($a:ident, $($rest_a:ident,)+) => { last_type!($($rest_a,)+) };
}

tuple! { T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, }

impl DebugPls for () {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_tuple().finish();
    }
}
