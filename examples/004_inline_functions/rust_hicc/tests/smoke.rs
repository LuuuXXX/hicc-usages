use inline_functions::*;

#[test]
fn math_basics() {
    assert_eq!(square(5), 25);
    assert_eq!(cube(3), 27);
    assert_eq!(factorial(0), 1);
    assert_eq!(factorial(5), 120);
}
