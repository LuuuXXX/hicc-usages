#include "friend_function.h"
#include <iostream>

int main() {
    Account* a = account_new(100);
    Account* b = account_new(50);
    Account c = merge(*a, *b);
    std::cout << "merged balance = " << c.balance() << std::endl;
    account_free(a);
    account_free(b);
    return 0;
}
