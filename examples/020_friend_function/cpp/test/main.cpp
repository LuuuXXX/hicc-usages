#include "hicc_usages/friend_function.h"
#include <cassert>
#include <iostream>
int main() {
    using namespace hicc_usages::friend_function;
    Account* a = Account::create(100);
    assert(a->balance() == 100);
    a->deposit(50);
    assert(a->balance() == 150);
    assert(balance_friend(a) == 150);
    assert(deposit_friend(a, 50) == 200);
    Account::free(a);
    std::cout << "[friend_function] C++ test OK" << std::endl;
    return 0;
}
