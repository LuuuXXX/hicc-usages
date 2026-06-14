#pragma once

// Friend functions have access to private members but are not member
// functions. hicc treats them as ordinary free functions.

class Account {
public:
    Account(int balance) : balance_(balance) {}
    int balance() const { return balance_; }
    friend Account merge(const Account& a, const Account& b);
private:
    int balance_;
};

// Friend — can access Account::balance_ directly.
Account merge(const Account& a, const Account& b);

Account* account_new(int balance);
void     account_free(Account* a);
