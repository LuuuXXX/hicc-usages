//! 自动生成：hicc_usage_inheritance_single
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/inheritance_single.h"
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::inheritance_single::Animal", destroy = "hicc_usages::inheritance_single::Animal::free")]
    pub class Animal {
        #[cpp(method = "const char * get_name() const")]
        pub fn get_name(&self) -> *const i8;
        #[cpp(method = "int get_legs() const")]
        pub fn get_legs(&self) -> i32;
    }
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::inheritance_single::Dog", destroy = "hicc_usages::inheritance_single::Dog::free")]
    pub class Dog {
        #[cpp(method = "const char * bark() const")]
        pub fn bark(&self) -> *const i8;
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_inheritance_single_adapter"]
    pub class Animal;
    pub class Dog;
    #[cpp(func = "hicc_usages::inheritance_single::Animal * hicc_usages::inheritance_single::Animal::create(const char *)")]
    pub fn animal_new(name: *const i8) -> Animal;
    #[cpp(func = "hicc_usages::inheritance_single::Dog * hicc_usages::inheritance_single::Dog::create(const char *)")]
    pub fn dog_new(name: *const i8) -> Dog;
}
