use crate::{DebugPls, Formatter};

macro_rules! fnptr_impls_safety_abi {
    ($FnTy: ty, $($Arg: ident),*) => {
        impl<Ret, $($Arg),*> DebugPls for $FnTy {
            fn fmt(&self, f: Formatter<'_>) {
                // HACK: The intermediate cast as usize is required for AVR
                // so that the address space of the source function pointer
                // is preserved in the final function pointer.
                //
                // https://github.com/avr-rust/rust/issues/143
                DebugPls::fmt(&(*self as usize as *const ()), f)
            }
        }
    }
}

macro_rules! peel {
    ($name:ident, $($other:ident,)*) => (fnptr_impls_args! { $($other),* })
}

macro_rules! fnptr_impls_args {
    ($($Arg: ident),+) => {
        fnptr_impls_safety_abi! { extern "Rust" fn($($Arg),+) -> Ret, $($Arg),+ }
        fnptr_impls_safety_abi! { extern "C" fn($($Arg),+) -> Ret, $($Arg),+ }
        fnptr_impls_safety_abi! { extern "C" fn($($Arg),+ , ...) -> Ret, $($Arg),+ }
        fnptr_impls_safety_abi! { unsafe extern "Rust" fn($($Arg),+) -> Ret, $($Arg),+ }
        fnptr_impls_safety_abi! { unsafe extern "C" fn($($Arg),+) -> Ret, $($Arg),+ }
        fnptr_impls_safety_abi! { unsafe extern "C" fn($($Arg),+ , ...) -> Ret, $($Arg),+ }

        peel! { $($Arg,)+ }
    };
    () => {
        // No variadic functions with 0 parameters
        fnptr_impls_safety_abi! { extern "Rust" fn() -> Ret, }
        fnptr_impls_safety_abi! { extern "C" fn() -> Ret, }
        fnptr_impls_safety_abi! { unsafe extern "Rust" fn() -> Ret, }
        fnptr_impls_safety_abi! { unsafe extern "C" fn() -> Ret, }
    };
}

fnptr_impls_args! { A, B, C, D, E, F, G, H, I, J, K, L }
