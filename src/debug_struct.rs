use syn::__private::Span;

use crate::{DebugPls, Formatter};

pub struct DebugStruct<'a> {
    formatter: Formatter<'a>,
    expr: syn::ExprStruct,
}

impl<'a> DebugStruct<'a> {
    pub(crate) fn new(formatter: Formatter<'a>, name: &str) -> Self {
        DebugStruct {
            formatter,
            expr: syn::ExprStruct {
                attrs: vec![],
                path: syn::Ident::new(name, Span::call_site()).into(),
                brace_token: syn::token::Brace::default(),
                fields: syn::punctuated::Punctuated::new(),
                dot2_token: None,
                rest: None,
            },
        }
    }

    pub fn field(mut self, name: &str, value: &dyn DebugPls) -> DebugStruct<'a> {
        self.expr.fields.push(syn::FieldValue {
            expr: Formatter::process(value),
            attrs: vec![],
            member: syn::Member::Named(syn::Ident::new(name, Span::call_site())),
            colon_token: Some(syn::token::Colon::default()),
        });
        self
    }

    pub fn finish(self) {
        *self.formatter.expr = syn::Expr::Struct(self.expr);
    }

    pub fn finish_non_exhaustive(mut self) {
        self.expr.dot2_token = Some(syn::token::Dot2::default());
        self.finish()
    }
}
