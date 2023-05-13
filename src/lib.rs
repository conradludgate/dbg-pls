#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]
//! Syntax aware debug printing.
//!
//! Makes use of `syn` and `prettyplease` in order to provide the most
//! canonincal rust debug lines as possible, quickly.
//!
//! # Example usage
//!
//! ```
//! use dbg_pls::{pretty, DebugPls};
//!
//! #[derive(DebugPls, Copy, Clone)]
//! pub struct Demo {
//!     foo: i32,
//!     bar: &'static str,
//! }
//!
//! let mut val = [Demo { foo: 5, bar: "hello" }; 10];
//! val[6].bar = "Hello, world! I am a very long string";
//!
//! let output = format!("{}", pretty(&val));
//! let expected = r#"[
//!     Demo { foo: 5, bar: "hello" },
//!     Demo { foo: 5, bar: "hello" },
//!     Demo { foo: 5, bar: "hello" },
//!     Demo { foo: 5, bar: "hello" },
//!     Demo { foo: 5, bar: "hello" },
//!     Demo { foo: 5, bar: "hello" },
//!     Demo {
//!         foo: 5,
//!         bar: "Hello, world! I am a very long string",
//!     },
//!     Demo { foo: 5, bar: "hello" },
//!     Demo { foo: 5, bar: "hello" },
//!     Demo { foo: 5, bar: "hello" },
//! ]"#;
//!
//! assert_eq!(output, expected);
//! ```
//!
//! # Example with highlighting
//!
//! ```
//! use dbg_pls::{color, DebugPls};
//!
//! #[derive(DebugPls, Copy, Clone)]
//! pub struct Demo {
//!     foo: i32,
//!     bar: &'static str,
//! }
//!
//! let mut val = [Demo { foo: 5, bar: "hello" }; 10];
//! val[6].bar = "Hello, world! I am a very long string";
//!
//! println!("{}", color(&val));
//! ```
//! Outputs:
//!
//! ![](https://raw.githubusercontent.com/conradludgate/dbg-pls/5dee03187a3f83693739e0288d56da5980e1d486/readme/highlighted.png)
//!
//! # Why
//!
//! For the sake of demonstration, let's take a look at the snippet from above.
//! It provides an array of 10 `Demo` structs. You could imagine this to
//! be representative of a complex deep struct.
//!
//! ```
//! #[derive(Debug, Copy, Clone)]
//! pub struct Demo {
//!     foo: i32,
//!     bar: &'static str,
//! }
//!
//! let mut val = [Demo { foo: 5, bar: "hello" }; 10];
//! val[6].bar = "Hello, world! I am a very long string";
//!
//! println!("{:?}", val);
//! ```
//!
//! This outputs
//!
//! ```text
//! [Demo { foo: 5, bar: "hello" }, Demo { foo: 5, bar: "hello" }, Demo { foo: 5, bar: "hello" }, Demo { foo: 5, bar: "hello" }, Demo { foo: 5, bar: "hello" }, Demo { foo: 5, bar: "hello" }, Demo { foo: 5, bar: "Hello, world! I am a very long string" }, Demo { foo: 5, bar: "hello" }, Demo { foo: 5, bar: "hello" }, Demo { foo: 5, bar: "hello" }]
//! ```
//!
//! Switching to the alternative output format `{:#?}` you get the following
//!
//! ```text
//! [
//!    Demo {
//!        foo: 5,
//!        bar: "hello",
//!    },
//!    Demo {
//!        foo: 5,
//!        bar: "hello",
//!    },
//!    Demo {
//!        foo: 5,
//!        bar: "hello",
//!    },
//!    Demo {
//!        foo: 5,
//!        bar: "hello",
//!    },
//!    Demo {
//!        foo: 5,
//!        bar: "hello",
//!    },
//!    Demo {
//!        foo: 5,
//!        bar: "hello",
//!    },
//!    Demo {
//!        foo: 5,
//!        bar: "Hello, world! I am a very long string",
//!    },
//!    Demo {
//!        foo: 5,
//!        bar: "hello",
//!    },
//!    Demo {
//!        foo: 5,
//!        bar: "hello",
//!    },
//!    Demo {
//!        foo: 5,
//!        bar: "hello",
//!    },
//! ]
//! ```
//!
//! Both of these are very unweildy to read through. Compare that to our `pretty` formatting:
//!
//! ```
//! # use dbg_pls::pretty;
//! # let val = 0;
//! println!("{}", pretty(&val));
//! ```
//!
//! And you will see
//!
//! ```text
//! [
//!     Demo { foo: 5, bar: "hello" },
//!     Demo { foo: 5, bar: "hello" },
//!     Demo { foo: 5, bar: "hello" },
//!     Demo { foo: 5, bar: "hello" },
//!     Demo { foo: 5, bar: "hello" },
//!     Demo { foo: 5, bar: "hello" },
//!     Demo {
//!         foo: 5,
//!         bar: "Hello, world! I am a very long string",
//!     },
//!     Demo { foo: 5, bar: "hello" },
//!     Demo { foo: 5, bar: "hello" },
//!     Demo { foo: 5, bar: "hello" },
//! ]
//! ```
//!
//! # How it works
//!
//! All [`DebugPls`] implementations are forced to output only valid
//! [`syn::Expr`] values. These are then formatted using [`prettyplease::unparse`].
//! Finally, it uses [`syntect`] to provide syntax highlighting, with theme provided by
//! <https://github.com/jonschlinkert/sublime-monokai-extended>

use syn::__private::{Span, TokenStream2};

mod impls;

mod debug_list;
mod debug_map;
mod debug_set;
mod debug_struct;
mod debug_tuple;
mod debug_tuple_struct;
pub use debug_list::DebugList;
pub use debug_map::DebugMap;
pub use debug_set::DebugSet;
pub use debug_struct::DebugStruct;
pub use debug_tuple::DebugTuple;
pub use debug_tuple_struct::DebugTupleStruct;

#[cfg(feature = "pretty")]
mod pretty;
#[cfg(feature = "pretty")]
pub use pretty::pretty;

#[cfg(feature = "colors")]
mod colors;
#[cfg(feature = "colors")]
pub use colors::color;

#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
/// Derives the standard `DebugPls` implementation.
///
/// Works exactly like [`Debug`]
///
/// ```
/// use dbg_pls::{pretty, DebugPls};
/// #[derive(DebugPls)]
/// struct Point {
///     x: i32,
///     y: i32,
/// }
///
/// let origin = Point { x: 0, y: 0 };
///
/// assert_eq!(
///     format!("The origin is: {}", pretty(&origin)),
///     "The origin is: Point { x: 0, y: 0 }",
/// );
/// ```
pub use dbg_pls_derive::DebugPls;

#[doc(hidden)]
pub mod __private {
    #[cfg(feature = "colors")]
    pub use crate::colors::ColorStr;
    #[cfg(feature = "pretty")]
    pub use crate::pretty::Str as PrettyStr;
}

/// Syntax aware pretty-printed debug formatting.
///
/// `DebugPls` should format the output in a programmer-facing, debugging context.
///
/// Generally speaking, you should just `derive` a `Debug` implementation.
///
/// # Examples
///
/// Deriving an implementation:
///
/// ```
/// use dbg_pls::{pretty, DebugPls};
/// #[derive(DebugPls)]
/// struct Point {
///     x: i32,
///     y: i32,
/// }
///
/// let origin = Point { x: 0, y: 0 };
///
/// assert_eq!(format!("The origin is: {}", pretty(&origin)), "The origin is: Point { x: 0, y: 0 }");
/// ```
///
/// Manually implementing:
///
/// ```
/// use dbg_pls::{pretty, DebugPls, Formatter};
/// struct Point {
///     x: i32,
///     y: i32,
/// }
///
/// impl DebugPls for Point {
///     fn fmt(&self, f: Formatter<'_>) {
///         f.debug_struct("Point")
///          .field("x", &self.x)
///          .field("y", &self.y)
///          .finish()
///     }
/// }
///
/// let origin = Point { x: 0, y: 0 };
///
/// assert_eq!(format!("The origin is: {}", pretty(&origin)), "The origin is: Point { x: 0, y: 0 }");
/// ```
pub trait DebugPls {
    /// Formats the value using the given formatter.
    ///
    /// # Examples
    ///
    /// ```
    /// use dbg_pls::{pretty, DebugPls, Formatter};
    ///
    /// struct Position {
    ///     longitude: f32,
    ///     latitude: f32,
    /// }
    ///
    /// impl DebugPls for Position {
    ///     fn fmt(&self, f: Formatter<'_>) {
    ///         f.debug_tuple()
    ///          .field(&self.longitude)
    ///          .field(&self.latitude)
    ///          .finish()
    ///     }
    /// }
    ///
    /// let position = Position { longitude: 1.987, latitude: 2.983 };
    /// assert_eq!(format!("{}", pretty(&position)), "(1.987, 2.983)");
    /// ```
    fn fmt(&self, f: Formatter<'_>);
}

/// Tool for formatting, used within [`DebugPls`] implementations
#[repr(transparent)]
pub struct Formatter<'a> {
    expr: &'a mut syn::Expr,
}

impl<'a> Formatter<'a> {
    pub(crate) fn process(value: &dyn DebugPls) -> syn::Expr {
        let mut expr = syn::Expr::Verbatim(TokenStream2::new());
        value.fmt(Formatter { expr: &mut expr });
        expr
    }

    /// Writes a wrap expression into the formatter.
    /// This is typically reserved for more advanced uses
    pub fn write_expr(self, expr: impl Into<syn::Expr>) {
        *self.expr = expr.into();
    }

    /// Creates a [`DebugStruct`] builder designed to assist with creation of
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
    #[must_use]
    pub fn debug_struct(self, name: &str) -> DebugStruct<'a> {
        DebugStruct::new(self, name)
    }

    /// Creates a [`DebugTuple`] builder designed to assist with creation of
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
    #[must_use]
    pub fn debug_tuple(self) -> DebugTuple<'a> {
        DebugTuple::new(self)
    }

    /// Creates a [`DebugTupleStruct`] builder designed to assist with creation of
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
    #[must_use]
    pub fn debug_tuple_struct(self, name: &str) -> DebugTupleStruct<'a> {
        DebugTupleStruct::new(self, name)
    }

    /// Creates a [`DebugList`] builder designed to assist with creation of
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
    #[must_use]
    pub fn debug_list(self) -> DebugList<'a> {
        DebugList::new(self)
    }

    /// Creates a [`DebugMap`] builder designed to assist with creation of
    /// [`DebugPls`] implementations for maps.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dbg_pls::{pretty, DebugPls, Formatter};
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
    ///     format!("{}", pretty(&value)),
    /// "{
    ///     \"Hello\" = 5;
    ///     \"World\" = 10;
    /// }",
    /// );
    /// ```
    #[must_use]
    pub fn debug_map(self) -> DebugMap<'a> {
        DebugMap::new(self)
    }

    /// Creates a [`DebugSet`] builder designed to assist with creation of
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
    #[must_use]
    pub fn debug_set(self) -> DebugSet<'a> {
        DebugSet::new(self)
    }

    /// Writes an identifier into the formatter. Useful for unit structs/variants
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dbg_pls::{pretty, DebugPls, Formatter};
    ///
    /// struct Foo;
    ///
    /// impl DebugPls for Foo {
    ///     fn fmt(&self, f: Formatter<'_>) {
    ///         f.debug_ident("Foo");
    ///     }
    /// }
    ///
    /// assert_eq!(format!("{}", pretty(&Foo)), "Foo");
    /// ```
    pub fn debug_ident(self, name: &str) {
        let path: syn::Path = syn::Ident::into(syn::parse_str(name).unwrap());
        self.write_expr(syn::ExprPath {
            attrs: vec![],
            qself: None,
            path,
        });
    }
}
