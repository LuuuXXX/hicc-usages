//! 自动生成：hicc_usage_friend_function
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/friend_function.h"
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::friend_function::Account", destroy = "hicc_usages::friend_function::Account::free")]
    pub class Account {
        #[cpp(method = "int balance() const")]
        pub fn balance(&self) -> i32;
        #[cpp(method = "int deposit(int)")]
        pub fn deposit(&mut self, amount: i32) -> i32;
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_friend_function_adapter"]
    pub class Account;
    #[cpp(func = "hicc_usages::friend_function::Account * hicc_usages::friend_function::Account::create(int)")]
    pub fn account_new(initial: i32) -> Account;
}
