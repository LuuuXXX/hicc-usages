#include "constexpr_basic.h"
#include <iostream>

int main() {
    using namespace constexpr_basic_ns;
    std::cout << "PI = " << Constants::PI << std::endl;
    std::cout << "E = " << Constants::E << std::endl;
    std::cout << "BUFFER_SIZE = " << Constants::BUFFER_SIZE << std::endl;
    std::cout << "MAX_TRIES = " << Constants::MAX_TRIES << std::endl;
    std::cout << "BIG_NUMBER = " << Constants::BIG_NUMBER << std::endl;

    std::cout << "square(7) = " << square(7) << std::endl;
    std::cout << "factorial(5) = " << factorial(5) << std::endl;

    Circle c(2.0);
    std::cout << "circle.area() = " << c.area() << std::endl;
    std::cout << "compute_area(2.0) = " << compute_area(2.0) << std::endl;

    return 0;
}
