use std_function::*;

#[test]
fn std_function_apply_dbl_and_chain() {
    let dbl_fn: hicc::Function<fn(i32) -> i32> = (|v: i32| v * 2).into();
    assert_eq!(apply_dbl(dbl_fn, 5), 20);

    let d_fn = make_doubler();
    let add5_fn: hicc::Function<fn(i32) -> i32> = (|v: i32| v + 5).into();
    assert_eq!(chain(d_fn, add5_fn, 3), 16);
}

#[test]
fn callback_invoke_replace_n_times() {
    let add100_fn: hicc::Function<fn(i32) -> i32> = (|v: i32| v + 100).into();
    let mut cb = Callback::new(add100_fn);
    assert_eq!(cb.invoke(1), 101);
    assert_eq!(cb.call_n_times(2, 3), 306);  // (100+2)*3 = 306

    let sq_fn: hicc::Function<fn(i32) -> i32> = (|v: i32| v * v).into();
    cb.replace(sq_fn);
    assert_eq!(cb.invoke(3), 9);
}
