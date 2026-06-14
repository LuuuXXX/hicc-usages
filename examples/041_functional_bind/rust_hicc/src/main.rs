use functional_bind::*;

fn main() {
    let adder = make_adder(10);     // _1 + 10
    let adder_fn = adder.into();
    println!("adder(5) = {}", adder_fn(5));

    let mult = make_multiplier(3);  // _1 * 3
    let mult_fn = mult.into();
    println!("mult(4) = {}", mult_fn(4));

    let sub = make_subtractor(100); // _1 - 100
    let sub_fn = sub.into();
    println!("sub(150) = {}", sub_fn(150));

    println!("apply_bound(make_multiplier(3), 6) = {}", apply_bound(make_multiplier(3), 6));

    // (_1 + 10) * 3
    let composed = compose(make_multiplier(3), make_adder(10));
    let composed_fn = composed.into();
    println!("composed(2) = {}", composed_fn(2));

    let mut acc = BoundAccumulator::new(make_adder(10));
    println!("acc.call_and_accumulate(5) = {}", acc.call_and_accumulate(5));
    println!("acc.base() = {}", acc.base());
}
