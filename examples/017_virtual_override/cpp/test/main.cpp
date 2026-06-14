#include "hicc_usages/virtual_override.h"
#include <cassert>
#include <cstring>
#include <iostream>
int main() {
    using namespace hicc_usages::virtual_override;
    Base* b = Base::create();
    assert(std::strcmp(b->name(), "Base") == 0);
    assert(b->compute(5) == 5);
    Derived* d = Derived::create(3);
    assert(std::strcmp(d->name(), "Derived") == 0);
    assert(d->compute(5) == 15);
    Base* polymorphic = d;
    assert(std::strcmp(polymorphic->name(), "Derived") == 0);
    assert(polymorphic->compute(4) == 12);
    Base::free(b); Derived::free(d);
    std::cout << "[virtual_override] C++ test OK" << std::endl;
    return 0;
}
