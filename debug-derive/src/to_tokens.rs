use crate::{DebugImpl, Var};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{spanned::Spanned, Field, Fields};

impl ToTokens for DebugImpl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            krate,
            ident,
            generics,
            variants,
        } = self;

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        tokens.extend(quote! {
            impl #impl_generics #krate::DebugPls for #ident #ty_generics #where_clause {
                fn fmt(&self, f: #krate::Formatter<'_>) {
                    match *self {
                        #( #variants )*
                    }
                }
            }
        })
    }
}

impl<'a> ToTokens for Var {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Var { path, fields } = self;
        let name = path.segments.last().unwrap().ident.to_string();
        match fields {
            Fields::Named(n) => {
                let pat_args = n.named.iter().map(|f| f.ident.as_ref().unwrap().clone());
                let args = pat_args.clone().map(|f| {
                    let name = f.to_string();
                    quote! { #name, #f }
                });
                tokens.extend(quote! {
                    #path { #( ref #pat_args ),* } => f.debug_struct(#name) #( .field(#args) )* .finish(),
                });
            }
            Fields::Unnamed(n) => {
                pub fn ident((i, field): (usize, &Field)) -> Ident {
                    format_ident!("__self_{}", i, span = field.span())
                }
                let pat_args = n.unnamed.iter().enumerate().map(ident);
                let args = pat_args.clone();
                tokens.extend(quote! {
                    #path ( #( ref #pat_args ),* ) => f.debug_tuple_struct(#name) #( .field(#args) )* .finish(),
                });
            }
            Fields::Unit => tokens.extend(quote! { #path => f.debug_ident(#name), }),
        };
    }
}
