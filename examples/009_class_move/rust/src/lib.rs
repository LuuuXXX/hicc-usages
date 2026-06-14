//! 自动生成：hicc_usage_class_move
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/class_move.h"
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::class_move::Owner", destroy = "hicc_usages::class_move::Owner::free")]
    pub class Owner {
        #[cpp(method = "int get_value() const")]
        pub fn get_value(&self) -> i32;
        #[cpp(method = "bool is_valid() const")]
        pub fn is_valid(&self) -> bool;
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_class_move_adapter"]
    pub class Owner;
    #[cpp(func = "hicc_usages::class_move::Owner * hicc_usages::class_move::Owner::create(int)")]
    pub fn owner_new(value: i32) -> Owner;
}
