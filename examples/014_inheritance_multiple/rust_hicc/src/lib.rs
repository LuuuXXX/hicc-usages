//! 014_inheritance_multiple: 多继承（Drawable + Serializable）
//!
//! hicc 模式：直接绑定具体派生类 Circle/Square。多继承基类不暴露（多继承
//! 的 #[interface] hicc 不支持）。所有基类方法在派生类中通过 override 实现，
//! 派生类调用这些方法名时走派生类的 vtable。

hicc::cpp! {
    #include "inheritance_multiple.h"
    #include <hicc/std/string.hpp>
}

hicc::import_class! {
    class string = hicc_std::string;

    #[cpp(class = "inheritance_multiple_ns::Circle")]
    pub class Circle {
        #[cpp(method = "void draw() const")]
        pub fn draw(&self);

        #[cpp(method = "std::string shape() const")]
        pub fn shape(&self) -> string;

        #[cpp(method = "std::string serialize() const")]
        pub fn serialize(&self) -> string;

        #[cpp(method = "int bytes() const")]
        pub fn bytes(&self) -> i32;

        #[cpp(method = "float radius() const")]
        pub fn radius(&self) -> f32;

        pub fn new(r: f32) -> Self { circle_new(r) }
    }

    #[cpp(class = "inheritance_multiple_ns::Square")]
    pub class Square {
        #[cpp(method = "void draw() const")]
        pub fn draw(&self);

        #[cpp(method = "std::string shape() const")]
        pub fn shape(&self) -> string;

        #[cpp(method = "std::string serialize() const")]
        pub fn serialize(&self) -> string;

        #[cpp(method = "float side() const")]
        pub fn side(&self) -> f32;

        pub fn new(side: f32) -> Self { square_new(side) }
    }
}

hicc::import_lib! {
    #![link_name = "inheritance_multiple"]

    #[cpp(func = "std::unique_ptr<inheritance_multiple_ns::Circle> hicc::make_unique<inheritance_multiple_ns::Circle, float>(float&&)")]
    pub fn circle_new(r: f32) -> Circle;

    #[cpp(func = "std::unique_ptr<inheritance_multiple_ns::Square> hicc::make_unique<inheritance_multiple_ns::Square, float>(float&&)")]
    pub fn square_new(side: f32) -> Square;
}
