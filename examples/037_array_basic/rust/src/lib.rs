//! 自动生成：hicc_usage_array_basic
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/array_basic.h"
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::array_basic::FixedArray", destroy = "hicc_usages::array_basic::FixedArray::free")]
    pub class FixedArray {
        #[cpp(method = "void set(std::size_t, int)")]
        pub fn set(&mut self, idx: usize, v: i32) -> ();
        #[cpp(method = "int get(std::size_t) const")]
        pub fn get(&self, idx: usize) -> i32;
        #[cpp(method = "int sum() const")]
        pub fn sum(&self) -> i32;
        #[cpp(method = "int max() const")]
        pub fn max(&self) -> i32;
        #[cpp(method = "void fill(int)")]
        pub fn fill(&mut self, v: i32) -> ();
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_array_basic_adapter"]
    pub class FixedArray;
    #[cpp(func = "hicc_usages::array_basic::FixedArray * hicc_usages::array_basic::FixedArray::create()")]
    pub fn fixedarray_new() -> FixedArray;
}
