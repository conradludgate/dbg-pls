use crate::{DebugPls, DebugWith, Formatter};

/// A helper designed to assist with creation of
/// [`DebugPls`] implementations for sets.
///
/// # Examples
///
/// ```rust
/// use dbg_pls::{pretty, DebugPls, Formatter};
/// use std::collections::BTreeSet;
///
/// struct Foo(BTreeSet<String>);
///
/// impl DebugPls for Foo {
///     fn fmt(&self, f: Formatter) {
///         f.debug_set().entries(&self.0).finish()
///     }
/// }
/// let mut value = Foo(BTreeSet::from([
///     "Hello".to_string(),
///     "World".to_string(),
/// ]));
/// assert_eq!(
///     format!("{}", pretty(&value)),
/// "{
///     \"Hello\";
///     \"World\"
/// }",
/// );
/// ```
pub struct DebugSet<'a> {
    formatter: Formatter<'a>,
    set: syn::Block,
}

impl<'a> DebugSet<'a> {
    pub(crate) fn new(formatter: Formatter<'a>) -> Self {
        DebugSet {
            formatter,
            set: syn::Block {
                brace_token: syn::token::Brace::default(),
                stmts: vec![],
            },
        }
    }

    /// Adds the entry to the set output.
    #[must_use]
    pub fn entry(mut self, value: &dyn DebugPls) -> Self {
        let expr = Formatter::process(value);
        self.set
            .stmts
            .push(syn::Stmt::Expr(expr, Some(syn::token::Semi::default())));
        self
    }

    /// Adds all the entries to the set output.
    #[must_use]
    pub fn entries<V, I>(self, entries: I) -> Self
    where
        V: DebugPls,
        I: IntoIterator<Item = V>,
    {
        entries.into_iter().fold(self, |f, entry| f.entry(&entry))
    }

    /// Adds the entry to the set output.
    #[must_use]
    pub fn entry_with<T>(mut self, value: &dyn DebugWith<T>, with: &T) -> Self {
        let expr = Formatter::process_with(value, with);
        self.set
            .stmts
            .push(syn::Stmt::Expr(expr, Some(syn::token::Semi::default())));
        self
    }

    /// Adds all the entries to the set output.
    #[must_use]
    pub fn entries_with<T, V, I>(self, entries: I, with: &T) -> Self
    where
        V: DebugWith<T>,
        I: IntoIterator<Item = V>,
    {
        entries
            .into_iter()
            .fold(self, |f, entry| f.entry_with(&entry, with))
    }

    /// Closes off the set.
    pub fn finish(mut self) {
        // remove the last semicolon
        if let Some(syn::Stmt::Expr(entry, Some(_))) = self.set.stmts.pop() {
            self.set.stmts.push(syn::Stmt::Expr(entry, None));
        }

        self.formatter.write_expr(syn::ExprBlock {
            attrs: vec![],
            label: None,
            block: self.set,
        });
    }
}
