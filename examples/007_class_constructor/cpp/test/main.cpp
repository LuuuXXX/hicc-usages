#include "hicc_usages/class_constructor.h"
#include <cassert>
#include <iostream>
int main() {
    using namespace hicc_usages::class_constructor;
    Point* p = Point::create(3, 4);
    assert(p->get_x() == 3);
    assert(p->get_y() == 4);
    assert(p->distance_from_origin() == 25);
    Point::free(p);
    std::cout << "[class_constructor] C++ test OK" << std::endl;
    return 0;
}
