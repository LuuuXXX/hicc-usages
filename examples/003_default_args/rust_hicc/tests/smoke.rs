use default_args::add;

#[test]
fn add_full_arity() {
    assert_eq!(add(5, 20), 25);
}

#[test]
fn add_with_cpp_default_value_explicit() {
    // The C++ default `b = 10` must be passed explicitly from Rust.
    assert_eq!(add(5, 10), 15);
}
