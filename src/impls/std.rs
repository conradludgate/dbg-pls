mod collections;
mod fnptr;
mod tuple;

use std::{
    ops,
    ops::ControlFlow,
    rc::Rc,
    sync::{Arc, Mutex, MutexGuard, TryLockError},
    task::Poll,
};

use crate::{DebugWith, Formatter};
use syn::RangeLimits;
use syn::__private::Span;

impl<W, T: ?Sized + DebugWith<W>> DebugWith<W> for Box<T> {
    fn fmt(&self, with: &W, f: Formatter<'_>) {
        DebugWith::fmt(&**self, with, f);
    }
}

impl<W, D: DebugWith<W> + ?Sized> DebugWith<W> for *mut D {
    fn fmt(&self, with: &W, f: Formatter<'_>) {
        <*const D>::fmt(&(*self).cast_const(), with, f);
    }
}

impl<W, D: ?Sized> DebugWith<W> for *const D {
    fn fmt(&self, _with: &W, f: Formatter<'_>) {
        /// Since the formatting will be identical for all pointer types, use a non-monomorphized
        /// implementation for the actual formatting to reduce the amount of codegen work needed
        fn inner(ptr: *const (), f: Formatter<'_>) {
            let output = format!("{:#x?}", ptr as usize);
            f.write_expr(syn::ExprLit {
                attrs: vec![],
                lit: syn::LitFloat::new(&output, Span::call_site()).into(),
            });
        }

        inner((*self).cast(), f);
    }
}

impl<'a, W, D: DebugWith<W> + ?Sized> DebugWith<W> for &'a D {
    fn fmt(&self, with: &W, f: Formatter<'_>) {
        D::fmt(self, with, f);
    }
}

impl<'a, W, D: DebugWith<W> + ?Sized> DebugWith<W> for &'a mut D {
    fn fmt(&self, with: &W, f: Formatter<'_>) {
        D::fmt(self, with, f);
    }
}

impl<W, T: ?Sized + DebugWith<W>> DebugWith<W> for Rc<T> {
    fn fmt(&self, with: &W, f: Formatter<'_>) {
        DebugWith::fmt(&**self, with, f);
    }
}

impl<W, T: ?Sized + DebugWith<W>> DebugWith<W> for Arc<T> {
    fn fmt(&self, with: &W, f: Formatter<'_>) {
        DebugWith::fmt(&**self, with, f);
    }
}

impl<W, T: ?Sized + DebugWith<W>> DebugWith<W> for MutexGuard<'_, T> {
    fn fmt(&self, with: &W, f: Formatter<'_>) {
        DebugWith::fmt(&**self, with, f);
    }
}

impl<W, T: ?Sized + DebugWith<W>> DebugWith<W> for Mutex<T> {
    fn fmt(&self, with: &W, f: Formatter<'_>) {
        let d = f.debug_struct("Mutex");
        match self.try_lock() {
            Ok(guard) => d.field_with("data", &&*guard, with),
            Err(TryLockError::Poisoned(err)) => d.field_with("data", &&**err.get_ref(), with),
            Err(TryLockError::WouldBlock) => d.field("data", &"<locked>"),
        }
        .field("poisoned", &self.is_poisoned())
        .finish_non_exhaustive();
    }
}

macro_rules! debug_integers {
    ($($T:ident)*) => {$(
        impl<W> DebugWith<W> for $T {
            fn fmt(&self, _with: &W, f: Formatter<'_>) {
                let mut buf = itoa::Buffer::new();
                f.write_expr(syn::ExprLit {
                    attrs: vec![],
                    lit: syn::LitInt::new(buf.format(*self), Span::call_site()).into(),
                });
            }
        }
    )*};
}

debug_integers! {
  i8 i16 i32 i64 i128 isize
  u8 u16 u32 u64 u128 usize
}

macro_rules! debug_non_zero_integers {
    ($($T:ident)*) => {$(
        impl<W> DebugWith<W> for std::num::$T {
            fn fmt(&self, _with: &W, f: Formatter<'_>) {
                let mut buf = itoa::Buffer::new();
                f.write_expr(syn::ExprLit {
                    attrs: vec![],
                    lit: syn::LitInt::new(buf.format(self.get()), Span::call_site()).into(),
                });
            }
        }
    )*};
}

debug_non_zero_integers! {
  NonZeroI8 NonZeroI16 NonZeroI32 NonZeroI64 NonZeroI128 NonZeroIsize
  NonZeroU8 NonZeroU16 NonZeroU32 NonZeroU64 NonZeroU128 NonZeroUsize
}

macro_rules! debug_floats {
    ($ty:ident) => {
        impl<W> DebugWith<W> for $ty {
            fn fmt(&self, _with: &W, f: Formatter<'_>) {
                let mut buf = ryu::Buffer::new();
                f.write_expr(syn::ExprLit {
                    attrs: vec![],
                    lit: syn::LitFloat::new(buf.format(*self), Span::call_site()).into(),
                });
            }
        }
    };
}

debug_floats! { f32 }
debug_floats! { f64 }

impl<W> DebugWith<W> for bool {
    fn fmt(&self, _with: &W, f: Formatter<'_>) {
        f.write_expr(syn::ExprLit {
            attrs: vec![],
            lit: syn::Lit::Bool(syn::LitBool {
                value: *self,
                span: Span::call_site(),
            }),
        });
    }
}

impl<W, D: DebugWith<W>> DebugWith<W> for [D] {
    fn fmt(&self, with: &W, f: Formatter<'_>) {
        f.debug_list().entries_with(self, with).finish();
    }
}

impl<W, D: DebugWith<W>, const N: usize> DebugWith<W> for [D; N] {
    fn fmt(&self, with: &W, f: Formatter<'_>) {
        f.debug_list().entries_with(self, with).finish();
    }
}

impl<W> DebugWith<W> for char {
    fn fmt(&self, _with: &W, f: Formatter<'_>) {
        f.write_expr(syn::ExprLit {
            attrs: vec![],
            lit: syn::LitChar::new(*self, Span::call_site()).into(),
        });
    }
}

impl<W> DebugWith<W> for str {
    fn fmt(&self, _with: &W, f: Formatter<'_>) {
        f.write_expr(syn::ExprLit {
            attrs: vec![],
            lit: syn::LitStr::new(self, Span::call_site()).into(),
        });
    }
}

impl<W> DebugWith<W> for String {
    fn fmt(&self, with: &W, f: Formatter<'_>) {
        DebugWith::fmt(self.as_str(), with, f);
    }
}

impl<W, T: DebugWith<W>, E: DebugWith<W>> DebugWith<W> for Result<T, E> {
    fn fmt(&self, with: &W, f: Formatter<'_>) {
        match self {
            Ok(t) => f.debug_tuple_struct("Ok").field_with(t, with).finish(),
            Err(e) => f.debug_tuple_struct("Err").field_with(e, with).finish(),
        }
    }
}

impl<W, B: DebugWith<W>, C: DebugWith<W>> DebugWith<W> for ControlFlow<B, C> {
    fn fmt(&self, with: &W, f: Formatter<'_>) {
        match self {
            ControlFlow::Break(b) => f.debug_tuple_struct("Break").field_with(b, with).finish(),
            ControlFlow::Continue(c) => f
                .debug_tuple_struct("Continue")
                .field_with(c, with)
                .finish(),
        }
    }
}

impl<W, T: DebugWith<W>> DebugWith<W> for Option<T> {
    fn fmt(&self, with: &W, f: Formatter<'_>) {
        match self {
            Some(t) => f.debug_tuple_struct("Some").field_with(t, with).finish(),
            None => f.debug_ident("None"),
        }
    }
}

impl<W, T: DebugWith<W>> DebugWith<W> for Poll<T> {
    fn fmt(&self, with: &W, f: Formatter<'_>) {
        match self {
            Poll::Ready(t) => f.debug_tuple_struct("Ready").field_with(t, with).finish(),
            Poll::Pending => f.debug_ident("Pending"),
        }
    }
}

impl<W, T: DebugWith<W>> DebugWith<W> for ops::Range<T> {
    fn fmt(&self, with: &W, f: Formatter<'_>) {
        f.write_expr(syn::ExprRange {
            attrs: vec![],
            start: Some(Box::new(Formatter::process_with(&self.start, with))),
            limits: RangeLimits::HalfOpen(syn::token::DotDot::default()),
            end: Some(Box::new(Formatter::process_with(&self.end, with))),
        });
    }
}

impl<W, T: DebugWith<W>> DebugWith<W> for ops::RangeFrom<T> {
    fn fmt(&self, with: &W, f: Formatter<'_>) {
        f.write_expr(syn::ExprRange {
            attrs: vec![],
            start: Some(Box::new(Formatter::process_with(&self.start, with))),
            limits: RangeLimits::HalfOpen(syn::token::DotDot::default()),
            end: None,
        });
    }
}

impl<W, T: DebugWith<W>> DebugWith<W> for ops::RangeTo<T> {
    fn fmt(&self, with: &W, f: Formatter<'_>) {
        f.write_expr(syn::ExprRange {
            attrs: vec![],
            start: None,
            limits: RangeLimits::HalfOpen(syn::token::DotDot::default()),
            end: Some(Box::new(Formatter::process_with(&self.end, with))),
        });
    }
}

impl<W> DebugWith<W> for ops::RangeFull {
    fn fmt(&self, _with: &W, f: Formatter<'_>) {
        f.write_expr(syn::ExprRange {
            attrs: vec![],
            start: None,
            limits: RangeLimits::HalfOpen(syn::token::DotDot::default()),
            end: None,
        });
    }
}

impl<W, T: DebugWith<W>> DebugWith<W> for ops::RangeInclusive<T> {
    fn fmt(&self, with: &W, f: Formatter<'_>) {
        f.write_expr(syn::ExprRange {
            attrs: vec![],
            start: Some(Box::new(Formatter::process_with(&self.start(), with))),
            limits: RangeLimits::Closed(syn::token::DotDotEq::default()),
            end: Some(Box::new(Formatter::process_with(&self.end(), with))),
        });
    }
}

impl<W, T: DebugWith<W>> DebugWith<W> for ops::RangeToInclusive<T> {
    fn fmt(&self, with: &W, f: Formatter<'_>) {
        f.write_expr(syn::ExprRange {
            attrs: vec![],
            start: None,
            limits: RangeLimits::Closed(syn::token::DotDotEq::default()),
            end: Some(Box::new(Formatter::process_with(&self.end, with))),
        });
    }
}
