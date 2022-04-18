use debug::DebugImpl;
use predicate::predicate;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, DeriveInput, parse_quote, Attribute, Path};

mod debug;
mod pat;
mod predicate;

/// Derives the standard `DebugPls` implementation.
///
/// Works exactly like [`Debug`]
#[proc_macro_derive(DebugPls, attributes(dbg_pls))]
pub fn derive(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    let path = match get_crate(&input.attrs) {
        Ok(path) => path,
        Err(err) => return err.into_compile_error().into_token_stream().into(),
    };

    predicate(&mut input, path.clone());
    DebugImpl((path, input)).into_token_stream().into()
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
