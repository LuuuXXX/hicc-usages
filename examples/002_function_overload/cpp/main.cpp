#include "function_overload.h"
#include <iostream>

int main() {
    std::cout << "add(int)    = " << add(2, 3) << std::endl;
    std::cout << "add(double) = " << add(1.5, 2.5) << std::endl;
    return 0;
}
