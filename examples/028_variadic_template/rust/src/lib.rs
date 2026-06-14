//! 自动生成：hicc_usage_variadic_template
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/variadic_template.h"

    // ============================================================
    // 自动生成的活跃 C++ 包装（untouched C++ 模式）
    // 这些代码是 Rust crate 正常工作所必需的（类模板 typedef + factory 等）
    // ============================================================
    namespace hicc_usages::variadic_template { inline int sum_two(int a0, int a1) { return sum_all(a0, a1); } }
    namespace hicc_usages::variadic_template { inline int sum_three(int a0, int a1, int a2) { return sum_all(a0, a1, a2); } }
    namespace hicc_usages::variadic_template { inline int count_two(int a0, int a1) { return count_all(a0, a1); } }
    namespace hicc_usages::variadic_template { inline int count_three(int a0, int a1, int a2) { return count_all(a0, a1, a2); } }
    namespace hicc_usages::variadic_template { inline int max_three(int a0, int a1, int a2) { return max_all(a0, a1, a2); } }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_variadic_template_adapter"]
    #[cpp(func = "int hicc_usages::variadic_template::sum_two(int a0, int a1)")]
    pub fn sum_two(a0: i32, a1: i32) -> i32;
    #[cpp(func = "int hicc_usages::variadic_template::sum_three(int a0, int a1, int a2)")]
    pub fn sum_three(a0: i32, a1: i32, a2: i32) -> i32;
    #[cpp(func = "int hicc_usages::variadic_template::count_two(int a0, int a1)")]
    pub fn count_two(a0: i32, a1: i32) -> i32;
    #[cpp(func = "int hicc_usages::variadic_template::count_three(int a0, int a1, int a2)")]
    pub fn count_three(a0: i32, a1: i32, a2: i32) -> i32;
    #[cpp(func = "int hicc_usages::variadic_template::max_three(int a0, int a1, int a2)")]
    pub fn max_three(a0: i32, a1: i32, a2: i32) -> i32;
}
