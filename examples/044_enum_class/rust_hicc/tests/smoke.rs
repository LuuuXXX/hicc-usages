use enum_class::*;
use std::ffi::CStr;

fn cs(s: &hicc_std::string) -> &str {
    unsafe { CStr::from_ptr(s.c_str()) }.to_str().unwrap_or("<bad utf-8>")
}

#[test]
fn color_round_trip() {
    assert_eq!(color_to_int(Color::Red), 0);
    assert_eq!(color_to_int(Color::Green), 1);
    assert_eq!(color_to_int(Color::Blue), 2);

    assert_eq!(color_from_int(0), Color::Red);
    assert_eq!(color_from_int(1), Color::Green);
    assert_eq!(color_from_int(2), Color::Blue);

    assert_eq!(cs(&color_name(Color::Red)), "red");
    assert_eq!(cs(&color_name(Color::Green)), "green");
    assert_eq!(cs(&color_name(Color::Blue)), "blue");
}

#[test]
fn color_parse_string() {
    assert_eq!(color_parse(&hicc_std::string::from(c"red")), Color::Red);
    assert_eq!(color_parse(&hicc_std::string::from(c"green")), Color::Green);
    assert_eq!(color_parse(&hicc_std::string::from(c"blue")), Color::Blue);
    // 未知 → 默认为 Red
    assert_eq!(color_parse(&hicc_std::string::from(c"orange")), Color::Red);
}

#[test]
fn status_round_trip() {
    assert_eq!(status_to_int(Status::Active), 10);
    assert_eq!(status_to_int(Status::Inactive), 20);
    assert_eq!(status_to_int(Status::Pending), 30);

    assert_eq!(status_from_int(10), Status::Active);
    assert_eq!(status_from_int(20), Status::Inactive);
    assert_eq!(status_from_int(30), Status::Pending);
}

#[test]
fn light_get_set() {
    let mut l = Light::new(Color::Red);
    assert_eq!(l.current(), Color::Red);
    assert_eq!(l.brightness(), 100);

    l.set(Color::Blue);
    assert_eq!(l.current(), Color::Blue);
    assert_eq!(l.brightness(), 300);
}
