use constexpr_basic::{cube, square_plus_magic, sq};

#[test]
fn constexpr_transparent() {
    assert_eq!(sq(5), 25);
    assert_eq!(cube(3), 27);
    assert_eq!(square_plus_magic(4), 16 + 7);
}
