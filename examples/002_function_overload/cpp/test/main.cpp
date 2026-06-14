#include "hicc_usages/function_overload.h"
#include <cassert>
#include <iostream>
int main() {
    using namespace hicc_usages::function_overload;
    assert(add(1, 2) == 3);
    assert(add(1.5, 2.5) == 4.0);
    assert(add(1, 2, 3) == 6);
    std::cout << "[function_overload] C++ test OK" << std::endl;
    return 0;
}
