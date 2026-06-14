#include "hicc_usages/class_volatile.h"
#include <cassert>
#include <iostream>
int main() {
    using namespace hicc_usages::class_volatile;
    Sensor* s = Sensor::create();
    s->update(42);
    assert(s->read() == 42);
    assert(s->read_volatile() == 42);
    Sensor::free(s);
    std::cout << "[class_volatile] C++ test OK" << std::endl;
    return 0;
}
