#include "hicc_usages/typeid_rtti.h"
#include <cassert>
#include <cstring>
#include <iostream>
int main() {
    using namespace hicc_usages::typeid_rtti;
    Circle* c = Circle::create(2.0);
    Square* sq = Square::create(3.0);
    assert(std::strcmp(c->type_name(), "Circle") == 0);
    assert(c->id() == 1);
    assert(std::strcmp(sq->type_name(), "Square") == 0);
    assert(sq->id() == 2);
    Shape* s = c;
    assert(is_circle_shape(s));
    assert(!is_square_shape(s));
    Circle::free(c); Square::free(sq);
    std::cout << "[typeid_rtti] C++ test OK" << std::endl;
    return 0;
}
