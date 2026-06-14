//! 自动生成：hicc_usage_template_specialization
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/template_specialization.h"

    // ============================================================
    // 自动生成的活跃 C++ 包装（untouched C++ 模式）
    // 这些代码是 Rust crate 正常工作所必需的（类模板 typedef + factory 等）
    // ============================================================
    namespace hicc_usages::template_specialization { inline const char * type_name_int() { return TypeInfo<int>::name(); } }
    namespace hicc_usages::template_specialization { inline std::size_t size_of_int() { return TypeInfo<int>::size_of(); } }
    namespace hicc_usages::template_specialization { inline const char * type_name_double() { return TypeInfo<double>::name(); } }
    namespace hicc_usages::template_specialization { inline std::size_t size_of_double() { return TypeInfo<double>::size_of(); } }
    namespace hicc_usages::template_specialization { inline const char * type_name_char() { return TypeInfo<char>::name(); } }
    namespace hicc_usages::template_specialization { inline std::size_t size_of_char() { return TypeInfo<char>::size_of(); } }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_template_specialization_adapter"]
    #[cpp(func = "const char * hicc_usages::template_specialization::type_name_int()")]
    pub fn type_name_int() -> *const i8;
    #[cpp(func = "std::size_t hicc_usages::template_specialization::size_of_int()")]
    pub fn size_of_int() -> usize;
    #[cpp(func = "const char * hicc_usages::template_specialization::type_name_double()")]
    pub fn type_name_double() -> *const i8;
    #[cpp(func = "std::size_t hicc_usages::template_specialization::size_of_double()")]
    pub fn size_of_double() -> usize;
    #[cpp(func = "const char * hicc_usages::template_specialization::type_name_char()")]
    pub fn type_name_char() -> *const i8;
    #[cpp(func = "std::size_t hicc_usages::template_specialization::size_of_char()")]
    pub fn size_of_char() -> usize;
}
