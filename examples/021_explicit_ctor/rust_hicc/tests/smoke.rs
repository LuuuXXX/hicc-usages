use explicit_ctor::{convert_to_celsius, fahrenheit_new};

#[test]
fn explicit_conversion_via_factory() {
    let f = fahrenheit_new(212.0);
    let c = convert_to_celsius(&f);
    assert!((c.value() - 100.0).abs() < 1e-9);
}
