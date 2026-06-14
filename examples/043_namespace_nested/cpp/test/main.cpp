#include "hicc_usages/namespace_nested.h"
#include <cassert>
#include <iostream>
int main() {
    using namespace hicc_usages::namespace_nested;
    assert(outer::inner::add(3, 4) == 7);
    assert(outer::inner::multiply(3, 4) == 12);
    assert(outer::subtract(10, 4) == 6);
    assert(outer_inner_sum(1, 2, 3) == 6);

    outer::inner::Calculator* c = outer::inner::Calculator::create();
    assert(c->compute(2, 3) == 11);  // 2+3 + 2*3 = 5 + 6 = 11
    outer::inner::Calculator::free(c);

    outer::Helper* h = outer::Helper::create();
    assert(h->doubled(5) == 10);
    outer::Helper::free(h);
    std::cout << "[namespace_nested] C++ test OK" << std::endl;
    return 0;
}
