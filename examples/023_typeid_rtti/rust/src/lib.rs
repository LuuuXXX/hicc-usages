//! 自动生成：hicc_usage_typeid_rtti
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/typeid_rtti.h"
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::typeid_rtti::Shape")]
    pub class Shape {
        #[cpp(method = "const char * type_name() const")]
        pub fn type_name(&self) -> *const i8;
        #[cpp(method = "int id() const")]
        pub fn id(&self) -> i32;
    }
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::typeid_rtti::Circle", destroy = "hicc_usages::typeid_rtti::Circle::free")]
    pub class Circle {
        #[cpp(method = "const char * type_name() const")]
        pub fn type_name(&self) -> *const i8;
        #[cpp(method = "int id() const")]
        pub fn id(&self) -> i32;
        #[cpp(method = "double radius() const")]
        pub fn radius(&self) -> f64;
        #[cpp(method = "bool is_circle() const")]
        pub fn is_circle(&self) -> bool;
    }
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::typeid_rtti::Square", destroy = "hicc_usages::typeid_rtti::Square::free")]
    pub class Square {
        #[cpp(method = "const char * type_name() const")]
        pub fn type_name(&self) -> *const i8;
        #[cpp(method = "int id() const")]
        pub fn id(&self) -> i32;
        #[cpp(method = "double side() const")]
        pub fn side(&self) -> f64;
        #[cpp(method = "bool is_square() const")]
        pub fn is_square(&self) -> bool;
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_typeid_rtti_adapter"]
    pub class Shape;
    pub class Circle;
    pub class Square;
    #[cpp(func = "hicc_usages::typeid_rtti::Circle * hicc_usages::typeid_rtti::Circle::create(double)")]
    pub fn circle_new(r: f64) -> Circle;
    #[cpp(func = "hicc_usages::typeid_rtti::Square * hicc_usages::typeid_rtti::Square::create(double)")]
    pub fn square_new(s: f64) -> Square;
}
