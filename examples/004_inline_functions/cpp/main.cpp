#include "inline_functions.h"

int main() {
    std::cout << "square(5) = " << inline_ns::square(5) << std::endl;
    std::cout << "cube(3) = " << inline_ns::cube(3) << std::endl;
    std::cout << "factorial(5) = " << inline_ns::factorial(5) << std::endl;
    return 0;
}
