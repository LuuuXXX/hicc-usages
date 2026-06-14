#include "hicc_usages/virtual_basic.h"
#include <cassert>
#include <iostream>
#include <cstring>
int main() {
    using namespace hicc_usages::virtual_basic;
    Shape* s = Shape::create();
    assert(std::strcmp(s->name(), "Shape") == 0);
    Shape::free(s);
    Circle* c = Circle::create(2.0);
    assert(std::strcmp(c->name(), "Circle") == 0);
    assert(std::abs(c->area() - 12.566) < 0.01);
    Shape* polymorphic = c;  // 虚函数分派
    assert(std::strcmp(polymorphic->name(), "Circle") == 0);
    Circle::free(c);
    std::cout << "[virtual_basic] C++ test OK" << std::endl;
    return 0;
}
