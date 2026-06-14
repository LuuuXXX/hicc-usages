// Friend function `merge` is bound as an ordinary free function — friend
// access is a C++ compile-time concept, irrelevant to FFI binding.

hicc::cpp! {
    #include "friend_function.h"
}

hicc::import_class! {
    #[cpp(class = "Account", destroy = "account_free")]
    pub class Account {
        #[cpp(method = "int balance() const")]
        pub fn balance(&self) -> i32;
    }
}

hicc::import_lib! {
    #![link_name = "friend_function_hicc"]

    #[cpp(func = "Account* account_new(int)")]
    pub fn account_new(balance: i32) -> Account;

    #[cpp(func = "Account merge(const Account&, const Account&)")]
    pub fn merge(a: &Account, b: &Account) -> Account;
}
