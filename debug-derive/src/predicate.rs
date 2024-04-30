use proc_macro2::{Ident, Span};
use syn::{
    punctuated::Punctuated, token, AngleBracketedGenericArguments, GenericArgument, Generics, Path,
    PredicateType, TraitBound, Type, TypeParamBound, TypePath, WhereClause, WherePredicate,
};

const TRAIT_WITH: &str = "DebugWith";

pub fn predicate_with(generics: &mut Generics, mut krate: Path, with_ident: Ident) {
    let Generics {
        params,
        where_clause,
        ..
    } = generics;

    let mut bounds = Punctuated::new();
    // `#krate::DebugWith<#with_ident>`
    krate.segments.push(syn::PathSegment {
        ident: Ident::new(TRAIT_WITH, Span::call_site()),
        arguments: syn::PathArguments::AngleBracketed(AngleBracketedGenericArguments {
            colon2_token: Default::default(),
            lt_token: Default::default(),
            args: Punctuated::from_iter([GenericArgument::Type(Type::Path(TypePath {
                qself: None,
                path: Path {
                    leading_colon: Default::default(),
                    segments: Punctuated::from_iter([syn::PathSegment {
                        ident: with_ident,
                        arguments: syn::PathArguments::None,
                    }]),
                },
            }))]),
            gt_token: Default::default(),
        }),
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
