use quote::format_ident;
use syn::{
    punctuated::Punctuated, token, Data, DeriveInput, Field, Fields, Path, PredicateType,
    TraitBound, TypeParamBound, WhereClause, WherePredicate,
};

pub fn predicate(input: &mut DeriveInput, path: Path) {
    let mut pred = Pred {
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
    wc: &'a mut WhereClause,
    path: Path,
}

impl<'a> Pred<'a> {
    fn dbg_pls(&mut self, field: &Field) {
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
