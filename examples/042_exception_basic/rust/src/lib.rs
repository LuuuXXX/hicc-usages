//! 自动生成：hicc_usage_exception_basic
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/exception_basic.h"
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::exception_basic::SafeStack", destroy = "hicc_usages::exception_basic::SafeStack::free")]
    pub class SafeStack {
        #[cpp(method = "void push(int)")]
        pub fn push(&mut self, v: i32) -> ();
        #[cpp(method = "int pop()")]
        pub fn pop(&mut self) -> i32;
        #[cpp(method = "int peek() const")]
        pub fn peek(&self) -> i32;
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_exception_basic_adapter"]
    pub class SafeStack;
    #[cpp(func = "hicc_usages::exception_basic::SafeStack * hicc_usages::exception_basic::SafeStack::create(std::size_t)")]
    pub fn safestack_new(capacity: usize) -> SafeStack;
    #[cpp(func = "int hicc_usages::exception_basic::safe_divide(int, int)")]
    pub fn safe_divide(a: i32, b: i32) -> i32;
    #[cpp(func = "int hicc_usages::exception_basic::safe_at(int *, std::size_t, std::size_t)")]
    pub fn safe_at(arr: *mut i32, size: usize, idx: usize) -> i32;
    #[cpp(func = "int hicc_usages::exception_basic::safe_parse(const char *)")]
    pub fn safe_parse(s: *const i8) -> i32;
    #[cpp(func = "const char * hicc_usages::exception_basic::last_error()")]
    pub fn last_error() -> *const i8;
    #[cpp(func = "void hicc_usages::exception_basic::clear_error()")]
    pub fn clear_error() -> ();
}
