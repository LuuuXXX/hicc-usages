//! 自动生成：hicc_usage_vector_basic
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/vector_basic.h"
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::vector_basic::IntVector", destroy = "hicc_usages::vector_basic::IntVector::free")]
    pub class IntVector {
        #[cpp(method = "void push_back(int)")]
        pub fn push_back(&mut self, v: i32) -> ();
        #[cpp(method = "int at(std::size_t) const")]
        pub fn at(&self, idx: usize) -> i32;
        #[cpp(method = "void pop_back()")]
        pub fn pop_back(&mut self) -> ();
        #[cpp(method = "int sum() const")]
        pub fn sum(&self) -> i32;
        #[cpp(method = "void clear()")]
        pub fn clear(&mut self) -> ();
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_vector_basic_adapter"]
    pub class IntVector;
    #[cpp(func = "hicc_usages::vector_basic::IntVector * hicc_usages::vector_basic::IntVector::create()")]
    pub fn intvector_new() -> IntVector;
}
