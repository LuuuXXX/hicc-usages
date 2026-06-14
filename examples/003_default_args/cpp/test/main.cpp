#include "hicc_usages/default_args.h"
#include <cassert>
#include <iostream>
int main() {
    using namespace hicc_usages::default_args;
    assert(power(2, 3) == 8);
    assert(power(3) == 9);
    assert(power() == 4);
    std::cout << "[default_args] C++ test OK" << std::endl;
    return 0;
}
