//! 自动生成：hicc_usage_hello_world
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/hello_world.h"
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::hello_world::Greeter", destroy = "hicc_usages::hello_world::Greeter::free")]
    pub class Greeter {
        #[cpp(method = "void greet() const")]
        pub fn greet(&self) -> ();
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_hello_world_adapter"]
    pub class Greeter;
    #[cpp(func = "hicc_usages::hello_world::Greeter * hicc_usages::hello_world::Greeter::create()")]
    pub fn greeter_new() -> Greeter;
    #[cpp(func = "void hicc_usages::hello_world::hello()")]
    pub fn hello() -> ();
}
