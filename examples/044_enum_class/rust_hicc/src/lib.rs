//! 044_enum_class: enum class via int<->enum converters
//!
//! hicc 模式：`enum class` 不能直接 FFI（hicc 端无法表达 C++ scoped enum 的 ABI），
//! 所以 C++ 侧写 `*_to_int` / `*_from_int` 转换器，Rust 侧定义对应的 `#[repr(i32)] enum`
//! 作为镜像（仅作类型安全），所有跨 FFI 边界都用 `i32` 传输。
//!
//! 关键点：
//! - Rust 端的 `Color`/`Status` 镜像 enum 只是类型糖，FFI 时仍按 `i32` 传
//! - 类 `Light` 的 `current()` 返回 `Color`，所以 C++ 侧加一个 `current_int()`
//!   返回 `int` 的 inline 包装；同理 `set_from_int`

hicc::cpp! {
    #include "enum_class.h"
    #include <hicc/std/string.hpp>

    // Inline wrappers that return/accept int directly to bridge enum class
    inline int light_current_int(const enum_class_ns::Light& l) {
        return static_cast<int>(l.current());
    }
    inline void light_set_int(enum_class_ns::Light& l, int c) {
        l.set(static_cast<enum_class_ns::Color>(c));
    }
}

/// Rust-side mirror of `enum_class_ns::Color`. Sent across FFI as `i32`.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color { Red = 0, Green = 1, Blue = 2 }

/// Rust-side mirror of `enum_class_ns::Status`. Sent across FFI as `i32`.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status { Active = 10, Inactive = 20, Pending = 30 }

hicc::import_class! {
    #[cpp(class = "enum_class_ns::Light")]
    pub class Light {
        #[cpp(method = "int brightness() const")]
        pub fn brightness(&self) -> i32;

        // Bridge to int-returning inline wrapper
        pub fn current(&self) -> Color {
            let v: i32 = light_current_int(self);
            color_from_int(v)
        }
        pub fn set(&mut self, c: Color) {
            light_set_int(self, c as i32);
        }
        pub fn new(initial: Color) -> Self { make_light(initial) }
    }
}

hicc::import_lib! {
    #![link_name = "enum_class"]

    class string = hicc_std::string;

    #[cpp(func = "int enum_class_ns::color_to_int(enum_class_ns::Color)")]
    pub fn color_to_int_raw(c: i32) -> i32;

    #[cpp(func = "enum_class_ns::Color enum_class_ns::color_from_int(int)")]
    pub fn color_from_int_raw(v: i32) -> i32;

    #[cpp(func = "std::string enum_class_ns::color_name(enum_class_ns::Color)")]
    pub fn color_name_raw(c: i32) -> hicc_std::string;

    #[cpp(func = "enum_class_ns::Color enum_class_ns::color_parse(const std::string&)")]
    pub fn color_parse_raw(s: &hicc_std::string) -> i32;

    #[cpp(func = "int enum_class_ns::status_to_int(enum_class_ns::Status)")]
    pub fn status_to_int_raw(s: i32) -> i32;

    #[cpp(func = "enum_class_ns::Status enum_class_ns::status_from_int(int)")]
    pub fn status_from_int_raw(v: i32) -> i32;

    // Light factory takes Color — wrap to int via inline wrapper
    #[cpp(func = "enum_class_ns::Light enum_class_ns::make_light(enum_class_ns::Color)")]
    pub fn make_light_raw(initial: i32) -> Light;

    // Inline wrappers from cpp! block
    #[cpp(func = "int light_current_int(const enum_class_ns::Light&)")]
    pub fn light_current_int(l: &Light) -> i32;

    #[cpp(func = "void light_set_int(enum_class_ns::Light&, int)")]
    pub fn light_set_int(l: &mut Light, c: i32);
}

// ---- Type-safe Rust wrappers ----

pub fn color_to_int(c: Color) -> i32 { color_to_int_raw(c as i32) }
pub fn color_from_int(v: i32) -> Color {
    let raw = color_from_int_raw(v);
    match raw {
        0 => Color::Red,
        1 => Color::Green,
        2 => Color::Blue,
        _ => Color::Red,
    }
}
pub fn color_name(c: Color) -> hicc_std::string { color_name_raw(c as i32) }
pub fn color_parse(s: &hicc_std::string) -> Color {
    color_from_int(color_parse_raw(s))
}
pub fn status_to_int(s: Status) -> i32 { status_to_int_raw(s as i32) }
pub fn status_from_int(v: i32) -> Status {
    match status_from_int_raw(v) {
        10 => Status::Active,
        20 => Status::Inactive,
        _ => Status::Pending,
    }
}
pub fn make_light(initial: Color) -> Light { make_light_raw(initial as i32) }
