#include "hicc_usages/inline_functions.h"
#include <cassert>
#include <iostream>
int main() {
    using namespace hicc_usages::inline_functions;
    assert(square(3) == 9);
    assert(cube(2) == 8);
    assert(compute(2) == 12);
    std::cout << "[inline_functions] C++ test OK" << std::endl;
    return 0;
}
