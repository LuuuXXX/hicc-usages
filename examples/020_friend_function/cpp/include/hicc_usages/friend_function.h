#pragma once
namespace hicc_usages::friend_function {
class Account {
    friend int deposit_friend(Account* acc, int amount);
    friend int balance_friend(const Account* acc);
public:
    static Account* create(int initial);
    static void free(Account* self);
    int balance() const;
    int deposit(int amount);
private:
    explicit Account(int initial);
    int balance_;
};
// 友元自由函数（对 Rust 透明，按普通函数处理）
int deposit_friend(Account* acc, int amount);
int balance_friend(const Account* acc);
}
