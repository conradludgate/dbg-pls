use syn::__private::Span;

use crate::{DebugPls, Formatter};

pub(crate) fn process(value: &dyn DebugPls) -> String {
    let output = prettyplease::unparse(&syn::File {
        shebang: None,
        attrs: vec![],
        items: vec![syn::Item::Const(syn::ItemConst {
            expr: Box::new(Formatter::process(value)),
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
    });

    // strip out the junk
    let output = &output[14..];
    let output = &output[..output.len() - 2];
    textwrap::dedent(output)
}

#[repr(transparent)]
struct PolyFill(dyn DebugPls);

impl std::fmt::Debug for PolyFill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&process(&self.0))
    }
}

impl std::fmt::Display for PolyFill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "pretty")))]
/// Wraps a [`Debug`] type into a [`std::fmt::Debug`] type for use in regular [`format!`]
pub fn debug(value: &dyn DebugPls) -> impl std::fmt::Debug + std::fmt::Display + '_ {
    debug_impl(value)
}

fn debug_impl(value: &dyn DebugPls) -> &PolyFill {
    unsafe { std::mem::transmute(value) }
}

#[cfg_attr(docsrs, doc(cfg(feature = "pretty")))]
#[macro_export]
/// Prints and returns the value of a given expression for quick and dirty
/// debugging. Same as [`std::dbg`]
///
/// An example:
///
/// ```rust
/// # use dbg_pls::dbg_pls;
/// let a = 2;
/// let b = dbg_pls!(a * 2) + 1;
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
/// to give up ownership, you can instead borrow with `dbg!(&expr)`
/// for some expression `expr`.
///
/// The `dbg_pls!` macro works exactly the same in release builds.
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
macro_rules! dbg_pls {
    () => {
        ::std::eprintln!("[{}:{}]", ::std::file!(), ::std::line!())
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                ::std::eprintln!("[{}:{}] {} = {:#?}",
                    ::std::file!(), ::std::line!(), ::std::stringify!($val), $crate::debug(&tmp));
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg_pls!($val)),+,)
    };
}
