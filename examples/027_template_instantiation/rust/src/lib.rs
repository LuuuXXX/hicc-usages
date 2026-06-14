//! 自动生成：hicc_usage_template_instantiation
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/template_instantiation.h"

    // ============================================================
    // 自动生成的活跃 C++ 包装（untouched C++ 模式）
    // 这些代码是 Rust crate 正常工作所必需的（类模板 typedef + factory 等）
    // ============================================================
    namespace hicc_usages::template_instantiation {
        using IntContainer = Container<int>;
        inline hicc_usages::template_instantiation::IntContainer* create_int_container() { return new IntContainer(); }
        inline void free_int_container(hicc_usages::template_instantiation::IntContainer* self) { delete self; }
    }
    namespace hicc_usages::template_instantiation {
        using DoubleContainer = Container<double>;
        inline hicc_usages::template_instantiation::DoubleContainer* create_double_container() { return new DoubleContainer(); }
        inline void free_double_container(hicc_usages::template_instantiation::DoubleContainer* self) { delete self; }
    }
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::template_instantiation::IntContainer", destroy = "hicc_usages::template_instantiation::free_int_container")]
    pub class IntContainer {
        #[cpp(method = "void set(int)")]
        pub fn set(&mut self, v: i32) -> ();
        #[cpp(method = "int get() const")]
        pub fn get(&self) -> i32;
        #[cpp(method = "int doubled() const")]
        pub fn doubled(&self) -> i32;
    }
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::template_instantiation::DoubleContainer", destroy = "hicc_usages::template_instantiation::free_double_container")]
    pub class DoubleContainer {
        #[cpp(method = "void set(double)")]
        pub fn set(&mut self, v: f64) -> ();
        #[cpp(method = "double get() const")]
        pub fn get(&self) -> f64;
        #[cpp(method = "double doubled() const")]
        pub fn doubled(&self) -> f64;
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_template_instantiation_adapter"]
    pub class IntContainer;
    pub class DoubleContainer;
    #[cpp(func = "hicc_usages::template_instantiation::IntContainer * hicc_usages::template_instantiation::create_int_container()")]
    pub fn int_container_new() -> IntContainer;
    #[cpp(func = "hicc_usages::template_instantiation::DoubleContainer * hicc_usages::template_instantiation::create_double_container()")]
    pub fn double_container_new() -> DoubleContainer;
}
