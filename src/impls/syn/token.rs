use syn::token::{
    And, At, Box, Brace, Bracket, Colon, Colon2, Comma, Dot, Dot2, DotDotEq, Eq, Let, Mut, Or,
    Paren, Ref, Semi, Sub, Underscore,
};

use crate::{DebugPls, Formatter};

macro_rules! debug_units {
    ($($T:ident,)*) => {$(
        impl DebugPls for $T {
            fn fmt(&self, f: Formatter<'_>) {
                f.debug_ident(stringify!($T))
            }
        }
    )*};
}

debug_units![
    And, At, Box, Brace, Bracket, Colon, Colon2, Comma, Dot, Dot2, DotDotEq, Eq, Let, Mut, Or,
    Paren, Ref, Semi, Sub, Underscore,
];
