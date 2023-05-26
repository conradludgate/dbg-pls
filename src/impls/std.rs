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

use crate::{DebugPls, Formatter};
use syn::RangeLimits;
use syn::__private::Span;

impl<T: ?Sized + DebugPls> DebugPls for Box<T> {
    fn fmt(&self, f: Formatter<'_>) {
        DebugPls::fmt(&**self, f);
    }
}

impl<D: DebugPls + ?Sized> DebugPls for *mut D {
    fn fmt(&self, f: Formatter<'_>) {
        <*const D>::fmt(&(*self as *const D), f);
    }
}

impl<D: DebugPls + ?Sized> DebugPls for *const D {
    fn fmt(&self, f: Formatter<'_>) {
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

impl<'a, D: DebugPls + ?Sized> DebugPls for &'a D {
    fn fmt(&self, f: Formatter<'_>) {
        D::fmt(self, f);
    }
}

impl<'a, D: DebugPls + ?Sized> DebugPls for &'a mut D {
    fn fmt(&self, f: Formatter<'_>) {
        D::fmt(self, f);
    }
}

impl<T: ?Sized + DebugPls> DebugPls for Rc<T> {
    fn fmt(&self, f: Formatter<'_>) {
        DebugPls::fmt(&**self, f);
    }
}

impl<T: ?Sized + DebugPls> DebugPls for Arc<T> {
    fn fmt(&self, f: Formatter<'_>) {
        DebugPls::fmt(&**self, f);
    }
}

impl<T: ?Sized + DebugPls> DebugPls for MutexGuard<'_, T> {
    fn fmt(&self, f: Formatter<'_>) {
        DebugPls::fmt(&**self, f);
    }
}

impl<T: ?Sized + DebugPls> DebugPls for Mutex<T> {
    fn fmt(&self, f: Formatter<'_>) {
        let d = f.debug_struct("Mutex");
        match self.try_lock() {
            Ok(guard) => d.field("data", &&*guard),
            Err(TryLockError::Poisoned(err)) => d.field("data", &&**err.get_ref()),
            Err(TryLockError::WouldBlock) => d.field("data", &"<locked>"),
        }
        .field("poisoned", &self.is_poisoned())
        .finish_non_exhaustive();
    }
}

macro_rules! debug_integers {
    ($($T:ident)*) => {$(
        impl DebugPls for $T {
            fn fmt(&self, f: Formatter<'_>) {
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
        impl DebugPls for std::num::$T {
            fn fmt(&self, f: Formatter<'_>) {
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
        impl DebugPls for $ty {
            fn fmt(&self, f: Formatter<'_>) {
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

impl DebugPls for bool {
    fn fmt(&self, f: Formatter<'_>) {
        f.write_expr(syn::ExprLit {
            attrs: vec![],
            lit: syn::Lit::Bool(syn::LitBool {
                value: *self,
                span: Span::call_site(),
            }),
        });
    }
}

impl<D: DebugPls> DebugPls for [D] {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_list().entries(self).finish();
    }
}

impl<D: DebugPls, const N: usize> DebugPls for [D; N] {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_list().entries(self).finish();
    }
}

impl DebugPls for char {
    fn fmt(&self, f: Formatter<'_>) {
        f.write_expr(syn::ExprLit {
            attrs: vec![],
            lit: syn::LitChar::new(*self, Span::call_site()).into(),
        });
    }
}

impl DebugPls for str {
    fn fmt(&self, f: Formatter<'_>) {
        f.write_expr(syn::ExprLit {
            attrs: vec![],
            lit: syn::LitStr::new(self, Span::call_site()).into(),
        });
    }
}

impl DebugPls for String {
    fn fmt(&self, f: Formatter<'_>) {
        DebugPls::fmt(self.as_str(), f);
    }
}

impl<T: DebugPls, E: DebugPls> DebugPls for Result<T, E> {
    fn fmt(&self, f: Formatter<'_>) {
        match self {
            Ok(t) => f.debug_tuple_struct("Ok").field(t).finish(),
            Err(e) => f.debug_tuple_struct("Err").field(e).finish(),
        }
    }
}

impl<B: DebugPls, C: DebugPls> DebugPls for ControlFlow<B, C> {
    fn fmt(&self, f: Formatter<'_>) {
        match self {
            ControlFlow::Break(b) => f.debug_tuple_struct("Break").field(b).finish(),
            ControlFlow::Continue(c) => f.debug_tuple_struct("Continue").field(c).finish(),
        }
    }
}

impl<T: DebugPls> DebugPls for Option<T> {
    fn fmt(&self, f: Formatter<'_>) {
        match self {
            Some(t) => f.debug_tuple_struct("Some").field(t).finish(),
            None => f.debug_ident("None"),
        }
    }
}

impl<T: DebugPls> DebugPls for Poll<T> {
    fn fmt(&self, f: Formatter<'_>) {
        match self {
            Poll::Ready(t) => f.debug_tuple_struct("Ready").field(t).finish(),
            Poll::Pending => f.debug_ident("Pending"),
        }
    }
}

impl<T: DebugPls> DebugPls for ops::Range<T> {
    fn fmt(&self, f: Formatter<'_>) {
        f.write_expr(syn::ExprRange {
            attrs: vec![],
            start: Some(Box::new(Formatter::process(&self.start))),
            limits: RangeLimits::HalfOpen(syn::token::DotDot::default()),
            end: Some(Box::new(Formatter::process(&self.end))),
        });
    }
}

impl<T: DebugPls> DebugPls for ops::RangeFrom<T> {
    fn fmt(&self, f: Formatter<'_>) {
        f.write_expr(syn::ExprRange {
            attrs: vec![],
            start: Some(Box::new(Formatter::process(&self.start))),
            limits: RangeLimits::HalfOpen(syn::token::DotDot::default()),
            end: None,
        });
    }
}

impl<T: DebugPls> DebugPls for ops::RangeTo<T> {
    fn fmt(&self, f: Formatter<'_>) {
        f.write_expr(syn::ExprRange {
            attrs: vec![],
            start: None,
            limits: RangeLimits::HalfOpen(syn::token::DotDot::default()),
            end: Some(Box::new(Formatter::process(&self.end))),
        });
    }
}

impl DebugPls for ops::RangeFull {
    fn fmt(&self, f: Formatter<'_>) {
        f.write_expr(syn::ExprRange {
            attrs: vec![],
            start: None,
            limits: RangeLimits::HalfOpen(syn::token::DotDot::default()),
            end: None,
        });
    }
}

impl<T: DebugPls> DebugPls for ops::RangeInclusive<T> {
    fn fmt(&self, f: Formatter<'_>) {
        f.write_expr(syn::ExprRange {
            attrs: vec![],
            start: Some(Box::new(Formatter::process(&self.start()))),
            limits: RangeLimits::Closed(syn::token::DotDotEq::default()),
            end: Some(Box::new(Formatter::process(&self.end()))),
        });
    }
}

impl<T: DebugPls> DebugPls for ops::RangeToInclusive<T> {
    fn fmt(&self, f: Formatter<'_>) {
        f.write_expr(syn::ExprRange {
            attrs: vec![],
            start: None,
            limits: RangeLimits::Closed(syn::token::DotDotEq::default()),
            end: Some(Box::new(Formatter::process(&self.end))),
        });
    }
}
