use proc_macro::TokenStream;
use proc_macro2::{Delimiter, Group, Ident, TokenStream as TokenStream2, TokenTree};
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_macro_input, parse_quote, spanned::Spanned, Attribute, Data, DataEnum, DataStruct,
    DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Path, Variant,
};

#[proc_macro_derive(DebugPls, attributes(dbg_pls))]
pub fn derive(input: TokenStream) -> TokenStream {
    DebugImpl(parse_macro_input!(input as DeriveInput))
        .into_token_stream()
        .into()
}

struct DebugImpl<T>(T);

impl ToTokens for DebugImpl<DeriveInput> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let DeriveInput {
            attrs,
            vis: _,
            ident,
            generics,
            data,
        } = &self.0;

        let path = match get_crate(attrs) {
            Ok(path) => path,
            Err(err) => return tokens.extend(err.into_compile_error()),
        };

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let body = DebugImpl((ident, data));

        tokens.extend(quote! {
            impl #impl_generics #path::DebugPls for #ident #ty_generics #where_clause {
                fn fmt(&self, f: #path::Formatter<'_>) {
                    #body
                }
            }
        })
    }
}

fn get_crate(attrs: &[Attribute]) -> syn::Result<Path> {
    fn parse_crate(lit: syn::Lit) -> syn::Result<Path> {
        match lit {
            syn::Lit::Str(s) => syn::parse_str(&s.value()),
            _ => Err(syn::Error::new(lit.span(), "invalid crate name")),
        }
    }

    fn parse_meta(meta: syn::Meta) -> Option<syn::Result<Path>> {
        if let syn::Meta::List(list) = meta {
            for meta in list.nested {
                if let syn::NestedMeta::Meta(syn::Meta::NameValue(nv)) = meta {
                    if let Some(ident) = nv.path.get_ident() {
                        if *ident == "crate" {
                            return Some(parse_crate(nv.lit));
                        }
                    }
                }
            }
        }
        None
    }

    for attr in attrs {
        if let Some(ident) = attr.path.get_ident() {
            if *ident == "dbg_pls" {
                if let Some(path) = parse_meta(Attribute::parse_meta(attr)?).transpose()? {
                    return Ok(path);
                }
            }
        }
    }

    Ok(parse_quote! { ::dbg_pls })
}

impl<'a> ToTokens for DebugImpl<(&'a Ident, &'a Data)> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let (name, data) = self.0;
        match data {
            Data::Struct(s) => DebugImpl((name, s)).to_tokens(tokens),
            Data::Enum(e) => DebugImpl(e).to_tokens(tokens),
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

impl<'a> ToTokens for DebugImpl<&'a DataEnum> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let variants = self.0.variants.iter().map(DebugImpl);
        tokens.extend(quote! {
            match self {
                #( #variants )*
            }
        });
    }
}

impl<'a> ToTokens for DebugImpl<&'a Variant> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Variant {
            attrs: _,
            ident,
            fields,
            discriminant: _,
        } = &self.0;
        let pattern = PatternImpl(fields);
        let debug = DebugImpl((ident, fields));

        tokens.extend(quote! {
            #ident #pattern => { #debug }
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
            f.debug_tuple(#name)
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

struct PatternImpl<T>(T);

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

fn unnamed_idents(fields: &FieldsUnnamed) -> impl Iterator<Item = Ident> + '_ {
    fields
        .unnamed
        .iter()
        .enumerate()
        .map(|(i, _)| format_ident!("val{}", i))
}
fn named_idents(fields: &FieldsNamed) -> impl Iterator<Item = &Ident> + '_ {
    fields
        .named
        .iter()
        .map(|field| field.ident.as_ref().unwrap())
}
