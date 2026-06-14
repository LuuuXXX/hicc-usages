//! 017_virtual_override: 派生类 override 虚函数（部分 override，部分新增）
//!
//! hicc 模式：派生类 Triangle/Pentagon 独立 import_class!。
//! Triangle 只 override sides；Pentagon 额外 override describe。

hicc::cpp! {
    #include "virtual_override.h"
    #include <hicc/std/string.hpp>
}

hicc::import_class! {
    class string = hicc_std::string;

    #[cpp(class = "virtual_override_ns::Triangle")]
    pub class Triangle {
        #[cpp(method = "const std::string& name() const")]
        pub fn name(&self) -> &string;

        #[cpp(method = "int sides() const")]
        pub fn sides(&self) -> i32;

        #[cpp(method = "std::string describe() const")]
        pub fn describe(&self) -> string;

        pub fn new(name: &string) -> Self { triangle_new(name) }
    }

    #[cpp(class = "virtual_override_ns::Pentagon")]
    pub class Pentagon {
        #[cpp(method = "const std::string& name() const")]
        pub fn name(&self) -> &string;

        #[cpp(method = "int sides() const")]
        pub fn sides(&self) -> i32;

        #[cpp(method = "std::string describe() const")]
        pub fn describe(&self) -> string;

        pub fn new(name: &string) -> Self { pentagon_new(name) }
    }
}

hicc::import_lib! {
    #![link_name = "virtual_override"]

    #[cpp(func = "std::unique_ptr<virtual_override_ns::Triangle> hicc::make_unique<virtual_override_ns::Triangle, const std::string&>(const std::string&)")]
    pub fn triangle_new(name: &hicc_std::string) -> Triangle;

    #[cpp(func = "std::unique_ptr<virtual_override_ns::Pentagon> hicc::make_unique<virtual_override_ns::Pentagon, const std::string&>(const std::string&)")]
    pub fn pentagon_new(name: &hicc_std::string) -> Pentagon;
}
