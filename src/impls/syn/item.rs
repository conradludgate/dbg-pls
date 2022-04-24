use syn::Item;

use crate::{DebugPls, Formatter};

impl DebugPls for Item {
    fn fmt(&self, _f: Formatter<'_>) {
        match self {
            // Item::Const(_) => todo!(),
            // Item::Enum(_) => todo!(),
            // Item::ExternCrate(_) => todo!(),
            // Item::Fn(_) => todo!(),
            // Item::ForeignMod(_) => todo!(),
            // Item::Impl(_) => todo!(),
            // Item::Macro(_) => todo!(),
            // Item::Macro2(_) => todo!(),
            // Item::Mod(_) => todo!(),
            // Item::Static(_) => todo!(),
            // Item::Struct(_) => todo!(),
            // Item::Trait(_) => todo!(),
            // Item::TraitAlias(_) => todo!(),
            // Item::Type(_) => todo!(),
            // Item::Union(_) => todo!(),
            // Item::Use(_) => todo!(),
            _ => todo!(),
        }
    }
}
