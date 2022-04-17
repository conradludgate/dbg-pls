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
    easy::HighlightLines,
    highlighting::{Theme, ThemeSet},
    parsing::{SyntaxDefinition, SyntaxSet, SyntaxSetBuilder},
    util::{as_24_bit_terminal_escaped, LinesWithEndings},
};

use crate::{pretty::pretty_string, DebugPls, Formatter};

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

fn highlight(s: &str, mut w: impl std::fmt::Write) -> std::fmt::Result {
    let ps = syntax();
    let syntax = ps.find_syntax_by_name("Rust").unwrap();
    let theme = theme();

    let mut h = HighlightLines::new(syntax, theme);

    for line in LinesWithEndings::from(s) {
        let ranges = h.highlight(line, ps);
        write!(w, "{}", as_24_bit_terminal_escaped(&ranges[..], false))?;
    }
    write!(w, "\x1b[0m") // reset the color
}

/// Implementation detail for the `color!` macro
pub struct ColorStr<'a>(pub &'a str);

impl<'a> std::fmt::Display for ColorStr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let expr = syn::parse_str(self.0).map_err(|_| std::fmt::Error)?;
        highlight(&pretty_string(expr), f)
    }
}

struct Color<'a>(&'a dyn DebugPls);

impl<'a> std::fmt::Debug for Color<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        highlight(&pretty_string(Formatter::process(self.0)), f)
    }
}

impl<'a> std::fmt::Display for Color<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "colors")))]
/// Wraps a [`Debug`] type into a [`std::fmt::Debug`] type for use in regular [`format!`]
pub fn color(value: &impl DebugPls) -> impl std::fmt::Debug + std::fmt::Display + '_ {
    Color(value)
}

#[cfg_attr(docsrs, doc(cfg(feature = "colors")))]
#[macro_export]
/// Prints and returns the value of a given expression for quick and dirty
/// debugging. Same as [`std::dbg`]
///
/// An example:
///
/// ```rust
/// # use dbg_pls::color;
/// let a = 2;
/// let b = color!(a * 2) + 1;
/// //      ^-- prints: [src/main.rs:2] a * 2 = 4
/// //          with syntax highlighting
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
/// to give up ownership, you can instead borrow with `color!(&expr)`
/// for some expression `expr`.
///
/// The `color!` macro works exactly the same in release builds.
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
macro_rules! color {
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
                    $crate::__private::ColorStr(::std::stringify!($val)),
                    $crate::color(&tmp)
                );
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::color!($val)),+,)
    };
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::color;

    #[test]
    fn colors() {
        let map = color! {
            HashMap::from([
                ("hello", 1),
                ("world", 2),
            ])
        };
        // map is moved through properly
        assert_eq!(map, HashMap::from([("hello", 1), ("world", 2),]));
    }
}
