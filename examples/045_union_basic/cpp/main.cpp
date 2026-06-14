#include "union_basic.h"
#include <iostream>

int main() {
    ValueBox* b = value_box_new();
    b->set_int(42);
    std::cout << "tag=" << b->tag() << " int=" << b->get_int() << std::endl;
    b->set_float(3.5f);
    std::cout << "tag=" << b->tag() << " float=" << b->get_float() << std::endl;
    value_box_free(b);
    return 0;
}
