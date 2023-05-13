use syn::__private::Span;

use crate::{DebugPls, Formatter};

/// A helper designed to assist with creation of
/// [`DebugPls`] implementations for structs.
///
/// # Examples
///
/// ```rust
/// use dbg_pls::{pretty, DebugPls, Formatter};
///
/// struct Foo {
///     bar: i32,
///     baz: String,
/// }
///
/// impl DebugPls for Foo {
///     fn fmt(&self, f: Formatter) {
///         f.debug_struct("Foo")
///             .field("bar", &self.bar)
///             .field("baz", &self.baz)
///             .finish()
///     }
/// }
/// let value = Foo {
///     bar: 10,
///     baz: "Hello World".to_string(),
/// };
/// assert_eq!(
///     format!("{}", pretty(&value)),
///     "Foo { bar: 10, baz: \"Hello World\" }",
/// );
/// ```
pub struct DebugStruct<'a> {
    formatter: Formatter<'a>,
    expr: syn::ExprStruct,
}

impl<'a> DebugStruct<'a> {
    pub(crate) fn new(formatter: Formatter<'a>, name: &str) -> Self {
        DebugStruct {
            formatter,
            expr: syn::ExprStruct {
                attrs: vec![],
                path: syn::Ident::into(syn::parse_str(name).unwrap()),
                brace_token: syn::token::Brace::default(),
                fields: syn::punctuated::Punctuated::new(),
                dot2_token: None,
                rest: None,
            },
        }
    }

    /// Adds the field to the struct output.
    #[must_use]
    pub fn field(mut self, name: &str, value: &dyn DebugPls) -> Self {
        self.expr.fields.push(syn::FieldValue {
            expr: Formatter::process(value),
            attrs: vec![],
            member: syn::Member::Named(syn::parse_str(name).unwrap()),
            colon_token: Some(syn::token::Colon::default()),
        });
        self
    }

    /// Closes off the struct.
    pub fn finish(self) {
        self.formatter.write_expr(self.expr);
    }

    /// Closes off the struct with `..`.
    pub fn finish_non_exhaustive(mut self) {
        self.expr.dot2_token = Some(syn::token::Dot2::default());
        self.finish();
    }
}
