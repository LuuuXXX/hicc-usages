use template_function::{add_tmpl_f64, add_tmpl_i32, identity_f64, identity_i32};

#[test]
fn template_per_instantiation() {
    assert_eq!(identity_i32(7), 7);
    assert!((identity_f64(2.5) - 2.5).abs() < 1e-9);
    assert_eq!(add_tmpl_i32(2, 3), 5);
    assert!((add_tmpl_f64(1.5, 2.5) - 4.0).abs() < 1e-9);
}
