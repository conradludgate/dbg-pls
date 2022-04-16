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

impl DebugPls for i32 {
    fn fmt(&self, f: Formatter<'_>) {
        f.write_expr(syn::ExprLit {
            attrs: vec![],
            lit: syn::LitInt::new(&format!("{}", self), Span::call_site()).into(),
        });
    }
}

impl<D: DebugPls> DebugPls for [D] {
    fn fmt(&self, f: Formatter<'_>) {
        self.iter()
            .fold(f.debug_list(), |list, value| list.entry(value))
            .finish()
    }
}

impl<D: DebugPls, const N: usize> DebugPls for [D; N] {
    fn fmt(&self, f: Formatter<'_>) {
        DebugPls::fmt(self.as_slice(), f)
    }
}

impl<D: DebugPls> DebugPls for Vec<D> {
    fn fmt(&self, f: Formatter<'_>) {
        DebugPls::fmt(self.as_slice(), f)
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
