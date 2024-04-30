use crate::{DebugImpl, Var};
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
            with_ident,
        } = self;

        let generic_params = generics.params.iter();
        let (_, ty_generics, where_clause) = generics.split_for_impl();

        let body = match mode {
            crate::Mode::Struct(fields) => {
                let name = ident.to_string();

                let fields = match &fields.0 {
                    Fields::Named(n) => {
                        let pat_args = n.named.iter().map(|f| f.ident.as_ref().unwrap().clone());
                        let args = pat_args.map(|f| {
                            let name = f.to_string();
                            quote! { #name, &self.#f as &dyn #krate::DebugWith<#with_ident> }
                        });
                        quote! {
                            f.debug_struct(name) #( .field_with(#args, __with_associated_data) )* .finish()
                        }
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
                        quote! {
                            f.debug_tuple_struct(name) #( .field_with(&self.#args as &dyn #krate::DebugWith<#with_ident>, __with_associated_data) )* .finish()
                        }
                    }
                    Fields::Unit => quote! { f.debug_ident(name) },
                };

                quote! {{
                    let name = #name;
                    #fields
                }}
            }
            crate::Mode::Enum(variants) => {
                let variants = variants.iter().map(|var| {
                let Var { ident, fields } = var;
                let name = ident.to_string();
                match fields {
                    Fields::Named(n) => {
                        let pat_args = n.named.iter().map(|f| f.ident.as_ref().unwrap().clone());
                        let args = pat_args.clone().map(|f| {
                            let name = f.to_string();
                            quote! { #name, #f as &dyn #krate::DebugWith<#with_ident> }
                        });
                        quote! {
                            Self::#ident { #( ref #pat_args ),* } => f.debug_struct(#name) #( .field_with(#args, __with_associated_data) )* .finish(),
                        }
                    }
                    Fields::Unnamed(n) => {
                        pub fn i((i, field): (usize, &Field)) -> Ident {
                            format_ident!("__self_{}", i, span = field.span())
                        }
                        let pat_args = n.unnamed.iter().enumerate().map(i);
                        let args = pat_args.clone();
                        quote! {
                            Self::#ident ( #( ref #pat_args ),* ) => f.debug_tuple_struct(#name) #( .field_with(#args as &dyn #krate::DebugWith<#with_ident>, __with_associated_data) )* .finish(),
                        }
                    }
                    Fields::Unit => quote! { Self::#ident => f.debug_ident(#name), },
                }});

                quote! {{
                    match *self {
                        #( #variants )*
                    }
                }}
            }
        };
        tokens.extend(quote! {
            #[automatically_derived]
            impl <#(#generic_params,)* #with_ident> #krate::DebugWith<#with_ident> for #ident #ty_generics #where_clause {
                fn fmt(&self, __with_associated_data: &#with_ident, f: #krate::Formatter<'_>) #body
            }
        })
    }
}
