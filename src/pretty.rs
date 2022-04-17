use syn::__private::Span;

use crate::{DebugPls, Formatter};

pub(crate) fn pretty_string(expr: syn::Expr) -> String {
    // unparse requires a `syn::File`, so we are forced to wrap
    // our expression in some junk. This is equivalent to
    // ```rust
    // const _: () = {
    //     #expr
    // };
    // ```
    let file = syn::File {
        shebang: None,
        attrs: vec![],
        items: vec![syn::Item::Const(syn::ItemConst {
            expr: Box::new(expr),
            // junk...
            attrs: vec![],
            vis: syn::Visibility::Inherited,
            const_token: syn::token::Const::default(),
            ident: syn::Ident::new("_", Span::call_site()),
            colon_token: syn::token::Colon::default(),
            ty: Box::new(syn::Type::Tuple(syn::TypeTuple {
                paren_token: syn::token::Paren::default(),
                elems: syn::punctuated::Punctuated::default(),
            })),
            eq_token: syn::token::Eq::default(),
            semi_token: syn::token::Semi::default(),
        })],
    };
    let output = prettyplease::unparse(&file);

    // strip out the junk
    let output = &output[14..];
    let output = &output[..output.len() - 2];
    textwrap::dedent(output)
}

/// Implementation detail for the `pretty!` macro
pub struct Str<'a>(pub &'a str);

impl<'a> std::fmt::Display for Str<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let expr = syn::parse_str(self.0).map_err(|_| std::fmt::Error)?;
        f.write_str(&pretty_string(expr))
    }
}

struct Pretty<'a>(&'a dyn DebugPls);

impl<'a> std::fmt::Debug for Pretty<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&pretty_string(Formatter::process(self.0)))
    }
}

impl<'a> std::fmt::Display for Pretty<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "pretty")))]
/// Wraps a [`Debug`] type into a [`std::fmt::Debug`] type for use in regular [`format!`]
pub fn pretty(value: &impl DebugPls) -> impl std::fmt::Debug + std::fmt::Display + '_ {
    Pretty(value)
}

#[cfg_attr(docsrs, doc(cfg(feature = "pretty")))]
#[macro_export]
/// Prints and returns the value of a given expression for quick and dirty
/// debugging. Same as [`std::dbg`]
///
/// An example:
///
/// ```rust
/// # use dbg_pls::pretty;
/// let a = 2;
/// let b = pretty!(a * 2) + 1;
/// //      ^-- prints: [src/main.rs:2] a * 2 = 4
/// assert_eq!(b, 5);
/// ```
///
/// The macro works by using the [`DebugPls`] implementation of the type of
/// the given expression to print the value to [stderr] along with the
/// source location of the macro invocation as well as the source code
/// of the expression.
///
/// Invoking the macro on an expression moves and takes ownership of it
/// before returning the evaluated expression unchanged. If the type
/// of the expression does not implement `Copy` and you don't want
/// to give up ownership, you can instead borrow with `pretty!(&expr)`
/// for some expression `expr`.
///
/// The `pretty!` macro works exactly the same in release builds.
/// This is useful when debugging issues that only occur in release
/// builds or when debugging in release mode is significantly faster.
///
/// Note that the macro is intended as a debugging tool and therefore you
/// should avoid having uses of it in version control for long periods
/// (other than in tests and similar).
/// Debug output from production code is better done with other facilities
/// such as the [`debug!`] macro from the [`log`] crate.
///
/// [stderr]: https://en.wikipedia.org/wiki/Standard_streams#Standard_error_(stderr)
/// [`debug!`]: https://docs.rs/log/*/log/macro.debug.html
/// [`log`]: https://crates.io/crates/log
macro_rules! pretty {
    () => {
        ::std::eprintln!("[{}:{}]", ::std::file!(), ::std::line!())
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                ::std::eprintln!(
                    "[{}:{}] {} => {}",
                    ::std::file!(),
                    ::std::line!(),
                    $crate::__private::PrettyStr(::std::stringify!($val)),
                    $crate::pretty(&tmp)
                );
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::pretty!($val)),+,)
    };
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::pretty;

    #[test]
    fn pretty_macro() {
        let map = pretty! {
            HashMap::from([
                ("hello", 1),
                ("world", 2),
            ])
        };
        // map is moved through properly
        assert_eq!(map, HashMap::from([("hello", 1), ("world", 2),]));
    }
}
