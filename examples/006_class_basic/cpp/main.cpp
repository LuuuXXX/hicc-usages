#include "class_basic.h"
#include <iostream>

int main() {
    Counter* c = counter_new();
    std::cout << "init:         " << c->get() << std::endl;
    c->inc(); c->inc(); c->inc();
    std::cout << "after 3x inc: " << c->get() << std::endl;
    c->reset();
    std::cout << "after reset:  " << c->get() << std::endl;
    counter_free(c);
    return 0;
}
