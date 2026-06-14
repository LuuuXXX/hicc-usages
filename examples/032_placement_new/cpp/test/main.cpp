#include "hicc_usages/placement_new.h"
#include <cassert>
#include <iostream>
int main() {
    using namespace hicc_usages::placement_new;
    Buffer* b = Buffer::create(1024);
    assert(b->capacity() == 1024);
    b->place_int(42);
    b->place_int(100);
    assert(b->get_int(0) == 42);
    assert(b->get_int(1) == 100);
    b->place_double(3.14);
    assert(b->get_double(0) == 3.14);
    b->reset();
    assert(b->used() == 0);
    Buffer::free(b);
    std::cout << "[placement_new] C++ test OK" << std::endl;
    return 0;
}
