//! 自动生成：hicc_usage_virtual_basic
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/virtual_basic.h"
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::virtual_basic::Shape", destroy = "hicc_usages::virtual_basic::Shape::free")]
    pub class Shape {
        #[cpp(method = "double area() const")]
        pub fn area(&self) -> f64;
        #[cpp(method = "const char * name() const")]
        pub fn name(&self) -> *const i8;
    }
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::virtual_basic::Circle", destroy = "hicc_usages::virtual_basic::Circle::free")]
    pub class Circle {
        #[cpp(method = "double area() const")]
        pub fn area(&self) -> f64;
        #[cpp(method = "const char * name() const")]
        pub fn name(&self) -> *const i8;
        #[cpp(method = "double radius() const")]
        pub fn radius(&self) -> f64;
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_virtual_basic_adapter"]
    pub class Shape;
    pub class Circle;
    #[cpp(func = "hicc_usages::virtual_basic::Shape * hicc_usages::virtual_basic::Shape::create()")]
    pub fn shape_new() -> Shape;
    #[cpp(func = "hicc_usages::virtual_basic::Circle * hicc_usages::virtual_basic::Circle::create(double)")]
    pub fn circle_new(radius: f64) -> Circle;
}
