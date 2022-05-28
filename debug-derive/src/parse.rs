use quote::format_ident;
use syn::{spanned::Spanned, Attribute, Data, DeriveInput, Path, PathSegment};

use crate::{predicate::predicate, DebugImpl, Var};

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
        let krate = get_crate(&*attrs)?;
        predicate(&mut generics, krate.clone());
        let mut variants = vec![];
        match data {
            Data::Struct(s) => variants.push(Var {
                path: ident.clone().into(),
                fields: s.fields,
            }),
            Data::Enum(e) => {
                for v in e.variants {
                    variants.push(Var {
                        path: Path {
                            leading_colon: None,
                            segments: [ident.clone(), v.ident]
                                .into_iter()
                                .map(PathSegment::from)
                                .collect(),
                        },
                        fields: v.fields,
                    })
                }
            }
            Data::Union(_) => return Err(syn::Error::new(span, "unions not supported")),
        }
        Ok(Self {
            krate,
            ident,
            generics,
            variants,
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

    Ok(Path {
        leading_colon: Some(Default::default()),
        segments: [PathSegment::from(format_ident!("dbg_pls"))]
            .into_iter()
            .collect(),
    })
}
