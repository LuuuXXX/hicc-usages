use functional_bind::{
    add_bound_10, bind_point_new, mul_bound_3, point_x_plus_offset, sub_bind_first, BindPoint,
};

#[test]
fn bind_results_via_named_wrappers() {
    assert_eq!(add_bound_10(5), 15);
    assert_eq!(mul_bound_3(7), 21);
    assert_eq!(sub_bind_first(10, 3), 7);

    let p: BindPoint = bind_point_new(42, 100);
    assert_eq!(point_x_plus_offset(&p, 8), 50);
}
