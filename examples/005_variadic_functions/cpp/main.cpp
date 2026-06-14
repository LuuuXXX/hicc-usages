#include "variadic_functions.h"
#include <iostream>

int main() {
    std::cout << "sum(2, 10, 20)   = " << sum(2, 10, 20) << std::endl;
    std::cout << "sum(3, 1, 2, 3)  = " << sum(3, 1, 2, 3) << std::endl;
    std::cout << "sum2(10, 20)     = " << sum2(10, 20) << std::endl;
    std::cout << "sum3(1, 2, 3)    = " << sum3(1, 2, 3) << std::endl;
    return 0;
}
