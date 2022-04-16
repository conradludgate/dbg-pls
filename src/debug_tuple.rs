use syn::__private::Span;

use crate::{DebugPls, Formatter};

pub struct DebugTuple<'a> {
    formatter: Formatter<'a>,
    expr: syn::ExprCall,
}

impl<'a> DebugTuple<'a> {
    pub(crate) fn new(formatter: Formatter<'a>, name: &str) -> Self {
        DebugTuple {
            formatter,
            expr: syn::ExprCall {
                attrs: vec![],
                func: Box::new(syn::Expr::Path(syn::ExprPath {
                    attrs: vec![],
                    qself: None,
                    path: syn::Ident::new(name, Span::call_site()).into(),
                })),
                paren_token: syn::token::Paren::default(),
                args: syn::punctuated::Punctuated::new(),
            },
        }
    }

    pub fn field(mut self, value: &dyn DebugPls) -> Self {
        self.expr.args.push(Formatter::process(value));
        self
    }

    pub fn finish(self) {
        self.formatter.write_expr(self.expr);
    }
}
