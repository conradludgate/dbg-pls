use quote::format_ident;
use syn::{
    punctuated::Punctuated, token, DeriveInput, Path, PredicateType, TraitBound, Type,
    TypeParamBound, TypePath, WhereClause, WherePredicate,
};

pub fn predicate(input: &mut DeriveInput, path: Path) {
    let generics = input.generics.clone();

    let mut pred = Pred {
        wc: input.generics.make_where_clause(),
        path,
    };

    for ty in generics.params {
        if let syn::GenericParam::Type(ty) = ty {
            let mut segments = Punctuated::new();
            segments.push(syn::PathSegment {
                ident: ty.ident,
                arguments: syn::PathArguments::None,
            });
            let ty = Type::Path(TypePath {
                qself: None,
                path: Path {
                    leading_colon: None,
                    segments,
                },
            });

            pred.dbg_pls(ty)
        }
    }
}

struct Pred<'a> {
    wc: &'a mut WhereClause,
    path: Path,
}

impl<'a> Pred<'a> {
    fn dbg_pls(&mut self, ty: Type) {
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
            bounded_ty: ty,
            colon_token: token::Colon::default(),
            bounds,
        }))
    }
}
