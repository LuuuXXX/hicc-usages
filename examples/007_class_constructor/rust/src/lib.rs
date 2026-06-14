//! 自动生成：hicc_usage_class_constructor
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/class_constructor.h"
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::class_constructor::Point", destroy = "hicc_usages::class_constructor::Point::free")]
    pub class Point {
        #[cpp(method = "int get_x() const")]
        pub fn get_x(&self) -> i32;
        #[cpp(method = "int get_y() const")]
        pub fn get_y(&self) -> i32;
        #[cpp(method = "int distance_from_origin() const")]
        pub fn distance_from_origin(&self) -> i32;
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_class_constructor_adapter"]
    pub class Point;
    #[cpp(func = "hicc_usages::class_constructor::Point * hicc_usages::class_constructor::Point::create(int, int)")]
    pub fn point_new(x: i32, y: i32) -> Point;
}
