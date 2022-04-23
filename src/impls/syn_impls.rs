use proc_macro2::{Literal, Span};
use syn::{
    parse_str,
    punctuated::{Pair, Punctuated},
    token::{Bracket, Colon, Colon2, Comma, Dot, Eq, Paren, Semi, Sub},
    Attribute, BinOp, Expr, ExprArray, ExprAssign, ExprBinary, ExprCall, ExprIndex, ExprLit,
    ExprMethodCall, ExprPath, Ident, Lit, LitInt, LitStr, MethodTurbofish, Path, PathArguments,
    PathSegment, QSelf, Stmt, Item,
};

use crate::{DebugPls, Formatter};

impl DebugPls for Stmt {
    fn fmt(&self, f: Formatter<'_>) {
        match self {
            Stmt::Local(local) => f.debug_tuple_struct("Local").field(local).finish(),
            Stmt::Item(item) => f.debug_tuple_struct("Item").field(item).finish(),
            Stmt::Expr(expr) => f.debug_tuple_struct("Expr").field(expr).finish(),
            Stmt::Semi(expr, semi) => f
                .debug_tuple_struct("Semi")
                .field(expr)
                .field(semi)
                .finish(),
        }
    }
}


impl DebugPls for Item {
    fn fmt(&self, f: Formatter<'_>) {
        match self {
            // Item::Const(_) => todo!(),
            // Item::Enum(_) => todo!(),
            // Item::ExternCrate(_) => todo!(),
            // Item::Fn(_) => todo!(),
            // Item::ForeignMod(_) => todo!(),
            // Item::Impl(_) => todo!(),
            // Item::Macro(_) => todo!(),
            // Item::Macro2(_) => todo!(),
            // Item::Mod(_) => todo!(),
            // Item::Static(_) => todo!(),
            // Item::Struct(_) => todo!(),
            // Item::Trait(_) => todo!(),
            // Item::TraitAlias(_) => todo!(),
            // Item::Type(_) => todo!(),
            // Item::Union(_) => todo!(),
            // Item::Use(_) => todo!(),
            _ => todo!(),
        }
    }
}


impl DebugPls for BinOp {
    fn fmt(&self, f: Formatter<'_>) {
        match self {
            BinOp::Add(_) => todo!(),
            BinOp::Sub(v0) => f.debug_tuple_struct("Sub").field(v0).finish(),
            BinOp::Mul(_) => todo!(),
            BinOp::Div(_) => todo!(),
            BinOp::Rem(_) => todo!(),
            BinOp::And(_) => todo!(),
            BinOp::Or(_) => todo!(),
            BinOp::BitXor(_) => todo!(),
            BinOp::BitAnd(_) => todo!(),
            BinOp::BitOr(_) => todo!(),
            BinOp::Shl(_) => todo!(),
            BinOp::Shr(_) => todo!(),
            BinOp::Eq(_) => todo!(),
            BinOp::Lt(_) => todo!(),
            BinOp::Le(_) => todo!(),
            BinOp::Ne(_) => todo!(),
            BinOp::Ge(_) => todo!(),
            BinOp::Gt(_) => todo!(),
            BinOp::AddEq(_) => todo!(),
            BinOp::SubEq(_) => todo!(),
            BinOp::MulEq(_) => todo!(),
            BinOp::DivEq(_) => todo!(),
            BinOp::RemEq(_) => todo!(),
            BinOp::BitXorEq(_) => todo!(),
            BinOp::BitAndEq(_) => todo!(),
            BinOp::BitOrEq(_) => todo!(),
            BinOp::ShlEq(_) => todo!(),
            BinOp::ShrEq(_) => todo!(),
        }
    }
}


macro_rules! debug_units {
    ($($T:ident),*) => {$(
        impl DebugPls for $T {
            fn fmt(&self, f: Formatter<'_>) {
                f.debug_ident(stringify!($T))
            }
        }
    )*};
}

debug_units![Comma, Bracket, Semi, Eq, Colon, Colon2, Sub, Dot, Paren];

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
