use syn::{Local, Stmt};

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

impl DebugPls for Local {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_struct("Local")
            .field("attrs", &self.attrs)
            .field("let_token", &self.let_token)
            .field("pat", &self.pat)
            .field("init", &self.init)
            .field("semi_token", &self.semi_token)
            .finish();
    }
}
