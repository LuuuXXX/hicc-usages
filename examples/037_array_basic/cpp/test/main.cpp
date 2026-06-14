#include "hicc_usages/array_basic.h"
#include <cassert>
#include <iostream>
int main() {
    using namespace hicc_usages::array_basic;
    FixedArray* a = FixedArray::create();
    a->fill(10);
    assert(a->size() == 8);
    assert(a->get(0) == 10);
    assert(a->sum() == 80);
    a->set(0, 100);
    assert(a->get(0) == 100);
    assert(a->max() == 100);
    FixedArray::free(a);
    std::cout << "[array_basic] C++ test OK" << std::endl;
    return 0;
}
