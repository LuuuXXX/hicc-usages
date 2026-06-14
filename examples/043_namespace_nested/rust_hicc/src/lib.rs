//! 043_namespace_nested: nested namespaces
//!
//! hicc 模式：完全透明 —— 直接用完整带命名空间的类型签名
//! `#[cpp(class = "n1::n2::n3::Foo")]` 或 `#[cpp(func = "... n1::n2::n3::foo(...)")]`。
//! 命名空间嵌套层数对 FFI 没有任何影响：C++ 符号查找按全名解析。

hicc::cpp! {
    #include "namespace_nested.h"
    #include <hicc/std/string.hpp>
}

hicc::import_class! {
    #[cpp(class = "n1::n2::n3::Foo")]
    pub class Foo {
        #[cpp(method = "int value() const")]
        pub fn value(&self) -> i32;

        #[cpp(method = "void set_value(int)")]
        pub fn set_value(&mut self, v: i32);

        #[cpp(method = "std::string describe() const")]
        pub fn describe(&self) -> hicc_std::string;

        pub fn new(v: i32) -> Self { make_foo(v) }
    }
}

hicc::import_class! {
    #[cpp(class = "n1::inner::Bar")]
    pub class Bar {
        #[cpp(method = "std::string name() const")]
        pub fn name(&self) -> hicc_std::string;

        #[cpp(method = "void rename(const std::string&)")]
        pub fn rename(&mut self, new_name: &hicc_std::string);

        pub fn new(name: &hicc_std::string) -> Self { make_bar(name) }
    }
}

hicc::import_lib! {
    #![link_name = "namespace_nested"]

    class string = hicc_std::string;

    #[cpp(func = "std::unique_ptr<n1::n2::n3::Foo> n1::n2::n3::make_foo(int)")]
    pub fn make_foo(v: i32) -> Foo;

    #[cpp(func = "int n1::n2::n3::compute(int)")]
    pub fn compute(x: i32) -> i32;

    #[cpp(func = "std::unique_ptr<n1::inner::Bar> n1::inner::make_bar(const std::string&)")]
    pub fn make_bar(name: &hicc_std::string) -> Bar;

    #[cpp(func = "int outer::deep::deeper::add(int, int)")]
    pub fn add(a: i32, b: i32) -> i32;

    #[cpp(func = "int outer::deep::deeper::triple(int)")]
    pub fn triple(x: i32) -> i32;
}
