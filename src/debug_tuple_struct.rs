use syn::__private::Span;

use crate::{DebugPls, Formatter};

/// A helper designed to assist with creation of
/// [`DebugPls`] implementations for tuple structs.
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
///         f.debug_tuple_struct("Foo")
///             .field(&self.0)
///             .field(&self.1)
///             .finish()
///     }
/// }
///
/// let value = Foo(10, "Hello".to_string());
/// assert_eq!(format!("{}", pretty(&value)), "Foo(10, \"Hello\")");
/// ```
pub struct DebugTupleStruct<'a> {
    formatter: Formatter<'a>,
    expr: syn::ExprCall,
}

impl<'a> DebugTupleStruct<'a> {
    pub(crate) fn new(formatter: Formatter<'a>, name: &str) -> Self {
        DebugTupleStruct {
            formatter,
            expr: syn::ExprCall {
                attrs: vec![],
                func: Box::new(syn::Expr::Path(syn::ExprPath {
                    attrs: vec![],
                    qself: None,
                    path: syn::Ident::into(syn::parse_str(name).unwrap()),
                })),
                paren_token: syn::token::Paren::default(),
                args: syn::punctuated::Punctuated::new(),
            },
        }
    }

    /// Adds the field to the tuple struct output.
    #[must_use]
    pub fn field(mut self, value: &dyn DebugPls) -> Self {
        self.expr.args.push(Formatter::process(value));
        self
    }

    /// Closes off the tuple struct.
    pub fn finish(self) {
        self.formatter.write_expr(self.expr);
    }
}
