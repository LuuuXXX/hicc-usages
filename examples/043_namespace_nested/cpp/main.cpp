#include "namespace_nested.h"
#include <iostream>

int main() {
    std::cout << "ns_add(2,3) = " << ns_add(2, 3) << std::endl;
    std::cout << "ns_mul(2,3) = " << ns_mul(2, 3) << std::endl;
    std::cout << "ns_combined(1,2,3) = " << ns_combined(1, 2, 3) << std::endl;
    return 0;
}
