use syn::{
    FieldPat, Pat, PatPath, PatRange, PatReference, PatRest, PatSlice, PatStruct, PatTuple,
    PatTupleStruct, PatType, PatWild, RangeLimits, PatOr, PatMacro, Macro, PatBox, PatIdent, PatLit,
};

use crate::{DebugPls, Formatter};

impl DebugPls for Pat {
    fn fmt(&self, f: Formatter<'_>) {
        match self {
            Pat::Box(v0) => f.debug_tuple_struct("Box").field(v0),
            Pat::Ident(v0) => f.debug_tuple_struct("Ident").field(v0),
            Pat::Lit(v0) => f.debug_tuple_struct("Lit").field(v0),
            Pat::Macro(v0) => f.debug_tuple_struct("Macro").field(v0),
            Pat::Or(v0) => f.debug_tuple_struct("Or").field(v0),
            Pat::Path(v0) => f.debug_tuple_struct("Path").field(v0),
            Pat::Range(v0) => f.debug_tuple_struct("Range").field(v0),
            Pat::Reference(v0) => f.debug_tuple_struct("Reference").field(v0),
            Pat::Rest(v0) => f.debug_tuple_struct("Rest").field(v0),
            Pat::Slice(v0) => f.debug_tuple_struct("Slice").field(v0),
            Pat::Struct(s) => f.debug_tuple_struct("Struct").field(s),
            Pat::Tuple(t) => f.debug_tuple_struct("Tuple").field(t),
            Pat::TupleStruct(t) => f.debug_tuple_struct("TupleStruct").field(t),
            Pat::Type(t) => f.debug_tuple_struct("Type").field(t),
            Pat::Wild(w) => f.debug_tuple_struct("Wild").field(w),
            _ => todo!(),
        }
        .finish();
    }
}


impl DebugPls for PatBox {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_struct("PatBox")
            .field("attrs", &self.attrs)
            .field("box_token", &self.box_token)
            .field("pat", &self.pat)
            .finish();
    }
}

impl DebugPls for PatIdent {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_struct("PatIdent")
            .field("attrs", &self.attrs)
            .field("by_ref", &self.by_ref)
            .field("mutability", &self.mutability)
            .field("ident", &self.ident)
            .field("subpat", &self.subpat)
            .finish();
    }
}
impl DebugPls for PatLit {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_struct("PatLit")
            .field("attrs", &self.attrs)
            .field("expr", &self.expr)
            .finish();
    }
}
impl DebugPls for PatMacro {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_struct("PatMacro")
            .field("attrs", &self.attrs)
            .field("mac", &self.mac)
            .finish();
    }
}

impl DebugPls for Macro {
    fn fmt(&self, _f: Formatter<'_>) {
        todo!()
    }
}

impl DebugPls for PatOr {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_struct("PatOr")
            .field("attrs", &self.attrs)
            .field("leading_vert", &self.leading_vert)
            .field("cases", &self.cases)
            .finish();
    }
}

impl DebugPls for PatPath {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_struct("PatPath")
            .field("attrs", &self.attrs)
            .field("qself", &self.qself)
            .field("path", &self.path)
            .finish();
    }
}

impl DebugPls for PatRange {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_struct("PatRange")
            .field("attrs", &self.attrs)
            .field("and_token", &self.lo)
            .field("mutability", &self.limits)
            .field("pat", &self.hi)
            .finish();
    }
}

impl DebugPls for RangeLimits {
    fn fmt(&self, f: Formatter<'_>) {
        match self {
            RangeLimits::HalfOpen(v0) => f.debug_tuple_struct("HalfOpen").field(v0),
            RangeLimits::Closed(v0) => f.debug_tuple_struct("Closed").field(v0),
        }
        .finish();
    }
}

impl DebugPls for PatReference {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_struct("PatReference")
            .field("attrs", &self.attrs)
            .field("and_token", &self.and_token)
            .field("mutability", &self.mutability)
            .field("pat", &self.pat)
            .finish();
    }
}

impl DebugPls for PatRest {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_struct("PatRest")
            .field("attrs", &self.attrs)
            .field("dot2_token", &self.dot2_token)
            .finish();
    }
}

impl DebugPls for PatSlice {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_struct("PatSlice")
            .field("attrs", &self.attrs)
            .field("bracket_token", &self.bracket_token)
            .field("elems", &self.elems)
            .finish();
    }
}

impl DebugPls for PatStruct {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_struct("PatStruct")
            .field("attrs", &self.attrs)
            .field("path", &self.path)
            .field("brace_token", &self.brace_token)
            .field("fields", &self.fields)
            .field("dot2_token", &self.dot2_token)
            .finish();
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

impl DebugPls for FieldPat {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_struct("FieldPat")
            .field("attrs", &self.attrs)
            .field("member", &self.member)
            .field("colon_token", &self.colon_token)
            .field("pat", &self.pat)
            .finish();
    }
}
