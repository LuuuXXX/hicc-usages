//! 021_explicit_ctor: explicit 构造
//!
//! hicc 模式：explicit 关键字对 FFI 透明（无隐式转换需求）。每个 ctor 一个
//! make_unique<T, Args...>(Args&&...) 工厂函数。

hicc::cpp! {
    #include "explicit_ctor.h"
    #include <hicc/std/string.hpp>
}

hicc::import_class! {
    class string = hicc_std::string;

    #[cpp(class = "explicit_ctor_ns::Distance")]
    pub class Distance {
        #[cpp(method = "double meters() const")]
        pub fn meters(&self) -> f64;

        #[cpp(method = "void add(const explicit_ctor_ns::Distance&)")]
        pub fn add(&mut self, other: &Distance);

        pub fn from_meters(m: f64) -> Self { distance_from_meters(m) }
        pub fn from_m_cm(m: i32, cm: i32) -> Self { distance_from_m_cm(m, cm) }
    }

    #[cpp(class = "explicit_ctor_ns::Wrapper")]
    pub class Wrapper {
        #[cpp(method = "const std::string& tag() const")]
        pub fn tag(&self) -> &string;

        #[cpp(method = "int level() const")]
        pub fn level(&self) -> i32;

        pub fn new(tag: &string, level: i32) -> Self { wrapper_new(tag, level) }
    }
}

hicc::import_lib! {
    #![link_name = "explicit_ctor"]

    #[cpp(func = "std::unique_ptr<explicit_ctor_ns::Distance> hicc::make_unique<explicit_ctor_ns::Distance, double>(double&&)")]
    pub fn distance_from_meters(m: f64) -> Distance;

    #[cpp(func = "std::unique_ptr<explicit_ctor_ns::Distance> hicc::make_unique<explicit_ctor_ns::Distance, int, int>(int&&, int&&)")]
    pub fn distance_from_m_cm(m: i32, cm: i32) -> Distance;

    #[cpp(func = "std::unique_ptr<explicit_ctor_ns::Wrapper> hicc::make_unique<explicit_ctor_ns::Wrapper, const std::string&, int>(const std::string&, int&&)")]
    pub fn wrapper_new(tag: &hicc_std::string, level: i32) -> Wrapper;
}
