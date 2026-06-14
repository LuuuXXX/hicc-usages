// enum class is not nameable across FFI; values pass as `int`. Rust
// mirrors the enum on its side and converts at the boundary.

use std::ffi::CStr;

hicc::cpp! {
    #include "enum_class.h"
}

hicc::import_lib! {
    #![link_name = "enum_class_hicc"]

    #[cpp(func = "int to_int_red()")]
    pub fn to_int_red() -> i32;

    #[cpp(func = "int to_int_green()")]
    pub fn to_int_green() -> i32;

    #[cpp(func = "int to_int_blue()")]
    pub fn to_int_blue() -> i32;

    #[cpp(func = "const char* color_name_for_int(int)")]
    pub fn color_name_for_int(v: i32) -> *const i8;
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color { Red = 0, Green = 1, Blue = 2 }

impl Color {
    pub fn from_raw(v: i32) -> Self {
        match v {
            0 => Color::Red,
            1 => Color::Green,
            2 => Color::Blue,
            _ => Color::Red,
        }
    }
    pub fn name(self) -> &'static str {
        let raw = unsafe { CStr::from_ptr(color_name_for_int(self as i32)) };
        match raw.to_str().unwrap_or("?") {
            "red" => "red",
            "green" => "green",
            "blue" => "blue",
            _ => "?",
        }
    }
}
