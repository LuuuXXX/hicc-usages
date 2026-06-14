#include "hicc_usages/variadic_template.h"
#include <cassert>
#include <iostream>
int main() {
    using namespace hicc_usages::variadic_template;
    assert(sum_all(3, 5) == 8);
    assert(sum_all(1, 2, 3) == 6);
    assert(count_all(0, 0) == 2);
    assert(count_all(0, 0, 0) == 3);
    assert(max_all(3, 7, 5) == 7);
    std::cout << "[variadic_template] C++ test OK" << std::endl;
    return 0;
}
