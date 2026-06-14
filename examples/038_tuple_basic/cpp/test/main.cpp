#include "hicc_usages/tuple_basic.h"
#include <cassert>
#include <iostream>
int main() {
    using namespace hicc_usages::tuple_basic;
    Triple* t = Triple::create(10, 2.5, 20);
    assert(t->first() == 10);
    assert(t->second() == 2.5);
    assert(t->third() == 20);
    assert(t->sum_ints() == 30);
    t->set_first(100);
    t->set_second(9.5);
    assert(t->first() == 100);
    assert(t->second() == 9.5);
    assert(t->sum_ints() == 120);
    Triple::free(t);
    std::cout << "[tuple_basic] C++ test OK" << std::endl;
    return 0;
}
