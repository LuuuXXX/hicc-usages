//! 自动生成：hicc_usage_functional_bind
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/functional_bind.h"
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::functional_bind::Binder", destroy = "hicc_usages::functional_bind::Binder::free")]
    pub class Binder {
        #[cpp(method = "void configure(int, int)")]
        pub fn configure(&mut self, base: i32, step: i32) -> ();
        #[cpp(method = "int next()")]
        pub fn next(&mut self) -> i32;
        #[cpp(method = "int peek() const")]
        pub fn peek(&self) -> i32;
        #[cpp(method = "int calls() const")]
        pub fn calls(&self) -> i32;
    }
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::functional_bind::Combiner", destroy = "hicc_usages::functional_bind::Combiner::free")]
    pub class Combiner {
        #[cpp(method = "void set_pipeline()")]
        pub fn set_pipeline(&mut self) -> ();
        #[cpp(method = "int process(int) const")]
        pub fn process(&self, x: i32) -> i32;
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_functional_bind_adapter"]
    pub class Binder;
    pub class Combiner;
    #[cpp(func = "hicc_usages::functional_bind::Binder * hicc_usages::functional_bind::Binder::create()")]
    pub fn binder_new() -> Binder;
    #[cpp(func = "hicc_usages::functional_bind::Combiner * hicc_usages::functional_bind::Combiner::create()")]
    pub fn combiner_new() -> Combiner;
    #[cpp(func = "int hicc_usages::functional_bind::double_it(int)")]
    pub fn double_it(x: i32) -> i32;
    #[cpp(func = "int hicc_usages::functional_bind::add_one(int)")]
    pub fn add_one(x: i32) -> i32;
}
