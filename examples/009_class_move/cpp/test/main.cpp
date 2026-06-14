#include "hicc_usages/class_move.h"
#include <cassert>
#include <iostream>
int main() {
    using namespace hicc_usages::class_move;
    Owner* a = Owner::create(42);
    assert(a->is_valid() && a->get_value() == 42);
    Owner* b = Owner::take_from(a);
    assert(b->is_valid() && b->get_value() == 42);
    assert(!a->is_valid());
    Owner::free(a); Owner::free(b);
    std::cout << "[class_move] C++ test OK" << std::endl;
    return 0;
}
