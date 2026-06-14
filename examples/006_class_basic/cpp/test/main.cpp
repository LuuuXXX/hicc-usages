#include "hicc_usages/class_basic.h"
#include <cassert>
#include <iostream>
int main() {
    using namespace hicc_usages::class_basic;
    Counter* c = Counter::create();
    assert(c->get() == 0);
    c->increment(); c->increment();
    assert(c->get() == 2);
    c->decrement();
    assert(c->get() == 1);
    c->reset();
    assert(c->get() == 0);
    Counter::free(c);
    std::cout << "[class_basic] C++ test OK" << std::endl;
    return 0;
}
