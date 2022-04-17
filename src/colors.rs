// use bat::PrettyPrinter;

// use crate::{pretty::process, DebugPls};

// /// Prints the pretty printed code to stdout
// pub fn print_colorful(value: &dyn DebugPls) {
//     let output = process(value);
//     let _ = PrettyPrinter::new()
//         .input_from_bytes(output.as_bytes())
//         .language("rust")
//         .line_numbers(true)
//         .print();
// }

use std::io::Cursor;

use once_cell::sync::OnceCell;
use syntect::{
    highlighting::{Theme, ThemeSet},
    parsing::{SyntaxDefinition, SyntaxSet, SyntaxSetBuilder},
};

use crate::{pretty::process, DebugPls};

fn syntax() -> &'static SyntaxSet {
    static INSTANCE: OnceCell<SyntaxSet> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let mut syntax_set = SyntaxSetBuilder::new();
        syntax_set.add(
            SyntaxDefinition::load_from_str(
                include_str!("../assets/syntaxes/Rust/Rust.sublime-syntax"),
                true,
                None,
            )
            .unwrap(),
        );
        syntax_set.build()
    })
}

fn theme() -> &'static Theme {
    static INSTANCE: OnceCell<Theme> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let s = include_str!("../assets/themes/sublime-monokai-extended/Monokai Extended.tmTheme");
        ThemeSet::load_from_reader(&mut Cursor::new(s.as_bytes())).unwrap()
    })
}

#[repr(transparent)]
struct ColorFill(dyn DebugPls);

impl std::fmt::Debug for ColorFill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use syntect::easy::HighlightLines;
        use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

        let ps = syntax();

        let syntax = ps.find_syntax_by_name("Rust").unwrap();
        let mut h = HighlightLines::new(syntax, theme());

        let s = process(&self.0);

        for line in LinesWithEndings::from(&s) {
            let ranges = h.highlight(line, ps);
            write!(f, "{}", as_24_bit_terminal_escaped(&ranges[..], false))?;
        }
        write!(f, "\x1b[0m")
    }
}

impl std::fmt::Display for ColorFill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "colors")))]
/// Wraps a [`Debug`] type into a [`std::fmt::Debug`] type for use in regular [`format!`]
pub fn color(value: &dyn DebugPls) -> impl std::fmt::Debug + std::fmt::Display + '_ {
    debug_impl(value)
}

fn debug_impl(value: &dyn DebugPls) -> &ColorFill {
    unsafe { std::mem::transmute(value) }
}

#[cfg_attr(docsrs, doc(cfg(feature = "colors")))]
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
macro_rules! color_pls {
    () => {
        ::std::eprintln!("[{}:{}]", ::std::file!(), ::std::line!())
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                ::std::eprintln!("[{}:{}] {} = {:#?}",
                    ::std::file!(), ::std::line!(), ::std::stringify!($val), $crate::color(&tmp));
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg_pls!($val)),+,)
    };
}
