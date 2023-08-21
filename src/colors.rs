use std::sync::OnceLock;

use stylish::{Ansi, Foreground, Style, Write};
use syntect::parsing::{
    BasicScopeStackOp, ParseScopeError, Scope, ScopeStack, ScopeStackOp, SyntaxDefinition,
    SyntaxReference, SyntaxSet, SyntaxSetBuilder,
};

use crate::{pretty::pretty_string, DebugPls, Formatter};

fn syntax() -> &'static SyntaxSet {
    static INSTANCE: OnceLock<SyntaxSet> = OnceLock::new();
    INSTANCE.get_or_init(|| {
        let mut syntax_set = SyntaxSetBuilder::new();
        syntax_set.add(
            SyntaxDefinition::load_from_str(
                include_str!("../assets/syntaxes/Rust/Rust.sublime-syntax"),
                false,
                None,
            )
            .unwrap(),
        );
        syntax_set.build()
    })
}

fn theme() -> &'static Theme {
    static INSTANCE: OnceLock<Theme> = OnceLock::new();
    INSTANCE.get_or_init(|| get_theme().unwrap())
}

fn highlight(s: &str, w: impl std::fmt::Write) -> std::fmt::Result {
    let syntax = syntax();
    let rust = syntax.find_syntax_by_name("Rust").unwrap();
    let theme = theme();

    let parsed = RustSyntax { syntax, rust }.parse_shell(s);

    theme.highlight(s, &parsed, Ansi::new(w))
}

/// Implementation detail for the `color!` macro
pub struct ColorStr<'a>(pub &'a str);

impl<'a> std::fmt::Display for ColorStr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let expr = &syn::parse_str(self.0).map_err(|_| std::fmt::Error)?;
        highlight(&pretty_string(expr), f)
    }
}

struct Color<'a>(&'a dyn DebugPls);

impl<'a> std::fmt::Debug for Color<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        highlight(&pretty_string(&Formatter::process(self.0)), f)
    }
}

impl<'a> std::fmt::Display for Color<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "colors")))]
/// Wraps a [`DebugPls`] type into a [`std::fmt::Debug`] type for use in regular [`format!`]
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

impl Theme {
    // this is a manual/simpler implementation of
    // syntect::highlight::HighlightIterator
    // to use a custom theme using `ratatui::Style`.
    // This is so we don't have to care about RGB and can instead use
    // terminal colours
    fn highlight(&self, h: &str, parsed: &ParsedSyntax, mut w: impl Write) -> std::fmt::Result {
        let mut stack = ScopeStack::default();
        let mut styles: Vec<(f64, Style)> = vec![];
        for (line, parsed_line) in h.lines().zip(parsed) {
            let mut last = 0;
            for &(index, ref op) in parsed_line {
                let style = styles.last().copied().unwrap_or_default().1;
                stack
                    .apply_with_hook(op, |op, stack| {
                        highlight_hook(&op, stack, &self.rules, &mut styles);
                    })
                    .unwrap();

                w.write_str(&line[last..index], style)?;
                last = index;
            }
            let style = styles.last().copied().unwrap_or_default().1;
            w.write_str(&line[last..], style)?;
            w.write_str("\n", style)?;
        }
        Ok(())
    }
}

#[allow(clippy::cast_possible_truncation)]
fn highlight_hook(
    op: &BasicScopeStackOp,
    stack: &[Scope],
    rules: &[ThemeRule],
    styles: &mut Vec<(f64, Style)>,
) {
    match op {
        BasicScopeStackOp::Push(scope) => {
            let mut scored_style = styles
                .last()
                .copied()
                .unwrap_or_else(|| (-1.0, Style::default()));

            for rule in rules.iter().filter(|a| a.scope.is_prefix_of(*scope)) {
                let single_score =
                    f64::from(rule.scope.len()) * f64::from(3 * ((stack.len() - 1) as u32)).exp2();

                if single_score > scored_style.0 {
                    scored_style.0 = single_score;
                    scored_style.1 = rule.style;
                }
            }

            styles.push(scored_style);
        }
        BasicScopeStackOp::Pop => {
            styles.pop();
        }
    }
}

#[derive(Clone, Copy)]
struct RustSyntax<'s> {
    syntax: &'s SyntaxSet,
    rust: &'s SyntaxReference,
}
type ParsedSyntax = Vec<Vec<(usize, ScopeStackOp)>>;

impl RustSyntax<'_> {
    fn parse_shell(self, h: &str) -> ParsedSyntax {
        let mut rust = syntect::parsing::ParseState::new(self.rust);

        let mut lines = vec![];
        for line in h.lines() {
            if let Ok(line) = rust.parse_line(line, self.syntax) {
                lines.push(line);
            } else {
                lines.push(Vec::new());
            }
        }
        lines
    }
}

struct Theme {
    rules: Vec<ThemeRule>,
}

struct ThemeRule {
    scope: Scope,
    style: Style,
}

// blame syntax highlighting
#[allow(clippy::too_many_lines)]
fn get_theme() -> Result<Theme, ParseScopeError> {
    use stylish::Color;
    let rules = vec![
        ThemeRule {
            scope: Scope::new("variable")?,
            style: Style::default().with(Foreground(Color::Blue)),
        },
        ThemeRule {
            scope: Scope::new("keyword")?,
            style: Style::default().with(Foreground(Color::Red)),
        },
        ThemeRule {
            scope: Scope::new("punctuation")?,
            style: Style::default().with(Foreground(Color::Red)),
        },
        ThemeRule {
            scope: Scope::new("storage")?,
            style: Style::default().with(Foreground(Color::Green)),
        },
        ThemeRule {
            scope: Scope::new("string")?,
            style: Style::default().with(Foreground(Color::Yellow)),
        },
    ];
    Ok(Theme { rules })
}
