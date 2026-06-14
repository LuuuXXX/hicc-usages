use summary::{calc_new, calc_version, describe};
use std::ffi::CStr;

#[test]
fn synthesis_end_to_end() {
    // Free function returning const char*.
    let v = unsafe { CStr::from_ptr(calc_version()) };
    assert_eq!(v.to_str().unwrap(), "summary-calc-v1");

    // Class with factory/free + methods.
    let c = calc_new(100);
    assert_eq!(c.base(), 100);

    // int-bridge enum: OpKind::Add = 0.
    let ok_add = 0;
    assert_eq!(c.apply(ok_add, 2, 3).ok().unwrap(), 105);

    // OpKind::Div = 3 with y=0 → exception.
    let ok_div = 3;
    let err = c.apply(ok_div, 1, 0).ok();
    assert!(err.is_err());
    let info = err.unwrap_err();
    assert!(info.what().contains("divide by zero"), "got: {}", info.what());

    // std::string → const char* at boundary.
    let raw = unsafe { CStr::from_ptr(describe(&c)) };
    assert_eq!(raw.to_str().unwrap(), "Calculator(seed=100)");
}
