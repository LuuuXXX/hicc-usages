#include "hicc_usages/class_static.h"
#include <cassert>
#include <iostream>
int main() {
    using namespace hicc_usages::class_static;
    int initial = Counter::get_instance_count();
    Counter* c1 = Counter::create();
    Counter* c2 = Counter::create();
    assert(Counter::get_instance_count() == initial + 2);
    c1->tick(); c1->tick(); c2->tick();
    assert(c1->get_ticks() == 2);
    assert(c2->get_ticks() == 1);
    Counter::free(c1); Counter::free(c2);
    assert(Counter::get_instance_count() == initial);
    std::cout << "[class_static] C++ test OK" << std::endl;
    return 0;
}
