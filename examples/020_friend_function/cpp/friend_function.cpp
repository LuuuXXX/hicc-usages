#include "friend_function.h"

Account merge(const Account& a, const Account& b) {
    return Account(a.balance_ + b.balance_);
}

Account* account_new(int balance) { return new Account(balance); }
void     account_free(Account* a) { delete a; }
