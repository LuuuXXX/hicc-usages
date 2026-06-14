//! 013_inheritance_single: 单继承 + virtual override
//!
//! hicc 模式：每个具体派生类（Dog/Cat）独立 import_class!。
//! 基类 Animal 的方法（name）在派生类里仍可调用（C++ 继承透明）。
//! 多态通过 Animal* 暂不暴露（需 #[interface] trait，本例不展示）。

hicc::cpp! {
    #include "inheritance_single.h"
    #include <hicc/std/string.hpp>
}

hicc::import_class! {
    class string = hicc_std::string;

    #[cpp(class = "inheritance_single_ns::Dog")]
    pub class Dog {
        #[cpp(method = "const std::string& name() const")]
        pub fn name(&self) -> &string;

        #[cpp(method = "std::string sound() const")]
        pub fn sound(&self) -> string;

        #[cpp(method = "int legs() const")]
        pub fn legs(&self) -> i32;

        #[cpp(method = "std::string breed() const")]
        pub fn breed(&self) -> string;

        pub fn new(name: &string) -> Self { dog_new(name) }
    }

    #[cpp(class = "inheritance_single_ns::Cat")]
    pub class Cat {
        #[cpp(method = "const std::string& name() const")]
        pub fn name(&self) -> &string;

        #[cpp(method = "std::string sound() const")]
        pub fn sound(&self) -> string;

        #[cpp(method = "int legs() const")]
        pub fn legs(&self) -> i32;

        pub fn new(name: &string) -> Self { cat_new(name) }
    }
}

hicc::import_lib! {
    #![link_name = "inheritance_single"]

    #[cpp(func = "std::unique_ptr<inheritance_single_ns::Dog> hicc::make_unique<inheritance_single_ns::Dog, const std::string&>(const std::string&)")]
    pub fn dog_new(name: &hicc_std::string) -> Dog;

    #[cpp(func = "std::unique_ptr<inheritance_single_ns::Cat> hicc::make_unique<inheritance_single_ns::Cat, const std::string&>(const std::string&)")]
    pub fn cat_new(name: &hicc_std::string) -> Cat;
}
