//! 027_template_instantiation: 模板显式实例化
//!
//! hicc 模式：与 025 同 — cpp! 块 `using` 每个具现化类型别名，import_class! 导入别名。
//! Pair<int> 完全可用；Pair<std::string> 的 sum() 拼接 string 也可用。

hicc::cpp! {
    #include "template_instantiation.h"
    #include <hicc/std/string.hpp>

    using PairInt = template_instantiation_ns::Pair<int>;
    using PairString = template_instantiation_ns::Pair<std::string>;
}

hicc::import_class! {
    class string = hicc_std::string;

    #[cpp(class = "PairInt")]
    pub class PairInt {
        #[cpp(method = "int first() const")]
        pub fn first(&self) -> i32;

        #[cpp(method = "int second() const")]
        pub fn second(&self) -> i32;

        #[cpp(method = "int sum() const")]
        pub fn sum(&self) -> i32;

        #[cpp(method = "void swap()")]
        pub fn swap(&mut self);

        pub fn new(a: i32, b: i32) -> Self { pair_int_new(a, b) }
    }

    #[cpp(class = "PairString")]
    pub class PairString {
        #[cpp(method = "std::string first() const")]
        pub fn first(&self) -> string;

        #[cpp(method = "std::string sum() const")]
        pub fn sum(&self) -> string;

        pub fn new(a: &string, b: &string) -> Self { pair_string_new(a, b) }
    }
}

hicc::import_lib! {
    #![link_name = "template_instantiation"]

    #[cpp(func = "std::unique_ptr<PairInt> hicc::make_unique<PairInt, int, int>(int&&, int&&)")]
    pub fn pair_int_new(a: i32, b: i32) -> PairInt;

    #[cpp(func = "std::unique_ptr<PairString> hicc::make_unique<PairString, const std::string&, const std::string&>(const std::string&, const std::string&)")]
    pub fn pair_string_new(a: &hicc_std::string, b: &hicc_std::string) -> PairString;
}
