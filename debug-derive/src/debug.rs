use crate::pat::{named_idents, unnamed_idents, PatternImpl};
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::{
    spanned::Spanned, Data, DataEnum, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed,
    Path, Variant,
};

pub struct DebugImpl<T>(pub T);

impl ToTokens for DebugImpl<(Path, DeriveInput)> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let (path, input) = &self.0;

        let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

        let ident = &input.ident;
        let body = DebugImpl((ident, &input.data));

        tokens.extend(quote! {
            impl #impl_generics #path::DebugPls for #ident #ty_generics #where_clause {
                fn fmt(&self, f: #path::Formatter<'_>) {
                    #body
                }
            }
        })
    }
}

impl<'a> ToTokens for DebugImpl<(&'a Ident, &'a Data)> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let (name, data) = self.0;
        match data {
            Data::Struct(s) => DebugImpl((name, s)).to_tokens(tokens),
            Data::Enum(e) => DebugImpl((name, e)).to_tokens(tokens),
            Data::Union(_) => tokens
                .extend(syn::Error::new(self.span(), "unions not supported").into_compile_error()),
        }
    }
}

impl<'a> ToTokens for DebugImpl<(&'a Ident, &'a DataStruct)> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let (name, data) = self.0;
        DebugImpl((name, &data.fields)).to_tokens(tokens)
    }
}

impl<'a> ToTokens for DebugImpl<(&'a Ident, &'a DataEnum)> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let (name, data) = self.0;
        let variants = data.variants.iter().map(|v| DebugImpl((name, v)));
        tokens.extend(quote! {
            match self {
                #( #variants )*
            }
        });
    }
}

impl<'a> ToTokens for DebugImpl<(&'a Ident, &'a Variant)> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let (name, variant) = self.0;
        let Variant {
            attrs: _,
            ident,
            fields,
            discriminant: _,
        } = &variant;
        let pattern = PatternImpl(fields);
        let debug = EnumImpl((ident, fields));

        tokens.extend(quote! {
            #name::#ident #pattern => { #debug }
        });
    }
}

impl<'a> ToTokens for DebugImpl<(&'a Ident, &'a Fields)> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let (name, fields) = self.0;
        match fields {
            Fields::Named(named) => DebugImpl((name, named)).to_tokens(tokens),
            Fields::Unnamed(unnamed) => DebugImpl((name, unnamed)).to_tokens(tokens),
            Fields::Unit => {
                let name = name.to_string();
                tokens.extend(quote! {
                    f.debug_ident(#name)
                })
            }
        }
    }
}

impl<'a> ToTokens for DebugImpl<(&'a Ident, &'a FieldsNamed)> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let (ident, fields) = self.0;
        let pat = PatternImpl(fields).into_token_stream();
        let name = ident.to_string();

        tokens.extend(quote! {
            let #ident #pat = &self;
            f.debug_struct(#name)
        });
        named_idents(fields).for_each(|ident| {
            let name = ident.to_string();
            tokens.extend(quote! {
                .field(#name, #ident)
            })
        });
        tokens.extend(quote! {
            .finish()
        });
    }
}

impl<'a> ToTokens for DebugImpl<(&'a Ident, &'a FieldsUnnamed)> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let (ident, fields) = self.0;
        let pat = PatternImpl(fields);
        let name = ident.to_string();

        tokens.extend(quote! {
            let #ident #pat = self;
            f.debug_tuple_struct(#name)
        });
        unnamed_idents(fields).for_each(|ident| {
            tokens.extend(quote! {
                .field(#ident)
            })
        });
        tokens.extend(quote! {
            .finish()
        });
    }
}

pub struct EnumImpl<T>(pub T);
impl<'a> ToTokens for EnumImpl<(&'a Ident, &'a Fields)> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let (name, fields) = self.0;
        match fields {
            Fields::Named(named) => EnumImpl((name, named)).to_tokens(tokens),
            Fields::Unnamed(unnamed) => EnumImpl((name, unnamed)).to_tokens(tokens),
            Fields::Unit => {
                let name = name.to_string();
                tokens.extend(quote! {
                    f.debug_ident(#name)
                })
            }
        }
    }
}

impl<'a> ToTokens for EnumImpl<(&'a Ident, &'a FieldsNamed)> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let (ident, fields) = self.0;
        let name = ident.to_string();

        tokens.extend(quote! {
            f.debug_struct(#name)
        });
        named_idents(fields).for_each(|ident| {
            let name = ident.to_string();
            tokens.extend(quote! {
                .field(#name, #ident)
            })
        });
        tokens.extend(quote! {
            .finish()
        });
    }
}

impl<'a> ToTokens for EnumImpl<(&'a Ident, &'a FieldsUnnamed)> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let (ident, fields) = self.0;
        let name = ident.to_string();

        tokens.extend(quote! {
            f.debug_tuple_struct(#name)
        });
        unnamed_idents(fields).for_each(|ident| {
            tokens.extend(quote! {
                .field(#ident)
            })
        });
        tokens.extend(quote! {
            .finish()
        });
    }
}
