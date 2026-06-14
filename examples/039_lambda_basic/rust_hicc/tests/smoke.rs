use lambda_basic::*;

#[test]
fn lambda_apply_int() {
    let dbl_fn: hicc::Function<fn(i32) -> i32> = (|v: i32| v * 2).into();
    assert_eq!(apply_int(5, dbl_fn), 10);

    let sq_fn: hicc::Function<fn(i32) -> i32> = (|v: i32| v * v).into();
    assert_eq!(apply_int(7, sq_fn), 49);
}

#[test]
fn lambda_make_adder_and_compose() {
    let add10_fn = make_adder(10);
    let add10 = add10_fn.into();
    assert_eq!(add10(7), 17);

    let mul2_fn: hicc::Function<fn(i32) -> i32> = (|v: i32| v * 2).into();
    let add5_fn = make_adder(5);
    let pipe = compose(mul2_fn, add5_fn);
    let pipe_cl = pipe.into();
    // (3+5) * 2 = 16
    assert_eq!(pipe_cl(3), 16);
}
