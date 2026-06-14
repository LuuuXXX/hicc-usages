//! 015_virtual_basic: virtual override + 基类有默认实现
//!
//! hicc 模式：派生类 Rectangle/Ellipse 独立 import_class!。
//! C++ 中 `shape.area()` 走派生类 vtable（C++ 自动），hicc 只需声明方法签名。

hicc::cpp! {
    #include "virtual_basic.h"
    #include <hicc/std/string.hpp>
}

hicc::import_class! {
    class string = hicc_std::string;

    #[cpp(class = "virtual_basic_ns::Rectangle")]
    pub class Rectangle {
        #[cpp(method = "const std::string& name() const")]
        pub fn name(&self) -> &string;

        #[cpp(method = "float area() const")]
        pub fn area(&self) -> f32;

        #[cpp(method = "float perimeter() const")]
        pub fn perimeter(&self) -> f32;

        #[cpp(method = "std::string describe() const")]
        pub fn describe(&self) -> string;

        pub fn new(w: f32, h: f32) -> Self { rectangle_new(w, h) }
    }

    #[cpp(class = "virtual_basic_ns::Ellipse")]
    pub class Ellipse {
        #[cpp(method = "const std::string& name() const")]
        pub fn name(&self) -> &string;

        #[cpp(method = "float area() const")]
        pub fn area(&self) -> f32;

        #[cpp(method = "float perimeter() const")]
        pub fn perimeter(&self) -> f32;

        #[cpp(method = "std::string describe() const")]
        pub fn describe(&self) -> string;

        pub fn new(a: f32, b: f32) -> Self { ellipse_new(a, b) }
    }
}

hicc::import_lib! {
    #![link_name = "virtual_basic"]

    #[cpp(func = "std::unique_ptr<virtual_basic_ns::Rectangle> hicc::make_unique<virtual_basic_ns::Rectangle, float, float>(float&&, float&&)")]
    pub fn rectangle_new(w: f32, h: f32) -> Rectangle;

    #[cpp(func = "std::unique_ptr<virtual_basic_ns::Ellipse> hicc::make_unique<virtual_basic_ns::Ellipse, float, float>(float&&, float&&)")]
    pub fn ellipse_new(a: f32, b: f32) -> Ellipse;
}
