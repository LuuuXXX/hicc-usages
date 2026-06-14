use class_const::vec2_new;

#[test]
fn magnitude_3_4_5() {
    let v = vec2_new(3.0, 4.0);
    assert!((v.magnitude() - 5.0).abs() < 1e-9);
}

#[test]
fn dot_product() {
    let a = vec2_new(1.0, 2.0);
    let b = vec2_new(3.0, 4.0);
    assert!((a.dot(&b) - 11.0).abs() < 1e-9);
}
