use crate::{DebugPls, Formatter};

/// A helper designed to assist with creation of
/// [`DebugPls`] implementations for maps.
///
/// # Examples
///
/// ```rust
/// use dbg_pls::{debug, DebugPls, Formatter};
/// use std::collections::BTreeMap;
///
/// struct Foo(BTreeMap<String, i32>);
///
/// impl DebugPls for Foo {
///     fn fmt(&self, f: Formatter) {
///         f.debug_map().entries(&self.0).finish()
///     }
/// }
/// let mut value = Foo(BTreeMap::from([
///     ("Hello".to_string(), 5),
///     ("World".to_string(), 10),
/// ]));
/// assert_eq!(
///     format!("{}", debug(&value)),
/// "{
///     \"Hello\" >> 5;
///     \"World\" >> 10;
/// }",
/// );
/// ```
pub struct DebugMap<'a> {
    formatter: Formatter<'a>,
    set: syn::Block,
    key: Option<syn::Expr>,
}

impl<'a> DebugMap<'a> {
    pub(crate) fn new(formatter: Formatter<'a>) -> Self {
        DebugMap {
            formatter,
            set: syn::Block {
                brace_token: syn::token::Brace::default(),
                stmts: vec![],
            },
            key: None,
        }
    }

    /// Adds the key part to the map output.
    pub fn key(mut self, key: &dyn DebugPls) -> Self {
        if self.key.replace(Formatter::process(key)).is_some() {
            panic!("key() method called twice on DebugMap value")
        }
        self
    }

    /// Adds the value part to the map output.
    pub fn value(mut self, value: &dyn DebugPls) -> Self {
        let key = self
            .key
            .take()
            .expect("value() method called before key() on DebugMap value");
        let value = Formatter::process(value);
        let entry = syn::ExprBinary {
            attrs: vec![],
            left: Box::new(key),
            op: syn::BinOp::Shr(syn::token::Shr::default()),
            right: Box::new(value),
        };
        self.set
            .stmts
            .push(syn::Stmt::Semi(entry.into(), syn::token::Semi::default()));
        self
    }

    /// Adds the entry to the map output.
    pub fn entry(self, key: &dyn DebugPls, value: &dyn DebugPls) -> Self {
        self.key(key).value(value)
    }

    /// Adds all the entries to the map output.
    pub fn entries<K, V, I>(self, entries: I) -> Self
    where
        K: DebugPls,
        V: DebugPls,
        I: IntoIterator<Item = (K, V)>,
    {
        entries
            .into_iter()
            .fold(self, |f, (key, value)| f.entry(&key, &value))
    }

    /// Closes off the map.
    pub fn finish(self) {
        self.formatter.write_expr(syn::ExprBlock {
            attrs: vec![],
            label: None,
            block: self.set,
        });
    }
}
