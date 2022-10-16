use crate::{DebugImpl, StructFields, Var};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{spanned::Spanned, Field, Fields, Index};

impl ToTokens for DebugImpl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            krate,
            ident,
            generics,
            mode,
        } = self;

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let body = match mode {
            crate::Mode::Struct(fields) => {
                let name = ident.to_string();
                quote! {{
                    let name = #name;
                    #fields
                }}
            }
            crate::Mode::Enum(variants) => quote! {{
                match *self {
                    #( #variants )*
                }
            }},
        };
        tokens.extend(quote! {
            #[automatically_derived]
            impl #impl_generics #krate::DebugPls for #ident #ty_generics #where_clause {
                fn fmt(&self, f: #krate::Formatter<'_>) #body
            }
        })
    }
}

impl ToTokens for Var {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Var { ident, fields } = self;
        let name = ident.to_string();
        match fields {
            Fields::Named(n) => {
                let pat_args = n.named.iter().map(|f| f.ident.as_ref().unwrap().clone());
                let args = pat_args.clone().map(|f| {
                    let name = f.to_string();
                    quote! { #name, #f }
                });
                tokens.extend(quote! {
                    Self::#ident { #( ref #pat_args ),* } => f.debug_struct(#name) #( .field(#args) )* .finish(),
                });
            }
            Fields::Unnamed(n) => {
                pub fn i((i, field): (usize, &Field)) -> Ident {
                    format_ident!("__self_{}", i, span = field.span())
                }
                let pat_args = n.unnamed.iter().enumerate().map(i);
                let args = pat_args.clone();
                tokens.extend(quote! {
                    Self::#ident ( #( ref #pat_args ),* ) => f.debug_tuple_struct(#name) #( .field(#args) )* .finish(),
                });
            }
            Fields::Unit => tokens.extend(quote! { Self::#ident => f.debug_ident(#name), }),
        };
    }
}

impl ToTokens for StructFields {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let fields = &self.0;
        match fields {
            Fields::Named(n) => {
                let pat_args = n.named.iter().map(|f| f.ident.as_ref().unwrap().clone());
                let args = pat_args.clone().map(|f| {
                    let name = f.to_string();
                    quote! { #name, &self.#f }
                });
                tokens.extend(quote! {
                    f.debug_struct(name) #( .field(#args) )* .finish()
                });
            }
            Fields::Unnamed(n) => {
                pub fn i((i, field): (usize, &Field)) -> Index {
                    Index {
                        index: i as u32,
                        span: field.span(),
                    }
                }
                let pat_args = n.unnamed.iter().enumerate().map(i);
                let args = pat_args.clone();
                tokens.extend(quote! {
                    f.debug_tuple_struct(name) #( .field(&self.#args) )* .finish()
                });
            }
            Fields::Unit => tokens.extend(quote! { f.debug_ident(name) }),
        };
    }
}
