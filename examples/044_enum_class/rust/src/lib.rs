//! 自动生成：hicc_usage_enum_class
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/enum_class.h"
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::enum_class::Pixel", destroy = "hicc_usages::enum_class::Pixel::free")]
    pub class Pixel {
        #[cpp(method = "int x() const")]
        pub fn x(&self) -> i32;
        #[cpp(method = "int y() const")]
        pub fn y(&self) -> i32;
        #[cpp(method = "bool is_warm() const")]
        pub fn is_warm(&self) -> bool;
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_enum_class_adapter"]
    pub class Pixel;
    #[cpp(func = "int hicc_usages::enum_class::direction_opposite(int)")]
    pub fn direction_opposite(d: i32) -> i32;
}
