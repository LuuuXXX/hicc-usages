#include "exception_basic.h"

namespace exception_basic_ns {

int safe_divide(int a, int b) {
    if (b == 0) throw std::invalid_argument("division by zero");
    return a / b;
}

int parse_int(const std::string& s) {
    try {
        size_t pos = 0;
        int v = std::stoi(s, &pos);
        if (pos != s.size()) throw std::invalid_argument("trailing characters");
        return v;
    } catch (const std::exception&) {
        throw std::invalid_argument("invalid integer: " + s);
    }
}

std::string nth_char(const std::string& s, int idx) {
    if (idx < 0 || static_cast<size_t>(idx) >= s.size()) {
        throw std::out_of_range("index out of range");
    }
    return std::string(1, s[static_cast<size_t>(idx)]);
}

BankAccount::BankAccount(int initial_balance) : balance_(initial_balance) {}

int BankAccount::balance() const { return balance_; }

void BankAccount::deposit(int amount) {
    if (amount < 0) throw std::invalid_argument("deposit amount must be non-negative");
    balance_ += amount;
}

int BankAccount::withdraw(int amount) {
    if (amount > balance_) throw std::runtime_error("insufficient funds");
    balance_ -= amount;
    return amount;
}

std::unique_ptr<BankAccount> make_account(int initial_balance) {
    return std::unique_ptr<BankAccount>(new BankAccount(initial_balance));
}

int require_even(int x) {
    if (x % 2 != 0) throw std::logic_error("value must be even");
    return 1;
}

int exception_basic_anchor() { return 42; }

} // namespace exception_basic_ns
