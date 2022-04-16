use crate::{DebugPls, Formatter};

pub struct DebugList<'a> {
    formatter: Formatter<'a>,
    expr: syn::ExprArray,
}

impl<'a> DebugList<'a> {
    pub(crate) fn new(formatter: Formatter<'a>) -> Self {
        DebugList {
            formatter,
            expr: syn::ExprArray {
                attrs: vec![],
                bracket_token: syn::token::Bracket::default(),
                elems: syn::punctuated::Punctuated::default(),
            },
        }
    }

    pub fn entry(mut self, entry: &dyn DebugPls) -> Self {
        self.expr.elems.push(Formatter::process(entry));
        self
    }

    pub fn finish(self) {
        *self.formatter.expr = syn::Expr::Array(self.expr);
    }
}
