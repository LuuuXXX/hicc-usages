//! 042_exception_basic: 通过 hicc::Exception<T> 实现 try/catch
//!
//! hicc 模式：把可能抛异常的 C++ 函数声明为返回 `hicc::Exception<T>`。
//! hicc-build 会在 C++ 侧自动用 `EXPORT_EXCEPT_METHOD`/`EXPORT_EXCEPT_MEMBER_METHOD` 包一层，
//! 捕获 `std::exception` 并把 `what()` 编码进 `ExceptionInfo`（64 字节）。
//! Rust 端调用 `.ok()` 拿到 `Result<T, ExceptionInfo>`。

hicc::cpp! {
    #include "exception_basic.h"
    #include <hicc/std/string.hpp>
}

hicc::import_class! {
    #[cpp(class = "exception_basic_ns::BankAccount")]
    pub class BankAccount {
        #[cpp(method = "int balance() const")]
        pub fn balance(&self) -> i32;

        #[cpp(method = "void deposit(int)")]
        pub fn deposit(&mut self, amount: i32) -> hicc::Exception<()>;

        #[cpp(method = "int withdraw(int)")]
        pub fn withdraw(&mut self, amount: i32) -> hicc::Exception<i32>;

        pub fn new(initial_balance: i32) -> Self { make_account(initial_balance) }
    }
}

hicc::import_lib! {
    #![link_name = "exception_basic"]

    class string = hicc_std::string;

    #[cpp(func = "int exception_basic_ns::safe_divide(int, int)")]
    pub fn safe_divide(a: i32, b: i32) -> hicc::Exception<i32>;

    #[cpp(func = "int exception_basic_ns::parse_int(const std::string&)")]
    pub fn parse_int(s: &hicc_std::string) -> hicc::Exception<i32>;

    #[cpp(func = "std::string exception_basic_ns::nth_char(const std::string&, int)")]
    pub fn nth_char(s: &hicc_std::string, idx: i32) -> hicc::Exception<hicc_std::string>;

    #[cpp(func = "int exception_basic_ns::require_even(int)")]
    pub fn require_even(x: i32) -> hicc::Exception<i32>;

    #[cpp(func = "std::unique_ptr<exception_basic_ns::BankAccount> exception_basic_ns::make_account(int)")]
    pub fn make_account(initial_balance: i32) -> BankAccount;
}
