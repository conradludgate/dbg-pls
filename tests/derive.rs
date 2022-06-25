#![allow(clippy::blacklisted_name)]

use std::{
    collections::{BTreeMap, BTreeSet},
    ops,
};

use dbg_pls::DebugPls;

macro_rules! assert_pretty_snapshot {
    ($expr:expr) => {
        let expr = $expr;
        let format = format!(
            ">>> pretty\n{}\n\n>>> color\n{}\n",
            dbg_pls::pretty(&expr),
            dbg_pls::color(&expr)
        );
        insta::assert_snapshot!(insta::_macro_support::AutoName, &format, stringify!($expr));
    };
}

#[derive(DebugPls, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Demo {
    foo: i32,
    bar: &'static str,
}

#[test]
fn debug_struct() {
    assert_pretty_snapshot!(Demo {
        foo: 5,
        bar: "hello",
    });
}

#[test]
fn debug_struct_big() {
    assert_pretty_snapshot!(Demo {
        foo: 5,
        bar: "Hello, world! I am a very long string",
    });
}

#[test]
fn debug_nested_struct() {
    assert_pretty_snapshot!({
        let mut val = [Demo {
            foo: 5,
            bar: "hello",
        }; 10];
        val[6].bar = "Hello, world! I am a very long string";
        val
    });
}

#[test]
fn debug_small_set() {
    assert_pretty_snapshot!(BTreeSet::from([420, 69]));
}

#[test]
fn debug_nested_set() {
    assert_pretty_snapshot!(BTreeSet::from([
        Demo {
            foo: 5,
            bar: "hello",
        },
        Demo {
            foo: 5,
            bar: "Hello, world! I am a very long string",
        },
    ]));
}

#[test]
fn debug_map() {
    assert_pretty_snapshot!(BTreeMap::from([
        ("hello", 60),
        ("Hello, world! I am a very long string", 12)
    ]));
}

#[test]
fn debug_nested_map() {
    assert_pretty_snapshot!(BTreeMap::from([
        (
            Demo {
                foo: 5,
                bar: "hello",
            },
            60,
        ),
        (
            Demo {
                foo: 5,
                bar: "Hello, world! I am a very long string",
            },
            12,
        ),
    ]));
}

#[derive(DebugPls)]
pub struct Generic<T> {
    arg: T,
}

#[test]
fn debug_generic() {
    assert_pretty_snapshot!(Generic { arg: "string" });
}

mod debug_enum_generic {
    use dbg_pls::DebugPls;

    #[derive(DebugPls)]
    pub enum Option2<T> {
        Some(T),
        None,
        Wtf { foo: i32 },
    }

    #[test]
    fn some() {
        assert_pretty_snapshot!(Option2::Some(42));
    }
    #[test]
    fn none() {
        assert_pretty_snapshot!(Option2::<i32>::None);
    }
    #[test]
    fn wtf() {
        assert_pretty_snapshot!(Option2::<i32>::Wtf { foo: 42 });
    }
}

#[derive(DebugPls)]
pub struct LinkedList {
    value: i32,
    next: Option<Box<LinkedList>>,
}

#[test]
fn debug_recursive() {
    assert_pretty_snapshot!(LinkedList {
        value: 0_i32,
        next: Some(Box::new(LinkedList {
            value: 1_i32,
            next: None,
        })),
    });
}

#[derive(DebugPls)]
pub struct LinkedList2<T> {
    value: T,
    next: Option<Box<LinkedList2<T>>>,
}

#[test]
fn debug_recursive2() {
    assert_pretty_snapshot!(LinkedList2 {
        value: 0_i32,
        next: Some(Box::new(LinkedList2 {
            value: 1_i32,
            next: None,
        })),
    });
}

#[derive(DebugPls)]
pub struct Rangeful<T> {
    range: ops::Range<T>,
    range_from: ops::RangeFrom<T>,
    range_to: ops::RangeTo<T>,
    range_full: ops::RangeFull,
    range_inclusive: ops::RangeInclusive<T>,
    range_inclusive_to: ops::RangeToInclusive<T>,
}

#[test]
fn ranges() {
    assert_pretty_snapshot!(Rangeful {
        range: 1..4,
        range_from: 1..,
        range_to: ..7,
        range_full: ..,
        range_inclusive: 1234..=1236,
        range_inclusive_to: ..=70,
    });
}

#[derive(DebugPls)]
enum EmptyEnum {}
