use crate::{DebugPls, Formatter};

/// A helper designed to assist with creation of
/// [`DebugPls`] implementations for tuples.
///
/// # Examples
///
/// ```rust
/// use dbg_pls::{pretty, DebugPls, Formatter};
///
/// struct Foo(i32, String);
///
/// impl DebugPls for Foo {
///     fn fmt(&self, f: Formatter) {
///         f.debug_tuple()
///             .field(&self.0)
///             .field(&self.1)
///             .finish()
///     }
/// }
///
/// let value = Foo(10, "Hello".to_string());
/// assert_eq!(format!("{}", pretty(&value)), "(10, \"Hello\")");
/// ```
pub struct DebugTuple<'a> {
    formatter: Formatter<'a>,
    expr: syn::ExprTuple,
}

impl<'a> DebugTuple<'a> {
    pub(crate) fn new(formatter: Formatter<'a>) -> Self {
        DebugTuple {
            formatter,
            expr: syn::ExprTuple {
                attrs: vec![],
                paren_token: syn::token::Paren::default(),
                elems: syn::punctuated::Punctuated::new(),
            },
        }
    }

    /// Adds the field to the tuple output.
    #[must_use]
    pub fn field(mut self, value: &dyn DebugPls) -> Self {
        self.expr.elems.push(Formatter::process(value));
        self
    }

    /// Closes off the tuple.
    pub fn finish(self) {
        self.formatter.write_expr(self.expr);
    }
}
