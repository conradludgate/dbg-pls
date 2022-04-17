use syn::{
    punctuated::{Pair, Punctuated},
    token::{Bracket, Comma},
    Attribute, Expr, ExprArray, ExprLit, Lit, LitInt, LitStr,
};

use crate::{DebugPls, Formatter};

impl DebugPls for Expr {
    fn fmt(&self, f: Formatter<'_>) {
        match self {
            Expr::Array(val0) => f.debug_tuple_struct("Array").field(val0).finish(),
            // Expr::Assign(val0) => f.debug_tuple_struct("Assign").field(val0).finish(),
            // Expr::AssignOp(val0) => f.debug_tuple_struct("AssignOp").field(val0).finish(),
            // Expr::Async(val0) => f.debug_tuple_struct("Async").field(val0).finish(),
            // Expr::Await(val0) => f.debug_tuple_struct("Await").field(val0).finish(),
            // Expr::Binary(val0) => f.debug_tuple_struct("Binary").field(val0).finish(),
            // Expr::Block(val0) => f.debug_tuple_struct("Block").field(val0).finish(),
            // Expr::Box(val0) => f.debug_tuple_struct("Box").field(val0).finish(),
            // Expr::Break(val0) => f.debug_tuple_struct("Break").field(val0).finish(),
            // Expr::Call(val0) => f.debug_tuple_struct("Call").field(val0).finish(),
            // Expr::Cast(val0) => f.debug_tuple_struct("Cast").field(val0).finish(),
            // Expr::Closure(val0) => f.debug_tuple_struct("Closure").field(val0).finish(),
            // Expr::Continue(val0) => f.debug_tuple_struct("Continue").field(val0).finish(),
            // Expr::Field(val0) => f.debug_tuple_struct("Field").field(val0).finish(),
            // Expr::ForLoop(val0) => f.debug_tuple_struct("ForLoop").field(val0).finish(),
            // Expr::Group(val0) => f.debug_tuple_struct("Group").field(val0).finish(),
            // Expr::If(val0) => f.debug_tuple_struct("If").field(val0).finish(),
            // Expr::Index(val0) => f.debug_tuple_struct("Index").field(val0).finish(),
            // Expr::Let(val0) => f.debug_tuple_struct("Let").field(val0).finish(),
            Expr::Lit(val0) => f.debug_tuple_struct("Lit").field(val0).finish(),
            // Expr::Loop(val0) => f.debug_tuple_struct("Loop").field(val0).finish(),
            // Expr::Macro(val0) => f.debug_tuple_struct("Macro").field(val0).finish(),
            // Expr::Match(val0) => f.debug_tuple_struct("Match").field(val0).finish(),
            // Expr::MethodCall(val0) => f.debug_tuple_struct("MethodCall").field(val0).finish(),
            // Expr::Paren(val0) => f.debug_tuple_struct("Paren").field(val0).finish(),
            // Expr::Path(val0) => f.debug_tuple_struct("Path").field(val0).finish(),
            // Expr::Range(val0) => f.debug_tuple_struct("Range").field(val0).finish(),
            // Expr::Reference(val0) => f.debug_tuple_struct("Reference").field(val0).finish(),
            // Expr::Repeat(val0) => f.debug_tuple_struct("Repeat").field(val0).finish(),
            // Expr::Return(val0) => f.debug_tuple_struct("Return").field(val0).finish(),
            // Expr::Struct(val0) => f.debug_tuple_struct("Struct").field(val0).finish(),
            // Expr::Try(val0) => f.debug_tuple_struct("Try").field(val0).finish(),
            // Expr::TryBlock(val0) => f.debug_tuple_struct("TryBlock").field(val0).finish(),
            // Expr::Tuple(val0) => f.debug_tuple_struct("Tuple").field(val0).finish(),
            // Expr::Type(val0) => f.debug_tuple_struct("Type").field(val0).finish(),
            // Expr::Unary(val0) => f.debug_tuple_struct("Unary").field(val0).finish(),
            // Expr::Unsafe(val0) => f.debug_tuple_struct("Unsafe").field(val0).finish(),
            // Expr::Verbatim(val0) => f.debug_tuple_struct("Verbatim").field(val0).finish(),
            // Expr::While(val0) => f.debug_tuple_struct("While").field(val0).finish(),
            // Expr::Yield(val0) => f.debug_tuple_struct("Yield").field(val0).finish(),
            _ => todo!(),
        }
    }
}

impl DebugPls for ExprArray {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_struct("ExprArray")
            .field("attrs", &self.attrs)
            .field("bracket_token", &self.bracket_token)
            .field("elems", &self.elems)
            .finish();
    }
}

impl DebugPls for ExprLit {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_struct("ExprLit")
            .field("attrs", &self.attrs)
            .field("lit", &self.lit)
            .finish();
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

impl DebugPls for LitStr {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_struct("LitStr")
            .field("value", &self.value())
            .finish();
    }
}

impl DebugPls for LitInt {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_struct("LitInt")
            .field("value", &self.base10_digits())
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

macro_rules! debug_units {
    ($($T:ident),*) => {$(
        impl DebugPls for $T {
            fn fmt(&self, f: Formatter<'_>) {
                f.debug_ident(stringify!($T))
            }
        }
    )*};
}

debug_units![Comma, Bracket];

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
}
