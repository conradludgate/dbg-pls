//! Syntax aware debug printing.
//!
//! Makes use of `syn` and `prettyplease` in order to provide the most
//! canonincal rust debug lines as possible, quickly.
//!
//! # Example usage
//!
//! ```
//! use dbg_pls::{debug, DebugPls};
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
//! let output = format!("{}", debug(&val));
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

use syn::__private::{Span, TokenStream2};

mod impls;

mod debug_list;
mod debug_struct;
mod debug_tuple;
mod debug_tuple_struct;
pub use debug_list::DebugList;
pub use debug_struct::DebugStruct;
pub use debug_tuple::DebugTuple;
pub use debug_tuple_struct::DebugTupleStruct;

#[cfg(feature = "pretty")]
#[cfg_attr(docsrs, doc(cfg(feature = "pretty")))]
mod pretty;
#[cfg(feature = "pretty")]
#[cfg_attr(docsrs, doc(cfg(feature = "pretty")))]
pub use pretty::debug;

#[cfg(feature = "colors")]
#[cfg_attr(docsrs, doc(cfg(feature = "colors")))]
mod colors;
#[cfg(feature = "colors")]
#[cfg_attr(docsrs, doc(cfg(feature = "colors")))]
pub use colors::color;

#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use dbg_pls_derive::DebugPls;

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
/// use dbg_pls::{debug, DebugPls};
/// #[derive(DebugPls)]
/// struct Point {
///     x: i32,
///     y: i32,
/// }
///
/// let origin = Point { x: 0, y: 0 };
///
/// assert_eq!(format!("The origin is: {}", debug(&origin)), "The origin is: Point { x: 0, y: 0 }");
/// ```
///
/// Manually implementing:
///
/// ```
/// use dbg_pls::{debug, DebugPls, Formatter};
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
/// assert_eq!(format!("The origin is: {}", debug(&origin)), "The origin is: Point { x: 0, y: 0 }");
/// ```
pub trait DebugPls {
    /// Formats the value using the given formatter.
    ///
    /// # Examples
    ///
    /// ```
    /// use dbg_pls::{debug, DebugPls, Formatter};
    /// use std::fmt;
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
    /// assert_eq!(format!("{}", debug(&position)), "(1.987, 2.983)");
    /// ```
    fn fmt(&self, f: Formatter<'_>);
}

pub struct Formatter<'a> {
    expr: &'a mut syn::Expr,
}

impl<'a> Formatter<'a> {
    pub(crate) fn process(value: &dyn DebugPls) -> syn::Expr {
        let mut expr = syn::Expr::Verbatim(TokenStream2::new());
        value.fmt(Formatter { expr: &mut expr });
        expr
    }

    pub fn write_expr(self, expr: impl Into<syn::Expr>) {
        *self.expr = expr.into();
    }

    pub fn debug_struct(self, name: &str) -> DebugStruct<'a> {
        DebugStruct::new(self, name)
    }
    pub fn debug_tuple(self) -> DebugTuple<'a> {
        DebugTuple::new(self)
    }
    pub fn debug_tuple_struct(self, name: &str) -> DebugTupleStruct<'a> {
        DebugTupleStruct::new(self, name)
    }
    pub fn debug_list(self) -> DebugList<'a> {
        DebugList::new(self)
    }
    pub fn debug_ident(self, name: &str) {
        let path: syn::Path = syn::Ident::new(name, Span::call_site()).into();
        self.write_expr(syn::ExprPath {
            attrs: vec![],
            qself: None,
            path,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(DebugPls, Copy, Clone)]
    #[dbg_pls(crate = "crate")]
    pub struct Demo {
        foo: i32,
        bar: &'static str,
    }

    #[test]
    fn debug_struct() {
        let val = Demo {
            foo: 5,
            bar: "hello",
        };
        assert_eq!(debug(&val).to_string(), r#"Demo { foo: 5, bar: "hello" }"#);
    }

    #[test]
    fn debug_struct_big() {
        let val = Demo {
            foo: 5,
            bar: "Hello, world! I am a very long string",
        };
        assert_eq!(
            debug(&val).to_string(),
            r#"Demo {
    foo: 5,
    bar: "Hello, world! I am a very long string",
}"#
        );
    }

    #[test]
    fn debug_nested_struct() {
        let mut val = [Demo {
            foo: 5,
            bar: "hello",
        }; 10];
        val[6].bar = "Hello, world! I am a very long string";

        assert_eq!(
            debug(&val).to_string(),
            r#"[
    Demo { foo: 5, bar: "hello" },
    Demo { foo: 5, bar: "hello" },
    Demo { foo: 5, bar: "hello" },
    Demo { foo: 5, bar: "hello" },
    Demo { foo: 5, bar: "hello" },
    Demo { foo: 5, bar: "hello" },
    Demo {
        foo: 5,
        bar: "Hello, world! I am a very long string",
    },
    Demo { foo: 5, bar: "hello" },
    Demo { foo: 5, bar: "hello" },
    Demo { foo: 5, bar: "hello" },
]"#
        );
    }
}
