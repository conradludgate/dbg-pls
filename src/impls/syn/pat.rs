use syn::{Pat, PatTuple, PatTupleStruct, PatType, PatWild};

use crate::{DebugPls, Formatter};

impl DebugPls for Pat {
    fn fmt(&self, f: Formatter<'_>) {
        match self {
            // Pat::Box(_) => todo!(),
            // Pat::Ident(_) => todo!(),
            // Pat::Lit(_) => todo!(),
            // Pat::Macro(_) => todo!(),
            // Pat::Or(_) => todo!(),
            // Pat::Path(_) => todo!(),
            // Pat::Range(_) => todo!(),
            // Pat::Reference(_) => todo!(),
            // Pat::Rest(_) => todo!(),
            // Pat::Slice(_) => todo!(),
            // Pat::Struct(_) => todo!(),
            Pat::Tuple(t) => f.debug_tuple_struct("Tuple").field(t).finish(),
            Pat::TupleStruct(t) => f.debug_tuple_struct("TupleStruct").field(t).finish(),
            Pat::Type(t) => f.debug_tuple_struct("Type").field(t).finish(),
            Pat::Wild(w) => f.debug_tuple_struct("Wild").field(w).finish(),
            _ => todo!(),
        }
    }
}

impl DebugPls for PatTuple {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_struct("PatTuple")
            .field("attrs", &self.attrs)
            .field("paren_token", &self.paren_token)
            .field("elems", &self.elems)
            .finish();
    }
}

impl DebugPls for PatTupleStruct {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_struct("PatTupleStruct")
            .field("attrs", &self.attrs)
            .field("path", &self.path)
            .field("pat", &self.pat)
            .finish();
    }
}

impl DebugPls for PatType {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_struct("PatType")
            .field("attrs", &self.attrs)
            .field("pat", &self.pat)
            .field("colon_token", &self.colon_token)
            .field("ty", &self.ty)
            .finish();
    }
}

impl DebugPls for PatWild {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_struct("PatWild")
            .field("attrs", &self.attrs)
            .field("underscore_token", &self.underscore_token)
            .finish();
    }
}
