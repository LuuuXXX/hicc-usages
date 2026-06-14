use std_function::*;

fn main() {
    // 1. apply_dbl with a Rust closure
    let dbl_fn: hicc::Function<fn(i32) -> i32> = (|v: i32| v * 2).into();
    println!("apply_dbl = {}", apply_dbl(dbl_fn, 5));

    // 2. Callback wraps a function; persist + invoke + replace
    let add100_fn: hicc::Function<fn(i32) -> i32> = (|v: i32| v + 100).into();
    let mut cb = Callback::new(add100_fn);
    println!("cb.invoke(1) = {}", cb.invoke(1));
    println!("cb.call_n_times(2, 3) = {}", cb.call_n_times(2, 3));

    let sq_fn: hicc::Function<fn(i32) -> i32> = (|v: i32| v * v).into();
    cb.replace(sq_fn);
    println!("after replace cb.invoke(3) = {}", cb.invoke(3));

    // 3. C++-built doubler, used in chain
    let d_fn = make_doubler();
    let add5_fn: hicc::Function<fn(i32) -> i32> = (|v: i32| v + 5).into();
    println!("chain(d, +5, 3) = {}", chain(d_fn, add5_fn, 3));
}
