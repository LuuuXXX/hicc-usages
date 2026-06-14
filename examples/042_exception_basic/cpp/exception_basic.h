#pragma once
#include <stdexcept>
#include <string>
#include <memory>
#include <iostream>

namespace exception_basic_ns {

// Free functions that may throw
int safe_divide(int a, int b);                  // throws std::invalid_argument if b == 0
int parse_int(const std::string& s);            // throws std::invalid_argument on bad input
std::string nth_char(const std::string& s, int idx); // throws std::out_of_range

// Class whose methods throw on error paths
class BankAccount {
public:
    explicit BankAccount(int initial_balance);
    int balance() const;
    void deposit(int amount);                   // throws std::invalid_argument if amount < 0
    int withdraw(int amount);                   // throws std::runtime_error if insufficient funds
private:
    int balance_;
};

std::unique_ptr<BankAccount> make_account(int initial_balance);

// Returns 1 if input even, else throws std::logic_error
int require_even(int x);

} // namespace exception_basic_ns
