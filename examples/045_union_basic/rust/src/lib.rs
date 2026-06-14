//! 自动生成：hicc_usage_union_basic
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/union_basic.h"

    // ============================================================
    // 自动生成的建议包装（untouched C++ 模式）
    // 取消注释以把对应的 C++ 功能（operator / union 等）暴露到 Rust
    // 不需要就保持注释，对生成的 Rust crate 无影响
    // ============================================================
    // namespace hicc_usages::union_basic_autogen {
    //     class ValueBox {
    //     public:
    //         static hicc_usages::union_basic_autogen::ValueBox* create() { ValueBox* b = new ValueBox; b->u_ = new hicc_usages::union_basic::Value{}; b->tag_ = 0; return b; }
    //         static hicc_usages::union_basic_autogen::ValueBox* from_as_int(int v) { ValueBox* b = create(); b->u_->as_int = v; b->tag_ = 0; return b; }
    //         static hicc_usages::union_basic_autogen::ValueBox* from_as_double(double v) { ValueBox* b = create(); b->u_->as_double = v; b->tag_ = 1; return b; }
    //         static hicc_usages::union_basic_autogen::ValueBox* from_as_long(long v) { ValueBox* b = create(); b->u_->as_long = v; b->tag_ = 2; return b; }
    //         static void free(hicc_usages::union_basic_autogen::ValueBox* self) { if (self) { delete self->u_; delete self; } }
    //         int as_int() const { return u_->as_int; }
    //         double as_double() const { return u_->as_double; }
    //         long as_long() const { return u_->as_long; }
    //         int type_tag() const { return tag_; }
    //     private:
    //         hicc_usages::union_basic::Value* u_;
    //         int tag_;
    //     };
    // }
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::union_basic::Pair", destroy = "hicc_usages::union_basic::Pair::free")]
    pub class Pair {
        #[cpp(method = "int first() const")]
        pub fn first(&self) -> i32;
        #[cpp(method = "int second() const")]
        pub fn second(&self) -> i32;
        #[cpp(method = "int sum() const")]
        pub fn sum(&self) -> i32;
        #[cpp(method = "int max() const")]
        pub fn max(&self) -> i32;
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_union_basic_adapter"]
    pub class Pair;
    #[cpp(func = "hicc_usages::union_basic::Pair * hicc_usages::union_basic::Pair::create(int, int)")]
    pub fn pair_new(first: i32, second: i32) -> Pair;
}

// ============================================================
// 自动生成的顶层建议（取消注释启用，例如 union 包装类）
// ============================================================
// hicc::import_class! {
//     #[cpp(class = "hicc_usages::union_basic_autogen::ValueBox", destroy = "hicc_usages::union_basic_autogen::ValueBox::free")]
//     pub class ValueBox {
//         #[cpp(method = "int as_int() const")]
//         pub fn as_int(&self) -> i32;
//         #[cpp(method = "double as_double() const")]
//         pub fn as_double(&self) -> f64;
//         #[cpp(method = "long as_long() const")]
//         pub fn as_long(&self) -> i64;
//     }
// }
// // 将以下条目添加到 import_lib! 块中：
//     pub class ValueBox;
//     #[cpp(func = "hicc_usages::union_basic_autogen::ValueBox * hicc_usages::union_basic_autogen::ValueBox::from_as_int(int)")]
//     pub fn value_box_from_int(v: i32) -> ValueBox;  // sample: 0
//     #[cpp(func = "hicc_usages::union_basic_autogen::ValueBox * hicc_usages::union_basic_autogen::ValueBox::from_as_double(double)")]
//     pub fn value_box_from_double(v: f64) -> ValueBox;  // sample: 0.0
//     #[cpp(func = "hicc_usages::union_basic_autogen::ValueBox * hicc_usages::union_basic_autogen::ValueBox::from_as_long(long)")]
//     pub fn value_box_from_long(v: i64) -> ValueBox;  // sample: 0
