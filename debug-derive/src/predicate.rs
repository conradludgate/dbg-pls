use proc_macro2::{Ident, Span};
use syn::{
    punctuated::Punctuated, token, Generics, Path, PredicateType, TraitBound, Type, TypeParamBound,
    TypePath, WhereClause, WherePredicate,
};

const TRAIT: &str = "DebugPls";

pub fn predicate(generics: &mut Generics, mut krate: Path) {
    let Generics {
        params,
        where_clause,
        ..
    } = generics;

    let mut bounds = Punctuated::new();
    krate.segments.push(syn::PathSegment {
        ident: Ident::new(TRAIT, Span::call_site()),
        arguments: syn::PathArguments::None,
    });
    bounds.push(TypeParamBound::Trait(TraitBound {
        paren_token: None,
        modifier: syn::TraitBoundModifier::None,
        lifetimes: None,
        path: krate,
    }));

    let wc = where_clause.get_or_insert_with(|| WhereClause {
        where_token: Default::default(),
        predicates: Punctuated::new(),
    });

    for ty in params {
        if let syn::GenericParam::Type(ty) = ty {
            wc.predicates.push(WherePredicate::Type(PredicateType {
                lifetimes: None,
                bounded_ty: Type::Path(TypePath {
                    qself: None,
                    path: ty.ident.clone().into(),
                }),
                colon_token: token::Colon::default(),
                bounds: bounds.clone(),
            }))
        }
    }
}
