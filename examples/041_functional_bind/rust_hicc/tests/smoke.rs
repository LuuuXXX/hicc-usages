use functional_bind::*;

#[test]
fn make_adder_multiplier_subtractor() {
    let adder = make_adder(10);
    let adder_fn = adder.into();
    assert_eq!(adder_fn(5), 15);

    let mult = make_multiplier(3);
    let mult_fn = mult.into();
    assert_eq!(mult_fn(4), 12);

    let sub = make_subtractor(100);
    let sub_fn = sub.into();
    assert_eq!(sub_fn(150), 50);
}

#[test]
fn apply_and_compose() {
    assert_eq!(apply_bound(make_multiplier(3), 6), 18);

    // (_1 + 10) * 3
    let composed = compose(make_multiplier(3), make_adder(10));
    let composed_fn = composed.into();
    assert_eq!(composed_fn(2), 36);
}

#[test]
fn accumulator_state() {
    let mut acc = BoundAccumulator::new(make_adder(10));
    // base_ = 0, x = 5 -> fn_(0+5) = 15, base_ becomes 15
    assert_eq!(acc.call_and_accumulate(5), 15);
    assert_eq!(acc.base(), 15);
    // base_ = 15, x = 5 -> fn_(20) = 30, base_ becomes 45
    assert_eq!(acc.call_and_accumulate(5), 30);
    assert_eq!(acc.base(), 45);

    acc.reset(0);
    assert_eq!(acc.base(), 0);
}
