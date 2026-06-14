#include "hicc_usages/unique_ptr.h"
#include <cassert>
#include <iostream>
int main() {
    using namespace hicc_usages::unique_ptr;
    Owner* o = Owner::create();
    o->acquire(Resource::create(1));
    o->acquire(Resource::create(2));
    o->acquire(Resource::create(3));
    assert(o->count() == 3);
    assert(o->has(2));
    o->release(2);
    assert(!o->has(2));
    assert(o->count() == 2);
    Owner::free(o);
    std::cout << "[unique_ptr] C++ test OK" << std::endl;
    return 0;
}
