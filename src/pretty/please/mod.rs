//! # Algorithm notes
//!
//! The approach and terminology used in the implementation are derived from
//! [*Derek C. Oppen, "Pretty Printing" (1979)*][paper], on which
//! rustc_ast_pretty is also based, and from rustc_ast_pretty's implementation
//! written by Graydon Hoare in 2011 (and modernized over the years by dozens of
//! volunteer maintainers).
//!
//! [paper]: http://i.stanford.edu/pub/cstr/reports/cs/tr/79/770/CS-TR-79-770.pdf
//!
//! The paper describes two language-agnostic interacting procedures `Scan()`
//! and `Print()`. Language-specific code decomposes an input data structure
//! into a stream of `string` and `break` tokens, and `begin` and `end` tokens
//! for grouping. Each `begin`&ndash;`end` range may be identified as either
//! "consistent breaking" or "inconsistent breaking". If a group is consistently
//! breaking, then if the whole contents do not fit on the line, *every* `break`
//! token in the group will receive a linebreak. This is appropriate, for
//! example, for Rust struct literals, or arguments of a function call. If a
//! group is inconsistently breaking, then the `string` tokens in the group are
//! greedily placed on the line until out of space, and linebroken only at those
//! `break` tokens for which the next string would not fit. For example, this is
//! appropriate for the contents of a braced `use` statement in Rust.
//!
//! Scan's job is to efficiently accumulate sizing information about groups and
//! breaks. For every `begin` token we compute the distance to the matched `end`
//! token, and for every `break` we compute the distance to the next `break`.
//! The algorithm uses a ringbuffer to hold tokens whose size is not yet
//! ascertained. The maximum size of the ringbuffer is bounded by the target
//! line length and does not grow indefinitely, regardless of deep nesting in
//! the input stream. That's because once a group is sufficiently big, the
//! precise size can no longer make a difference to linebreak decisions and we
//! can effectively treat it as "infinity".
//!
//! Print's job is to use the sizing information to efficiently assign a
//! "broken" or "not broken" status to every `begin` token. At that point the
//! output is easily constructed by concatenating `string` tokens and breaking
//! at `break` tokens contained within a broken group.
//!
//! Leveraging these primitives (i.e. cleverly placing the all-or-nothing
//! consistent breaks and greedy inconsistent breaks) to yield
//! rustfmt-compatible formatting for all of Rust's syntax tree nodes is a fun
//! challenge.
//!
//! Here is a visualization of some Rust tokens fed into the pretty printing
//! algorithm. Consistently breaking `begin`&mdash;`end` pairs are represented
//! by `«`&#8288;`»`, inconsistently breaking by `‹`&#8288;`›`, `break` by `·`,
//! and the rest of the non-whitespace are `string`.
//!
//! ```text
//! use crate::pretty::please::«{·
//! ‹    lazy::«{·‹Lazy,· SyncLazy,· SyncOnceCell›·}»,·
//!     panic,·
//!     sync::«{·
//! ‹        atomic::«{·‹AtomicUsize,· Ordering::SeqCst›·}»,·
//!         mpsc::channel,· Mutex›,·
//!     }»,·
//!     thread›,·
//! }»;·
//! «‹«impl<«·T‹›,· U‹›·»>» Into<«·U·»>· for T›·
//! where·
//!     U:‹ From<«·T·»>›,·
//! {·
//! «    fn into(·«·self·») -> U {·
//! ‹        U::from(«·self·»)›·
//! »    }·
//! »}·
//! ```
//!
//! The algorithm described in the paper is not quite sufficient for producing
//! well-formatted Rust code that is locally indistinguishable from rustfmt's
//! style. The reason is that in the paper, the complete non-whitespace contents
//! are assumed to be independent of linebreak decisions, with Scan and Print
//! being only in control of the whitespace (spaces and line breaks). In Rust as
//! idiomatically formattted by rustfmt, that is not the case. Trailing commas
//! are one example; the punctuation is only known *after* the broken vs
//! non-broken status of the surrounding group is known:
//!
//! ```
//! # struct Struct { x: u64, y: bool }
//! # let xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx = 0;
//! # let yyyyyyyyyyyyyyyyyyyyyyyyyyyyyy = true;
//! #
//! let _ = Struct { x: 0, y: true };
//!
//! let _ = Struct {
//!     x: xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx,
//!     y: yyyyyyyyyyyyyyyyyyyyyyyyyyyyyy,   //<- trailing comma if the expression wrapped
//! };
//! ```
//!
//! The formatting of `match` expressions is another case; we want small arms on
//! the same line as the pattern, and big arms wrapped in a brace. The presence
//! of the brace punctuation, comma, and semicolon are all dependent on whether
//! the arm fits on the line:
//!
//! ```
//! # struct Entry { nanos: u32 }
//! # let total_nanos = 0u64;
//! # let mut total_secs = 0u64;
//! # let tmp;
//! # let entry = Entry { nanos: 0 };
//! # const NANOS_PER_SEC: u32 = 1_000_000_000;
//! #
//! match total_nanos.checked_add(entry.nanos as u64) {
//!     Some(n) => tmp = n,   //<- small arm, inline with comma
//!     None => {
//!         total_secs = total_secs
//!             .checked_add(total_nanos / NANOS_PER_SEC as u64)
//!             .expect("overflow in iter::sum over durations");
//!     }   //<- big arm, needs brace added, and also semicolon^
//! }
//! ```
//!
//! The printing algorithm implementation in this crate accommodates all of
//! these situations with conditional punctuation tokens whose selection can be
//! deferred and populated after it's known that the group is or is not broken.

#![allow(
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::derive_partial_eq_without_eq,
    clippy::doc_markdown,
    clippy::enum_glob_use,
    clippy::items_after_statements,
    clippy::let_underscore_untyped,
    clippy::match_like_matches_macro,
    clippy::match_same_arms,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::needless_pass_by_value,
    clippy::similar_names,
    clippy::too_many_lines,
    clippy::unused_self,
    clippy::vec_init_then_push
)]
#![cfg_attr(all(test, exhaustive), feature(non_exhaustive_omitted_patterns_lint))]

mod algorithm;
mod attr;
mod convenience;
mod data;
mod expr;
mod generics;
mod item;
mod iter;
mod lifetime;
mod lit;
mod mac;
mod pat;
mod path;
mod ring;
mod stmt;
mod token;
mod ty;

use crate::pretty::please::algorithm::Printer;
use syn::Expr;

// Target line width.
const MARGIN: isize = 89;

// Number of spaces increment at each level of block indentation.
const INDENT: isize = 4;

// Every line is allowed at least this much space, even if highly indented.
const MIN_SPACE: isize = 60;

pub fn unparse(expr: &Expr) -> String {
    let mut p = Printer::new();
    p.expr(expr);
    p.eof()
}
