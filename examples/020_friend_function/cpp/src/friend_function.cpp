#include "hicc_usages/friend_function.h"
namespace hicc_usages::friend_function {
Account::Account(int initial) : balance_(initial) {}
Account* Account::create(int initial) { return new Account(initial); }
void Account::free(Account* self) { delete self; }
int Account::balance() const { return balance_; }
int Account::deposit(int amount) { balance_ += amount; return balance_; }
int deposit_friend(Account* acc, int amount) { return acc->deposit(amount); }
int balance_friend(const Account* acc) { return acc->balance(); }
}
