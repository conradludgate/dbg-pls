use syn::punctuated::{Pair, Punctuated};

mod gen;

use crate::{DebugPls, Formatter};

struct DebugSelf<T>(T);
impl<T: Into<syn::Expr> + Clone> DebugPls for DebugSelf<T> {
    fn fmt(&self, f: Formatter<'_>) {
        f.write_expr(self.0.clone());
    }
}

struct DebugLit<T>(T);
impl<T: Into<syn::Lit> + Clone> DebugPls for DebugLit<T> {
    fn fmt(&self, f: Formatter<'_>) {
        f.write_expr(syn::Expr::Lit(syn::ExprLit {
            attrs: vec![],
            lit: self.0.clone().into(),
        }));
    }
}

impl DebugPls for syn::LitStr {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_struct("LitStr")
            .field("token", &DebugLit(self.clone()))
            .finish();
    }
}

impl DebugPls for syn::LitInt {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_struct("LitInt")
            .field("token", &DebugLit(self.clone()))
            .finish();
    }
}

impl DebugPls for syn::LitByte {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_struct("LitByte")
            .field("token", &DebugLit(self.clone()))
            .finish();
    }
}

impl DebugPls for syn::LitChar {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_struct("LitChar")
            .field("token", &DebugLit(self.clone()))
            .finish();
    }
}

impl DebugPls for syn::LitBool {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_struct("LitBool")
            .field("token", &DebugLit(self.clone()))
            .finish();
    }
}

impl DebugPls for syn::LitFloat {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_struct("LitFloat")
            .field("token", &DebugLit(self.clone()))
            .finish();
    }
}

impl DebugPls for syn::LitByteStr {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_struct("LitByteStr")
            .field("token", &DebugLit(self.clone()))
            .finish();
    }
}

impl DebugPls for syn::token::Group {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_ident("Group");
    }
}

impl DebugPls for syn::token::Brace {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_ident("Brace");
    }
}

impl DebugPls for syn::token::Bracket {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_ident("Bracket");
    }
}

impl DebugPls for syn::token::Paren {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_ident("Paren");
    }
}

impl<T: DebugPls, P: DebugPls> DebugPls for Punctuated<T, P> {
    fn fmt(&self, f: Formatter<'_>) {
        self.pairs()
            .fold(f.debug_list(), |f, pair| match pair {
                Pair::Punctuated(t, p) => f.entry(t).entry(p),
                Pair::End(t) => f.entry(t),
            })
            .finish();
    }
}

#[cfg(test)]
mod tests {
    macro_rules! assert_pretty_syn_snapshot {
        ($ty:ty => $code:literal) => {
            let parsed = &syn::parse_str::<$ty>($code).unwrap();
            let format = format!("pretty:\n{}\n\ncolor:\n{}\n", crate::pretty(&parsed), crate::color(&parsed));
            insta::assert_snapshot!(
                insta::_macro_support::AutoName,
                &format,
                $code
            );
        };
    }

    #[test]
    fn expr_array() {
        assert_pretty_syn_snapshot!(syn::Expr => r#"
            [
                "Hello, World! I am a long string",
                420,
                "Wait, you can't mix and match types in arrays, is this python?",
                69,
                "Nice."
            ]
        "#);
    }

    #[test]
    fn item_macro() {
        assert_pretty_syn_snapshot!(syn::Item => r#"
            macro_rules! assert_pretty_syn_snapshot {
                ($ty:ty => $code:literal) => {
                    insta::assert_snapshot!(
                        insta::_macro_support::AutoName,
                        &crate::pretty(&syn::parse_str::<$ty>($code).unwrap()).to_string(),
                        $code
                    );
                };
            }
        "#);
    }
}
