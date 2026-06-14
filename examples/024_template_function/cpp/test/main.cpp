#include "hicc_usages/template_function.h"
#include <cassert>
#include <iostream>
int main() {
    using namespace hicc_usages::template_function;
    assert(max_of<int>(3, 5) == 5);
    assert(min_of<int>(3, 5) == 3);
    assert(add_of<int>(3, 5) == 8);
    assert(max_of<double>(2.5, 1.5) == 2.5);
    assert(add_of<double>(1.5, 2.5) == 4.0);
    std::cout << "[template_function] C++ test OK" << std::endl;
    return 0;
}
