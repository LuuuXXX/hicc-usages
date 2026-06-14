#pragma once
#include <stdexcept>
#include <string>
#include <memory>
#include <iostream>

namespace exception_basic_ns {

// 可能抛异常的自由函数
int safe_divide(int a, int b);                  // b == 0 时抛 std::invalid_argument
int parse_int(const std::string& s);            // 输入无效时抛 std::invalid_argument
std::string nth_char(const std::string& s, int idx); // 抛 std::out_of_range

// 方法在错误路径上抛异常的类
class BankAccount {
public:
    explicit BankAccount(int initial_balance);
    int balance() const;
    void deposit(int amount);                   // amount < 0 时抛 std::invalid_argument
    int withdraw(int amount);                   // 余额不足时抛 std::runtime_error
private:
    int balance_;
};

std::unique_ptr<BankAccount> make_account(int initial_balance);

// 输入为偶数时返回 1，否则抛 std::logic_error
int require_even(int x);

} // namespace exception_basic_ns
