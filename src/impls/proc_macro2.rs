use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

use crate::{DebugWith, Formatter};

impl<W> DebugWith<W> for TokenStream {
    fn fmt(&self, _with: &W, f: Formatter<'_>) {
        f.debug_list().entries(self.clone()).finish();
    }
}

impl<W> DebugWith<W> for TokenTree {
    fn fmt(&self, _with: &W, f: Formatter<'_>) {
        match self {
            TokenTree::Group(v0) => f.debug_tuple_struct("Group").field(v0),
            TokenTree::Ident(v0) => f.debug_tuple_struct("Ident").field(v0),
            TokenTree::Punct(v0) => f.debug_tuple_struct("Punct").field(v0),
            TokenTree::Literal(v0) => f.debug_tuple_struct("Literal").field(v0),
        }
        .finish();
    }
}

impl<W> DebugWith<W> for Span {
    fn fmt(&self, _with: &W, f: Formatter<'_>) {
        f.debug_ident("Span");
    }
}

impl<W> DebugWith<W> for Ident {
    fn fmt(&self, _with: &W, f: Formatter<'_>) {
        f.write_expr(syn::ExprPath {
            path: self.clone().into(),
            attrs: vec![],
            qself: None,
        });
    }
}

impl<W> DebugWith<W> for Group {
    fn fmt(&self, _with: &W, f: Formatter<'_>) {
        f.debug_struct("Group")
            .field("delimiter", &self.delimiter())
            .field("stream", &self.stream())
            .finish();
    }
}

impl<W> DebugWith<W> for Punct {
    fn fmt(&self, _with: &W, f: Formatter<'_>) {
        f.debug_struct("Punct")
            .field("ch", &self.as_char())
            .field("spacing", &self.spacing())
            .finish();
    }
}

impl<W> DebugWith<W> for Literal {
    fn fmt(&self, _with: &W, f: Formatter<'_>) {
        f.debug_struct("Literal").finish_non_exhaustive();
    }
}

impl<W> DebugWith<W> for Spacing {
    fn fmt(&self, _with: &W, f: Formatter<'_>) {
        match self {
            Spacing::Alone => f.debug_ident("Alone"),
            Spacing::Joint => f.debug_ident("Joint"),
        }
    }
}

impl<W> DebugWith<W> for Delimiter {
    fn fmt(&self, _with: &W, f: Formatter<'_>) {
        match self {
            Delimiter::Parenthesis => f.debug_ident("Parenthesis"),
            Delimiter::Brace => f.debug_ident("Brace"),
            Delimiter::Bracket => f.debug_ident("Bracket"),
            Delimiter::None => f.debug_ident("None"),
        }
    }
}
