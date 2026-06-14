#include "variadic_template.h"
#include <iostream>

int main() {
    std::cout << "sum2(1,2)    = " << sum2(1, 2)    << std::endl;
    std::cout << "sum3(1,2,3)  = " << sum3(1, 2, 3) << std::endl;
    std::cout << "sum4(1,2,3,4) = " << sum4(1, 2, 3, 4) << std::endl;
    return 0;
}
