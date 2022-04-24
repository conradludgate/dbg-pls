use syn::{
    punctuated::{Pair, Punctuated},
    Attribute, Ident, Lit, LitInt, LitStr, MethodTurbofish, Path, PathArguments, PathSegment,
    QSelf, Member, Index,
};

use crate::{DebugPls, Formatter};

mod expr;
mod item;
mod pat;
mod stmt;
mod token;
mod ty;

impl DebugPls for MethodTurbofish {
    fn fmt(&self, _f: Formatter<'_>) {
        todo!()
    }
}

impl DebugPls for Path {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_struct("Path")
            .field("leading_colon", &self.leading_colon)
            .field("segments", &self.segments)
            .finish();
    }
}

impl DebugPls for PathSegment {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_struct("PathSegment")
            .field("ident", &self.ident)
            .field("arguments", &self.arguments)
            .finish();
    }
}

impl DebugPls for Ident {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_tuple_struct("Ident")
            .field(&IdentSym(self.clone()))
            .finish();
    }
}

impl DebugPls for PathArguments {
    fn fmt(&self, f: Formatter<'_>) {
        match self {
            PathArguments::None => f.debug_ident("None"),
            // PathArguments::AngleBracketed(_) => todo!(),
            // PathArguments::Parenthesized(_) => todo!(),
            _ => todo!(),
        }
    }
}

impl DebugPls for QSelf {
    fn fmt(&self, _f: Formatter<'_>) {
        todo!()
    }
}

struct IdentSym(Ident);
impl DebugPls for IdentSym {
    fn fmt(&self, f: Formatter<'_>) {
        f.write_expr(syn::ExprPath {
            path: self.0.clone().into(),
            attrs: vec![],
            qself: None,
        });
    }
}

impl DebugPls for Lit {
    fn fmt(&self, f: Formatter<'_>) {
        match self {
            Lit::Str(v0) => f.debug_tuple_struct("Str").field(v0).finish(),
            // Lit::ByteStr(v0) => f.debug_tuple_struct("ByteStr").field(v0).finish(),
            // Lit::Byte(v0) => f.debug_tuple_struct("Byte").field(v0).finish(),
            // Lit::Char(v0) => f.debug_tuple_struct("Char").field(v0).finish(),
            Lit::Int(v0) => f.debug_tuple_struct("Int").field(v0).finish(),
            // Lit::Float(v0) => f.debug_tuple_struct("Float").field(v0).finish(),
            // Lit::Bool(v0) => f.debug_tuple_struct("Bool").field(v0).finish(),
            // Lit::Verbatim(v0) => f.debug_tuple_struct("Verbatim").field(v0).finish(),
            _ => todo!(),
        }
    }
}

struct DebugSelf<T>(T);
impl<T: Into<syn::Expr> + Clone> DebugPls for DebugSelf<T> {
    fn fmt(&self, f: Formatter<'_>) {
        f.write_expr(self.0.clone());
    }
}

impl DebugPls for LitStr {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_struct("LitStr")
            .field(
                "token",
                &DebugSelf(syn::Expr::Lit(syn::ExprLit {
                    attrs: vec![],
                    lit: Lit::Str(self.clone()),
                })),
            )
            .finish();
    }
}

impl DebugPls for LitInt {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_struct("LitInt")
            .field(
                "token",
                &DebugSelf(syn::Expr::Lit(syn::ExprLit {
                    attrs: vec![],
                    lit: Lit::Int(self.clone()),
                })),
            )
            .finish();
    }
}

impl DebugPls for Attribute {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_struct("Attribute").finish_non_exhaustive();
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

impl DebugPls for Member {
    fn fmt(&self, f: Formatter<'_>) {
        match self {
            Member::Named(n) => f.debug_tuple_struct("Named").field(n).finish(),
            Member::Unnamed(u) => f.debug_tuple_struct("Unnamed").field(u).finish(),
        }
    }
}

impl DebugPls for Index {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_tuple_struct("Index").field(&self.index).finish();
    }
}

#[cfg(test)]
mod tests {
    use crate::color;

    #[test]
    fn pretty_colors() {
        let code = r#"
            [
                "Hello, World! I am a long string",
                420,
                "Wait, you can't mix and match types in arrays, is this python?",
                69,
                "Nice."
            ]
        "#;
        let expr: syn::Expr = syn::parse_str(code).unwrap();
        println!("{}", color(&expr));
    }

    #[test]
    fn large_tree() {
        let code = r#"foo[foo.len() - 1] = 42;"#;
        let expr: syn::Stmt = syn::parse_str(code).unwrap();
        println!("{}", color(&expr));
        println!("{:#?}", expr);
    }
}
