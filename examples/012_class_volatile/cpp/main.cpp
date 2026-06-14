#include "class_volatile.h"
#include <iostream>

int main() {
    volatile VCounter* c = vcounter_new();
    c->inc(); c->inc();
    std::cout << "volatile counter = " << c->get() << std::endl;
    vcounter_free(const_cast<VCounter*>(c));
    return 0;
}
