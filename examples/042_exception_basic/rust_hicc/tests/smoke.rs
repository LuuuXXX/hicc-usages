use exception_basic::{checked_index, safe_divide, throwing_divide};

#[test]
fn exception_transparent_via_hicc_exception() {
    // Safe path: normal return.
    assert_eq!(safe_divide(10, 2), 5);

    // Exception path: hicc::Exception<T> carries what() string back to Rust.
    let ok = throwing_divide(10, 2).ok();
    assert_eq!(ok.unwrap(), 5);

    let err = throwing_divide(10, 0).ok();
    assert!(err.is_err());
    let info = err.unwrap_err();
    assert!(info.what().contains("divide by zero"), "got: {}", info.what());

    // Different exception type, same channel.
    let arr: [i32; 3] = [10, 20, 30];
    assert_eq!(checked_index(arr.as_ptr(), 3, 1).ok().unwrap(), 20);

    let oob = checked_index(arr.as_ptr(), 3, 9).ok();
    assert!(oob.is_err());
    let oob_info = oob.unwrap_err();
    assert!(oob_info.what().contains("out of range"), "got: {}", oob_info.what());
}
