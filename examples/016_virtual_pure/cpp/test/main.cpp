#include "hicc_usages/virtual_pure.h"
#include <cassert>
#include <iostream>
#include <cstring>
#include <vector>
int main() {
    using namespace hicc_usages::virtual_pure;
    Square* sq = Square::create(3.0);
    Triangle* tr = Triangle::create(4.0, 5.0);
    std::vector<Shape*> shapes = {sq, tr};
    assert(std::strcmp(shapes[0]->name(), "Square") == 0);
    assert(std::abs(shapes[0]->area() - 9.0) < 0.01);
    assert(std::strcmp(shapes[1]->name(), "Triangle") == 0);
    assert(std::abs(shapes[1]->area() - 10.0) < 0.01);
    Square::free(sq); Triangle::free(tr);
    std::cout << "[virtual_pure] C++ test OK" << std::endl;
    return 0;
}
