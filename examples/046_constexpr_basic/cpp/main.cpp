#include "constexpr_basic.h"
#include <iostream>

int main() {
    std::cout << "sq(5) = " << sq(5) << std::endl;
    std::cout << "cube(3) = " << cube(3) << std::endl;
    std::cout << "square_plus_magic(4) = " << square_plus_magic(4) << std::endl;
    return 0;
}
