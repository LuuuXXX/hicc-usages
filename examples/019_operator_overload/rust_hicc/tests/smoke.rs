use operator_overload::*;

#[test]
fn binary_ops() {
    let a = Vec2::new(1.0, 2.0);
    let b = Vec2::new(3.0, 4.0);
    let c = vec_add(&a, &b);
    assert!((c.x() - 4.0).abs() < 0.001);
    assert!((c.y() - 6.0).abs() < 0.001);
    let d = vec_sub(&a, &b);
    assert!((d.x() + 2.0).abs() < 0.001);
    assert!((d.y() + 2.0).abs() < 0.001);
}

#[test]
fn scale_neg_eq_at() {
    let a = Vec2::new(1.0, 2.0);
    let e = vec_scale(&a, 2.0);
    assert!((e.x() - 2.0).abs() < 0.001);
    let f = vec_neg(&a);
    assert!((f.x() + 1.0).abs() < 0.001);

    let b = Vec2::new(1.0, 2.0);
    assert!(vec_eq(&a, &b));

    assert!((vec_at(&a, 0) - 1.0).abs() < 0.001);
    assert!((vec_at(&a, 1) - 2.0).abs() < 0.001);
}

#[test]
fn compound_assign() {
    let mut g = Vec2::new(0.0, 0.0);
    let a = Vec2::new(1.0, 2.0);
    vec_iadd(&mut g, &a);
    assert!((g.x() - 1.0).abs() < 0.001);
    assert!((g.y() - 2.0).abs() < 0.001);
}
