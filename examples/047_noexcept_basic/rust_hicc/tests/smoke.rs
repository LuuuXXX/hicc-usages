use noexcept_basic::*;
use std::ffi::CStr;

fn cs(s: &hicc_std::string) -> &str {
    unsafe { CStr::from_ptr(s.c_str()) }.to_str().unwrap_or("<bad utf-8>")
}

#[test]
fn noexcept_free_functions() {
    assert_eq!(add_noexcept(2, 3), 5);
    assert_eq!(square_noexcept(7), 49);
    assert!((safe_reciprocal_noexcept(4.0) - 0.25).abs() < 1e-10);
    assert_eq!(safe_reciprocal_noexcept(0.0), 0.0);
    assert_eq!(compute_constant(), 42);
}

#[test]
fn safe_counter_methods() {
    let mut c = SafeCounter::new();
    assert_eq!(c.get(), 0);
    c.increment(5);
    c.increment(3);
    assert_eq!(c.get(), 8);
    assert_eq!(cs(&c.describe()), "SafeCounter(8)");
    c.reset();
    assert_eq!(c.get(), 0);
}

#[test]
fn buffer_get_set() {
    let mut b = Buffer::new(4);
    assert_eq!(b.size(), 4);
    b.set(0, 10);
    b.set(3, 40);
    assert_eq!(b.get(0), 10);
    assert_eq!(b.get(3), 40);
    assert_eq!(b.get(1), 0);  // uninit -> 0

    // out-of-range get -> safe (noexcept)
    assert_eq!(b.get(99), 0);
    // out-of-range set -> safe
    b.set(99, 100);
    assert_eq!(b.get(0), 10);  // unchanged
}

#[test]
fn may_throw_via_exception() {
    assert_eq!(may_throw(5).ok().unwrap(), 10);
    let err = may_throw(-1).ok().unwrap_err();
    assert!(err.what().contains("negative"));
}
