#include "virtual_diamond.h"
#include <iostream>

int main() {
    Console* c = console_new();
    c->write(42);
    std::cout << "priority=" << c->priority()
              << " read=" << c->read() << std::endl;
    console_free(c);
    return 0;
}
