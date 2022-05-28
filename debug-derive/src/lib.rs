use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::ToTokens;
use syn::{parse_macro_input, DeriveInput, Fields, Generics, Path};

mod parse;
mod predicate;
mod to_tokens;

#[proc_macro_derive(DebugPls, attributes(dbg_pls))]
pub fn derive(input: TokenStream) -> TokenStream {
    match DebugImpl::try_from(parse_macro_input!(input as DeriveInput)) {
        Ok(output) => output.to_token_stream(),
        Err(err) => err.into_compile_error().into_token_stream(),
    }
    .into()
}

struct DebugImpl {
    krate: Path,
    ident: Ident,
    variants: Vec<Var>,
    generics: Generics,
}

struct Var {
    path: Path,
    fields: Fields,
}
