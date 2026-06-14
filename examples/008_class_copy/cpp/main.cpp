#include "class_copy.h"
#include <iostream>

int main() {
    Box* a = box_new(42);
    Box* b = box_clone(a);
    a->set(100);
    std::cout << "a=" << a->get() << " b=" << b->get() << std::endl;  // a=100 b=42
    box_free(a);
    box_free(b);
    return 0;
}
