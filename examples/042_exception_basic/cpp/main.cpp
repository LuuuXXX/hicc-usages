#include "exception_basic.h"
#include <iostream>

int main() {
    using namespace exception_basic_ns;

    try { std::cout << "10/3 = " << safe_divide(10, 3) << std::endl; }
    catch (const std::exception& e) { std::cout << "caught: " << e.what() << std::endl; }

    try { std::cout << "10/0 = " << safe_divide(10, 0) << std::endl; }
    catch (const std::exception& e) { std::cout << "caught: " << e.what() << std::endl; }

    auto acc = make_account(100);
    std::cout << "balance = " << acc->balance() << std::endl;
    acc->deposit(50);
    std::cout << "after deposit balance = " << acc->balance() << std::endl;

    try { acc->withdraw(1000); }
    catch (const std::exception& e) { std::cout << "withdraw caught: " << e.what() << std::endl; }

    try { acc->deposit(-5); }
    catch (const std::exception& e) { std::cout << "deposit caught: " << e.what() << std::endl; }

    return 0;
}
