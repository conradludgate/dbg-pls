use proc_macro2::Ident;
use quote::format_ident;
use syn::{
    punctuated::Punctuated, token, Data, DeriveInput, Field, Fields, GenericArgument, Generics,
    Path, PathArguments, PredicateType, TraitBound, Type, TypeParamBound, WhereClause,
    WherePredicate,
};

pub fn predicate(input: &mut DeriveInput, path: Path) {
    let mut pred = Pred {
        generics: input.generics.clone(),
        wc: input.generics.make_where_clause(),
        path,
    };

    match &input.data {
        Data::Struct(s) => pred.fields(&s.fields),
        Data::Enum(e) => e.variants.iter().for_each(|var| pred.fields(&var.fields)),
        Data::Union(_) => todo!(),
    }
}

struct Pred<'a> {
    generics: Generics,
    wc: &'a mut WhereClause,
    path: Path,
}

impl<'a> Pred<'a> {
    fn dbg_pls(&mut self, field: &Field) {
        let mut contains = false;
        for ty in &self.generics.params {
            if let syn::GenericParam::Type(ty) = ty {
                if type_contains(&field.ty, &ty.ident) {
                    contains = true;
                    break;
                }
            }
        }
        if !contains {
            return;
        }

        let mut bounds = Punctuated::new();

        let mut path = self.path.clone();
        path.segments.push(syn::PathSegment {
            ident: format_ident!("DebugPls"),
            arguments: syn::PathArguments::None,
        });

        bounds.push(TypeParamBound::Trait(TraitBound {
            paren_token: None,
            modifier: syn::TraitBoundModifier::None,
            lifetimes: None,
            path,
        }));

        self.wc.predicates.push(WherePredicate::Type(PredicateType {
            lifetimes: None,
            bounded_ty: field.ty.clone(),
            colon_token: token::Colon::default(),
            bounds,
        }))
    }

    fn fields(&mut self, fields: &Fields) {
        match fields {
            syn::Fields::Named(named) => named.named.iter().for_each(|f| self.dbg_pls(f)),
            syn::Fields::Unnamed(unnamed) => unnamed.unnamed.iter().for_each(|f| self.dbg_pls(f)),
            syn::Fields::Unit => {}
        }
    }
}

fn type_contains(t: &Type, i: &Ident) -> bool {
    match t {
        Type::Path(p) => {
            if let Some(ident) = p.path.get_ident() {
                if ident == i {
                    return true;
                }
            }
            for segment in &p.path.segments {
                if let PathArguments::AngleBracketed(a) = &segment.arguments {
                    for arg in &a.args {
                        if let GenericArgument::Type(t) = &arg {
                            if type_contains(t, i) {
                                return true;
                            }
                        }
                    }
                }
            }
            false
        }
        Type::Array(a) => type_contains(&a.elem, i),
        Type::Paren(p) => type_contains(&p.elem, i),
        Type::Ptr(p) => type_contains(&p.elem, i),
        Type::Reference(r) => type_contains(&r.elem, i),
        Type::Slice(s) => type_contains(&s.elem, i),
        Type::Tuple(t) => {
            for t in &t.elems {
                if type_contains(t, i) {
                    return true;
                }
            }
            false
        }
        _ => false,
    }
}
