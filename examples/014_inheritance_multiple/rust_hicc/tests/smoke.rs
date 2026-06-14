use inheritance_multiple::sprite_new;

#[test]
fn multi_inheritance_flattened() {
    let s = sprite_new(16, 8);
    s.draw();
    assert_eq!(s.byte_size(), 16 * 8 * 4);
    assert_eq!(s.width(), 16);
    assert_eq!(s.height(), 8);
}
