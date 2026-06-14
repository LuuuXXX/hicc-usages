//! 自动生成：hicc_usage_explicit_ctor
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/explicit_ctor.h"
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::explicit_ctor::Distance", destroy = "hicc_usages::explicit_ctor::Distance::free")]
    pub class Distance {
        #[cpp(method = "int meters() const")]
        pub fn meters(&self) -> i32;
        #[cpp(method = "int feet() const")]
        pub fn feet(&self) -> i32;
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_explicit_ctor_adapter"]
    pub class Distance;
    #[cpp(func = "hicc_usages::explicit_ctor::Distance * hicc_usages::explicit_ctor::Distance::create_from_meters(int)")]
    pub fn distance_create_from_meters(m: i32) -> Distance;
    #[cpp(func = "hicc_usages::explicit_ctor::Distance * hicc_usages::explicit_ctor::Distance::create_from_feet(int)")]
    pub fn distance_create_from_feet(f: i32) -> Distance;
}
