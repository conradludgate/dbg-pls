use crate::{DebugPls, Formatter};

pub struct DebugTuple<'a> {
    formatter: Formatter<'a>,
    expr: syn::ExprTuple,
}

impl<'a> DebugTuple<'a> {
    pub(crate) fn new(formatter: Formatter<'a>) -> Self {
        DebugTuple {
            formatter,
            expr: syn::ExprTuple {
                attrs: vec![],
                paren_token: syn::token::Paren::default(),
                elems: syn::punctuated::Punctuated::new(),
            },
        }
    }

    pub fn field(mut self, value: &dyn DebugPls) -> Self {
        self.expr.elems.push(Formatter::process(value));
        self
    }

    pub fn finish(self) {
        self.formatter.write_expr(self.expr);
    }
}
