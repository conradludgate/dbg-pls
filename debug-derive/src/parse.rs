use std::collections::HashSet;

use proc_macro2::{Ident, Span};
use quote::format_ident;
use syn::{
    spanned::Spanned,
    visit::{visit_type_path, Visit},
    Attribute, Data, DeriveInput, Path, PathSegment,
};

use crate::{predicate::predicate_with, DebugImpl, Mode, Var};

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

        let mut types = HashSet::new();

        let mode = match data {
            Data::Struct(s) => {
                for field in s.fields.iter() {
                    types.insert(field.ty.clone());
                }
                Mode::Struct(crate::StructFields(s.fields))
            }
            Data::Enum(e) => {
                for var in e.variants.iter() {
                    for field in var.fields.iter() {
                        types.insert(field.ty.clone());
                    }
                }
                Mode::Enum(
                    e.variants
                        .into_iter()
                        .map(|v| Var {
                            ident: v.ident,
                            fields: v.fields,
                        })
                        .collect(),
                )
            }
            Data::Union(_) => return Err(syn::Error::new(span, "unions not supported")),
        };

        let with_ident = format_ident!("__DebugWith");
        predicate_with(
            &mut generics,
            krate.clone(),
            with_ident.clone(),
            types.into_iter().filter(|ty| {
                let mut contains = ContainsIdent {
                    ident: ident.clone(),
                    contains: false,
                };
                contains.visit_type(ty);
                !contains.contains
            }),
        );

        Ok(Self {
            krate,
            ident,
            generics,
            mode,
            with_ident,
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
                attr.meta.require_list()?.parse_nested_meta(|meta| {
                    match () {
                        () if meta.path.is_ident("crate") => {
                            if args.krate.replace(Krate(meta.value()?.parse()?)).is_some() {
                                return Err(meta.error("duplicate `dbg_pls(crate)` arg"));
                            }
                        }
                        () => return Err(meta.error("unknown argument found")),
                    }

                    Ok(())
                })?;
            }
        }

        Ok(args)
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

#[derive(Debug)]
struct ContainsIdent {
    ident: Ident,
    contains: bool,
}

impl<'ast> Visit<'ast> for ContainsIdent {
    fn visit_type_path(&mut self, i: &'ast syn::TypePath) {
        self.contains |= i.path.leading_colon.is_none()
            && i.path.segments.len() == 1
            && i.path.segments[0].ident == self.ident;

        self.contains |= i.path.is_ident(&self.ident);
        visit_type_path(self, i)
    }
}
