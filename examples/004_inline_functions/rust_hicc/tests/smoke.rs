use inline_functions::*;

#[test]
fn math_basics() {
    assert_eq!(square(5), 25);
    assert_eq!(cube(3), 27);
    assert_eq!(factorial(0), 1);
    assert_eq!(factorial(5), 120);
}

#[test]
fn inline_edge_cases() {
    assert_eq!(square(0), 0);
    assert_eq!(square(-4), 16);
    assert_eq!(cube(-2), -8);
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(10), 3628800);
}
