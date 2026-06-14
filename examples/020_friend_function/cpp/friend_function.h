#pragma once
#include <string>
#include <iostream>

namespace friend_function_ns {

class Account {
public:
    Account(const std::string& owner, long balance)
        : owner_(owner), balance_(balance) {}

    const std::string& owner() const { return owner_; }
    long balance() const { return balance_; }

    // 友元函数：可访问 private 字段
    friend long audit_total(const Account& a);
    friend std::ostream& operator<<(std::ostream& os, const Account& a);

private:
    std::string owner_;
    long balance_;
};

inline long audit_total(const Account& a) {
    return a.balance_;
}

inline std::ostream& operator<<(std::ostream& os, const Account& a) {
    return os << "Account(" << a.owner_ << ", $" << a.balance_ << ")";
}

} // namespace friend_function_ns
