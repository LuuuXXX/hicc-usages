use default_args::*;

#[test]
fn greet_arities() {
    let n = hicc_std::string::from(c"X");
    assert_eq!(greet_default(&n), 1);
    assert_eq!(greet_times(&n, 3), 3);
    let s = hicc_std::string::from(c"?");
    assert_eq!(greet_full(&n, 2, &s), 2);
}

#[test]
fn compute_arities() {
    assert_eq!(compute_one(1), 111);   // 1 + 10 + 100
    assert_eq!(compute_two(1, 2), 103); // 1 + 2 + 100
    assert_eq!(compute_full(1, 2, 3), 6);
}
