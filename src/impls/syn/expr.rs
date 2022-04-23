use syn::{Expr, ExprArray, ExprAssign, ExprBinary, ExprIndex, ExprLit, ExprMethodCall, ExprPath};

use crate::{DebugPls, Formatter};

mod binop;

impl DebugPls for Expr {
    fn fmt(&self, f: Formatter<'_>) {
        match self {
            Expr::Array(val0) => f.debug_tuple_struct("Array").field(val0).finish(),
            Expr::Assign(val0) => f.debug_tuple_struct("Assign").field(val0).finish(),
            // Expr::AssignOp(val0) => f.debug_tuple_struct("AssignOp").field(val0).finish(),
            // Expr::Async(val0) => f.debug_tuple_struct("Async").field(val0).finish(),
            // Expr::Await(val0) => f.debug_tuple_struct("Await").field(val0).finish(),
            Expr::Binary(val0) => f.debug_tuple_struct("Binary").field(val0).finish(),
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
            Expr::Index(val0) => f.debug_tuple_struct("Index").field(val0).finish(),
            // Expr::Let(val0) => f.debug_tuple_struct("Let").field(val0).finish(),
            Expr::Lit(val0) => f.debug_tuple_struct("Lit").field(val0).finish(),
            // Expr::Loop(val0) => f.debug_tuple_struct("Loop").field(val0).finish(),
            // Expr::Macro(val0) => f.debug_tuple_struct("Macro").field(val0).finish(),
            // Expr::Match(val0) => f.debug_tuple_struct("Match").field(val0).finish(),
            Expr::MethodCall(val0) => f.debug_tuple_struct("MethodCall").field(val0).finish(),
            // Expr::Paren(val0) => f.debug_tuple_struct("Paren").field(val0).finish(),
            Expr::Path(val0) => f.debug_tuple_struct("Path").field(val0).finish(),
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

impl DebugPls for ExprIndex {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_struct("ExprIndex")
            .field("attrs", &self.attrs)
            .field("expr", &self.expr)
            .field("bracket_token", &self.bracket_token)
            .field("index", &self.index)
            .finish();
    }
}

impl DebugPls for ExprBinary {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_struct("ExprBinary")
            .field("attrs", &self.attrs)
            .field("left", &self.left)
            .field("op", &self.op)
            .field("right", &self.right)
            .finish();
    }
}

impl DebugPls for ExprMethodCall {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_struct("ExprMethodCall")
            .field("attrs", &self.attrs)
            .field("receiver", &self.receiver)
            .field("dot_token", &self.dot_token)
            .field("method", &self.method)
            .field("turbofish", &self.turbofish)
            .field("paren_token", &self.paren_token)
            .field("args", &self.args)
            .finish();
    }
}

impl DebugPls for ExprAssign {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_struct("ExprAssign")
            .field("attrs", &self.attrs)
            .field("left", &self.left)
            .field("eq_token", &self.eq_token)
            .field("right", &self.right)
            .finish();
    }
}

impl DebugPls for ExprPath {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_struct("ExprPath")
            .field("attrs", &self.attrs)
            .field("qself", &self.qself)
            .field("path", &self.path)
            .finish();
    }
}
