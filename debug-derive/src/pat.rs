use proc_macro2::{Delimiter, Group, Ident, TokenStream as TokenStream2, TokenTree};
use quote::{format_ident, quote, ToTokens};
use syn::{Fields, FieldsNamed, FieldsUnnamed};

pub struct PatternImpl<T>(pub T);

impl<'a> ToTokens for PatternImpl<&'a Fields> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match &self.0 {
            Fields::Named(named) => PatternImpl(named).to_tokens(tokens),
            Fields::Unnamed(unnamed) => PatternImpl(unnamed).to_tokens(tokens),
            Fields::Unit => {}
        }
    }
}

impl<'a> ToTokens for PatternImpl<&'a FieldsNamed> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let mut inner = TokenStream2::new();
        named_idents(self.0).for_each(|ident| inner.extend(quote! { #ident, }));
        tokens.extend([TokenTree::Group(Group::new(Delimiter::Brace, inner))])
    }
}

impl<'a> ToTokens for PatternImpl<&'a FieldsUnnamed> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let mut inner = TokenStream2::new();
        unnamed_idents(self.0).for_each(|ident| inner.extend(quote! { #ident, }));
        tokens.extend([TokenTree::Group(Group::new(Delimiter::Parenthesis, inner))])
    }
}

pub fn unnamed_idents(fields: &FieldsUnnamed) -> impl Iterator<Item = Ident> + '_ {
    fields
        .unnamed
        .iter()
        .enumerate()
        .map(|(i, _)| format_ident!("val{}", i))
}
pub fn named_idents(fields: &FieldsNamed) -> impl Iterator<Item = &Ident> + '_ {
    fields
        .named
        .iter()
        .map(|field| field.ident.as_ref().unwrap())
}