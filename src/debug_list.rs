use crate::{DebugPls, Formatter};

/// A helper designed to assist with creation of
/// [`DebugPls`] implementations for list-like structures.
///
/// # Examples
///
/// ```rust
/// use dbg_pls::{pretty, DebugPls, Formatter};
///
/// struct Foo(Vec<i32>);
///
/// impl DebugPls for Foo {
///     fn fmt(&self, f: Formatter<'_>) {
///         f.debug_list().entries(&self.0).finish()
///     }
/// }
///
/// let value = Foo(vec![10, 11]);
/// assert_eq!(format!("{}", pretty(&value)), "[10, 11]");
/// ```
pub struct DebugList<'a> {
    formatter: Formatter<'a>,
    expr: syn::ExprArray,
}

impl<'a> DebugList<'a> {
    pub(crate) fn new(formatter: Formatter<'a>) -> Self {
        DebugList {
            formatter,
            expr: syn::ExprArray {
                attrs: vec![],
                bracket_token: syn::token::Bracket::default(),
                elems: syn::punctuated::Punctuated::default(),
            },
        }
    }

    /// Adds a new entry to the list output.
    #[must_use]
    pub fn entry(mut self, entry: &dyn DebugPls) -> Self {
        self.expr.elems.push(Formatter::process(entry));
        self
    }

    /// Adds all the entries to the list output.
    #[must_use]
    pub fn entries<D, I>(mut self, entries: I) -> Self
    where
        D: DebugPls,
        I: IntoIterator<Item = D>,
    {
        self.extend(entries);
        self
    }

    /// Closes off the list
    pub fn finish(self) {
        self.formatter.write_expr(self.expr);
    }
}

impl<'f, D: DebugPls> Extend<D> for DebugList<'f> {
    fn extend<T: IntoIterator<Item = D>>(&mut self, iter: T) {
        self.expr
            .elems
            .extend(iter.into_iter().map(|entry| Formatter::process(&entry)));
    }
}
