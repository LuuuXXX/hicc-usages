use inheritance_single::square_new;

#[test]
fn derived_class_with_inherited_method() {
    let s = square_new(4);
    assert_eq!(s.area(), 16);
    assert_eq!(s.side(), 4);
    // id() is inherited from Shape but exposed through Square:
    assert_eq!(s.id(), 1);
}
