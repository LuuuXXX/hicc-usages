#include "hicc_usages/inheritance_multiple.h"
#include <cassert>
#include <iostream>
#include <cstring>
int main() {
    using namespace hicc_usages::inheritance_multiple;
    Shape* s = Shape::create();
    assert(std::strcmp(s->shape_name(), "Shape") == 0);
    assert(std::strcmp(s->printable_text(), "Shape printable") == 0);
    Shape::free(s);
    std::cout << "[inheritance_multiple] C++ test OK" << std::endl;
    return 0;
}
