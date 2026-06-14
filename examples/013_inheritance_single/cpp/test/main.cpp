#include "hicc_usages/inheritance_single.h"
#include <cassert>
#include <iostream>
#include <cstring>
int main() {
    using namespace hicc_usages::inheritance_single;
    Animal* a = Animal::create("cat");
    assert(std::strcmp(a->get_name(), "cat") == 0);
    assert(a->get_legs() == 4);
    Animal::free(a);
    Dog* d = Dog::create("rex");
    assert(std::strcmp(d->get_name(), "rex") == 0);
    assert(std::strcmp(d->bark(), "Woof!") == 0);
    Dog::free(d);
    std::cout << "[inheritance_single] C++ test OK" << std::endl;
    return 0;
}
