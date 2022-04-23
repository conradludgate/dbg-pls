use syn::token::{Bracket, Colon, Colon2, Comma, Dot, Eq, Let, Paren, Semi, Sub};

use crate::{DebugPls, Formatter};

macro_rules! debug_units {
    ($($T:ident),*) => {$(
        impl DebugPls for $T {
            fn fmt(&self, f: Formatter<'_>) {
                f.debug_ident(stringify!($T))
            }
        }
    )*};
}

debug_units![Comma, Bracket, Semi, Eq, Colon, Colon2, Sub, Dot, Paren, Let];
