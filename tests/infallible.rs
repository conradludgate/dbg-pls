use std::{convert::Infallible, ops::ControlFlow};

use dbg_pls::DebugWith;

#[test]
fn infallible_types_support_any_context() {
    fn assert_context<T: DebugWith<u32>>() {}

    assert_context::<Infallible>();
    assert_context::<Result<u32, Infallible>>();
    assert_context::<ControlFlow<Infallible, u32>>();
}

#[test]
fn infallible_containers_format_their_inhabited_variants() {
    let result: Result<u32, Infallible> = Ok(42);
    let flow: ControlFlow<Infallible, u32> = ControlFlow::Continue(42);

    assert_eq!(dbg_pls::pretty(&result).to_string(), "Ok(42)");
    assert_eq!(dbg_pls::pretty(&flow).to_string(), "Continue(42)");
}
