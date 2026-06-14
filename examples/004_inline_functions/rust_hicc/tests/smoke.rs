use inline_functions::{cube, square};

#[test]
fn square_inline() {
    assert_eq!(square(4), 16);
    assert_eq!(square(-3), 9);
}

#[test]
fn cube_inline() {
    assert_eq!(cube(3), 27);
    assert_eq!(cube(-2), -8);
}
