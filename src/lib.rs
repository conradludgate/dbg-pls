//! Syntax aware debug printing.
//!
//! Makes use of `syn` and `prettyplease` in order to provide the most
//! canonincal rust debug lines as possible, quickly.
//!
//! # Example usage
//!
//! ```
//! use dbg_pls::debug;
//! # #[derive(Copy, Clone)]
//! # pub struct Demo {
//! #     foo: i32,
//! #     bar: &'static str,
//! # }

//! # impl dbg_pls::DebugPls for Demo {
//! #     fn fmt(&self, f: dbg_pls::Formatter<'_>) {
//! #         f.debug_struct("Demo").field("foo", &self.foo).field("bar", &self.bar).finish()
//! #     }
//! # }
//!
//! let mut val = [Demo { foo: 5, bar: "hello" }; 10];
//! val[6].bar = "Hello, world! I am a very long string";
//!
//! assert_eq!(debug(&val).to_string(), r#"[
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
//! ]"#);
//! ```

use debug_list::DebugList;
use debug_struct::DebugStruct;
use debug_tuple::DebugTuple;
use syn::__private::{Span, TokenStream2};

mod debug_list;
mod debug_struct;
mod debug_tuple;
mod impls;
#[cfg(feature = "pretty")]
mod pretty;
#[cfg(feature = "pretty")]
pub use pretty::debug;

pub use dbg_pls_derive::DebugPls;

/// Syntax aware pretty-printed debug formatting.
///
/// `DebugPls` should format the output in a programmer-facing, debugging context.
///
/// Generally speaking, you should just `derive` a `Debug` implementation.
pub trait DebugPls {
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
    pub fn debug_tuple(self, name: &str) -> DebugTuple<'a> {
        DebugTuple::new(self, name)
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
