use syn::Type;

use crate::{DebugPls, Formatter};

impl DebugPls for Type {
    fn fmt(&self, _f: Formatter<'_>) {
        match self {
            // Type::Array(_) => todo!(),
            // Type::BareFn(_) => todo!(),
            // Type::Group(_) => todo!(),
            // Type::ImplTrait(_) => todo!(),
            // Type::Infer(_) => todo!(),
            // Type::Macro(_) => todo!(),
            // Type::Never(_) => todo!(),
            // Type::Paren(_) => todo!(),
            // Type::Path(_) => todo!(),
            // Type::Ptr(_) => todo!(),
            // Type::Reference(_) => todo!(),
            // Type::Slice(_) => todo!(),
            // Type::TraitObject(_) => todo!(),
            // Type::Tuple(_) => todo!(),
            // Type::Verbatim(_) => todo!(),
            _ => todo!(),
        }
    }
}
