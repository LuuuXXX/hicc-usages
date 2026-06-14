#include "hicc_usages/shared_ptr.h"
#include <cassert>
#include <iostream>
int main() {
    using namespace hicc_usages::shared_ptr;
    Counter* c1 = Counter::create(10);
    Counter* c2 = Counter::create(20);
    c1->increment();
    assert(c1->value() == 11);

    Registry* r = Registry::create();
    r->add(1, c1);
    r->add(2, c2);
    assert(r->size() == 2);
    assert(r->sum() == 31);
    Registry::free(r);
    Counter::free(c1);
    Counter::free(c2);
    std::cout << "[shared_ptr] C++ test OK" << std::endl;
    return 0;
}
