//! 自动生成：hicc_usage_template_class
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/template_class.h"

    // ============================================================
    // 自动生成的活跃 C++ 包装（untouched C++ 模式）
    // 这些代码是 Rust crate 正常工作所必需的（类模板 typedef + factory 等）
    // ============================================================
    namespace hicc_usages::template_class {
        using IntStack = Stack<int>;
        inline hicc_usages::template_class::IntStack* create_int_stack() { return new IntStack(); }
        inline void free_int_stack(hicc_usages::template_class::IntStack* self) { delete self; }
    }
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::template_class::IntStack", destroy = "hicc_usages::template_class::free_int_stack")]
    pub class IntStack {
        #[cpp(method = "void push(int)")]
        pub fn push(&mut self, v: i32) -> ();
        #[cpp(method = "int pop()")]
        pub fn pop(&mut self) -> i32;
        #[cpp(method = "std::size_t size() const")]
        pub fn size(&self) -> usize;
        #[cpp(method = "int top() const")]
        pub fn top(&self) -> i32;
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_template_class_adapter"]
    pub class IntStack;
    #[cpp(func = "hicc_usages::template_class::IntStack * hicc_usages::template_class::create_int_stack()")]
    pub fn int_stack_new() -> IntStack;
}
