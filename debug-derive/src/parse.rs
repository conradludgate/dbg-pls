use proc_macro2::{Ident, Span};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Attribute, Data, DeriveInput, Path, PathSegment, Token,
};

use crate::{predicate::predicate, DebugImpl, Mode, Var};

impl TryFrom<DeriveInput> for DebugImpl {
    type Error = syn::Error;
    fn try_from(input: DeriveInput) -> syn::Result<Self> {
        let span = input.span();
        let DeriveInput {
            ident,
            data,
            mut generics,
            attrs,
            ..
        } = input;

        let args = Args::parse_attrs(&attrs)?;
        let Krate(krate) = args.krate.unwrap_or_default();

        predicate(&mut generics, krate.clone());

        let mode = match data {
            Data::Struct(s) => Mode::Struct(crate::StructFields(s.fields)),
            Data::Enum(e) => Mode::Enum(
                e.variants
                    .into_iter()
                    .map(|v| Var {
                        ident: v.ident,
                        fields: v.fields,
                    })
                    .collect(),
            ),
            Data::Union(_) => return Err(syn::Error::new(span, "unions not supported")),
        };
        Ok(Self {
            krate,
            ident,
            generics,
            mode,
        })
    }
}

const ATTR: &str = "dbg_pls";
const CRATE: &str = "dbg_pls";

/// Args of `dbg_pls`
#[derive(Default)]
struct Args {
    /// Optional `crate = $:path` arg
    krate: Option<Krate>,
}

impl Args {
    fn parse_attrs(attrs: &[Attribute]) -> syn::Result<Self> {
        let mut args = Args::default();
        for attr in attrs {
            if attr.path().is_ident(ATTR) {
                args = attr.parse_args_with(|input: ParseStream| args.parse(input))?;
            }
        }
        Ok(args)
    }
}

impl Args {
    fn parse(mut self, input: ParseStream) -> syn::Result<Self> {
        let mut first = true;
        while !input.is_empty() {
            if !first {
                input.parse::<Token![,]>()?;
            }
            first = false;

            match () {
                _ if Krate::peek(&input) => {
                    if self.krate.replace(input.parse()?).is_some() {
                        return Err(input.error("duplicate `crate` arg"));
                    }
                }
                _ => return Err(input.error("unknown argument found")),
            }
        }
        Ok(self)
    }
}

struct Krate(Path);

impl Default for Krate {
    fn default() -> Self {
        Self(Path {
            leading_colon: Some(Default::default()),
            segments: [PathSegment::from(Ident::new(CRATE, Span::call_site()))]
                .into_iter()
                .collect(),
        })
    }
}

impl Krate {
    fn peek(peek: &ParseStream) -> bool {
        peek.peek(Token![crate])
    }
}

impl Parse for Krate {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _: Token![crate] = input.parse()?;
        let _: Token![=] = input.parse()?;
        let krate = input.parse()?;
        Ok(Krate(krate))
    }
}
