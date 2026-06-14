use class_constructor::point_new;

#[test]
fn point_construct_and_query() {
    let p = point_new(3, -4);
    assert_eq!(p.get_x(), 3);
    assert_eq!(p.get_y(), -4);
    assert_eq!(p.manhattan(), 7);
}
