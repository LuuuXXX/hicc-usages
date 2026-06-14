//! 自动生成：hicc_usage_std_function
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/std_function.h"
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::std_function::FuncStore", destroy = "hicc_usages::std_function::FuncStore::free")]
    pub class FuncStore {
        #[cpp(method = "void set_adder()")]
        pub fn set_adder(&mut self) -> ();
        #[cpp(method = "void set_multiplier()")]
        pub fn set_multiplier(&mut self) -> ();
        #[cpp(method = "void set_constant(int)")]
        pub fn set_constant(&mut self, c: i32) -> ();
        #[cpp(method = "int call(int) const")]
        pub fn call(&self, x: i32) -> i32;
        #[cpp(method = "bool has_func() const")]
        pub fn has_func(&self) -> bool;
    }
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::std_function::Dispatcher", destroy = "hicc_usages::std_function::Dispatcher::free")]
    pub class Dispatcher {
        #[cpp(method = "void set_mode(int)")]
        pub fn set_mode(&mut self, mode: i32) -> ();
        #[cpp(method = "int run(int) const")]
        pub fn run(&self, x: i32) -> i32;
        #[cpp(method = "int run_twice(int) const")]
        pub fn run_twice(&self, x: i32) -> i32;
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_std_function_adapter"]
    pub class FuncStore;
    pub class Dispatcher;
    #[cpp(func = "hicc_usages::std_function::FuncStore * hicc_usages::std_function::FuncStore::create()")]
    pub fn funcstore_new() -> FuncStore;
    #[cpp(func = "hicc_usages::std_function::Dispatcher * hicc_usages::std_function::Dispatcher::create()")]
    pub fn dispatcher_new() -> Dispatcher;
}
