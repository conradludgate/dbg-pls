use syn::punctuated::{Pair, Punctuated};

#[allow(clippy::match_single_binding, unused_variables)]
mod gen;

use crate::{DebugWith, Formatter};

struct DebugSelf<T>(T);
impl<W, T: Into<syn::Expr> + Clone> DebugWith<W> for DebugSelf<T> {
    fn fmt(&self, _with: &W, f: Formatter<'_>) {
        f.write_expr(self.0.clone());
    }
}

struct DebugLit<'a, T>(&'a T);
impl<W, T: Into<syn::Lit> + Clone> DebugWith<W> for DebugLit<'_, T> {
    fn fmt(&self, _with: &W, f: Formatter<'_>) {
        f.write_expr(syn::Expr::Lit(syn::ExprLit {
            attrs: vec![],
            lit: self.0.clone().into(),
        }));
    }
}

impl<W> DebugWith<W> for syn::LitStr {
    fn fmt(&self, with: &W, f: Formatter<'_>) {
        f.debug_struct("LitStr")
            .field_with("token", &DebugLit(self) as &dyn DebugWith<W>, with)
            .finish();
    }
}

impl<W> DebugWith<W> for syn::LitCStr {
    fn fmt(&self, with: &W, f: Formatter<'_>) {
        f.debug_struct("LitCStr")
            .field_with("token", &DebugLit(self) as &dyn DebugWith<W>, with)
            .finish();
    }
}

impl<W> DebugWith<W> for syn::LitInt {
    fn fmt(&self, with: &W, f: Formatter<'_>) {
        f.debug_struct("LitInt")
            .field_with("token", &DebugLit(self) as &dyn DebugWith<W>, with)
            .finish();
    }
}

impl<W> DebugWith<W> for syn::LitByte {
    fn fmt(&self, with: &W, f: Formatter<'_>) {
        f.debug_struct("LitByte")
            .field_with("token", &DebugLit(self) as &dyn DebugWith<W>, with)
            .finish();
    }
}

impl<W> DebugWith<W> for syn::LitChar {
    fn fmt(&self, with: &W, f: Formatter<'_>) {
        f.debug_struct("LitChar")
            .field_with("token", &DebugLit(self) as &dyn DebugWith<W>, with)
            .finish();
    }
}

impl<W> DebugWith<W> for syn::LitBool {
    fn fmt(&self, with: &W, f: Formatter<'_>) {
        f.debug_struct("LitBool")
            .field_with("token", &DebugLit(self) as &dyn DebugWith<W>, with)
            .finish();
    }
}

impl<W> DebugWith<W> for syn::LitFloat {
    fn fmt(&self, with: &W, f: Formatter<'_>) {
        f.debug_struct("LitFloat")
            .field_with("token", &DebugLit(self) as &dyn DebugWith<W>, with)
            .finish();
    }
}

impl<W> DebugWith<W> for syn::LitByteStr {
    fn fmt(&self, with: &W, f: Formatter<'_>) {
        f.debug_struct("LitByteStr")
            .field_with("token", &DebugLit(self) as &dyn DebugWith<W>, with)
            .finish();
    }
}

impl<W> DebugWith<W> for syn::token::Group {
    fn fmt(&self, _with: &W, f: Formatter<'_>) {
        f.debug_ident("Group");
    }
}

impl<W> DebugWith<W> for syn::token::Brace {
    fn fmt(&self, _with: &W, f: Formatter<'_>) {
        f.debug_ident("Brace");
    }
}

impl<W> DebugWith<W> for syn::token::Bracket {
    fn fmt(&self, _with: &W, f: Formatter<'_>) {
        f.debug_ident("Bracket");
    }
}

impl<W> DebugWith<W> for syn::token::Paren {
    fn fmt(&self, _with: &W, f: Formatter<'_>) {
        f.debug_ident("Paren");
    }
}

impl<W, T: DebugWith<W>, P: DebugWith<W>> DebugWith<W> for Punctuated<T, P> {
    fn fmt(&self, with: &W, f: Formatter<'_>) {
        self.pairs()
            .fold(f.debug_list(), |f, pair| match pair {
                Pair::Punctuated(t, p) => f.entry_with(t, with).entry_with(p, with),
                Pair::End(t) => f.entry_with(t, with),
            })
            .finish();
    }
}
