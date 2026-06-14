//! 自动生成：hicc_usage_virtual_pure
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/virtual_pure.h"
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::virtual_pure::Shape")]
    pub class Shape {
        #[cpp(method = "double area() const")]
        pub fn area(&self) -> f64;
        #[cpp(method = "const char * name() const")]
        pub fn name(&self) -> *const i8;
    }
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::virtual_pure::Square", destroy = "hicc_usages::virtual_pure::Square::free")]
    pub class Square {
        #[cpp(method = "double area() const")]
        pub fn area(&self) -> f64;
        #[cpp(method = "const char * name() const")]
        pub fn name(&self) -> *const i8;
        #[cpp(method = "double side() const")]
        pub fn side(&self) -> f64;
    }
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::virtual_pure::Triangle", destroy = "hicc_usages::virtual_pure::Triangle::free")]
    pub class Triangle {
        #[cpp(method = "double area() const")]
        pub fn area(&self) -> f64;
        #[cpp(method = "const char * name() const")]
        pub fn name(&self) -> *const i8;
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_virtual_pure_adapter"]
    pub class Shape;
    pub class Square;
    pub class Triangle;
    #[cpp(func = "hicc_usages::virtual_pure::Square * hicc_usages::virtual_pure::Square::create(double)")]
    pub fn square_new(side: f64) -> Square;
    #[cpp(func = "hicc_usages::virtual_pure::Triangle * hicc_usages::virtual_pure::Triangle::create(double, double)")]
    pub fn triangle_new(base: f64, height: f64) -> Triangle;
}
