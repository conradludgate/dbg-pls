use syn::BinOp;

use crate::{DebugPls, Formatter};

impl DebugPls for BinOp {
    fn fmt(&self, f: Formatter<'_>) {
        match self {
            // BinOp::Add(_) => todo!(),
            BinOp::Sub(v0) => f.debug_tuple_struct("Sub").field(v0).finish(),
            // BinOp::Mul(_) => todo!(),
            // BinOp::Div(_) => todo!(),
            // BinOp::Rem(_) => todo!(),
            // BinOp::And(_) => todo!(),
            // BinOp::Or(_) => todo!(),
            // BinOp::BitXor(_) => todo!(),
            // BinOp::BitAnd(_) => todo!(),
            // BinOp::BitOr(_) => todo!(),
            // BinOp::Shl(_) => todo!(),
            // BinOp::Shr(_) => todo!(),
            // BinOp::Eq(_) => todo!(),
            // BinOp::Lt(_) => todo!(),
            // BinOp::Le(_) => todo!(),
            // BinOp::Ne(_) => todo!(),
            // BinOp::Ge(_) => todo!(),
            // BinOp::Gt(_) => todo!(),
            // BinOp::AddEq(_) => todo!(),
            // BinOp::SubEq(_) => todo!(),
            // BinOp::MulEq(_) => todo!(),
            // BinOp::DivEq(_) => todo!(),
            // BinOp::RemEq(_) => todo!(),
            // BinOp::BitXorEq(_) => todo!(),
            // BinOp::BitAndEq(_) => todo!(),
            // BinOp::BitOrEq(_) => todo!(),
            // BinOp::ShlEq(_) => todo!(),
            // BinOp::ShrEq(_) => todo!(),
            _ => todo!(),
        }
    }
}
