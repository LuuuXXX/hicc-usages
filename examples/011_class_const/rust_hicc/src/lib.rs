//! 011_class_const: const 成员函数与 const 正确性
//!
//! hicc 模式：const 方法在 #[cpp(method = "...")] 中标注 const 后缀；
//! 非 const 方法（set_value/convert_to）不带 const。

hicc::cpp! {
    #include "class_const.h"
    #include <hicc/std/string.hpp>
}

hicc::import_class! {
    class string = hicc_std::string;

    #[cpp(class = "class_const_ns::Temperature")]
    pub class Temperature {
        #[cpp(method = "float value() const")]
        pub fn value(&self) -> f32;

        #[cpp(method = "const std::string& unit() const")]
        pub fn unit(&self) -> &string;

        #[cpp(method = "float to_fahrenheit() const")]
        pub fn to_fahrenheit(&self) -> f32;

        #[cpp(method = "void set_value(float)")]
        pub fn set_value(&mut self, v: f32);

        #[cpp(method = "void convert_to(const std::string&)")]
        pub fn convert_to(&mut self, new_unit: &string);

        pub fn new(v: f32) -> Self { temp_new(v) }
        pub fn new_with_unit(v: f32, u: &string) -> Self { temp_new_unit(v, u) }
    }
}

hicc::import_lib! {
    #![link_name = "class_const"]

    #[cpp(func = "std::unique_ptr<class_const_ns::Temperature> hicc::make_unique<class_const_ns::Temperature, float>(float&&)")]
    pub fn temp_new(v: f32) -> Temperature;

    #[cpp(func = "std::unique_ptr<class_const_ns::Temperature> hicc::make_unique<class_const_ns::Temperature, float, const std::string&>(float&&, const std::string&)")]
    pub fn temp_new_unit(v: f32, u: &hicc_std::string) -> Temperature;
}
