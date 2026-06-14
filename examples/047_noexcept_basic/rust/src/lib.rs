//! 自动生成：hicc_usage_noexcept_basic
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/noexcept_basic.h"
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::noexcept_basic::NoexceptBuffer", destroy = "hicc_usages::noexcept_basic::NoexceptBuffer::free")]
    pub class NoexceptBuffer {
        #[cpp(method = "void set(std::size_t, int)")]
        pub fn set(&mut self, idx: usize, v: i32) -> ();
        #[cpp(method = "int get(std::size_t) const")]
        pub fn get(&self, idx: usize) -> i32;
        #[cpp(method = "void clear()")]
        pub fn clear(&mut self) -> ();
        #[cpp(method = "int sum() const")]
        pub fn sum(&self) -> i32;
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_noexcept_basic_adapter"]
    pub class NoexceptBuffer;
    #[cpp(func = "hicc_usages::noexcept_basic::NoexceptBuffer * hicc_usages::noexcept_basic::NoexceptBuffer::create(std::size_t)")]
    pub fn noexceptbuffer_new(capacity: usize) -> NoexceptBuffer;
    #[cpp(func = "int hicc_usages::noexcept_basic::safe_add(int, int)")]
    pub fn safe_add(a: i32, b: i32) -> i32;
    #[cpp(func = "int hicc_usages::noexcept_basic::safe_sub(int, int)")]
    pub fn safe_sub(a: i32, b: i32) -> i32;
    #[cpp(func = "bool hicc_usages::noexcept_basic::safe_equals(int, int)")]
    pub fn safe_equals(a: i32, b: i32) -> bool;
    #[cpp(func = "int hicc_usages::noexcept_basic::no_throw_compute(int, int)")]
    pub fn no_throw_compute(a: i32, b: i32) -> i32;
    #[cpp(func = "int hicc_usages::noexcept_basic::maybe_throw_compute(int, int)")]
    pub fn maybe_throw_compute(a: i32, b: i32) -> i32;
}
