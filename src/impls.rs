use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use syn::__private::Span;

use crate::{DebugPls, Formatter};

impl<D: DebugPls + ?Sized> DebugPls for Box<D> {
    fn fmt(&self, f: Formatter<'_>) {
        D::fmt(self, f)
    }
}
impl<'a, D: DebugPls + ?Sized> DebugPls for &'a D {
    fn fmt(&self, f: Formatter<'_>) {
        D::fmt(self, f)
    }
}

macro_rules! debug_integers {
    ($($T:ident)*) => {$(
        impl DebugPls for $T {
            fn fmt(&self, f: Formatter<'_>) {
                let mut buf = itoa::Buffer::new();
                f.write_expr(syn::ExprLit {
                    attrs: vec![],
                    lit: syn::LitInt::new(buf.format(*self), Span::call_site()).into(),
                });
            }
        }
    )*};
}

debug_integers! {
  i8 i16 i32 i64 i128 isize
  u8 u16 u32 u64 u128 usize
}

macro_rules! debug_floats {
    ($ty:ident) => {
        impl DebugPls for $ty {
            fn fmt(&self, f: Formatter<'_>) {
                let mut buf = ryu::Buffer::new();
                f.write_expr(syn::ExprLit {
                    attrs: vec![],
                    lit: syn::LitFloat::new(buf.format(*self), Span::call_site()).into(),
                });
            }
        }
    };
}

debug_floats! { f32 }
debug_floats! { f64 }

impl<D: DebugPls> DebugPls for [D] {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_list().entries(self).finish()
    }
}

impl<D: DebugPls, const N: usize> DebugPls for [D; N] {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_list().entries(self).finish()
    }
}

impl<D: DebugPls> DebugPls for Vec<D> {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_list().entries(self).finish()
    }
}

impl DebugPls for str {
    fn fmt(&self, f: Formatter<'_>) {
        f.write_expr(syn::ExprLit {
            attrs: vec![],
            lit: syn::LitStr::new(self, Span::call_site()).into(),
        });
    }
}

impl DebugPls for String {
    fn fmt(&self, f: Formatter<'_>) {
        DebugPls::fmt(self.as_str(), f)
    }
}

impl<K: DebugPls, V: DebugPls> DebugPls for HashMap<K, V> {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_map().entries(self).finish()
    }
}

impl<K: DebugPls, V: DebugPls> DebugPls for BTreeMap<K, V> {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_map().entries(self).finish()
    }
}

impl<V: DebugPls> DebugPls for HashSet<V> {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_set().entries(self).finish()
    }
}

impl<V: DebugPls> DebugPls for BTreeSet<V> {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_set().entries(self).finish()
    }
}
