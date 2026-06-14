use enum_class::{color_name_for_int, to_int_blue, to_int_green, to_int_red, Color};
use std::ffi::CStr;

#[test]
fn enum_class_via_int_bridge() {
    assert_eq!(to_int_red(), 0);
    assert_eq!(to_int_green(), 1);
    assert_eq!(to_int_blue(), 2);

    let green = Color::from_raw(to_int_green());
    assert_eq!(green, Color::Green);

    let raw_name = unsafe { CStr::from_ptr(color_name_for_int(2)) };
    assert_eq!(raw_name.to_str().unwrap(), "blue");

    assert_eq!(Color::Red.name(), "red");
    assert_eq!(Color::Green.name(), "green");
    assert_eq!(Color::Blue.name(), "blue");
}
