//! 023_typeid_rtti: RTTI typeid
//!
//! hicc 模式：typeid 不能直接通过 #[cpp(method = ...)] 调用（结果是 std::type_info，
//! C++ ABI 内部类型）。**C++ 端写好的命名空间级包装函数**（type_name_base/same_type/
//! is_derived_a）直接 import_lib! 绑定。每个具体派生类独立 import_class!。

hicc::cpp! {
    #include "typeid_rtti.h"
    #include <hicc/std/string.hpp>
}

hicc::import_class! {
    class string = hicc_std::string;

    #[cpp(class = "typeid_rtti_ns::DerivedA")]
    pub class DerivedA {
        #[cpp(method = "std::string name() const")]
        pub fn name(&self) -> string;

        pub fn new() -> Self { derived_a_new() }
    }

    #[cpp(class = "typeid_rtti_ns::DerivedB")]
    pub class DerivedB {
        #[cpp(method = "std::string name() const")]
        pub fn name(&self) -> string;

        pub fn new() -> Self { derived_b_new() }
    }
}

hicc::import_lib! {
    #![link_name = "typeid_rtti"]

    #[cpp(func = "std::unique_ptr<typeid_rtti_ns::DerivedA> hicc::make_unique<typeid_rtti_ns::DerivedA>()")]
    pub fn derived_a_new() -> DerivedA;

    #[cpp(func = "std::unique_ptr<typeid_rtti_ns::DerivedB> hicc::make_unique<typeid_rtti_ns::DerivedB>()")]
    pub fn derived_b_new() -> DerivedB;

    // RTTI wrappers (already provided in C++ header as inline free functions)
    #[cpp(func = "const char* typeid_rtti_ns::type_name_base(const typeid_rtti_ns::Base&)")]
    pub fn type_name_base_a(a: &DerivedA) -> *const i8;

    #[cpp(func = "const char* typeid_rtti_ns::type_name_base(const typeid_rtti_ns::Base&)")]
    pub fn type_name_base_b(b: &DerivedB) -> *const i8;

    #[cpp(func = "bool typeid_rtti_ns::same_type(const typeid_rtti_ns::Base&, const typeid_rtti_ns::Base&)")]
    pub fn same_type_a_a(a: &DerivedA, b: &DerivedA) -> bool;

    #[cpp(func = "bool typeid_rtti_ns::same_type(const typeid_rtti_ns::Base&, const typeid_rtti_ns::Base&)")]
    pub fn same_type_a_b(a: &DerivedA, b: &DerivedB) -> bool;

    #[cpp(func = "bool typeid_rtti_ns::is_derived_a(const typeid_rtti_ns::Base&)")]
    pub fn is_derived_a_a(a: &DerivedA) -> bool;

    #[cpp(func = "bool typeid_rtti_ns::is_derived_a(const typeid_rtti_ns::Base&)")]
    pub fn is_derived_a_b(b: &DerivedB) -> bool;
}
