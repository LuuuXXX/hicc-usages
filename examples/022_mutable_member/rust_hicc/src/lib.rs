//! 022_mutable_member: mutable 成员
//!
//! hicc 模式：C++ mutable 字段在 const 方法内可修改，FFI 完全透明。
//! `execute()` 是 const（hicc 用 `&self`），内部修改 mutable 字段，无影响。

hicc::cpp! {
    #include "mutable_member.h"
    #include <hicc/std/string.hpp>
}

hicc::import_class! {
    class string = hicc_std::string;

    #[cpp(class = "mutable_member_ns::Query")]
    pub class Query {
        #[cpp(method = "const std::string& key() const")]
        pub fn key(&self) -> &string;

        #[cpp(method = "std::string execute() const")]
        pub fn execute(&self) -> string;

        #[cpp(method = "int call_count() const")]
        pub fn call_count(&self) -> i32;

        pub fn new(key: &string) -> Self { query_new(key) }
    }
}

hicc::import_lib! {
    #![link_name = "mutable_member"]

    #[cpp(func = "std::unique_ptr<mutable_member_ns::Query> hicc::make_unique<mutable_member_ns::Query, const std::string&>(const std::string&)")]
    pub fn query_new(key: &hicc_std::string) -> Query;
}
