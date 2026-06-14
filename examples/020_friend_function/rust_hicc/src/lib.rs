//! 020_friend_function: friend 友元函数
//!
//! hicc 模式：friend 在 Rust 侧透明 — friend function 普通是自由函数，
//! 直接 import_lib! 绑定即可。Account 类的 owner()/balance() 走 import_class!。

hicc::cpp! {
    #include "friend_function.h"
    #include <hicc/std/string.hpp>
}

hicc::import_class! {
    class string = hicc_std::string;

    #[cpp(class = "friend_function_ns::Account")]
    pub class Account {
        #[cpp(method = "const std::string& owner() const")]
        pub fn owner(&self) -> &string;

        #[cpp(method = "long balance() const")]
        pub fn balance(&self) -> i64;

        pub fn new(owner: &string, balance: i64) -> Self { account_new(owner, balance) }
    }
}

hicc::import_lib! {
    #![link_name = "friend_function"]

    #[cpp(func = "std::unique_ptr<friend_function_ns::Account> hicc::make_unique<friend_function_ns::Account, const std::string&, long>(const std::string&, long&&)")]
    pub fn account_new(owner: &hicc_std::string, balance: i64) -> Account;

    // friend 函数：透明的自由函数
    #[cpp(func = "long friend_function_ns::audit_total(const friend_function_ns::Account&)")]
    pub fn audit_total(a: &Account) -> i64;
}
