use std::{fmt::Debug, marker::PhantomData};

use syn::{parse::{ParseStream, Parse}, Expr};

use crate::DebugPls;

struct Polyfill<'a>(&'a dyn Debug);

/// Wraps a [`Debug`] type into a [`DebugPls`] type
pub fn polyfill(value: &impl Debug) -> impl DebugPls + '_ {
    Polyfill(value)
}

impl<'a> DebugPls for Polyfill<'a> {
    fn fmt(&self, f: crate::Formatter<'_>) {
        let s = format!("{:?}", self.0);
        let Parse2Polyfill(expr) = syn::parse_str(&s).unwrap();
        *f.expr = expr;
    }
}

struct Parse2Polyfill<P: Parse2>(P);
impl<P: Parse2> Parse for Parse2Polyfill<P> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(P::parse2(input)))
    }
}

/// Like [`syn::parse::Parse`] but is infallible.
/// In the event of a parse error, it should return a valid
/// value, just potentially incorrect
trait Parse2 {
    fn parse2(input: ParseStream<'_>) -> Self;
}

impl Parse2 for Expr {
    fn parse2(input: ParseStream<'_>) -> Self {
        todo!()
    }
}
