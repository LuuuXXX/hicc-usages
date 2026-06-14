#include "hicc_usages/explicit_ctor.h"
#include <cassert>
#include <iostream>
int main() {
    using namespace hicc_usages::explicit_ctor;
    Distance* d1 = Distance::create_from_meters(10);
    assert(d1->meters() == 10);
    Distance* d2 = Distance::create_from_feet(10);
    assert(d2->meters() == 3);
    Distance::free(d1); Distance::free(d2);
    std::cout << "[explicit_ctor] C++ test OK" << std::endl;
    return 0;
}
