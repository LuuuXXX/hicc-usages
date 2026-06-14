#include "hicc_usages/variadic_functions.h"
#include <cassert>
#include <iostream>
int main() {
    using namespace hicc_usages::variadic_functions;
    assert(sum_2(1,2) == 3);
    assert(sum_3(1,2,3) == 6);
    assert(sum_4(1,2,3,4) == 10);
    int arr[] = {1,2,3,4,5};
    assert(sum_array(arr, 5) == 15);
    std::cout << "[variadic_functions] C++ test OK" << std::endl;
    return 0;
}
